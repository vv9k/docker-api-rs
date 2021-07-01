//! Main entrypoint for interacting with the Docker API.
//!
//! API Reference: <https://docs.docker.com/engine/api/v1.41/>

use std::{collections::HashMap, convert::TryFrom, io};

use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
    TryStreamExt,
};
use hyper::{body::Bytes, header::HeaderMap, Body, Client, Method, Response};
use log::trace;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    api::{
        container::{Containers, Mount, Port},
        event::{Event, EventsOpts},
        image::Images,
        network::{NetworkEntry, Networks},
        service::Services,
        volume::{VolumeInfo, Volumes},
    },
    conn::{get_http_connector, Headers, Payload, Transport},
    errors::{Error, Result},
};

#[cfg(feature = "tls")]
use {crate::conn::get_https_connector, std::path::Path};

#[cfg(unix)]
use crate::conn::get_unix_connector;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

/// Entrypoint interface for communicating with docker daemon
#[derive(Debug, Clone)]
pub struct Docker {
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
    pub fn new<U>(uri: U) -> Result<Docker>
    where
        U: AsRef<str>,
    {
        let uri = uri.as_ref();
        let mut it = uri.split("://");

        match it.next() {
            #[cfg(unix)]
            Some("unix") => {
                if let Some(path) = it.next() {
                    Ok(Docker::unix(path))
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            #[cfg(not(unix))]
            Some("unix") => Err(Error::UnsupportedScheme("unix".to_string())),
            Some("tcp") | Some("http") => {
                if let Some(host) = it.next() {
                    Ok(Docker::tcp(host))
                } else {
                    Err(Error::MissingAuthority)
                }
            }
            Some(scheme) => Err(Error::UnsupportedScheme(scheme.to_string())),
            None => unreachable!(), // This is never possible because calling split on an empty string
                                    // always returns at least one element
        }
    }

    /// Creates a new docker instance for a docker host listening on a given Unix socket.
    ///
    /// `socket_path` is the part of URI that comes after the `unix://`. For example a URI `unix:///run/docker.sock` has a
    /// `socket_path` == "/run/docker.sock".
    #[cfg(unix)]
    pub fn unix<P>(socket_path: P) -> Docker
    where
        P: Into<String>,
    {
        Docker {
            transport: Transport::Unix {
                client: Client::builder()
                    .pool_max_idle_per_host(0)
                    .build(get_unix_connector()),
                path: socket_path.into(),
            },
        }
    }

    #[cfg(feature = "tls")]
    /// Creates a new docker instance for a docker host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` or `https://` schemes,
    /// also known as authority part.
    ///
    /// `cert_path` specifies the base path in the filesystem containing a certificate (`cert.pem`)
    /// and a key (`key.pem`) that will be used by the client. If verify is `true` a CA file will be
    /// added (`ca.pem`) to the connector.
    pub fn tls<H, P>(host: H, cert_path: P, verify: bool) -> Result<Docker>
    where
        H: AsRef<str>,
        P: AsRef<Path>,
    {
        Ok(Docker {
            transport: Transport::EncryptedTcp {
                client: Client::builder().build(get_https_connector(cert_path.as_ref(), verify)?),
                host: format!("https://{}", host.as_ref()),
            },
        })
    }

    /// Creates a new docker instance for a docker host listening on a given TCP socket `host`.
    /// `host` is the part of URI that comes after `tcp://` or `http://` schemes, also known as
    /// authority part.
    ///
    /// TLS is supported with feature `tls` enabled through [Docker::tls](Docker::tls) constructor.
    pub fn tcp<H>(host: H) -> Docker
    where
        H: AsRef<str>,
    {
        let http = get_http_connector();
        Docker {
            transport: Transport::Tcp {
                client: Client::builder().build(http),
                host: format!("tcp://{}", host.as_ref()),
            },
        }
    }

    /// Exports an interface for interacting with Docker images
    pub fn images(&'_ self) -> Images<'_> {
        Images::new(self)
    }

    /// Exports an interface for interacting with Docker containers
    pub fn containers(&'_ self) -> Containers<'_> {
        Containers::new(self)
    }

    /// Exports an interface for interacting with Docker services
    pub fn services(&'_ self) -> Services<'_> {
        Services::new(self)
    }

    /// Exports an interface for interacting with Docker networks
    pub fn networks(&'_ self) -> Networks<'_> {
        Networks::new(self)
    }

    /// Exports an interface for interacting with Docker volumes
    pub fn volumes(&'_ self) -> Volumes<'_> {
        Volumes::new(self)
    }

    /// Returns the version of Docker that is running and various information about the system that
    /// Docker is running on.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemVersion)
    pub async fn version(&self) -> Result<Version> {
        self.get_json("/version").await
    }

    /// Returns system information about Docker instance that is running
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemInfo)
    pub async fn info(&self) -> Result<Info> {
        self.get_json("/info").await
    }

    /// This is a dummy endpoint you can use to test if the server is accessible.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemPingHead)
    pub async fn ping(&self) -> Result<PingInfo> {
        self.get("/_ping")
            .await
            .and_then(|resp| PingInfo::try_from(resp.headers()))
    }

    /// Returns a stream of Docker events
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemEvents)
    pub fn events<'docker>(
        &'docker self,
        opts: &EventsOpts,
    ) -> impl Stream<Item = Result<Event>> + Unpin + 'docker {
        let mut path = vec!["/events".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        let reader = Box::pin(
            self.stream_get(path.join("?"))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        )
        .into_async_read();

        let codec = futures_codec::LinesCodec {};

        Box::pin(
            futures_codec::FramedRead::new(reader, codec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }

    /// Returns data usage of this Docker instance
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemDataUsage)
    pub async fn data_usage(&self) -> Result<DataUsage> {
        self.get_json("/system/df").await
    }

    //####################################################################################################
    //
    // Utility functions to make requests
    //
    //####################################################################################################

    pub(crate) async fn get(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(Method::GET, endpoint, Payload::empty(), Headers::none())
            .await
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(Method::GET, endpoint, Payload::empty(), Headers::none())
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn post<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(Method::POST, endpoint, body, Headers::none())
            .await
    }

    pub(crate) async fn put<B>(&self, endpoint: &str, body: Payload<B>) -> Result<String>
    where
        B: Into<Body>,
    {
        self.transport
            .request_string(Method::PUT, endpoint, body, Headers::none())
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
            .request_string(Method::POST, endpoint, body, Headers::none())
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&&raw_string)?)
    }

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
            .request_string(Method::POST, endpoint, body, headers)
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn delete(&self, endpoint: &str) -> Result<String> {
        self.transport
            .request_string(Method::DELETE, endpoint, Payload::empty(), Headers::none())
            .await
    }

    pub(crate) async fn delete_json<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T> {
        let raw_string = self
            .transport
            .request_string(Method::DELETE, endpoint, Payload::empty(), Headers::none())
            .await?;
        trace!("{}", raw_string);

        Ok(serde_json::from_str::<T>(&raw_string)?)
    }

    pub(crate) async fn head_response(&self, endpoint: &str) -> Result<Response<Body>> {
        self.transport
            .request(Method::HEAD, endpoint, Payload::empty(), Headers::none())
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
        self.transport
            .stream_chunks(Method::POST, endpoint, body, headers)
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
        self.transport
            .stream_json_chunks(Method::POST, endpoint, body, headers)
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
        self.transport
            .stream_chunks(Method::GET, endpoint, Payload::empty(), Headers::none())
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
            .stream_upgrade(Method::POST, endpoint, body)
            .await
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
    pub version: String,
    pub api_version: String,
    pub git_commit: String,
    pub go_version: String,
    pub os: String,
    pub arch: String,
    pub kernel_version: String,
    #[cfg(feature = "chrono")]
    pub build_time: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub build_time: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Info {
    #[serde(rename = "ID")]
    pub id: String,
    pub containers: usize,
    pub containers_running: usize,
    pub containers_paused: usize,
    pub containers_stopped: usize,
    pub images: usize,
    pub driver: String,
    pub driver_status: Vec<Vec<String>>,
    pub docker_root_dir: String,
    // TODO:
    //pub plugins: PluginsInfo,
    pub memory_limit: bool,
    pub swap_limit: bool,
    pub kernel_memory: bool,
    pub cpu_cfs_period: bool,
    pub cpu_cfs_quota: bool,
    #[serde(rename = "CPUShares")]
    pub cpu_shares: bool,
    #[serde(rename = "CPUSet")]
    pub cpu_set: bool,
    pub pids_limit: bool,
    pub oom_kill_disable: bool,
    #[serde(rename = "IPv4Forwarding")]
    pub ipv4_forwarding: bool,
    pub bridge_nf_iptables: bool,
    pub bridge_nf_ip6tables: bool,
    pub debug: bool,
    pub n_fd: usize,
    pub n_goroutines: usize,
    pub system_time: String,
    pub logging_driver: String,
    pub cgroup_driver: String,
    pub cgroup_version: String,
    pub n_events_listener: u64,
    pub kernel_version: String,
    pub operating_system: String,
    #[serde(rename = "OSVersion")]
    pub os_version: String,
    #[serde(rename = "OSType")]
    pub os_type: String,
    pub architecture: String,
    #[serde(rename = "NCPU")]
    pub n_cpu: u64,
    pub mem_total: u64,
    pub index_server_address: String,
    // TODO:
    //pub registry_config: Option<RegistryServiceConfig>,
    // TODO:
    //pub generic_resources: Vec<GenericResource>,
    pub http_proxy: String,
    pub https_proxy: String,
    pub no_proxy: String,
    pub name: String,
    pub labels: Vec<String>,
    pub experimental_build: bool,
    pub server_version: String,
    pub cluster_store: Option<String>,
    pub cluster_advertise: Option<String>,
    // TODO:
    //pub runtimes: Runtimes,
    pub default_runtime: String,
    // TODO:
    //pub swarm: SwarmInfo,
    pub live_restore_enabled: bool,
    // TODO: could be an enum
    pub isolation: String,
    pub init_binary: String,
    pub containerd_commit: Commit,
    pub runc_commit: Commit,
    pub init_commit: Commit,
    pub security_options: Vec<String>,
    pub product_license: Option<String>,
    pub default_address_pools: Option<Vec<AddressPool>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddressPool {
    base: String,
    size: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit {
    #[serde(rename = "ID")]
    pub id: String,
    pub expected: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataUsage {
    pub layer_size: Option<i64>,
    pub images: Vec<ImageSummary>,
    pub containers: Vec<ContainerSummary>,
    pub volumes: Vec<VolumeInfo>,
    pub build_cache: Option<Vec<BuildCache>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageSummary {
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Option<Vec<String>>,
    pub created: usize,
    pub size: usize,
    pub shared_size: usize,
    pub virtual_size: usize,
    pub labels: Option<HashMap<String, String>>,
    pub containers: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryHostConfig {
    network_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryNetworkSettings {
    pub networks: HashMap<String, NetworkEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerSummary {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub command: String,
    pub created: i64,
    pub ports: Vec<Port>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub labels: Option<HashMap<String, String>>,
    pub state: String,
    pub status: String,
    pub host_config: SummaryHostConfig,
    pub network_settings: SummaryNetworkSettings,
    pub mounts: Vec<Mount>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildCache {
    #[serde(rename = "ID")]
    pub id: String,
    pub parent: String,
    #[serde(rename = "Type")]
    pub type_: String,
    pub description: String,
    pub in_use: bool,
    pub shared: bool,
    pub size: usize,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub last_used_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub last_used_at: String,
    pub usage_count: usize,
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

#[derive(Serialize, Debug)]
pub struct PingInfo {
    pub api_version: String,
    pub builder_version: Option<String>,
    pub docker_experimental: bool,
    pub cache_control: String,
    pub pragma: String,
    pub os_type: String,
    pub server: String,
    pub date: String,
}

impl TryFrom<&HeaderMap> for PingInfo {
    type Error = Error;

    fn try_from(value: &HeaderMap) -> Result<Self> {
        macro_rules! extract_str {
            ($id:literal) => {{
                if let Some(val) = value.get($id) {
                    val.to_str().map(ToString::to_string).map_err(|e| {
                        Error::InvalidResponse(format!(
                            "failed to convert header to string - {}",
                            e
                        ))
                    })?
                } else {
                    return Err(Error::InvalidResponse(format!(
                        "expected `{}` field in headers",
                        $id
                    )));
                }
            }};
        }

        Ok(PingInfo {
            api_version: extract_str!("api-version"),
            builder_version: value
                .get("builder-version")
                .and_then(|v| v.to_str().map(ToString::to_string).ok()),
            docker_experimental: extract_str!("docker-experimental").parse().map_err(|e| {
                Error::InvalidResponse(format!("expected header value to be bool - {}", e))
            })?,
            cache_control: extract_str!("cache-control"),
            pragma: extract_str!("pragma"),
            os_type: extract_str!("ostype"),
            date: extract_str!("date"),
            server: extract_str!("server"),
        })
    }
}
