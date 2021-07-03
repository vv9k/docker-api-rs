//! Transports for communicating with the docker daemon

use crate::{Error, Result};
use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::{self, Stream},
    StreamExt, TryFutureExt,
};
use hyper::{
    body::Bytes,
    client::{Client, HttpConnector},
    header, Body, Method, Request, Response, StatusCode,
};
#[cfg(feature = "tls")]
use hyper_openssl::HttpsConnector;
#[cfg(unix)]
use hyperlocal::UnixConnector;
#[cfg(unix)]
use hyperlocal::Uri as DomainUri;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use std::{
    io,
    iter::IntoIterator,
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug, Default, Clone)]
/// Helper structure used as a container for HTTP headers passed to a request
pub(crate) struct Headers(Vec<(&'static str, String)>);

impl Headers {
    /// Shortcut for when one does not want headers in a request
    pub fn none() -> Option<Headers> {
        None
    }

    /// Adds a single key=value header pair
    pub fn add<V>(&mut self, key: &'static str, val: V)
    where
        V: Into<String>,
    {
        self.0.push((key, val.into()))
    }

    /// Constructs an instance of Headers with initial pair, usually used when there is only
    /// a need for one header.
    pub fn single<V>(key: &'static str, val: V) -> Self
    where
        V: Into<String>,
    {
        let mut h = Self::default();
        h.add(key, val);
        h
    }
}

impl IntoIterator for Headers {
    type Item = (&'static str, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Types of payload that can be sent
pub(crate) enum Payload<B: Into<Body>> {
    None,
    #[allow(dead_code)]
    Text(B),
    Json(B),
    XTar(B),
    Tar(B),
}

impl Payload<Body> {
    /// Creates an empty payload
    pub fn empty() -> Self {
        Payload::None
    }
}

impl<B: Into<Body>> Payload<B> {
    /// Extracts the inner body if there is one and returns it
    pub fn into_inner(self) -> Option<B> {
        match self {
            Self::None => None,
            Self::Text(b) => Some(b),
            Self::Json(b) => Some(b),
            Self::XTar(b) => Some(b),
            Self::Tar(b) => Some(b),
        }
    }

    /// Returns the mime type of this payload
    pub fn mime_type(&self) -> Option<mime::Mime> {
        match &self {
            Self::None => None,
            Self::Text(_) => None,
            Self::Json(_) => Some(mime::APPLICATION_JSON),
            Self::XTar(_) => Some("application/x-tar".parse().expect("parsed mime")),
            Self::Tar(_) => Some("application/tar".parse().expect("parsed mime")),
        }
    }

    /// Checks if there is no payload
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Transports are types which define the means of communication
/// with the docker daemon
#[derive(Clone, Debug)]
pub enum Transport {
    /// A network tcp interface
    Tcp {
        client: Client<HttpConnector>,
        host: String,
    },
    /// TCP/TLS
    #[cfg(feature = "tls")]
    EncryptedTcp {
        client: Client<HttpsConnector<HttpConnector>>,
        host: String,
    },
    /// A Unix domain socket
    #[cfg(unix)]
    Unix {
        client: Client<UnixConnector>,
        path: String,
    },
}

impl Transport {
    pub fn remote_addr(&self) -> &str {
        match &self {
            Self::Tcp { ref host, .. } => host.as_str(),
            #[cfg(feature = "tls")]
            Self::EncryptedTcp { ref host, .. } => host.as_str(),
            Self::Unix { ref path, .. } => path.as_str(),
        }
    }

    pub(crate) async fn request<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<Response<Body>>
    where
        B: Into<Body>,
    {
        let ep = endpoint.as_ref();
        // As noted in [Versioning](https://docs.docker.com/engine/api/v1.41/#section/Versioning), all requests
        // should be prefixed with the API version as the ones without will stop being supported in future releases
        let req = self.build_request(
            method,
            &format!(
                "{}{}{}",
                crate::VERSION,
                if !ep.starts_with('/') { "/" } else { "" },
                ep,
            ),
            body,
            headers,
            Request::builder(),
        )?;

        self.send_request(req).await
    }

    pub(crate) async fn request_string<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<String>
    where
        B: Into<Body>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;
        let bytes = hyper::body::to_bytes(body).await?;
        String::from_utf8(bytes.to_vec()).map_err(Error::from)
    }

    pub(crate) fn stream_chunks<'transport, B>(
        &'transport self,
        method: Method,
        endpoint: impl AsRef<str> + 'transport,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'transport
    where
        B: Into<Body> + 'transport,
    {
        self.get_chunk_stream(method, endpoint, body, headers)
            .try_flatten_stream()
    }

    pub(crate) fn stream_json_chunks<'transport, B>(
        &'transport self,
        method: Method,
        endpoint: impl AsRef<str> + 'transport,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'transport
    where
        B: Into<Body> + 'transport,
    {
        self.get_json_chunk_stream(method, endpoint, body, headers)
            .try_flatten_stream()
    }

    pub(crate) async fn stream_upgrade<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
    ) -> Result<impl AsyncRead + AsyncWrite>
    where
        B: Into<Body>,
    {
        let tokio_multiplexer = self.stream_upgrade_tokio(method, endpoint, body).await?;

        Ok(Compat { tokio_multiplexer })
    }

    async fn get_body<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<Body>
    where
        B: Into<Body>,
    {
        let response = self.request(method, endpoint, body, headers).await?;

        let status = response.status();

        match status {
            // Success case: pass on the response
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::SWITCHING_PROTOCOLS
            | StatusCode::NO_CONTENT => Ok(response.into_body()),
            _ => {
                let bytes = hyper::body::to_bytes(response.into_body()).await?;
                let message_body = String::from_utf8(bytes.to_vec())?;

                Err(Error::Fault {
                    code: status,
                    message: Self::get_error_message(&message_body).unwrap_or_else(|| {
                        status
                            .canonical_reason()
                            .unwrap_or("unknown error code")
                            .to_owned()
                    }),
                })
            }
        }
    }

    async fn get_chunk_stream<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<impl Stream<Item = Result<Bytes>>>
    where
        B: Into<Body>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;

        Ok(stream_body(body))
    }

    async fn get_json_chunk_stream<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<impl Stream<Item = Result<Bytes>>>
    where
        B: Into<Body>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;

        Ok(stream_json_body(body))
    }

    /// Builds an HTTP request.
    fn build_request<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
        builder: hyper::http::request::Builder,
    ) -> Result<Request<Body>>
    where
        B: Into<Body>,
    {
        let req = match self {
            Transport::Tcp { host, .. } => {
                builder
                    .method(method)
                    .uri(&format!("{}{}", host, endpoint.as_ref()))
            }
            #[cfg(feature = "tls")]
            Transport::EncryptedTcp { host, .. } => {
                builder
                    .method(method)
                    .uri(&format!("{}{}", host, endpoint.as_ref()))
            }
            #[cfg(unix)]
            Transport::Unix { path, .. } => {
                let uri = DomainUri::new(&path, endpoint.as_ref());
                builder.method(method).uri(uri)
            }
        };
        let mut req = req.header(header::HOST, "");

        if let Some(h) = headers {
            for (k, v) in h.into_iter() {
                req = req.header(k, v);
            }
        }

        // early return
        if body.is_none() {
            return Ok(req.body(Body::empty())?);
        }

        let mime = body.mime_type();
        if let Some(c) = mime {
            req = req.header(header::CONTENT_TYPE, &c.to_string()[..]);
        }

        // it's ok to unwrap, we check that the body is not none
        Ok(req.body(body.into_inner().unwrap().into())?)
    }

    /// Send the given request to the docker daemon and return a Future of the response.
    async fn send_request(&self, req: Request<Body>) -> Result<Response<Body>> {
        match self {
            Transport::Tcp { ref client, .. } => Ok(client.request(req).await?),
            #[cfg(feature = "tls")]
            Transport::EncryptedTcp { ref client, .. } => Ok(client.request(req).await?),
            #[cfg(unix)]
            Transport::Unix { ref client, .. } => Ok(client.request(req).await?),
        }
    }

    /// Makes an HTTP request, upgrading the connection to a TCP
    /// stream on success.
    ///
    /// This method can be used for operations such as viewing
    /// docker container logs interactively.
    async fn stream_upgrade_tokio<B>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
    ) -> Result<hyper::upgrade::Upgraded>
    where
        B: Into<Body>,
    {
        let req = self.build_request(
            method,
            endpoint,
            body,
            Headers::none(),
            Request::builder()
                .header(header::CONNECTION, "Upgrade")
                .header(header::UPGRADE, "tcp"),
        )?;

        let response = self.send_request(req).await?;

        match response.status() {
            StatusCode::SWITCHING_PROTOCOLS => Ok(hyper::upgrade::on(response).await?),
            _ => Err(Error::ConnectionNotUpgraded),
        }
    }

    /// Extract the error message content from an HTTP response that
    /// contains a Docker JSON error structure.
    fn get_error_message(body: &str) -> Option<String> {
        serde_json::from_str::<ErrorResponse>(body)
            .map(|e| e.message)
            .ok()
    }
}

#[pin_project]
struct Compat<S> {
    #[pin]
    tokio_multiplexer: S,
}

impl<S> AsyncRead for Compat<S>
where
    S: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut readbuf = tokio::io::ReadBuf::new(buf);
        match self.project().tokio_multiplexer.poll_read(cx, &mut readbuf) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(())) => Poll::Ready(Ok(readbuf.filled().len())),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
        }
    }
}

impl<S> AsyncWrite for Compat<S>
where
    S: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project().tokio_multiplexer.poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().tokio_multiplexer.poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().tokio_multiplexer.poll_shutdown(cx)
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

fn stream_body(body: Body) -> impl Stream<Item = Result<Bytes>> {
    async fn unfold(mut body: Body) -> Option<(Result<Bytes>, Body)> {
        let chunk_result = body.next().await?.map_err(Error::from);
        Some((chunk_result, body))
    }

    stream::unfold(body, unfold)
}

static JSON_WHITESPACE: &[u8] = b"\r\n";

fn stream_json_body(body: Body) -> impl Stream<Item = Result<Bytes>> {
    async fn unfold(mut body: Body) -> Option<(Result<Bytes>, Body)> {
        let mut chunk = Vec::new();
        while let Some(chnk) = body.next().await {
            match chnk {
                Ok(chnk) => {
                    chunk.extend(chnk.to_vec());
                    if chnk.ends_with(JSON_WHITESPACE) {
                        break;
                    }
                }
                Err(e) => {
                    return Some((Err(Error::from(e)), body));
                }
            }
        }

        Some((Ok(Bytes::from(chunk)), body))
    }

    stream::unfold(body, unfold)
}
