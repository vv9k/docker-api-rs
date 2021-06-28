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
    header, Body, Method, Request, StatusCode,
};
#[cfg(feature = "tls")]
use hyper_openssl::HttpsConnector;
#[cfg(feature = "unix-socket")]
use hyperlocal::UnixConnector;
#[cfg(feature = "unix-socket")]
use hyperlocal::Uri as DomainUri;
use pin_project::pin_project;
use serde::{Deserialize, Serialize};
use std::{
    fmt, io, iter,
    pin::Pin,
    task::{Context, Poll},
};

static JSON_WHITESPACE: &[u8] = b"\r\n";

pub(crate) type Headers = Option<Vec<(&'static str, String)>>;
// pub(crate) type Payload = Option<(Body, Mime)>;

pub enum Payload<B: Into<Body>> {
    None,
    Text(B),
    Json(B),
    XTar(B),
    Tar(B),
}

impl<B: Into<Body>> Payload<B> {
    pub fn to_inner(self) -> Option<B> {
        match self {
            Self::None => None,
            Self::Text(b) => Some(b),
            Self::Json(b) => Some(b),
            Self::XTar(b) => Some(b),
            Self::Tar(b) => Some(b),
        }
    }

    pub fn mime_type(&self) -> Option<mime::Mime> {
        match &self {
            Self::None => None,
            Self::Text(_) => None,
            Self::Json(_) => Some(mime::APPLICATION_JSON),
            Self::XTar(_) => Some("application/x-tar".parse().unwrap()),
            Self::Tar(_) => Some("application/tar".parse().unwrap()),
        }
    }

    pub fn is_none(&self) -> bool {
        match &self {
            Self::None => true,
            _ => false,
        }
    }
}

/// Transports are types which define the means of communication
/// with the docker daemon
#[derive(Clone)]
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
    #[cfg(feature = "unix-socket")]
    Unix {
        client: Client<UnixConnector>,
        path: String,
    },
}

impl fmt::Debug for Transport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Transport::Tcp { ref host, .. } => write!(f, "Tcp({})", host),
            #[cfg(feature = "tls")]
            Transport::EncryptedTcp { ref host, .. } => write!(f, "EncryptedTcp({})", host),
            #[cfg(feature = "unix-socket")]
            Transport::Unix { ref path, .. } => write!(f, "Unix({})", path),
        }
    }
}

impl Transport {
    /// Make a request and return the whole response in a `String`
    pub async fn request<B, H>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<H>,
    ) -> Result<String>
    where
        B: Into<Body>,
        H: IntoIterator<Item = (&'static str, String)>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;
        let bytes = hyper::body::to_bytes(body).await?;
        let string = String::from_utf8(bytes.to_vec())?;

        Ok(string)
    }

    async fn get_body<B, H>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<H>,
    ) -> Result<Body>
    where
        B: Into<Body>,
        H: IntoIterator<Item = (&'static str, String)>,
    {
        let req = self
            .build_request(method, endpoint, body, headers, Request::builder())
            .expect("Failed to build request!");

        let response = self.send_request(req).await?;

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

    async fn get_chunk_stream<B, H>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<H>,
    ) -> Result<impl Stream<Item = Result<Bytes>>>
    where
        B: Into<Body>,
        H: IntoIterator<Item = (&'static str, String)>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;

        Ok(stream_body(body))
    }

    pub fn stream_chunks<'stream, B, H>(
        &'stream self,
        method: Method,
        endpoint: impl AsRef<str> + 'stream,
        body: Payload<B>,
        headers: Option<H>,
    ) -> impl Stream<Item = Result<Bytes>> + 'stream
    where
        B: Into<Body> + 'stream,
        H: IntoIterator<Item = (&'static str, String)> + 'stream,
    {
        self.get_chunk_stream(method, endpoint, body, headers)
            .try_flatten_stream()
    }

    async fn get_json_chunk_stream<B, H>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<H>,
    ) -> Result<impl Stream<Item = Result<Bytes>>>
    where
        B: Into<Body>,
        H: IntoIterator<Item = (&'static str, String)>,
    {
        let body = self.get_body(method, endpoint, body, headers).await?;

        Ok(stream_json_body(body))
    }

    pub fn stream_json_chunks<'stream, B, H>(
        &'stream self,
        method: Method,
        endpoint: impl AsRef<str> + 'stream,
        body: Payload<B>,
        headers: Option<H>,
    ) -> impl Stream<Item = Result<Bytes>> + 'stream
    where
        B: Into<Body> + 'stream,
        H: IntoIterator<Item = (&'static str, String)> + 'stream,
    {
        self.get_json_chunk_stream(method, endpoint, body, headers)
            .try_flatten_stream()
    }

    /// Builds an HTTP request.
    fn build_request<B, H>(
        &self,
        method: Method,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<H>,
        builder: hyper::http::request::Builder,
    ) -> Result<Request<Body>>
    where
        B: Into<Body>,
        H: IntoIterator<Item = (&'static str, String)>,
    {
        let req = match *self {
            Transport::Tcp { ref host, .. } => {
                builder
                    .method(method)
                    .uri(&format!("{}{}", host, endpoint.as_ref()))
            }
            #[cfg(feature = "tls")]
            Transport::EncryptedTcp { ref host, .. } => {
                builder
                    .method(method)
                    .uri(&format!("{}{}", host, endpoint.as_ref()))
            }
            #[cfg(feature = "unix-socket")]
            Transport::Unix { ref path, .. } => {
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

        if body.is_none() {
            return Ok(req.body(Body::empty())?);
        }

        let mime = body.mime_type();

        if let Some(c) = mime {
            req = req.header(header::CONTENT_TYPE, &c.to_string()[..]);
        }

        Ok(req.body(body.to_inner().unwrap().into())?)
    }

    /// Send the given request to the docker daemon and return a Future of the response.
    async fn send_request(&self, req: Request<hyper::Body>) -> Result<hyper::Response<Body>> {
        match self {
            Transport::Tcp { ref client, .. } => Ok(client.request(req).await?),
            #[cfg(feature = "tls")]
            Transport::EncryptedTcp { ref client, .. } => Ok(client.request(req).await?),
            #[cfg(feature = "unix-socket")]
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
        let req = self
            .build_request(
                method,
                endpoint,
                body,
                None::<iter::Empty<_>>,
                Request::builder()
                    .header(header::CONNECTION, "Upgrade")
                    .header(header::UPGRADE, "tcp"),
            )
            .expect("Failed to build request!");

        let response = self.send_request(req).await?;

        match response.status() {
            StatusCode::SWITCHING_PROTOCOLS => Ok(hyper::upgrade::on(response).await?),
            _ => Err(Error::ConnectionNotUpgraded),
        }
    }

    pub async fn stream_upgrade<B>(
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
