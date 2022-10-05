//! Main entrypoint for interacting with the Docker API.
//!
//! API Reference: <https://docs.docker.com/engine/api/v1.41/>
use crate::{
    conn::{get_http_connector, Headers, Payload, Transport},
    errors::{Error, Result},
    ApiVersion, Containers, Images, Networks, Volumes, LATEST_API_VERSION,
};
use containers_api::conn::RequestClient;

#[cfg(feature = "swarm")]
use crate::{Configs, Nodes, Plugins, Secrets, Services, Swarm, Tasks};

#[cfg(feature = "tls")]
use crate::conn::get_https_connector;
#[cfg(unix)]
use crate::conn::get_unix_connector;

use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
};
use hyper::{body::Bytes, Body, Client, Response};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

/// Entrypoint interface for communicating with docker daemon
#[derive(Debug, Clone)]
pub struct Docker {
    version: ApiVersion,
    client: RequestClient<Error>,
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
    pub fn new_versioned<U>(uri: U, version: impl Into<ApiVersion>) -> Result<Docker>
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

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    /// Same as [`Docker::unix`](Docker::unix) but the API version can be explicitly specified.
    pub fn unix_versioned<P>(socket_path: P, version: impl Into<ApiVersion>) -> Docker
    where
        P: AsRef<Path>,
    {
        Docker {
            version: version.into(),
            client: RequestClient::new(
                Transport::Unix {
                    client: Client::builder()
                        .pool_max_idle_per_host(0)
                        .build(get_unix_connector()),
                    path: socket_path.as_ref().to_path_buf(),
                },
                Box::new(validate_response),
            ),
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
        version: impl Into<ApiVersion>,
        cert_path: P,
        verify: bool,
    ) -> Result<Docker>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Ok(Docker {
            version: version.into(),
            client: RequestClient::new(
                Transport::EncryptedTcp {
                    client: Client::builder()
                        .build(get_https_connector(cert_path.as_ref(), verify)?),
                    host: url::Url::parse(&format!("https://{}", host.as_ref()))
                        .map_err(Error::InvalidUrl)?,
                },
                Box::new(validate_response),
            ),
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
    pub fn tcp_versioned<H>(host: H, version: impl Into<ApiVersion>) -> Result<Docker>
    where
        H: AsRef<str>,
    {
        Ok(Docker {
            version: version.into(),
            client: RequestClient::new(
                Transport::Tcp {
                    client: Client::builder().build(get_http_connector()),
                    host: url::Url::parse(&format!("tcp://{}", host.as_ref()))
                        .map_err(Error::InvalidUrl)?,
                },
                Box::new(validate_response),
            ),
        })
    }

    /// Exports an interface for interacting with Docker images
    pub fn images(&'_ self) -> Images {
        Images::new(self.clone())
    }

    /// Exports an interface for interacting with Docker containers
    pub fn containers(&'_ self) -> Containers {
        Containers::new(self.clone())
    }

    /// Exports an interface for interacting with Docker networks
    pub fn networks(&'_ self) -> Networks {
        Networks::new(self.clone())
    }

    /// Exports an interface for interacting with Docker volumes
    pub fn volumes(&'_ self) -> Volumes {
        Volumes::new(self.clone())
    }

    /// Verifies the API version returned by the server and adjusts the version used by this client
    /// in future requests.
    pub async fn adjust_api_version(&mut self) -> Result<()> {
        let server_version: ApiVersion = self.version().await.and_then(|v| {
            v.api_version
                .unwrap_or_default()
                .parse::<ApiVersion>()
                .map_err(Error::MalformedVersion)
        })?;

        if server_version <= self.version {
            self.version = server_version;
        }

        Ok(())
    }

    //####################################################################################################
    //
    // Utility functions to make requests
    //
    //####################################################################################################

    pub(crate) async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        self.client.get(self.version.make_endpoint(endpoint)).await
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.client
            .get_json(self.version.make_endpoint(endpoint))
            .await
    }

    #[allow(dead_code)]
    pub(crate) async fn post<B>(
        &self,
        endpoint: &str,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<Response<Body>>
    where
        B: Into<Body>,
    {
        self.client
            .post(self.version.make_endpoint(endpoint), body, headers)
            .await
    }

    pub(crate) async fn post_string<B>(
        &self,
        endpoint: &str,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<String>
    where
        B: Into<Body>,
    {
        self.client
            .post_string(self.version.make_endpoint(endpoint), body, headers)
            .await
    }

    pub(crate) async fn post_json<B, T>(
        &self,
        endpoint: impl AsRef<str>,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Into<Body>,
    {
        self.client
            .post_json(self.version.make_endpoint(endpoint), body, headers)
            .await
    }

    pub(crate) async fn put<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.client
            .put_string(self.version.make_endpoint(endpoint), body)
            .await
    }

    pub(crate) async fn delete(&self, endpoint: &str) -> Result<String> {
        self.client
            .delete_string(self.version.make_endpoint(endpoint))
            .await
    }

    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        self.client
            .delete_json(self.version.make_endpoint(endpoint))
            .await
    }

    pub(crate) async fn head(&self, endpoint: &str) -> Result<Response<Body>> {
        self.client.head(self.version.make_endpoint(endpoint)).await
    }

    /// Send a streaming post request.
    ///
    /// Use stream_post_into_values if the endpoint returns JSON values
    pub(crate) fn post_stream<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<Bytes>> + 'a
    where
        B: Into<Body> + 'a,
    {
        self.client
            .post_stream(self.version.make_endpoint(endpoint), body, headers)
    }

    /// Send a streaming post request that returns a stream of JSON values
    ///
    /// When a received chunk does not contain a full JSON reads more chunks from the stream
    pub(crate) fn post_into_stream<'a, B, T>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
        headers: Option<Headers>,
    ) -> impl Stream<Item = Result<T>> + 'a
    where
        B: Into<Body> + 'a,
        T: DeserializeOwned + 'a,
    {
        self.client
            .post_into_stream(self.version.make_endpoint(endpoint), body, headers)
    }

    pub(crate) fn get_stream<'a>(
        &'a self,
        endpoint: impl AsRef<str> + Unpin + 'a,
    ) -> impl Stream<Item = Result<Bytes>> + 'a {
        self.client.get_stream(self.version.make_endpoint(endpoint))
    }

    pub(crate) async fn post_upgrade_stream<'a, B>(
        &'a self,
        endpoint: impl AsRef<str> + 'a,
        body: Payload<B>,
    ) -> Result<impl AsyncRead + AsyncWrite + 'a>
    where
        B: Into<Body> + 'a,
    {
        self.client
            .post_upgrade_stream(self.version.make_endpoint(endpoint), body)
            .await
    }
}

fn validate_response(
    response: Response<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>>> + Send + Sync>> {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    struct ErrorResponse {
        message: String,
    }

    Box::pin(async move {
        log::trace!(
            "got response {} {:?}",
            response.status(),
            response.headers()
        );
        let status = response.status();

        use crate::conn::{self, hyper::StatusCode};
        match status {
            // Success case: pass on the response
            StatusCode::OK
            | StatusCode::CREATED
            | StatusCode::SWITCHING_PROTOCOLS
            | StatusCode::NO_CONTENT => Ok(response),
            // Error case: try to deserialize error message
            _ => {
                let body = response.into_body();
                let bytes = hyper::body::to_bytes(body)
                    .await
                    .map_err(conn::Error::from)?;
                let message_body = String::from_utf8(bytes.to_vec()).map_err(conn::Error::from)?;
                log::trace!("{message_body:#?}");
                let message = serde_json::from_str::<ErrorResponse>(&message_body)
                    .map(|e| e.message)
                    .unwrap_or_else(|_| {
                        status
                            .canonical_reason()
                            .unwrap_or("unknown error code")
                            .to_owned()
                    });
                Err(Error::Fault {
                    code: status,
                    message,
                })
            }
        }
    })
}

#[cfg(feature = "swarm")]
impl Docker {
    /// Exports an interface for interacting with Docker services.
    pub fn services(&'_ self) -> Services {
        Services::new(self.clone())
    }

    /// Exports an interface for interacting with Docker configs.
    pub fn configs(&'_ self) -> Configs {
        Configs::new(self.clone())
    }

    /// Exports an interface for interacting with Docker tasks.
    pub fn tasks(&'_ self) -> Tasks {
        Tasks::new(self.clone())
    }

    /// Exports an interface for interacting with Docker secrets.
    pub fn secrets(&'_ self) -> Secrets {
        Secrets::new(self.clone())
    }

    /// Exports an interface for interacting with Docker swarm.
    pub fn swarm(&'_ self) -> Swarm {
        Swarm::new(self.clone())
    }

    /// Exports an interface for interacting with Docker nodes.
    pub fn nodes(&'_ self) -> Nodes {
        Nodes::new(self.clone())
    }

    /// Exports an interface for interacting with Docker plugins.
    pub fn plugins(&'_ self) -> Plugins {
        Plugins::new(self.clone())
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
        {
            let d = Docker::new("unix://127.0.0.1:80");
            d.unwrap();
        }
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
