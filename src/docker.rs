//! Main entrypoint for interacting with the Docker API.
//!
//! API Reference: <https://docs.docker.com/engine/api/v1.41/>
use crate::{
    conn::{get_http_connector, Headers, Payload, Transport},
    errors::{Error, Result},
    Containers, Images, Networks, Volumes, LATEST_API_VERSION,
};

#[cfg(feature = "swarm")]
use crate::{Configs, Nodes, Plugins, Secrets, Services, Swarm, Tasks};

#[cfg(feature = "tls")]
use crate::conn::get_https_connector;
#[cfg(unix)]
use crate::conn::get_unix_connector;

use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
    TryStreamExt,
};
use hyper::{body::Bytes, Body, Client, Method, Response};
use log::trace;
use serde::de::DeserializeOwned;

use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Version {
    major: usize,
    minor: usize,
}

impl Version {
    pub const fn new(major: usize, minor: usize) -> Self {
        Self { major, minor }
    }
    fn make_endpoint(&self, ep: impl AsRef<str>) -> String {
        // As noted in [Versioning](https://docs.docker.com/engine/api/v1.41/#section/Versioning), all requests
        // should be prefixed with the API version as the ones without will stop being supported in future releases

        let ep = ep.as_ref();
        format!(
            "/v{}{}{}",
            self,
            if !ep.starts_with('/') { "/" } else { "" },
            ep
        )
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl From<(usize, usize)> for Version {
    fn from(v: (usize, usize)) -> Self {
        Version {
            major: v.0,
            minor: v.1,
        }
    }
}

impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut elems = s.split('.');
        let major = if let Some(maj) = elems.next() {
            match maj.parse::<usize>() {
                Ok(maj) => maj,
                Err(e) => return Err(Error::MalformedVersion(e.to_string())),
            }
        } else {
            return Err(Error::MalformedVersion(s.to_string()));
        };
        let minor = if let Some(min) = elems.next() {
            match min.parse::<usize>() {
                Ok(min) => min,
                Err(e) => return Err(Error::MalformedVersion(e.to_string())),
            }
        } else {
            return Err(Error::MalformedVersion(
                "expected minor version".to_string(),
            ));
        };
        if elems.next().is_some() {
            return Err(Error::MalformedVersion(
                "unexpected extra tokens".to_string(),
            ));
        }
        Ok(Self { major, minor })
    }
}

/// Entrypoint interface for communicating with docker daemon
#[derive(Debug, Clone)]
pub struct Docker {
    version: Version,
    transport: Transport,
}

impl Docker {
    /// Creates a new Docker instance by automatically choosing appropriate connection type based
    /// on provided `uri`.
    ///
    /// Supported schemes are:
    ///  - `unix://` only works when build target is `unix`, otherwise returns an Error
    ///  - `tcp://`
    ///  - `http://`
    ///
    ///  To create a Docker instance utilizing TLS use explicit [Docker::tls](Docker::tls)
    ///  constructor (this requires `tls` feature enabled).
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Docker::new_versioned`](Docker::new_versioned).
    pub fn new<U>(uri: U) -> Result<Docker>
    where
        U: AsRef<str>,
    {
        Self::new_versioned(uri, LATEST_API_VERSION)
    }

    /// Same as [`Docker::new`](Docker::new) but the API version can be explicitly specified.
    pub fn new_versioned<U>(uri: U, version: impl Into<Version>) -> Result<Docker>
    where
        U: AsRef<str>,
    {
        let version = version.into();
        let uri = uri.as_ref();
        let mut it = uri.split("://");

        match it.next() {
            #[cfg(unix)]
            Some("unix") => {
                if let Some(path) = it.next() {
                    Ok(Docker::unix_versioned(path, version))
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            #[cfg(not(unix))]
            Some("unix") => Err(Error::UnsupportedScheme("unix".to_string())),
            Some("tcp") | Some("http") => {
                if let Some(host) = it.next() {
                    Docker::tcp_versioned(host, version)
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            Some(scheme) => Err(Error::UnsupportedScheme(scheme.to_string())),
            None => unreachable!(), // This is never possible because calling split on an empty string
                                    // always returns at least one element
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Creates a new docker instance for a docker host listening on a given Unix socket.
    ///
    /// `socket_path` is the part of URI that comes after the `unix://`. For example a URI `unix:///run/docker.sock` has a
    /// `socket_path` == "/run/docker.sock".
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Docker::unix_versioned`](Docker::unix_versioned).
    pub fn unix<P>(socket_path: P) -> Docker
    where
        P: AsRef<Path>,
    {
        Self::unix_versioned(socket_path, LATEST_API_VERSION)
    }

    /// Same as [`Docker::unix`](Docker::unix) but the API version can be explicitly specified.
    pub fn unix_versioned<P>(socket_path: P, version: impl Into<Version>) -> Docker
    where
        P: AsRef<Path>,
    {
        Docker {
            version: version.into(),
            transport: Transport::Unix {
                client: Client::builder()
                    .pool_max_idle_per_host(0)
                    .build(get_unix_connector()),
                path: socket_path.as_ref().to_path_buf(),
            },
        }
    }

    #[cfg(feature = "tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
    /// Creates a new docker instance for a docker host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` or `https://` schemes,
    /// also known as authority part.
    ///
    /// `cert_path` specifies the base path in the filesystem containing a certificate (`cert.pem`)
    /// and a key (`key.pem`) that will be used by the client. If verify is `true` a CA file will be
    /// added (`ca.pem`) to the connector.
    ///
    /// Returns an error if the provided host will fail to parse as URL or reading the certificate
    /// files will fail.
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Docker::tls_versioned`](Docker::tls_versioned).
    pub fn tls<H, P>(host: H, cert_path: P, verify: bool) -> Result<Docker>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Self::tls_versioned(host, LATEST_API_VERSION, cert_path, verify)
    }

    #[cfg(feature = "tls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
    /// Same as [`Docker::tls`](Docker::tls) but the API version can be explicitly specified.
    pub fn tls_versioned<H, P>(
        host: H,
        version: impl Into<Version>,
        cert_path: P,
        verify: bool,
    ) -> Result<Docker>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Ok(Docker {
            version: version.into(),
            transport: Transport::EncryptedTcp {
                client: Client::builder().build(get_https_connector(cert_path.as_ref(), verify)?),
                host: url::Url::parse(&format!("https://{}", host.as_ref()))
                    .map_err(Error::InvalidUrl)?,
            },
        })
    }

    /// Creates a new docker instance for a docker host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` schemes, also known as
    /// authority part.
    ///
    /// TLS is supported with feature `tls` enabled through [Docker::tls](Docker::tls) constructor.
    ///
    /// Returns an error if the provided host will fail to parse as URL.
    ///  
    ///  Uses [`LATEST_API_VERSION`](crate::LATEST_API_VERSION), to use a specific version see
    ///  [`Docker::tcp_versioned`](Docker::tcp_versioned).
    pub fn tcp<H>(host: H) -> Result<Docker>
    where
        H: AsRef<str>,
    {
        Self::tcp_versioned(host, LATEST_API_VERSION)
    }

    /// Same as [`Docker::tcp`](Docker::tcp) but the API version can be explicitly specified.
    pub fn tcp_versioned<H>(host: H, version: impl Into<Version>) -> Result<Docker>
    where
        H: AsRef<str>,
    {
        Ok(Docker {
            version: version.into(),
            transport: Transport::Tcp {
                client: Client::builder().build(get_http_connector()),
                host: url::Url::parse(&format!("tcp://{}", host.as_ref()))
                    .map_err(Error::InvalidUrl)?,
            },
        })
    }

    /// Exports an interface for interacting with Docker images
    pub fn images(&'_ self) -> Images<'_> {
        Images::new(self)
    }

    /// Exports an interface for interacting with Docker containers
    pub fn containers(&'_ self) -> Containers<'_> {
        Containers::new(self)
    }

    /// Exports an interface for interacting with Docker networks
    pub fn networks(&'_ self) -> Networks<'_> {
        Networks::new(self)
    }

    /// Exports an interface for interacting with Docker volumes
    pub fn volumes(&'_ self) -> Volumes<'_> {
        Volumes::new(self)
    }

    //####################################################################################################
    //
    // Utility functions to make requests
    //
    //####################################################################################################

    pub(crate) async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(
                Method::GET,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(
                Method::GET,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn post<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await
    }

    pub(crate) async fn post_headers<B>(
        &self,
        endpoint: &str,
        body: Payload<B>,
        headers: Headers,
    ) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                Some(headers),
            )
            .await
    }

    pub(crate) async fn put<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(
                Method::PUT,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await
    }

    pub(crate) async fn post_json<B, T>(
        &self,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        let raw_string = self
            .transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    #[allow(dead_code)]
    pub(crate) async fn post_json_headers<'a, B, T>(
        &self,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        let raw_string = self
            .transport
            .request_string(
                Method::POST,
                self.version.make_endpoint(endpoint),
                body,
                headers,
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn delete(&self, endpoint: &str) -> Result<String> {
        self.transport
            .request_string(
                Method::DELETE,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }

    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(
                Method::DELETE,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn head_response(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(
                Method::HEAD,
                self.version.make_endpoint(endpoint),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }

    /// Send a streaming post request.
    ///
    /// Use stream_post_into_values if the endpoint returns JSON values
    pub(crate) fn stream_post<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'a
    where
        B: Into<Body> + 'a,
    {
        self.transport.stream_chunks(
            Method::POST,
            self.version.make_endpoint(endpoint),
            body,
            headers,
        )
    }

    /// Send a streaming post request.
    fn stream_json_post<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'a
    where
        B: Into<Body> + 'a,
    {
        self.transport.stream_json_chunks(
            Method::POST,
            self.version.make_endpoint(endpoint),
            body,
            headers,
        )
    }

    /// Send a streaming post request that returns a stream of JSON values
    ///
    /// When a received chunk does not contain a full JSON reads more chunks from the stream
    pub(crate) fn stream_post_into<'a, B, T>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<T>> + 'a
    where
        B: Into<Body> + 'a,
        T: DeserializeOwned,
    {
        self.stream_json_post(endpoint, body, headers)
            .and_then(|chunk| async move {
                trace!("got chunk {:?}", chunk);
                let stream = futures_util::stream::iter(
                    serde_json::Deserializer::from_slice(&chunk)
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
                .map_err(Error::from);

                Ok(stream)
            })
            .try_flatten()
    }

    pub(crate) fn stream_get<'a>(
        &'a self,
        endpoint: impl AsRef<str> + Unpin + 'a,
    ) -> impl Stream<Item = Result<Bytes>> + 'a {
        self.transport.stream_chunks(
            Method::GET,
            self.version.make_endpoint(endpoint),
            Payload::empty(),
            Headers::none(),
        )
    }

    pub(crate) async fn stream_post_upgrade<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
    ) -> Result<impl AsyncRead + AsyncWrite + 'a>
    where
        B: Into<Body> + 'a,
    {
        self.transport
            .stream_upgrade(Method::POST, self.version.make_endpoint(endpoint), body)
            .await
    }
}

#[cfg(feature = "swarm")]
impl Docker {
    /// Exports an interface for interacting with Docker services.
    pub fn services(&'_ self) -> Services<'_> {
        Services::new(self)
    }

    /// Exports an interface for interacting with Docker configs.
    pub fn configs(&'_ self) -> Configs<'_> {
        Configs::new(self)
    }

    /// Exports an interface for interacting with Docker tasks.
    pub fn tasks(&'_ self) -> Tasks<'_> {
        Tasks::new(self)
    }

    /// Exports an interface for interacting with Docker secrets.
    pub fn secrets(&'_ self) -> Secrets<'_> {
        Secrets::new(self)
    }

    /// Exports an interface for interacting with Docker swarm.
    pub fn swarm(&'_ self) -> Swarm<'_> {
        Swarm::new(self)
    }

    /// Exports an interface for interacting with Docker nodes.
    pub fn nodes(&'_ self) -> Nodes<'_> {
        Nodes::new(self)
    }

    /// Exports an interface for interacting with Docker plugins.
    pub fn plugins(&'_ self) -> Plugins<'_> {
        Plugins::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{Docker, Error};
    #[test]
    fn creates_correct_docker() {
        let d = Docker::new("tcp://127.0.0.1:80");
        d.unwrap();
        let d = Docker::new("http://127.0.0.1:80");
        d.unwrap();

        #[cfg(unix)]
        let d = Docker::new("unix://127.0.0.1:80");
        d.unwrap();

        #[cfg(not(unix))]
        {
            let d = Docker::new("unix://127.0.0.1:80");
            assert!(d.is_err());
            match d.unwrap_err() {
                Error::UnsupportedScheme(scheme) if &scheme == "unix" => {}
                e => panic!(r#"Expected Error::UnsupportedScheme("unix"), got {}"#, e),
            }
        }

        let d = Docker::new("rand://127.0.0.1:80");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if &scheme == "rand" => {}
            e => panic!(r#"Expected Error::UnsupportedScheme("rand"), got {}"#, e),
        }

        let d = Docker::new("invalid_uri");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if &scheme == "invalid_uri" => {}
            e => panic!(
                r#"Expected Error::UnsupportedScheme("invalid_uri"), got {}"#,
                e
            ),
        }
        let d = Docker::new("");
        match d.unwrap_err() {
            Error::UnsupportedScheme(scheme) if scheme.is_empty() => {}
            e => panic!(r#"Expected Error::UnsupportedScheme(""), got {}"#, e),
        }
    }
}
