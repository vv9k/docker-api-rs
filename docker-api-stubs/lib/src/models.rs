#![allow(
    non_snake_case,
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use chrono::{DateTime, Utc};

fn deserialize_nonoptional_vec<
    'de,
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    serde::de::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

fn deserialize_nonoptional_map<
    'de,
    D: serde::de::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
>(
    d: D,
) -> Result<HashMap<String, T>, D::Error> {
    serde::de::Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Address represents an IPv4 or IPv6 IP address.
pub struct Address {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IP address.
    pub addr: Option<String>,
    #[serde(rename = "PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the IP address.
    pub prefix_len: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serveraddress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// BuildCache contains information about a build cache record.
pub struct BuildCache {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the build cache was created in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Description of the build-step that produced the build cache.
    pub description: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique ID of the build cache record.
    pub id: Option<String>,
    #[serde(rename = "InUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the build cache is in use.
    pub in_use: Option<bool>,
    #[serde(rename = "LastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the build cache was last used in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the parent build cache record.
    pub parent: Option<String>,
    #[serde(rename = "Shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the build cache is shared.
    pub shared: Option<bool>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of disk space used by the build cache (in bytes).
    pub size: Option<isize>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cache record type.
    pub type_: Option<String>,
    #[serde(rename = "UsageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Cache record type.
pub enum BuildCacheTypeInlineItem {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "source.local")]
    SourceLocal,
    #[serde(rename = "source.git.checkout")]
    SourceGitCheckout,
    #[serde(rename = "exec.cachemount")]
    ExecCachemount,
    #[serde(rename = "regular")]
    Regular,
}

impl AsRef<str> for BuildCacheTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            BuildCacheTypeInlineItem::Internal => "internal",
            BuildCacheTypeInlineItem::Frontend => "frontend",
            BuildCacheTypeInlineItem::SourceLocal => "source.local",
            BuildCacheTypeInlineItem::SourceGitCheckout => "source.git.checkout",
            BuildCacheTypeInlineItem::ExecCachemount => "exec.cachemount",
            BuildCacheTypeInlineItem::Regular => "regular",
        }
    }
}

impl std::fmt::Display for BuildCacheTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildInfo {
    pub aux: Option<ImageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorDetail")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct BuildPrune200Response {
    #[serde(rename = "CachesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caches_deleted: Option<Vec<String>>,
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disk space reclaimed in bytes
    pub space_reclaimed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ClusterInfo represents information about the swarm as is returned by the
/// "/info" endpoint. Join-tokens are not included.
pub struct ClusterInfo {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the swarm was initialised in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DataPathPort specifies the data path port number for data traffic.
    /// Acceptable port range is 1024 to 49151.
    /// If no port is set or is set to 0, the default port (4789) is used.
    pub data_path_port: Option<u32>,
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Default Address Pool specifies default subnet pools for global scope
    /// networks.
    pub default_addr_pool: Option<Vec<String>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the swarm.
    pub id: Option<String>,
    #[serde(rename = "RootRotationInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether there is currently a root CA rotation in progress for the swarm
    pub root_rotation_in_progress: Option<bool>,
    #[serde(rename = "Spec")]
    pub spec: Option<SwarmSpec>,
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubnetSize specifies the subnet size of the networks created from the
    /// default subnet pool.
    pub subnet_size: Option<u32>,
    #[serde(rename = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the swarm was last updated in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Commit holds the Git-commit (SHA1) that a binary was built from, as
/// reported in the version-string of external tools, such as `containerd`,
/// or `runC`.
pub struct Commit {
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Commit ID of external tool expected by dockerd as set at build time.
    pub expected: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Actual commit ID of external tool.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentVersion {
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Key/value pairs of strings with additional information about the
    /// component. These values are intended for informational purposes
    /// only, and their content is not defined, and not part of the API
    /// specification.
    ///
    /// These messages can be printed by the client as information to the user.
    pub details: Option<Value>,
    #[serde(rename = "Name")]
    /// Name of the component
    pub name: String,
    #[serde(rename = "Version")]
    /// Version of the component
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<ConfigSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigCreateBodyParam {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5))
    /// config data.
    pub data: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the config.
    pub name: Option<String>,
    #[serde(rename = "Templating")]
    pub templating: Option<Driver>,
}

/// no error
pub type ConfigList200Response = Vec<Config>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSpec {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5))
    /// config data.
    pub data: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the config.
    pub name: Option<String>,
    #[serde(rename = "Templating")]
    pub templating: Option<Driver>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// change item in response to ContainerChanges operation
pub struct ContainerChangeResponseItem {
    #[serde(rename = "Kind")]
    /// Kind of change
    pub kind: u8,
    #[serde(rename = "Path")]
    /// Path to file that has changed
    pub path: String,
}

/// The list of changes
pub type ContainerChanges200Response = Vec<ContainerChangeResponseItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Configuration for a container that is portable between hosts.
///
/// When used as `ContainerConfig` field in an image, `ContainerConfig` is an
/// optional field containing the configuration of the container that was last
/// committed when creating the image.
///
/// Previous versions of Docker builder used this field to store build cache,
/// and it is not in active use anymore.
pub struct ContainerConfig {
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command is already escaped (Windows only)
    pub args_escaped: Option<bool>,
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stderr`.
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stdin`.
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stdout`.
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run specified as a string or an array of strings.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The domain name to use for the container.
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The entry point for the container as a string or an array of strings.
    ///
    /// If the array consists of exactly one empty string (`[""]`) then the
    /// entry point is reset to system default (i.e., the entry point used by
    /// docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables to set inside the container in the
    /// form `["VAR=value", ...]`. A variable without `=` is removed from the
    /// environment, rather than to have an empty value.
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An object mapping ports to an empty object in the form:
    ///
    /// `{"<port>/<tcp|udp|sctp>": {}}`
    pub exposed_ports: Option<HashMap<String, Value>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<HealthConfig>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name (or reference) of the image to use when creating the container,
    /// or which was used when the container was created.
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MAC address of the container.
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable networking for the container.
    pub network_disabled: Option<bool>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Open `stdin`
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    pub shell: Option<Vec<String>>,
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Close `stdin` after one attached client disconnects
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Signal to stop a container as a string or unsigned integer.
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout to stop a container in seconds.
    pub stop_timeout: Option<isize>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user that commands are run as inside the container.
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An object mapping mount point paths inside the container to empty
    /// objects.
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for commands to run in.
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Container created successfully
pub struct ContainerCreate201Response {
    #[serde(rename = "Id")]
    /// The ID of the created container
    pub id: String,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Warnings encountered when creating the container
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Configuration for a container that is portable between hosts.
///
/// When used as `ContainerConfig` field in an image, `ContainerConfig` is an
/// optional field containing the configuration of the container that was last
/// committed when creating the image.
///
/// Previous versions of Docker builder used this field to store build cache,
/// and it is not in active use anymore.
pub struct ContainerCreateBodyParam {
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command is already escaped (Windows only)
    pub args_escaped: Option<bool>,
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stderr`.
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stdin`.
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to attach to `stdout`.
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run specified as a string or an array of strings.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The domain name to use for the container.
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The entry point for the container as a string or an array of strings.
    ///
    /// If the array consists of exactly one empty string (`[""]`) then the
    /// entry point is reset to system default (i.e., the entry point used by
    /// docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables to set inside the container in the
    /// form `["VAR=value", ...]`. A variable without `=` is removed from the
    /// environment, rather than to have an empty value.
    pub env: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An object mapping ports to an empty object in the form:
    ///
    /// `{"<port>/<tcp|udp|sctp>": {}}`
    pub exposed_ports: Option<HashMap<String, Value>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<HealthConfig>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<Value>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name (or reference) of the image to use when creating the container,
    /// or which was used when the container was created.
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MAC address of the container.
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable networking for the container.
    pub network_disabled: Option<bool>,
    #[serde(rename = "NetworkingConfig")]
    pub networking_config: Option<NetworkingConfig>,
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Open `stdin`
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    pub shell: Option<Vec<String>>,
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Close `stdin` after one attached client disconnects
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Signal to stop a container as a string or unsigned integer.
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timeout to stop a container in seconds.
    pub stop_timeout: Option<isize>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user that commands are run as inside the container.
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An object mapping mount point paths inside the container to empty
    /// objects.
    pub volumes: Option<HashMap<String, Value>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for commands to run in.
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerExecExecConfigParam {
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to `stderr` of the exec command.
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to `stdin` of the exec command.
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Attach to `stdout` of the exec command.
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run, as a string or array of strings.
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Override the key sequence for detaching a container. Format is
    /// a single character `[a-Z]` or `ctrl-<value>` where `<value>`
    /// is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    pub detach_keys: Option<String>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables in the form `["VAR=value", ...]`.
    pub env: Option<Vec<String>>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runs the exec process with extended privileges.
    pub privileged: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY.
    pub tty: Option<bool>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user, and optionally, group to run the exec process inside
    /// the container. Format is one of: `user`, `user:group`, `uid`,
    /// or `uid:gid`.
    pub user: Option<String>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for the exec process inside the container.
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// no error
pub struct ContainerInspect200Response {
    #[serde(rename = "AppArmorProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The arguments to the command being run
    pub args: Option<Vec<String>>,
    #[serde(rename = "Config")]
    pub config: Option<ContainerConfig>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time the container was created
    pub created: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "ExecIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IDs of exec instances that are running in the container.
    pub exec_i_ds: Option<Vec<String>>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<Value>,
    #[serde(rename = "HostnamePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the container
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The container's image ID
    pub image: Option<String>,
    #[serde(rename = "LogPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "MountLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<NetworkSettings>,
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The path to the command being run
    pub path: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "ProcessLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "ResolvConfPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<isize>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total size of all the files in this container.
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The size of files that have been created or changed by this
    /// container.
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<ContainerState>,
}

/// no error
pub type ContainerList200Response = Vec<ContainerSummary>;

/// logs returned as a stream in response body.
/// For the stream format, [see the documentation for the attach endpoint](#operation/ContainerAttach).
/// Note that unlike the attach endpoint, the logs endpoint does not
/// upgrade the connection and does not set Content-Type.
pub type ContainerLogs200Response = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct ContainerPrune200Response {
    #[serde(rename = "ContainersDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container IDs that were deleted
    pub containers_deleted: Option<Vec<String>>,
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disk space reclaimed in bytes
    pub space_reclaimed: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ContainerState stores container's running state. It's part of ContainerJSONBase
/// and will be returned by the "inspect" command.
pub struct ContainerState {
    #[serde(rename = "Dead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The last exit code of this container
    pub exit_code: Option<isize>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time when this container last exited.
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<Health>,
    #[serde(rename = "OOMKilled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this container has been killed because it ran out of memory.
    pub oom_killed: Option<bool>,
    #[serde(rename = "Paused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this container is paused.
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The process ID of this container
    pub pid: Option<isize>,
    #[serde(rename = "Restarting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this container is restarting.
    pub restarting: Option<bool>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether this container is running.
    ///
    /// Note that a running container can be _paused_. The `Running` and `Paused`
    /// booleans are not mutually exclusive:
    ///
    /// When pausing a container (on Linux), the freezer cgroup is used to suspend
    /// all processes in the container. Freezing the process requires the process to
    /// be running. As a result, paused containers are both `Running` _and_ `Paused`.
    ///
    /// Use the `Status` field instead to determine if a container's state is "running".
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time when this container was last started.
    pub started_at: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// String representation of the container state. Can be one of "created",
    /// "running", "paused", "restarting", "removing", "exited", or "dead".
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// String representation of the container state. Can be one of "created",
/// "running", "paused", "restarting", "removing", "exited", or "dead".
pub enum ContainerStateStatusInlineItem {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "dead")]
    Dead,
}

impl AsRef<str> for ContainerStateStatusInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ContainerStateStatusInlineItem::Created => "created",
            ContainerStateStatusInlineItem::Running => "running",
            ContainerStateStatusInlineItem::Paused => "paused",
            ContainerStateStatusInlineItem::Restarting => "restarting",
            ContainerStateStatusInlineItem::Removing => "removing",
            ContainerStateStatusInlineItem::Exited => "exited",
            ContainerStateStatusInlineItem::Dead => "dead",
        }
    }
}

impl std::fmt::Display for ContainerStateStatusInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// no error
pub type ContainerStats200Response = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummary {
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Command to run when starting the container
    pub command: Option<String>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// When the container was created
    pub created: Option<i64>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<ContainerSummaryHostConfigInlineItem>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of this container
    pub id: Option<String>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the image used when creating this container
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the image that this container was created from
    pub image_id: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The names that this container has been given
    pub names: Option<Vec<String>>,
    #[serde(rename = "NetworkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A summary of the container's network settings
    pub network_settings: Option<ContainerSummaryNetworkSettingsInlineItem>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ports exposed by this container
    pub ports: Option<Vec<Port>>,
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total size of all the files in this container
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The size of files that have been created or changed by this container
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The state of this container (e.g. `Exited`)
    pub state: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional human-readable status of this container (e.g. `Exit 0`)
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummaryHostConfigInlineItem {
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A summary of the container's network settings
pub struct ContainerSummaryNetworkSettingsInlineItem {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// no error
pub struct ContainerTop200Response {
    #[serde(rename = "Processes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Each process running in the container, where each is process
    /// is an array of values corresponding to the titles.
    pub processes: Option<Vec<Vec<String>>>,
    #[serde(rename = "Titles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ps column titles
    pub titles: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The container has been updated.
pub struct ContainerUpdate200Response {
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A container's resources (cgroups config, ulimits, etc)
pub struct ContainerUpdateUpdateParam {
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (bytes per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (IO per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (bytes per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (IO per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative weight).
    pub blkio_weight: Option<isize>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Weight": weight}]
    /// ```
    pub blkio_weight_device: Option<Vec<ContainerUpdateUpdateParamBlkioWeightDeviceInlineItem>>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path to `cgroups` under which the container's `cgroup` is created. If
    /// the path is not absolute, the path is considered to be relative to the
    /// `cgroups` path of the init process. Cgroups are created if they do not
    /// already exist.
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of usable CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The usable percentage of the available CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU period in microseconds.
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Microseconds of CPU time that the container can get in a CPU period.
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time period in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An integer value representing this container's relative CPU weight
    /// versus other containers.
    pub cpu_shares: Option<isize>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
    /// effective on NUMA systems.
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// a list of cgroup rules to apply to the container
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of requests for devices to be sent to device drivers.
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of devices to add to the container.
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IO in bytes per second for the container system drive
    /// (Windows only).
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IOps for the container system drive (Windows only)
    pub io_maximum_i_ops: Option<i64>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Run an init inside the container that forwards signals and reaps
    /// processes. This field is omitted if empty, and the default (as
    /// configured on the daemon) is used.
    pub init: Option<bool>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel memory limit in bytes.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated
    /// > `kmem.limit_in_bytes`.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit for kernel TCP buffer memory (in bytes).
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory limit in bytes.
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory soft limit in bytes.
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited
    /// swap.
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's memory swappiness behavior. Accepts an integer
    /// between 0 and 100.
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    pub nano_cpus: Option<i64>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable OOM Killer for the container.
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null`
    /// to not change.
    pub pids_limit: Option<i64>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of resource limits to set in the container. For example:
    ///
    /// ```
    /// {"Name": "nofile", "Soft": 1024, "Hard": 2048}
    /// ```
    pub ulimits: Option<Vec<ContainerUpdateUpdateParamUlimitsInlineItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerUpdateUpdateParamBlkioWeightDeviceInlineItem {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerUpdateUpdateParamUlimitsInlineItem {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit
    pub hard: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of ulimit
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft limit
    pub soft: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// container waiting error, if any
pub struct ContainerWaitExitError {
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Details of an error
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// OK response to ContainerWait operation
pub struct ContainerWaitResponse {
    #[serde(rename = "Error")]
    pub error: Option<ContainerWaitExitError>,
    #[serde(rename = "StatusCode")]
    /// Exit code of the container
    pub status_code: isize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateImageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A device mapping between the host and container
pub struct DeviceMapping {
    #[serde(rename = "CgroupPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_permissions: Option<String>,
    #[serde(rename = "PathInContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_in_container: Option<String>,
    #[serde(rename = "PathOnHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A request for devices to be sent to device drivers
pub struct DeviceRequest {
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of capabilities; an OR list of AND lists of capabilities.
    pub capabilities: Option<Vec<Vec<String>>>,
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<isize>,
    #[serde(rename = "DeviceIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_i_ds: Option<Vec<String>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver-specific options, specified as a key/value pairs. These options
    /// are passed directly to the driver.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Describes the result obtained from contacting the registry to retrieve
/// image metadata.
pub struct DistributionInspect {
    #[serde(rename = "Descriptor")]
    pub descriptor: OciDescriptor,
    #[serde(rename = "Platforms")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// An array containing all platforms supported by the image.
    pub platforms: Vec<OciPlatform>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Driver represents a driver (network, logging, secrets).
pub struct Driver {
    #[serde(rename = "Name")]
    /// Name of the driver.
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Key/value map of driver-specific options.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EndpointIPAMConfig represents an endpoint's IPAM configuration.
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_address: Option<String>,
    #[serde(rename = "LinkLocalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_ps: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointPortConfig {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "PublishMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode in which port is published.
    ///
    /// <p><br /></p>
    ///
    /// - "ingress" makes the target port accessible on every node,
    ///   regardless of whether there is a task for the service running on
    ///   that node or not.
    /// - "host" bypasses the routing mesh and publish the port directly on
    ///   the swarm node where that service is running.
    pub publish_mode: Option<String>,
    #[serde(rename = "PublishedPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The port on the swarm hosts.
    pub published_port: Option<isize>,
    #[serde(rename = "TargetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The port inside the container.
    pub target_port: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndpointPortConfigProtocolInlineItem {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}

impl AsRef<str> for EndpointPortConfigProtocolInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            EndpointPortConfigProtocolInlineItem::Tcp => "tcp",
            EndpointPortConfigProtocolInlineItem::Udp => "udp",
            EndpointPortConfigProtocolInlineItem::Sctp => "sctp",
        }
    }
}

impl std::fmt::Display for EndpointPortConfigProtocolInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mode in which port is published.
///
/// <p><br /></p>
///
/// - "ingress" makes the target port accessible on every node,
///   regardless of whether there is a task for the service running on
///   that node or not.
/// - "host" bypasses the routing mesh and publish the port directly on
///   the swarm node where that service is running.
pub enum EndpointPortConfigPublishModeInlineItem {
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "host")]
    Host,
}

impl AsRef<str> for EndpointPortConfigPublishModeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            EndpointPortConfigPublishModeInlineItem::Ingress => "ingress",
            EndpointPortConfigPublishModeInlineItem::Host => "host",
        }
    }
}

impl std::fmt::Display for EndpointPortConfigPublishModeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Configuration for a network endpoint.
pub struct EndpointSettings {
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DriverOpts is a mapping of driver options and values. These options
    /// are passed directly to the driver and are driver specific.
    pub driver_opts: Option<HashMap<String, String>>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique ID for the service endpoint in a Sandbox.
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway address for this network.
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Global IPv6 address.
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the global IPv6 address.
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<EndpointIpamConfig>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv4 address.
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the IPv4 address.
    pub ip_prefix_len: Option<isize>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6 gateway address.
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MAC address for the endpoint on this network.
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique ID of the network.
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Properties that can be configured to access and load balance a service.
pub struct EndpointSpec {
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode of resolution to use for internal load balancing between tasks.
    pub mode: Option<String>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of exposed ports that this service is accessible on from the
    /// outside. Ports can only be provided if `vip` resolution mode is used.
    pub ports: Option<Vec<EndpointPortConfig>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mode of resolution to use for internal load balancing between tasks.
pub enum EndpointSpecModeInlineItem {
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "dnsrr")]
    Dnsrr,
}

impl AsRef<str> for EndpointSpecModeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            EndpointSpecModeInlineItem::Vip => "vip",
            EndpointSpecModeInlineItem::Dnsrr => "dnsrr",
        }
    }
}

impl std::fmt::Display for EndpointSpecModeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EngineDescription provides information about an engine.
pub struct EngineDescription {
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<EngineDescriptionPluginsInlineItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EngineDescriptionPluginsInlineItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents an error.
pub struct ErrorResponse {
    /// The error message.
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Actor describes something that generates events, like a container, network,
/// or a volume.
pub struct EventActor {
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Various key/value attributes of the object, depending on its type.
    pub attributes: Option<HashMap<String, String>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the object emitting the event
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// EventMessage represents the information an event contains.
pub struct EventMessage {
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The type of event
    pub action: Option<String>,
    #[serde(rename = "Actor")]
    pub actor: Option<EventActor>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The type of object emitting the event
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scope of the event. Engine events are `local` scope. Cluster (Swarm)
    /// events are `swarm` scope.
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timestamp of event
    pub time: Option<i64>,
    #[serde(rename = "timeNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Timestamp of event, with nanosecond accuracy
    pub time_nano: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The type of object emitting the event
pub enum EventMessageTypeInlineItem {
    #[serde(rename = "builder")]
    Builder,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "daemon")]
    Daemon,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "plugin")]
    Plugin,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "volume")]
    Volume,
}

impl AsRef<str> for EventMessageTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            EventMessageTypeInlineItem::Builder => "builder",
            EventMessageTypeInlineItem::Config => "config",
            EventMessageTypeInlineItem::Container => "container",
            EventMessageTypeInlineItem::Daemon => "daemon",
            EventMessageTypeInlineItem::Image => "image",
            EventMessageTypeInlineItem::Network => "network",
            EventMessageTypeInlineItem::Node => "node",
            EventMessageTypeInlineItem::Plugin => "plugin",
            EventMessageTypeInlineItem::Secret => "secret",
            EventMessageTypeInlineItem::Service => "service",
            EventMessageTypeInlineItem::Volume => "volume",
        }
    }
}

impl std::fmt::Display for EventMessageTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Scope of the event. Engine events are `local` scope. Cluster (Swarm)
/// events are `swarm` scope.
pub enum EventMessagescopeInlineItem {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "swarm")]
    Swarm,
}

impl AsRef<str> for EventMessagescopeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            EventMessagescopeInlineItem::Local => "local",
            EventMessagescopeInlineItem::Swarm => "swarm",
        }
    }
}

impl std::fmt::Display for EventMessagescopeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct ExecInspect200Response {
    #[serde(rename = "CanRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_remove: Option<bool>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<isize>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OpenStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stderr: Option<bool>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "OpenStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdout: Option<bool>,
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The system process ID for the exec process.
    pub pid: Option<isize>,
    #[serde(rename = "ProcessConfig")]
    pub process_config: Option<ProcessConfig>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecStartExecStartConfigParam {
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Detach from the command.
    pub detach: Option<bool>,
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocate a pseudo-TTY.
    pub tty: Option<bool>,
}

/// User-defined resources can be either Integer resources (e.g, `SSD=3`) or
/// String resources (e.g, `GPU=UUID1`).
pub type GenericResources = Vec<GenericResourcesInlineItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInlineItem {
    #[serde(rename = "DiscreteResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete_resource_spec: Option<GenericResourcesInlineItemDiscreteResourceSpecInlineItem>,
    #[serde(rename = "NamedResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_resource_spec: Option<GenericResourcesInlineItemNamedResourceSpecInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInlineItemDiscreteResourceSpecInlineItem {
    #[serde(rename = "Kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInlineItemNamedResourceSpecInlineItem {
    #[serde(rename = "Kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// no error
pub type GetPluginPrivileges200Response = Vec<PluginPrivilege>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the storage driver used to store the container's and
/// image's filesystem.
pub struct GraphDriverData {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// Low-level storage metadata, provided as key/value pairs.
    ///
    /// This information is driver-specific, and depends on the storage-driver
    /// in use, and should be used for informational purposes only.
    pub data: HashMap<String, String>,
    #[serde(rename = "Name")]
    /// Name of the storage driver.
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Health stores information about the container's healthcheck results.
pub struct Health {
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// FailingStreak is the number of consecutive failures
    pub failing_streak: Option<isize>,
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Log contains the last few results (oldest first)
    pub log: Option<Vec<HealthcheckResult>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Status is one of `none`, `starting`, `healthy` or `unhealthy`
    ///
    /// - "none"      Indicates there is no healthcheck
    /// - "starting"  Starting indicates that the container is not yet ready
    /// - "healthy"   Healthy indicates that the container is running correctly
    /// - "unhealthy" Unhealthy indicates that the container has a problem
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A test to perform to check that the container is healthy.
pub struct HealthConfig {
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time to wait between checks in nanoseconds. It should be 0 or at
    /// least 1000000 (1 ms). 0 means inherit.
    pub interval: Option<isize>,
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of consecutive failures needed to consider a container as
    /// unhealthy. 0 means inherit.
    pub retries: Option<isize>,
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Start period for the container to initialize before starting
    /// health-retries countdown in nanoseconds. It should be 0 or at least
    /// 1000000 (1 ms). 0 means inherit.
    pub start_period: Option<isize>,
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The test to perform. Possible values are:
    ///
    /// - `[]` inherit healthcheck from image or parent image
    /// - `["NONE"]` disable healthcheck
    /// - `["CMD", args...]` exec arguments directly
    /// - `["CMD-SHELL", command]` run command with system's default shell
    pub test: Option<Vec<String>>,
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time to wait before considering the check to have hung. It should
    /// be 0 or at least 1000000 (1 ms). 0 means inherit.
    pub timeout: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Status is one of `none`, `starting`, `healthy` or `unhealthy`
///
/// - "none"      Indicates there is no healthcheck
/// - "starting"  Starting indicates that the container is not yet ready
/// - "healthy"   Healthy indicates that the container is running correctly
/// - "unhealthy" Unhealthy indicates that the container has a problem
pub enum HealthStatusInlineItem {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "unhealthy")]
    Unhealthy,
}

impl AsRef<str> for HealthStatusInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            HealthStatusInlineItem::None => "none",
            HealthStatusInlineItem::Starting => "starting",
            HealthStatusInlineItem::Healthy => "healthy",
            HealthStatusInlineItem::Unhealthy => "unhealthy",
        }
    }
}

impl std::fmt::Display for HealthStatusInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// HealthcheckResult stores information about a single run of a healthcheck probe
pub struct HealthcheckResult {
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which this check ended in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub end: Option<DateTime<Utc>>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ExitCode meanings:
    ///
    /// - `0` healthy
    /// - `1` unhealthy
    /// - `2` reserved (considered unhealthy)
    /// - other values: error running probe
    pub exit_code: Option<isize>,
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Output from last check
    pub output: Option<String>,
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which this check started in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub start: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// individual image layer information in response to ImageHistory operation
pub struct HistoryResponseItem {
    #[serde(rename = "Comment")]
    pub comment: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Container configuration that depends on the host we are running on
pub struct HostConfig {
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Automatically remove the container when the container's process
    /// exits. This has no effect if `RestartPolicy` is set.
    pub auto_remove: Option<bool>,
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of volume bindings for this container. Each volume binding
    /// is a string in one of these forms:
    ///
    /// - `host-src:container-dest[:options]` to bind-mount a host path
    ///   into the container. Both `host-src`, and `container-dest` must
    ///   be an _absolute_ path.
    /// - `volume-name:container-dest[:options]` to bind-mount a volume
    ///   managed by a volume driver into the container. `container-dest`
    ///   must be an _absolute_ path.
    ///
    /// `options` is an optional, comma-delimited list of:
    ///
    /// - `nocopy` disables automatic copying of data from the container
    ///   path to the volume. The `nocopy` flag only applies to named volumes.
    /// - `[ro|rw]` mounts a volume read-only or read-write, respectively.
    ///   If omitted or set to `rw`, volumes are mounted read-write.
    /// - `[z|Z]` applies SELinux labels to allow or deny multiple containers
    ///   to read and write to the same volume.
    ///     - `z`: a _shared_ content label is applied to the content. This
    ///       label indicates that multiple containers can share the volume
    ///       content, for both reading and writing.
    ///     - `Z`: a _private unshared_ label is applied to the content.
    ///       This label indicates that only the current container can use
    ///       a private volume. Labeling systems such as SELinux require
    ///       proper labels to be placed on volume content that is mounted
    ///       into a container. Without a label, the security system can
    ///       prevent a container's processes from using the content. By
    ///       default, the labels set by the host operating system are not
    ///       modified.
    /// - `[[r]shared|[r]slave|[r]private]` specifies mount
    ///   [propagation behavior](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt).
    ///   This only applies to bind-mounted volumes, not internal volumes
    ///   or named volumes. Mount propagation requires the source mount
    ///   point (the location where the source directory is mounted in the
    ///   host operating system) to have the correct propagation properties.
    ///   For shared volumes, the source mount point must be set to `shared`.
    ///   For slave volumes, the mount must be set to either `shared` or
    ///   `slave`.
    pub binds: Option<Vec<String>>,
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (bytes per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (IO per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (bytes per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (IO per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative weight).
    pub blkio_weight: Option<isize>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Weight": weight}]
    /// ```
    pub blkio_weight_device: Option<Vec<HostConfigBlkioWeightDeviceInlineItem>>,
    #[serde(rename = "CapAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of kernel capabilities to add to the container. Conflicts
    /// with option 'Capabilities'.
    pub cap_add: Option<Vec<String>>,
    #[serde(rename = "CapDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of kernel capabilities to drop from the container. Conflicts
    /// with option 'Capabilities'.
    pub cap_drop: Option<Vec<String>>,
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cgroup to use for the container.
    pub cgroup: Option<String>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path to `cgroups` under which the container's `cgroup` is created. If
    /// the path is not absolute, the path is considered to be relative to the
    /// `cgroups` path of the init process. Cgroups are created if they do not
    /// already exist.
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CgroupnsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// cgroup namespace mode for the container. Possible values are:
    ///
    /// - `"private"`: the container runs in its own private cgroup namespace
    /// - `"host"`: use the host system's cgroup namespace
    ///
    /// If not specified, the daemon default is used, which can either be `"private"`
    /// or `"host"`, depending on daemon version, kernel support and configuration.
    pub cgroupns_mode: Option<String>,
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Initial console size, as an `[height, width]` array. (Windows only)
    pub console_size: Option<Vec<isize>>,
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path to a file where the container ID is written
    pub container_id_file: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of usable CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The usable percentage of the available CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU period in microseconds.
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Microseconds of CPU time that the container can get in a CPU period.
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time period in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An integer value representing this container's relative CPU weight
    /// versus other containers.
    pub cpu_shares: Option<isize>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
    /// effective on NUMA systems.
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// a list of cgroup rules to apply to the container
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of requests for devices to be sent to device drivers.
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of devices to add to the container.
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "Dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of DNS servers for the container to use.
    pub dns: Option<Vec<String>>,
    #[serde(rename = "DnsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of DNS options.
    pub dns_options: Option<Vec<String>>,
    #[serde(rename = "DnsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of DNS search domains.
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of hostnames/IP mappings to add to the container's `/etc/hosts`
    /// file. Specified in the form `["hostname:IP"]`.
    pub extra_hosts: Option<Vec<String>>,
    #[serde(rename = "GroupAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of additional groups that the container process will run as.
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IO in bytes per second for the container system drive
    /// (Windows only).
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IOps for the container system drive (Windows only)
    pub io_maximum_i_ops: Option<i64>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Run an init inside the container that forwards signals and reaps
    /// processes. This field is omitted if empty, and the default (as
    /// configured on the daemon) is used.
    pub init: Option<bool>,
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPC sharing mode for the container. Possible values are:
    ///
    /// - `"none"`: own private IPC namespace, with /dev/shm not mounted
    /// - `"private"`: own private IPC namespace
    /// - `"shareable"`: own private IPC namespace, with a possibility to share it with other containers
    /// - `"container:<name|id>"`: join another (shareable) container's IPC namespace
    /// - `"host"`: use the host system's IPC namespace
    ///
    /// If not specified, daemon default is used, which can either be `"private"`
    /// or `"shareable"`, depending on daemon version and configuration.
    pub ipc_mode: Option<String>,
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Isolation technology of the container. (Windows only)
    pub isolation: Option<String>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel memory limit in bytes.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated
    /// > `kmem.limit_in_bytes`.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit for kernel TCP buffer memory (in bytes).
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of links for the container in the form `container_name:alias`.
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The logging configuration for this container
    pub log_config: Option<HostConfigLogConfigInlineItem>,
    #[serde(rename = "MaskedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of paths to be masked inside the container (this overrides
    /// the default set of paths).
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory limit in bytes.
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory soft limit in bytes.
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited
    /// swap.
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's memory swappiness behavior. Accepts an integer
    /// between 0 and 100.
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for mounts to be added to the container.
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Network mode to use for this container. Supported standard values
    /// are: `bridge`, `host`, `none`, and `container:<name|id>`. Any
    /// other value is taken as a custom network's name to which this
    /// container should connect to.
    pub network_mode: Option<String>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable OOM Killer for the container.
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An integer value containing the score given to the container in
    /// order to tune OOM killer preferences.
    pub oom_score_adj: Option<isize>,
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Set the PID (Process) Namespace mode for the container. It can be
    /// either:
    ///
    /// - `"container:<name|id>"`: joins another container's PID namespace
    /// - `"host"`: use the host's PID namespace inside the container
    pub pid_mode: Option<String>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null`
    /// to not change.
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<PortMap>,
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gives the container full access to the host.
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Allocates an ephemeral host port for all of a container's
    /// exposed ports.
    ///
    /// Ports are de-allocated when the container stops and allocated when
    /// the container starts. The allocated port might be changed when
    /// restarting the container.
    ///
    /// The port is selected from the ephemeral port range that depends on
    /// the kernel. For example, on Linux the range is defined by
    /// `/proc/sys/net/ipv4/ip_local_port_range`.
    pub publish_all_ports: Option<bool>,
    #[serde(rename = "ReadonlyPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of paths to be set as read-only inside the container
    /// (this overrides the default set of paths).
    pub readonly_paths: Option<Vec<String>>,
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mount the container's root filesystem as read only.
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runtime to use with this container.
    pub runtime: Option<String>,
    #[serde(rename = "SecurityOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of string values to customize labels for MLS systems, such
    /// as SELinux.
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Size of `/dev/shm` in bytes. If omitted, the system uses 64MB.
    pub shm_size: Option<isize>,
    #[serde(rename = "StorageOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Storage driver options for this container, in the form `{"size": "120G"}`.
    pub storage_opt: Option<HashMap<String, String>>,
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of kernel parameters (sysctls) to set in the container.
    /// For example:
    ///
    /// ```
    /// {"net.ipv4.ip_forward": "1"}
    /// ```
    pub sysctls: Option<HashMap<String, String>>,
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A map of container directories which should be replaced by tmpfs
    /// mounts, and their corresponding mount options. For example:
    ///
    /// ```
    /// { "/run": "rw,noexec,nosuid,size=65536k" }
    /// ```
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UTS namespace to use for the container.
    pub uts_mode: Option<String>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of resource limits to set in the container. For example:
    ///
    /// ```
    /// {"Name": "nofile", "Soft": 1024, "Hard": 2048}
    /// ```
    pub ulimits: Option<Vec<HostConfigUlimitsInlineItem>>,
    #[serde(rename = "UsernsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Sets the usernamespace mode for the container when usernamespace
    /// remapping option is enabled.
    pub userns_mode: Option<String>,
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver that this container uses to mount volumes.
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of volumes to inherit from another container, specified in
    /// the form `<container name>[:<ro|rw>]`.
    pub volumes_from: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostConfigBlkioWeightDeviceInlineItem {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// cgroup namespace mode for the container. Possible values are:
///
/// - `"private"`: the container runs in its own private cgroup namespace
/// - `"host"`: use the host system's cgroup namespace
///
/// If not specified, the daemon default is used, which can either be `"private"`
/// or `"host"`, depending on daemon version, kernel support and configuration.
pub enum HostConfigCgroupnsModeInlineItem {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "host")]
    Host,
}

impl AsRef<str> for HostConfigCgroupnsModeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigCgroupnsModeInlineItem::Private => "private",
            HostConfigCgroupnsModeInlineItem::Host => "host",
        }
    }
}

impl std::fmt::Display for HostConfigCgroupnsModeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Isolation technology of the container. (Windows only)
pub enum HostConfigIsolationInlineItem {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
}

impl AsRef<str> for HostConfigIsolationInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigIsolationInlineItem::Default => "default",
            HostConfigIsolationInlineItem::Process => "process",
            HostConfigIsolationInlineItem::Hyperv => "hyperv",
        }
    }
}

impl std::fmt::Display for HostConfigIsolationInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The logging configuration for this container
pub struct HostConfigLogConfigInlineItem {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HostConfigLogConfigInlineItemTypeInlineItem {
    #[serde(rename = "json-file")]
    JsonFile,
    #[serde(rename = "syslog")]
    Syslog,
    #[serde(rename = "journald")]
    Journald,
    #[serde(rename = "gelf")]
    Gelf,
    #[serde(rename = "fluentd")]
    Fluentd,
    #[serde(rename = "awslogs")]
    Awslogs,
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "etwlogs")]
    Etwlogs,
    #[serde(rename = "none")]
    None,
}

impl AsRef<str> for HostConfigLogConfigInlineItemTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigLogConfigInlineItemTypeInlineItem::JsonFile => "json-file",
            HostConfigLogConfigInlineItemTypeInlineItem::Syslog => "syslog",
            HostConfigLogConfigInlineItemTypeInlineItem::Journald => "journald",
            HostConfigLogConfigInlineItemTypeInlineItem::Gelf => "gelf",
            HostConfigLogConfigInlineItemTypeInlineItem::Fluentd => "fluentd",
            HostConfigLogConfigInlineItemTypeInlineItem::Awslogs => "awslogs",
            HostConfigLogConfigInlineItemTypeInlineItem::Splunk => "splunk",
            HostConfigLogConfigInlineItemTypeInlineItem::Etwlogs => "etwlogs",
            HostConfigLogConfigInlineItemTypeInlineItem::None => "none",
        }
    }
}

impl std::fmt::Display for HostConfigLogConfigInlineItemTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostConfigUlimitsInlineItem {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit
    pub hard: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of ulimit
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft limit
    pub soft: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ipam {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of IPAM configuration options, specified as a map:
    ///
    /// ```
    /// {"Subnet": <CIDR>, "IPRange": <CIDR>, "Gateway": <IP address>, "AuxAddress": <device_name:IP address>}
    /// ```
    pub config: Option<Vec<IpamConfig>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the IPAM driver to use.
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver-specific options, specified as a map.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpamConfig {
    #[serde(rename = "AuxiliaryAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_addresses: Option<HashMap<String, String>>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "IPRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response to an API call that returns just an Id
pub struct IdResponse {
    #[serde(rename = "Id")]
    /// The id of the newly created object.
    pub id: String,
}

pub type ImageBuildInputStreamParam = Vec<u8>;

pub type ImageCreateInputImageParam = String;

/// The image was deleted successfully
pub type ImageDelete200Response = Vec<ImageDeleteResponseItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteResponseItem {
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The image ID of an image that was deleted
    pub deleted: Option<String>,
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The image ID of an image that was untagged
    pub untagged: Option<String>,
}

/// no error
pub type ImageGet200Response = Vec<u8>;

/// no error
pub type ImageGetAll200Response = Vec<u8>;

/// List of image layers
pub type ImageHistory200Response = Vec<HistoryResponseItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Image ID or Digest
pub struct ImageId {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about an image in the local image cache.
pub struct ImageInspect {
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hardware CPU architecture that the image runs on.
    pub architecture: Option<String>,
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the author that was specified when committing the image, or as
    /// specified through MAINTAINER (deprecated) in the Dockerfile.
    pub author: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional message that was set when committing or importing the image.
    pub comment: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<ContainerConfig>,
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the container that was used to create the image.
    ///
    /// Depending on how the image was created, this field may be empty.
    pub container: Option<String>,
    #[serde(rename = "ContainerConfig")]
    pub container_config: Option<ContainerConfig>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the image was created, formatted in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created: Option<String>,
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version of Docker that was used to build the image.
    ///
    /// Depending on how the image was created, this field may be empty.
    pub docker_version: Option<String>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    pub id: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional metadata of the image in the local cache. This information
    /// is local to the daemon, and not part of the image itself.
    pub metadata: Option<ImageInspectMetadataInlineItem>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Operating System the image is built to run on.
    pub os: Option<String>,
    #[serde(rename = "OsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Operating System version the image is built to run on (especially
    /// for Windows).
    pub os_version: Option<String>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of content-addressable digests of locally available image manifests
    /// that the image is referenced from. Multiple manifests can refer to the
    /// same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of image names/tags in the local image cache that reference this
    /// image.
    ///
    /// Multiple image tags can refer to the same imagem and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about the image's RootFS, including the layer IDs.
    pub root_fs: Option<ImageInspectRootFsInlineItem>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total size of the image including all layers it is composed of.
    pub size: Option<i64>,
    #[serde(rename = "Variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU architecture variant (presently ARM-only).
    pub variant: Option<String>,
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total size of the image including all layers it is composed of.
    ///
    /// In versions of Docker before v1.10, this field was calculated from
    /// the image itself and all of its parent images. Docker v1.10 and up
    /// store images self-contained, and no longer use a parent-chain, making
    /// this field an equivalent of the Size field.
    ///
    /// This field is kept for backward compatibility, but may be removed in
    /// a future version of the API.
    pub virtual_size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Additional metadata of the image in the local cache. This information
/// is local to the daemon, and not part of the image itself.
pub struct ImageInspectMetadataInlineItem {
    #[serde(rename = "LastTagTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the image was last tagged in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    ///
    /// This information is only available if the image was tagged locally,
    /// and omitted otherwise.
    pub last_tag_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the image's RootFS, including the layer IDs.
pub struct ImageInspectRootFsInlineItem {
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "Type")]
    pub type_: String,
}

/// Summary image data for the images matching the query
pub type ImageList200Response = Vec<ImageSummary>;

pub type ImageLoadImagesTarballParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct ImagePrune200Response {
    #[serde(rename = "ImagesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Images that were deleted
    pub images_deleted: Option<Vec<ImageDeleteResponseItem>>,
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disk space reclaimed in bytes
    pub space_reclaimed: Option<i64>,
}

/// No error
pub type ImageSearch200Response = Vec<ImageSearchResponseItem>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSearchResponseItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_official: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageSummary {
    #[serde(rename = "Containers")]
    /// Number of containers using this image. Includes both stopped and running
    /// containers.
    ///
    /// This size is not calculated by default, and depends on which API endpoint
    /// is used. `-1` indicates that the value has not been set / calculated.
    pub containers: isize,
    #[serde(rename = "Created")]
    /// Date and time at which the image was created as a Unix timestamp
    /// (number of seconds sinds EPOCH).
    pub created: isize,
    #[serde(rename = "Id")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    pub id: String,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// User-defined key/value metadata.
    pub labels: HashMap<String, String>,
    #[serde(rename = "ParentId")]
    /// ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    pub parent_id: String,
    #[serde(rename = "RepoDigests")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// List of content-addressable digests of locally available image manifests
    /// that the image is referenced from. Multiple manifests can refer to the
    /// same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    pub repo_digests: Vec<String>,
    #[serde(rename = "RepoTags")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// List of image names/tags in the local image cache that reference this
    /// image.
    ///
    /// Multiple image tags can refer to the same imagem and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    pub repo_tags: Vec<String>,
    #[serde(rename = "SharedSize")]
    /// Total size of image layers that are shared between this image and other
    /// images.
    ///
    /// This size is not calculated by default. `-1` indicates that the value
    /// has not been set / calculated.
    pub shared_size: isize,
    #[serde(rename = "Size")]
    /// Total size of the image including all layers it is composed of.
    pub size: i64,
    #[serde(rename = "VirtualSize")]
    /// Total size of the image including all layers it is composed of.
    ///
    /// In versions of Docker before v1.10, this field was calculated from
    /// the image itself and all of its parent images. Docker v1.10 and up
    /// store images self-contained, and no longer use a parent-chain, making
    /// this field an equivalent of the Size field.
    ///
    /// This field is kept for backward compatibility, but may be removed in
    /// a future version of the API.
    pub virtual_size: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// IndexInfo contains information about a registry.
pub struct IndexInfo {
    #[serde(rename = "Mirrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of mirrors, expressed as URIs.
    pub mirrors: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the registry, such as "docker.io".
    pub name: Option<String>,
    #[serde(rename = "Official")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates whether this is an official registry (i.e., Docker Hub / docker.io)
    pub official: Option<bool>,
    #[serde(rename = "Secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the registry is part of the list of insecure
    /// registries.
    ///
    /// If `false`, the registry is insecure. Insecure registries accept
    /// un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from
    /// unknown CAs) communication.
    ///
    /// > **Warning**: Insecure registries can be useful when running a local
    /// > registry. However, because its use creates security vulnerabilities
    /// > it should ONLY be enabled for testing purposes. For increased
    /// > security, users should add their CA to their system's list of
    /// > trusted CAs instead of enabling this option.
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// JoinTokens contains the tokens workers and managers need to join the swarm.
pub struct JoinTokens {
    #[serde(rename = "Manager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The token managers can use to join the swarm.
    pub manager: Option<String>,
    #[serde(rename = "Worker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The token workers can use to join the swarm.
    pub worker: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An object describing a limit on resources which can be requested by a task.
pub struct Limit {
    #[serde(rename = "MemoryBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    #[serde(rename = "NanoCPUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cp_us: Option<i64>,
    #[serde(rename = "Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limits the maximum number of PIDs in the container. Set `0` for unlimited.
    pub pids: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Current local status of this node.
pub enum LocalNodeState {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}

impl AsRef<str> for LocalNodeState {
    fn as_ref(&self) -> &str {
        match self {
            LocalNodeState::Empty => "",
            LocalNodeState::Inactive => "inactive",
            LocalNodeState::Pending => "pending",
            LocalNodeState::Active => "active",
            LocalNodeState::Error => "error",
            LocalNodeState::Locked => "locked",
        }
    }
}

impl std::fmt::Display for LocalNodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ManagerStatus represents the status of a manager.
///
/// It provides the current status of a node's manager component, if the node
/// is a manager.
pub struct ManagerStatus {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The IP address and port at which the manager is reachable.
    pub addr: Option<String>,
    #[serde(rename = "Leader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Reachability")]
    pub reachability: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "BindOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional configuration for the `bind` type.
    pub bind_options: Option<MountBindOptionsInlineItem>,
    #[serde(rename = "Consistency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`.
    pub consistency: Option<String>,
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the mount should be read-only.
    pub read_only: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mount source (e.g. a volume name, a host path).
    pub source: Option<String>,
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container path.
    pub target: Option<String>,
    #[serde(rename = "TmpfsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional configuration for the `tmpfs` type.
    pub tmpfs_options: Option<MountTmpfsOptionsInlineItem>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mount type. Available types:
    ///
    /// - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container.
    /// - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed.
    /// - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs.
    /// - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container.
    pub type_: Option<String>,
    #[serde(rename = "VolumeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional configuration for the `volume` type.
    pub volume_options: Option<MountVolumeOptionsInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Optional configuration for the `bind` type.
pub struct MountBindOptionsInlineItem {
    #[serde(rename = "NonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable recursive bind mount.
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
    pub propagation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
pub enum MountBindOptionsInlineItemPropagationInlineItem {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "rprivate")]
    Rprivate,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "rshared")]
    Rshared,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "rslave")]
    Rslave,
}

impl AsRef<str> for MountBindOptionsInlineItemPropagationInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            MountBindOptionsInlineItemPropagationInlineItem::Private => "private",
            MountBindOptionsInlineItemPropagationInlineItem::Rprivate => "rprivate",
            MountBindOptionsInlineItemPropagationInlineItem::Shared => "shared",
            MountBindOptionsInlineItemPropagationInlineItem::Rshared => "rshared",
            MountBindOptionsInlineItemPropagationInlineItem::Slave => "slave",
            MountBindOptionsInlineItemPropagationInlineItem::Rslave => "rslave",
        }
    }
}

impl std::fmt::Display for MountBindOptionsInlineItemPropagationInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// MountPoint represents a mount point configuration inside the container.
/// This is used for reporting the mountpoints in use by a container.
pub struct MountPoint {
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Destination is the path relative to the container root (`/`) where
    /// the `Source` is mounted inside the container.
    pub destination: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver is the volume driver used to create the volume (if it is a volume).
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode is a comma separated list of options supplied by the user when
    /// creating the bind/volume mount.
    ///
    /// The default is platform-specific (`"z"` on Linux, empty on Windows).
    pub mode: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name is the name reference to the underlying data defined by `Source`
    /// e.g., the volume name.
    pub name: Option<String>,
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Propagation describes how mounts are propagated from the host into the
    /// mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt)
    /// for details. This field is not used on Windows.
    pub propagation: Option<String>,
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the mount is mounted writable (read-write).
    pub rw: Option<bool>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Source location of the mount.
    ///
    /// For volumes, this contains the storage location of the volume (within
    /// `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains
    /// the source (host) part of the bind-mount. For `tmpfs` mount points, this
    /// field is empty.
    pub source: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mount type:
    ///
    /// - `bind` a mount of a file or directory from the host into the container.
    /// - `volume` a docker volume with the given `Name`.
    /// - `tmpfs` a `tmpfs`.
    /// - `npipe` a named pipe from the host into the container.
    pub type_: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mount type:
///
/// - `bind` a mount of a file or directory from the host into the container.
/// - `volume` a docker volume with the given `Name`.
/// - `tmpfs` a `tmpfs`.
/// - `npipe` a named pipe from the host into the container.
pub enum MountPointTypeInlineItem {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "tmpfs")]
    Tmpfs,
    #[serde(rename = "npipe")]
    Npipe,
}

impl AsRef<str> for MountPointTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            MountPointTypeInlineItem::Bind => "bind",
            MountPointTypeInlineItem::Volume => "volume",
            MountPointTypeInlineItem::Tmpfs => "tmpfs",
            MountPointTypeInlineItem::Npipe => "npipe",
        }
    }
}

impl std::fmt::Display for MountPointTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Optional configuration for the `tmpfs` type.
pub struct MountTmpfsOptionsInlineItem {
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The permission mode for the tmpfs mount in an integer.
    pub mode: Option<isize>,
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The size for the tmpfs mount in bytes.
    pub size_bytes: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mount type. Available types:
///
/// - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container.
/// - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed.
/// - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs.
/// - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container.
pub enum MountTypeInlineItem {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "tmpfs")]
    Tmpfs,
    #[serde(rename = "npipe")]
    Npipe,
}

impl AsRef<str> for MountTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            MountTypeInlineItem::Bind => "bind",
            MountTypeInlineItem::Volume => "volume",
            MountTypeInlineItem::Tmpfs => "tmpfs",
            MountTypeInlineItem::Npipe => "npipe",
        }
    }
}

impl std::fmt::Display for MountTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Optional configuration for the `volume` type.
pub struct MountVolumeOptionsInlineItem {
    #[serde(rename = "DriverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Map of driver specific options
    pub driver_config: Option<MountVolumeOptionsInlineItemDriverConfigInlineItem>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "NoCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Populate volume with data from the target.
    pub no_copy: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Map of driver specific options
pub struct MountVolumeOptionsInlineItemDriverConfigInlineItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the driver to use to create the volume.
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// key/value map of driver specific options.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Network {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<HashMap<String, NetworkContainer>>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv_6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specifies how a service should be attached to a particular network.
pub struct NetworkAttachmentConfig {
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Discoverable alternate names for the service on this network.
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver attachment options for the network target.
    pub driver_opts: Option<HashMap<String, String>>,
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The target network for attachment. Must be a network name or ID.
    pub target: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnectContainerParam {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID or name of the container to connect to the network.
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    pub endpoint_config: Option<EndpointSettings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkContainer {
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv_6_address: Option<String>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct NetworkCreate201Response {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the created network.
    pub id: Option<String>,
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkCreateNetworkConfigParam {
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Globally scoped network is manually attachable by regular
    /// containers from workers in swarm mode.
    pub attachable: Option<bool>,
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Check for networks with duplicate names. Since Network is
    /// primarily keyed based on a random ID and not on the name, and
    /// network name is strictly a user-friendly alias to the network
    /// which is uniquely identified using ID, there is no guaranteed
    /// way to check for duplicates. CheckDuplicate is there to provide
    /// a best effort checking of any networks which has the same name
    /// but it is not guaranteed to catch all name collisions.
    pub check_duplicate: Option<bool>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the network driver plugin to use.
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Enable IPv6 on the network.
    pub enable_i_pv_6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ingress network is the network which provides the routing-mesh
    /// in swarm mode.
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Restrict external access to the network.
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    /// The network's name.
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Network specific options to be used by the drivers.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkDisconnectContainerParam {
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID or name of the container to disconnect from the network.
    pub container: Option<String>,
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Force the container to disconnect from the network.
    pub force: Option<bool>,
}

/// No error
pub type NetworkList200Response = Vec<Network>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct NetworkPrune200Response {
    #[serde(rename = "NetworksDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Networks that were deleted
    pub networks_deleted: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkSettings exposes the network settings in the API
pub struct NetworkSettings {
    #[serde(rename = "Bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the network'a bridge (for example, `docker0`).
    pub bridge: Option<String>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// EndpointID uniquely represents a service endpoint in a Sandbox.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Gateway address for the default "bridge" network.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Global IPv6 address for the default "bridge" network.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the global IPv6 address.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub global_i_pv_6_prefix_len: Option<isize>,
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if hairpin NAT should be enabled on the virtual interface.
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv4 address for the default "bridge" network.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the IPv4 address.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub ip_prefix_len: Option<isize>,
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6 gateway address for this network.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6 unicast address using the link-local prefix.
    pub link_local_i_pv_6_address: Option<String>,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Prefix length of the IPv6 unicast address.
    pub link_local_i_pv_6_prefix_len: Option<isize>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// MAC address for the container on the default "bridge" network.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when attached to the
    /// > default "bridge" network. Use the information from the "bridge"
    /// > network inside the `Networks` map instead, which contains the same
    /// > information. This field was deprecated in Docker 1.9 and is scheduled
    /// > to be removed in Docker 17.12.0
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about all networks that the container is connected to.
    pub networks: Option<HashMap<String, EndpointSettings>>,
    #[serde(rename = "Ports")]
    pub ports: Option<PortMap>,
    #[serde(rename = "SandboxID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SandboxID uniquely represents a container's network stack.
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SandboxKey identifies the sandbox
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_i_pv_6_addresses: Option<Vec<Address>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NetworkingConfig represents the container's networking configuration for
/// each of its interfaces.
/// It is used for the networking configs specified in the `docker create`
/// and `docker network connect` commands.
pub struct NetworkingConfig {
    #[serde(rename = "EndpointsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A mapping of network name to endpoint configuration for that network.
    pub endpoints_config: Option<HashMap<String, EndpointSettings>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the node was added to the swarm in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Description")]
    pub description: Option<NodeDescription>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagerStatus")]
    pub manager_status: Option<ManagerStatus>,
    #[serde(rename = "Spec")]
    pub spec: Option<NodeSpec>,
    #[serde(rename = "Status")]
    pub status: Option<NodeStatus>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the node was last updated in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NodeDescription encapsulates the properties of the Node as reported by the
/// agent.
pub struct NodeDescription {
    #[serde(rename = "Engine")]
    pub engine: Option<EngineDescription>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Platform")]
    pub platform: Option<Platform>,
    #[serde(rename = "Resources")]
    pub resources: Option<ResourceObject>,
    #[serde(rename = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,
}

/// no error
pub type NodeList200Response = Vec<Node>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeSpec {
    #[serde(rename = "Availability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Availability of the node.
    pub availability: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name for the node.
    pub name: Option<String>,
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Role of the node.
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Availability of the node.
pub enum NodeSpecAvailabilityInlineItem {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "drain")]
    Drain,
}

impl AsRef<str> for NodeSpecAvailabilityInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            NodeSpecAvailabilityInlineItem::Active => "active",
            NodeSpecAvailabilityInlineItem::Pause => "pause",
            NodeSpecAvailabilityInlineItem::Drain => "drain",
        }
    }
}

impl std::fmt::Display for NodeSpecAvailabilityInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Role of the node.
pub enum NodeSpecRoleInlineItem {
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "manager")]
    Manager,
}

impl AsRef<str> for NodeSpecRoleInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            NodeSpecRoleInlineItem::Worker => "worker",
            NodeSpecRoleInlineItem::Manager => "manager",
        }
    }
}

impl std::fmt::Display for NodeSpecRoleInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NodeState represents the state of a node.
pub enum NodeState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "disconnected")]
    Disconnected,
}

impl AsRef<str> for NodeState {
    fn as_ref(&self) -> &str {
        match self {
            NodeState::Unknown => "unknown",
            NodeState::Down => "down",
            NodeState::Ready => "ready",
            NodeState::Disconnected => "disconnected",
        }
    }
}

impl std::fmt::Display for NodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NodeStatus represents the status of a node.
///
/// It provides the current status of the node, as seen by the manager.
pub struct NodeStatus {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IP address of the node.
    pub addr: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A descriptor struct containing digest, media type, and size, as defined in
/// the [OCI Content Descriptors Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md).
pub struct OciDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The digest of the targeted content.
    pub digest: Option<String>,
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The media type of the object this schema refers to.
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The size in bytes of the blob.
    pub size: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Describes the platform which the image in the manifest runs on, as defined
/// in the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/image-index.md).
pub struct OciPlatform {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The CPU architecture, for example `amd64` or `ppc64`.
    pub architecture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The operating system, for example `linux` or `windows`.
    pub os: Option<String>,
    #[serde(rename = "os.features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional field specifying an array of strings, each listing a required
    /// OS feature (for example on Windows `win32k`).
    pub os_features: Option<Vec<String>>,
    #[serde(rename = "os.version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional field specifying the operating system version, for example on
    /// Windows `10.0.19041.1165`.
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional field specifying a variant of the CPU, for example `v7` to
    /// specify ARMv7 when architecture is `arm`.
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The version number of the object such as node, service, etc. This is needed
/// to avoid conflicting writes. The client must send the version number along
/// with the modified specification when updating these objects.
///
/// This approach ensures safe concurrency and determinism in that the change
/// on the object may not be applied if the version number has changed from the
/// last read. In other words, if two update requests specify the same base
/// version, only one of the requests can succeed. As a result, two separate
/// update requests that happen at the same time will not unintentionally
/// overwrite each other.
pub struct ObjectVersion {
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents a peer-node in the swarm
pub struct PeerNode {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IP address and ports at which this node can be reached.
    pub addr: Option<String>,
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier of for this node in the swarm.
    pub node_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Platform represents the platform (Arch/OS).
pub struct Platform {
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Architecture represents the hardware architecture (for example,
    /// `x86_64`).
    pub architecture: Option<String>,
    #[serde(rename = "OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// OS represents the Operating System (for example, `linux` or `windows`).
    pub os: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A plugin for the Engine API
pub struct Plugin {
    #[serde(rename = "Config")]
    /// The config of a plugin.
    pub config: PluginConfigInlineItem,
    #[serde(rename = "Enabled")]
    /// True if the plugin is running. False if the plugin is not running, only installed.
    pub enabled: bool,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PluginReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// plugin remote reference used to push/pull the plugin
    pub plugin_reference: Option<String>,
    #[serde(rename = "Settings")]
    /// Settings that can be modified by users.
    pub settings: PluginSettingsInlineItem,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The config of a plugin.
pub struct PluginConfigInlineItem {
    #[serde(rename = "Args")]
    pub args: PluginConfigInlineItemArgsInlineItem,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Docker Version used to create the plugin
    pub docker_version: Option<String>,
    #[serde(rename = "Documentation")]
    pub documentation: String,
    #[serde(rename = "Entrypoint")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "Env")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<PluginEnv>,
    #[serde(rename = "Interface")]
    /// The interface between Docker and the plugin
    pub interface: PluginConfigInlineItemInterfaceInlineItem,
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "Linux")]
    pub linux: PluginConfigInlineItemLinuxInlineItem,
    #[serde(rename = "Mounts")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
    #[serde(rename = "Network")]
    pub network: PluginConfigInlineItemNetworkInlineItem,
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<PluginConfigInlineItemUserInlineItem>,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<PluginConfigInlineItemrootfsInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInlineItemArgsInlineItem {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub value: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The interface between Docker and the plugin
pub struct PluginConfigInlineItemInterfaceInlineItem {
    #[serde(rename = "ProtocolScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protocol to use for clients connecting to the plugin.
    pub protocol_scheme: Option<String>,
    #[serde(rename = "Socket")]
    pub socket: String,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub types: Vec<PluginInterfaceType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Protocol to use for clients connecting to the plugin.
pub enum PluginConfigInlineItemInterfaceInlineItemProtocolSchemeInlineItem {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "moby.plugins.http/v1")]
    MobyPluginsHttpV1,
}

impl AsRef<str> for PluginConfigInlineItemInterfaceInlineItemProtocolSchemeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            PluginConfigInlineItemInterfaceInlineItemProtocolSchemeInlineItem::Empty => "",
            PluginConfigInlineItemInterfaceInlineItemProtocolSchemeInlineItem::MobyPluginsHttpV1 => "moby.plugins.http/v1",
        }
    }
}

impl std::fmt::Display for PluginConfigInlineItemInterfaceInlineItemProtocolSchemeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInlineItemLinuxInlineItem {
    #[serde(rename = "AllowAllDevices")]
    pub allow_all_devices: bool,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub capabilities: Vec<String>,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInlineItemNetworkInlineItem {
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInlineItemUserInlineItem {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInlineItemrootfsInlineItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_ids: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

pub type PluginCreateTarContextParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginDevice {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginEnv {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginInterfaceType {
    #[serde(rename = "Capability")]
    pub capability: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Version")]
    pub version: String,
}

/// No error
pub type PluginList200Response = Vec<Plugin>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginMount {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub options: Vec<String>,
    #[serde(rename = "Settable")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Describes a permission the user has to accept upon installing
/// the plugin.
pub struct PluginPrivilege {
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

pub type PluginPullBodyParam = Vec<PluginPrivilege>;

pub type PluginSetBodyParam = Vec<String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Settings that can be modified by users.
pub struct PluginSettingsInlineItem {
    #[serde(rename = "Args")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub args: Vec<String>,
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
    #[serde(rename = "Env")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<String>,
    #[serde(rename = "Mounts")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
}

pub type PluginUpgradeBodyParam = Vec<PluginPrivilege>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Available plugins per type.
///
/// <p><br /></p>
///
/// > **Note**: Only unmanaged (V1) plugins are included in this list.
/// > V1 plugins are "lazily" loaded, and are not returned in this list
/// > if there is no resource using the plugin.
pub struct PluginsInfo {
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Names of available authorization plugins.
    pub authorization: Option<Vec<String>>,
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Names of available logging-drivers, and logging-driver plugins.
    pub log: Option<Vec<String>>,
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Names of available network-drivers, and network-driver plugins.
    pub network: Option<Vec<String>>,
    #[serde(rename = "Volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Names of available volume-drivers, and network-driver plugins.
    pub volume: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An open port on a container
pub struct Port {
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Host IP address that the container's port is mapped to
    pub ip: Option<String>,
    #[serde(rename = "PrivatePort")]
    /// Port on the container
    pub private_port: u16,
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Port exposed on the host
    pub public_port: Option<u16>,
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// PortBinding represents a binding between a host IP address and a host
/// port.
pub struct PortBinding {
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Host IP address that the container's port is mapped to.
    pub host_ip: Option<String>,
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Host port number that the container's port is mapped to.
    pub host_port: Option<String>,
}

/// PortMap describes the mapping of container ports to host ports, using the
/// container's port-number and protocol as key in the format `<port>/<protocol>`,
/// for example, `80/udp`.
///
/// If a container's port is mapped for multiple protocols, separate entries
/// are added to the mapping table.
pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PortTypeInlineItem {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}

impl AsRef<str> for PortTypeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            PortTypeInlineItem::Tcp => "tcp",
            PortTypeInlineItem::Udp => "udp",
            PortTypeInlineItem::Sctp => "sctp",
        }
    }
}

impl std::fmt::Display for PortTypeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushImageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    pub progress_detail: Option<ProgressDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

pub type PutContainerArchiveInputStreamParam = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Reachability represents the reachability of a node.
pub enum Reachability {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "reachable")]
    Reachable,
}

impl AsRef<str> for Reachability {
    fn as_ref(&self) -> &str {
        match self {
            Reachability::Unknown => "unknown",
            Reachability::Unreachable => "unreachable",
            Reachability::Reachable => "reachable",
        }
    }
}

impl std::fmt::Display for Reachability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// RegistryServiceConfig stores daemon registry services configuration.
pub struct RegistryServiceConfig {
    #[serde(rename = "AllowNondistributableArtifactsCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of IP ranges to which nondistributable artifacts can be pushed,
    /// using the CIDR syntax [RFC 4632](https://tools.ietf.org/html/4632).
    ///
    /// Some images (for example, Windows base images) contain artifacts
    /// whose distribution is restricted by license. When these images are
    /// pushed to a registry, restricted artifacts are not included.
    ///
    /// This configuration override this behavior, and enables the daemon to
    /// push nondistributable artifacts to all registries whose resolved IP
    /// address is within the subnet described by the CIDR syntax.
    ///
    /// This option is useful when pushing images containing
    /// nondistributable artifacts to a registry on an air-gapped network so
    /// hosts on that network can pull the images without connecting to
    /// another server.
    ///
    /// > **Warning**: Nondistributable artifacts typically have restrictions
    /// > on how and where they can be distributed and shared. Only use this
    /// > feature to push artifacts to private registries and ensure that you
    /// > are in compliance with any terms that cover redistributing
    /// > nondistributable artifacts.
    pub allow_nondistributable_artifacts_cid_rs: Option<Vec<String>>,
    #[serde(rename = "AllowNondistributableArtifactsHostnames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of registry hostnames to which nondistributable artifacts can be
    /// pushed, using the format `<hostname>[:<port>]` or `<IP address>[:<port>]`.
    ///
    /// Some images (for example, Windows base images) contain artifacts
    /// whose distribution is restricted by license. When these images are
    /// pushed to a registry, restricted artifacts are not included.
    ///
    /// This configuration override this behavior for the specified
    /// registries.
    ///
    /// This option is useful when pushing images containing
    /// nondistributable artifacts to a registry on an air-gapped network so
    /// hosts on that network can pull the images without connecting to
    /// another server.
    ///
    /// > **Warning**: Nondistributable artifacts typically have restrictions
    /// > on how and where they can be distributed and shared. Only use this
    /// > feature to push artifacts to private registries and ensure that you
    /// > are in compliance with any terms that cover redistributing
    /// > nondistributable artifacts.
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,
    #[serde(rename = "IndexConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_configs: Option<HashMap<String, IndexInfo>>,
    #[serde(rename = "InsecureRegistryCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of IP ranges of insecure registries, using the CIDR syntax
    /// ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries
    /// accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates
    /// from unknown CAs) communication.
    ///
    /// By default, local registries (`127.0.0.0/8`) are configured as
    /// insecure. All other registries are secure. Communicating with an
    /// insecure registry is not possible if the daemon assumes that registry
    /// is secure.
    ///
    /// This configuration override this behavior, insecure communication with
    /// registries whose resolved IP address is within the subnet described by
    /// the CIDR syntax.
    ///
    /// Registries can also be marked insecure by hostname. Those registries
    /// are listed under `IndexConfigs` and have their `Secure` field set to
    /// `false`.
    ///
    /// > **Warning**: Using this option can be useful when running a local
    /// > registry, but introduces security vulnerabilities. This option
    /// > should therefore ONLY be used for testing purposes. For increased
    /// > security, users should add their CA to their system's list of trusted
    /// > CAs instead of enabling this option.
    pub insecure_registry_cid_rs: Option<Vec<String>>,
    #[serde(rename = "Mirrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of registry URLs that act as a mirror for the official
    /// (`docker.io`) registry.
    pub mirrors: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An object describing the resources which can be advertised by a node and
/// requested by a task.
pub struct ResourceObject {
    #[serde(rename = "GenericResources")]
    pub generic_resources: Option<GenericResources>,
    #[serde(rename = "MemoryBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    #[serde(rename = "NanoCPUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cp_us: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// A container's resources (cgroups config, ulimits, etc)
pub struct Resources {
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (bytes per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit read rate (IO per second) from a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (bytes per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Limit write rate (IO per second) to a device, in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Rate": rate}]
    /// ```
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative weight).
    pub blkio_weight: Option<isize>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Weight": weight}]
    /// ```
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDeviceInlineItem>>,
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Path to `cgroups` under which the container's `cgroup` is created. If
    /// the path is not absolute, the path is considered to be relative to the
    /// `cgroups` path of the init process. Cgroups are created if they do not
    /// already exist.
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of usable CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The usable percentage of the available CPUs (Windows only).
    ///
    /// On Windows Server containers, the processor resource controls are
    /// mutually exclusive. The order of precedence is `CPUCount` first, then
    /// `CPUShares`, and `CPUPercent` last.
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU period in microseconds.
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Microseconds of CPU time that the container can get in a CPU period.
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time period in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to
    /// allocate no time allocated to real-time tasks.
    pub cpu_realtime_runtime: Option<i64>,
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An integer value representing this container's relative CPU weight
    /// versus other containers.
    pub cpu_shares: Option<isize>,
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only
    /// effective on NUMA systems.
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// a list of cgroup rules to apply to the container
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of requests for devices to be sent to device drivers.
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of devices to add to the container.
    pub devices: Option<Vec<DeviceMapping>>,
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IO in bytes per second for the container system drive
    /// (Windows only).
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum IOps for the container system drive (Windows only)
    pub io_maximum_i_ops: Option<i64>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Run an init inside the container that forwards signals and reaps
    /// processes. This field is omitted if empty, and the default (as
    /// configured on the daemon) is used.
    pub init: Option<bool>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel memory limit in bytes.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated
    /// > `kmem.limit_in_bytes`.
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit for kernel TCP buffer memory (in bytes).
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory limit in bytes.
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Memory soft limit in bytes.
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited
    /// swap.
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's memory swappiness behavior. Accepts an integer
    /// between 0 and 100.
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    pub nano_cpus: Option<i64>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable OOM Killer for the container.
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null`
    /// to not change.
    pub pids_limit: Option<i64>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of resource limits to set in the container. For example:
    ///
    /// ```
    /// {"Name": "nofile", "Soft": 1024, "Hard": 2048}
    /// ```
    pub ulimits: Option<Vec<ResourcesUlimitsInlineItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesBlkioWeightDeviceInlineItem {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourcesUlimitsInlineItem {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit
    pub hard: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of ulimit
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft limit
    pub soft: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The behavior to apply when the container exits. The default is not to
/// restart.
///
/// An ever increasing delay (double the previous delay, starting at 100ms) is
/// added before each restart to prevent flooding the server.
pub struct RestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If `on-failure` is used, the number of times to retry before giving up.
    pub maximum_retry_count: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// - Empty string means not to restart
    /// - `no` Do not automatically restart
    /// - `always` Always restart
    /// - `unless-stopped` Restart always except when the user has manually stopped the container
    /// - `on-failure` Restart only when the container exit code is non-zero
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// - Empty string means not to restart
/// - `no` Do not automatically restart
/// - `always` Always restart
/// - `unless-stopped` Restart always except when the user has manually stopped the container
/// - `on-failure` Restart only when the container exit code is non-zero
pub enum RestartPolicyNameInlineItem {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
    #[serde(rename = "on-failure")]
    OnFailure,
}

impl AsRef<str> for RestartPolicyNameInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            RestartPolicyNameInlineItem::Empty => "",
            RestartPolicyNameInlineItem::No => "no",
            RestartPolicyNameInlineItem::Always => "always",
            RestartPolicyNameInlineItem::UnlessStopped => "unless-stopped",
            RestartPolicyNameInlineItem::OnFailure => "on-failure",
        }
    }
}

impl std::fmt::Display for RestartPolicyNameInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec)
/// runtime.
///
/// The runtime is invoked by the daemon via the `containerd` daemon. OCI
/// runtimes act as an interface to the Linux kernel namespaces, cgroups,
/// and SELinux.
pub struct Runtime {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name and, optional, path, of the OCI executable binary.
    ///
    /// If the path is omitted, the daemon searches the host's `$PATH` for the
    /// binary and uses the first result.
    pub path: Option<String>,
    #[serde(rename = "runtimeArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of command-line arguments to pass to the runtime when invoked.
    pub runtime_args: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretCreateBodyParam {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5))
    /// data to store as secret.
    ///
    /// This field is only used to _create_ a secret, and is not returned by
    /// other endpoints.
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<Driver>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the secret.
    pub name: Option<String>,
    #[serde(rename = "Templating")]
    pub templating: Option<Driver>,
}

/// no error
pub type SecretList200Response = Vec<Secret>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecretSpec {
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5))
    /// data to store as secret.
    ///
    /// This field is only used to _create_ a secret, and is not returned by
    /// other endpoints.
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<Driver>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the secret.
    pub name: Option<String>,
    #[serde(rename = "Templating")]
    pub templating: Option<Driver>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<ServiceEndpointInlineItem>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of the service when it is in one of ReplicatedJob or
    /// GlobalJob modes. Absent on Replicated and Global mode services. The
    /// JobIteration is an ObjectVersion, but unlike the Service's version,
    /// does not need to be sent with an update request.
    pub job_status: Option<ServiceJobStatusInlineItem>,
    #[serde(rename = "ServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of the service's tasks. Provided only when requested as
    /// part of a ServiceList operation.
    pub service_status: Option<ServiceServiceStatusInlineItem>,
    #[serde(rename = "Spec")]
    pub spec: Option<ServiceSpec>,
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of a service update.
    pub update_status: Option<ServiceUpdateStatusInlineItem>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// no error
pub struct ServiceCreate201Response {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the created service.
    pub id: Option<String>,
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional warning message
    pub warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable configuration for a service.
pub struct ServiceCreateBodyParam {
    #[serde(rename = "EndpointSpec")]
    pub endpoint_spec: Option<EndpointSpec>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scheduling mode for the service.
    pub mode: Option<ServiceCreateBodyParamModeInlineItem>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the service.
    pub name: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies which networks the service should attach to.
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the rollback strategy of the service.
    pub rollback_config: Option<ServiceCreateBodyParamRollbackConfigInlineItem>,
    #[serde(rename = "TaskTemplate")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the update strategy of the service.
    pub update_config: Option<ServiceCreateBodyParamUpdateConfigInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Scheduling mode for the service.
pub struct ServiceCreateBodyParamModeInlineItem {
    #[serde(rename = "Global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<Value>,
    #[serde(rename = "GlobalJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services which run a task to the completed state
    /// on each valid node.
    pub global_job: Option<Value>,
    #[serde(rename = "Replicated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated: Option<ServiceCreateBodyParamModeInlineItemReplicatedInlineItem>,
    #[serde(rename = "ReplicatedJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services with a finite number of tasks that run
    /// to a completed state.
    pub replicated_job: Option<ServiceCreateBodyParamModeInlineItemReplicatedJobInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceCreateBodyParamModeInlineItemReplicatedInlineItem {
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mode used for services with a finite number of tasks that run
/// to a completed state.
pub struct ServiceCreateBodyParamModeInlineItemReplicatedJobInlineItem {
    #[serde(rename = "MaxConcurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of replicas to run simultaneously.
    pub max_concurrent: Option<i64>,
    #[serde(rename = "TotalCompletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total number of replicas desired to reach the Completed
    /// state. If unset, will default to the value of `MaxConcurrent`
    pub total_completions: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the rollback strategy of the service.
pub struct ServiceCreateBodyParamRollbackConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between rollback iterations, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an rolled back task fails to run, or stops
    /// running during the rollback.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during a rollback before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each rolled back task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling back a task. Either the old
    /// task is shut down before the new task is started, or the new task
    /// is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be rolled back in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an rolled back task fails to run, or stops
/// running during the rollback.
pub enum ServiceCreateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}

impl AsRef<str> for ServiceCreateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceCreateBodyParamRollbackConfigInlineItemFailureActionInlineItem::Continue => {
                "continue"
            }
            ServiceCreateBodyParamRollbackConfigInlineItemFailureActionInlineItem::Pause => "pause",
        }
    }
}

impl std::fmt::Display for ServiceCreateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling back a task. Either the old
/// task is shut down before the new task is started, or the new task
/// is started before the old task is shut down.
pub enum ServiceCreateBodyParamRollbackConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceCreateBodyParamRollbackConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceCreateBodyParamRollbackConfigInlineItemOrderInlineItem::StopFirst => {
                "stop-first"
            }
            ServiceCreateBodyParamRollbackConfigInlineItemOrderInlineItem::StartFirst => {
                "start-first"
            }
        }
    }
}

impl std::fmt::Display for ServiceCreateBodyParamRollbackConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the update strategy of the service.
pub struct ServiceCreateBodyParamUpdateConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between updates, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an updated task fails to run, or stops running
    /// during the update.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during an update before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each updated task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling out an updated task. Either
    /// the old task is shut down before the new task is started, or the
    /// new task is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be updated in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an updated task fails to run, or stops running
/// during the update.
pub enum ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "rollback")]
    Rollback,
}

impl AsRef<str> for ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Continue => {
                "continue"
            }
            ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Pause => "pause",
            ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Rollback => {
                "rollback"
            }
        }
    }
}

impl std::fmt::Display for ServiceCreateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling out an updated task. Either
/// the old task is shut down before the new task is started, or the
/// new task is started before the old task is shut down.
pub enum ServiceCreateBodyParamUpdateConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceCreateBodyParamUpdateConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceCreateBodyParamUpdateConfigInlineItemOrderInlineItem::StopFirst => "stop-first",
            ServiceCreateBodyParamUpdateConfigInlineItemOrderInlineItem::StartFirst => {
                "start-first"
            }
        }
    }
}

impl std::fmt::Display for ServiceCreateBodyParamUpdateConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpointInlineItem {
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<EndpointPortConfig>>,
    #[serde(rename = "Spec")]
    pub spec: Option<EndpointSpec>,
    #[serde(rename = "VirtualIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_i_ps: Option<Vec<ServiceEndpointInlineItemVirtualIPsInlineItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpointInlineItemVirtualIPsInlineItem {
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The status of the service when it is in one of ReplicatedJob or
/// GlobalJob modes. Absent on Replicated and Global mode services. The
/// JobIteration is an ObjectVersion, but unlike the Service's version,
/// does not need to be sent with an update request.
pub struct ServiceJobStatusInlineItem {
    #[serde(rename = "JobIteration")]
    pub job_iteration: Option<ObjectVersion>,
    #[serde(rename = "LastExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The last time, as observed by the server, that this job was
    /// started.
    pub last_execution: Option<DateTime<Utc>>,
}

/// no error
pub type ServiceList200Response = Vec<Service>;

/// logs returned as a stream in response body
pub type ServiceLogs200Response = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The status of the service's tasks. Provided only when requested as
/// part of a ServiceList operation.
pub struct ServiceServiceStatusInlineItem {
    #[serde(rename = "CompletedTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of tasks for a job that are in the Completed state.
    /// This field must be cross-referenced with the service type, as the
    /// value of 0 may mean the service is not in a job mode, or it may
    /// mean the job-mode service has no tasks yet Completed.
    pub completed_tasks: Option<u64>,
    #[serde(rename = "DesiredTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of tasks for the service desired to be running.
    /// For replicated services, this is the replica count from the
    /// service spec. For global services, this is computed by taking
    /// count of all tasks for the service with a Desired State other
    /// than Shutdown.
    pub desired_tasks: Option<u64>,
    #[serde(rename = "RunningTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of tasks for the service currently in the Running state.
    pub running_tasks: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable configuration for a service.
pub struct ServiceSpec {
    #[serde(rename = "EndpointSpec")]
    pub endpoint_spec: Option<EndpointSpec>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scheduling mode for the service.
    pub mode: Option<ServiceSpecModeInlineItem>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the service.
    pub name: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies which networks the service should attach to.
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the rollback strategy of the service.
    pub rollback_config: Option<ServiceSpecRollbackConfigInlineItem>,
    #[serde(rename = "TaskTemplate")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the update strategy of the service.
    pub update_config: Option<ServiceSpecUpdateConfigInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Scheduling mode for the service.
pub struct ServiceSpecModeInlineItem {
    #[serde(rename = "Global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<Value>,
    #[serde(rename = "GlobalJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services which run a task to the completed state
    /// on each valid node.
    pub global_job: Option<Value>,
    #[serde(rename = "Replicated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated: Option<ServiceSpecModeInlineItemReplicatedInlineItem>,
    #[serde(rename = "ReplicatedJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services with a finite number of tasks that run
    /// to a completed state.
    pub replicated_job: Option<ServiceSpecModeInlineItemReplicatedJobInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecModeInlineItemReplicatedInlineItem {
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mode used for services with a finite number of tasks that run
/// to a completed state.
pub struct ServiceSpecModeInlineItemReplicatedJobInlineItem {
    #[serde(rename = "MaxConcurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of replicas to run simultaneously.
    pub max_concurrent: Option<i64>,
    #[serde(rename = "TotalCompletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total number of replicas desired to reach the Completed
    /// state. If unset, will default to the value of `MaxConcurrent`
    pub total_completions: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the rollback strategy of the service.
pub struct ServiceSpecRollbackConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between rollback iterations, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an rolled back task fails to run, or stops
    /// running during the rollback.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during a rollback before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each rolled back task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling back a task. Either the old
    /// task is shut down before the new task is started, or the new task
    /// is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be rolled back in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an rolled back task fails to run, or stops
/// running during the rollback.
pub enum ServiceSpecRollbackConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}

impl AsRef<str> for ServiceSpecRollbackConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecRollbackConfigInlineItemFailureActionInlineItem::Continue => "continue",
            ServiceSpecRollbackConfigInlineItemFailureActionInlineItem::Pause => "pause",
        }
    }
}

impl std::fmt::Display for ServiceSpecRollbackConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling back a task. Either the old
/// task is shut down before the new task is started, or the new task
/// is started before the old task is shut down.
pub enum ServiceSpecRollbackConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceSpecRollbackConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecRollbackConfigInlineItemOrderInlineItem::StopFirst => "stop-first",
            ServiceSpecRollbackConfigInlineItemOrderInlineItem::StartFirst => "start-first",
        }
    }
}

impl std::fmt::Display for ServiceSpecRollbackConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the update strategy of the service.
pub struct ServiceSpecUpdateConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between updates, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an updated task fails to run, or stops running
    /// during the update.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during an update before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each updated task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling out an updated task. Either
    /// the old task is shut down before the new task is started, or the
    /// new task is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be updated in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an updated task fails to run, or stops running
/// during the update.
pub enum ServiceSpecUpdateConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "rollback")]
    Rollback,
}

impl AsRef<str> for ServiceSpecUpdateConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecUpdateConfigInlineItemFailureActionInlineItem::Continue => "continue",
            ServiceSpecUpdateConfigInlineItemFailureActionInlineItem::Pause => "pause",
            ServiceSpecUpdateConfigInlineItemFailureActionInlineItem::Rollback => "rollback",
        }
    }
}

impl std::fmt::Display for ServiceSpecUpdateConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling out an updated task. Either
/// the old task is shut down before the new task is started, or the
/// new task is started before the old task is shut down.
pub enum ServiceSpecUpdateConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceSpecUpdateConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecUpdateConfigInlineItemOrderInlineItem::StopFirst => "stop-first",
            ServiceSpecUpdateConfigInlineItemOrderInlineItem::StartFirst => "start-first",
        }
    }
}

impl std::fmt::Display for ServiceSpecUpdateConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable configuration for a service.
pub struct ServiceUpdateBodyParam {
    #[serde(rename = "EndpointSpec")]
    pub endpoint_spec: Option<EndpointSpec>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scheduling mode for the service.
    pub mode: Option<ServiceUpdateBodyParamModeInlineItem>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the service.
    pub name: Option<String>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies which networks the service should attach to.
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the rollback strategy of the service.
    pub rollback_config: Option<ServiceUpdateBodyParamRollbackConfigInlineItem>,
    #[serde(rename = "TaskTemplate")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the update strategy of the service.
    pub update_config: Option<ServiceUpdateBodyParamUpdateConfigInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Scheduling mode for the service.
pub struct ServiceUpdateBodyParamModeInlineItem {
    #[serde(rename = "Global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<Value>,
    #[serde(rename = "GlobalJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services which run a task to the completed state
    /// on each valid node.
    pub global_job: Option<Value>,
    #[serde(rename = "Replicated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated: Option<ServiceUpdateBodyParamModeInlineItemReplicatedInlineItem>,
    #[serde(rename = "ReplicatedJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The mode used for services with a finite number of tasks that run
    /// to a completed state.
    pub replicated_job: Option<ServiceUpdateBodyParamModeInlineItemReplicatedJobInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateBodyParamModeInlineItemReplicatedInlineItem {
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The mode used for services with a finite number of tasks that run
/// to a completed state.
pub struct ServiceUpdateBodyParamModeInlineItemReplicatedJobInlineItem {
    #[serde(rename = "MaxConcurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of replicas to run simultaneously.
    pub max_concurrent: Option<i64>,
    #[serde(rename = "TotalCompletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total number of replicas desired to reach the Completed
    /// state. If unset, will default to the value of `MaxConcurrent`
    pub total_completions: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the rollback strategy of the service.
pub struct ServiceUpdateBodyParamRollbackConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between rollback iterations, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an rolled back task fails to run, or stops
    /// running during the rollback.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during a rollback before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each rolled back task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling back a task. Either the old
    /// task is shut down before the new task is started, or the new task
    /// is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be rolled back in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an rolled back task fails to run, or stops
/// running during the rollback.
pub enum ServiceUpdateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}

impl AsRef<str> for ServiceUpdateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateBodyParamRollbackConfigInlineItemFailureActionInlineItem::Continue => {
                "continue"
            }
            ServiceUpdateBodyParamRollbackConfigInlineItemFailureActionInlineItem::Pause => "pause",
        }
    }
}

impl std::fmt::Display for ServiceUpdateBodyParamRollbackConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling back a task. Either the old
/// task is shut down before the new task is started, or the new task
/// is started before the old task is shut down.
pub enum ServiceUpdateBodyParamRollbackConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceUpdateBodyParamRollbackConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateBodyParamRollbackConfigInlineItemOrderInlineItem::StopFirst => {
                "stop-first"
            }
            ServiceUpdateBodyParamRollbackConfigInlineItemOrderInlineItem::StartFirst => {
                "start-first"
            }
        }
    }
}

impl std::fmt::Display for ServiceUpdateBodyParamRollbackConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the update strategy of the service.
pub struct ServiceUpdateBodyParamUpdateConfigInlineItem {
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time between updates, in nanoseconds.
    pub delay: Option<i64>,
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Action to take if an updated task fails to run, or stops running
    /// during the update.
    pub failure_action: Option<String>,
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The fraction of tasks that may fail during an update before the
    /// failure action is invoked, specified as a floating point number
    /// between 0 and 1.
    pub max_failure_ratio: Option<Value>,
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to monitor each updated task for failures, in
    /// nanoseconds.
    pub monitor: Option<i64>,
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The order of operations when rolling out an updated task. Either
    /// the old task is shut down before the new task is started, or the
    /// new task is started before the old task is shut down.
    pub order: Option<String>,
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of tasks to be updated in one iteration (0 means
    /// unlimited parallelism).
    pub parallelism: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Action to take if an updated task fails to run, or stops running
/// during the update.
pub enum ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "rollback")]
    Rollback,
}

impl AsRef<str> for ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Continue => {
                "continue"
            }
            ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Pause => "pause",
            ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem::Rollback => {
                "rollback"
            }
        }
    }
}

impl std::fmt::Display for ServiceUpdateBodyParamUpdateConfigInlineItemFailureActionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The order of operations when rolling out an updated task. Either
/// the old task is shut down before the new task is started, or the
/// new task is started before the old task is shut down.
pub enum ServiceUpdateBodyParamUpdateConfigInlineItemOrderInlineItem {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}

impl AsRef<str> for ServiceUpdateBodyParamUpdateConfigInlineItemOrderInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateBodyParamUpdateConfigInlineItemOrderInlineItem::StopFirst => "stop-first",
            ServiceUpdateBodyParamUpdateConfigInlineItemOrderInlineItem::StartFirst => {
                "start-first"
            }
        }
    }
}

impl std::fmt::Display for ServiceUpdateBodyParamUpdateConfigInlineItemOrderInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateResponse {
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional warning messages
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The status of a service update.
pub struct ServiceUpdateStatusInlineItem {
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceUpdateStatusInlineItemStateInlineItem {
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}

impl AsRef<str> for ServiceUpdateStatusInlineItemStateInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateStatusInlineItemStateInlineItem::Updating => "updating",
            ServiceUpdateStatusInlineItemStateInlineItem::Paused => "paused",
            ServiceUpdateStatusInlineItemStateInlineItem::Completed => "completed",
        }
    }
}

impl std::fmt::Display for ServiceUpdateStatusInlineItemStateInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// ClusterInfo represents information about the swarm as is returned by the
/// "/info" endpoint. Join-tokens are not included.
pub struct Swarm {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the swarm was initialised in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DataPathPort specifies the data path port number for data traffic.
    /// Acceptable port range is 1024 to 49151.
    /// If no port is set or is set to 0, the default port (4789) is used.
    pub data_path_port: Option<u32>,
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Default Address Pool specifies default subnet pools for global scope
    /// networks.
    pub default_addr_pool: Option<Vec<String>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the swarm.
    pub id: Option<String>,
    #[serde(rename = "JoinTokens")]
    pub join_tokens: Option<JoinTokens>,
    #[serde(rename = "RootRotationInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether there is currently a root CA rotation in progress for the swarm
    pub root_rotation_in_progress: Option<bool>,
    #[serde(rename = "Spec")]
    pub spec: Option<SwarmSpec>,
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubnetSize specifies the subnet size of the networks created from the
    /// default subnet pool.
    pub subnet_size: Option<u32>,
    #[serde(rename = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the swarm was last updated in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents generic information about swarm.
pub struct SwarmInfo {
    #[serde(rename = "Cluster")]
    pub cluster: Option<ClusterInfo>,
    #[serde(rename = "ControlAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_available: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "LocalNodeState")]
    pub local_node_state: Option<String>,
    #[serde(rename = "Managers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of managers in the swarm.
    pub managers: Option<isize>,
    #[serde(rename = "NodeAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IP address at which this node can be reached by other nodes in the
    /// swarm.
    pub node_addr: Option<String>,
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier of for this node in the swarm.
    pub node_id: Option<String>,
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of nodes in the swarm.
    pub nodes: Option<isize>,
    #[serde(rename = "RemoteManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of ID's and addresses of other managers in the swarm.
    pub remote_managers: Option<Vec<PeerNode>>,
}

/// no error
pub type SwarmInit200Response = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwarmInitBodyParam {
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Externally reachable address advertised to other nodes. This
    /// can either be an address/port combination in the form
    /// `192.168.1.1:4567`, or an interface followed by a port number,
    /// like `eth0:4567`. If the port number is omitted, the port
    /// number from the listen address is used. If `AdvertiseAddr` is
    /// not specified, it will be automatically detected when possible.
    pub advertise_addr: Option<String>,
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Address or interface to use for data path traffic (format:
    /// `<ip|interface>`), for example,  `192.168.1.1`, or an interface,
    /// like `eth0`. If `DataPathAddr` is unspecified, the same address
    /// as `AdvertiseAddr` is used.
    ///
    /// The `DataPathAddr` specifies the address that global scope
    /// network drivers will publish towards other  nodes in order to
    /// reach the containers running on this node. Using this parameter
    /// it is possible to separate the container data traffic from the
    /// management traffic of the cluster.
    pub data_path_addr: Option<String>,
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// DataPathPort specifies the data path port number for data traffic.
    /// Acceptable port range is 1024 to 49151.
    /// if no port is set or is set to 0, default port 4789 will be used.
    pub data_path_port: Option<u32>,
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Default Address Pool specifies default subnet pools for global
    /// scope networks.
    pub default_addr_pool: Option<Vec<String>>,
    #[serde(rename = "ForceNewCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Force creation of a new swarm.
    pub force_new_cluster: Option<bool>,
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Listen address used for inter-manager communication, as well
    /// as determining the networking interface used for the VXLAN
    /// Tunnel Endpoint (VTEP). This can either be an address/port
    /// combination in the form `192.168.1.1:4567`, or an interface
    /// followed by a port number, like `eth0:4567`. If the port number
    /// is omitted, the default swarm listening port is used.
    pub listen_addr: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<SwarmSpec>,
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubnetSize specifies the subnet size of the networks created
    /// from the default subnet pool.
    pub subnet_size: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwarmJoinBodyParam {
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Externally reachable address advertised to other nodes. This
    /// can either be an address/port combination in the form
    /// `192.168.1.1:4567`, or an interface followed by a port number,
    /// like `eth0:4567`. If the port number is omitted, the port
    /// number from the listen address is used. If `AdvertiseAddr` is
    /// not specified, it will be automatically detected when possible.
    pub advertise_addr: Option<String>,
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Address or interface to use for data path traffic (format:
    /// `<ip|interface>`), for example,  `192.168.1.1`, or an interface,
    /// like `eth0`. If `DataPathAddr` is unspecified, the same addres
    /// as `AdvertiseAddr` is used.
    ///
    /// The `DataPathAddr` specifies the address that global scope
    /// network drivers will publish towards other nodes in order to
    /// reach the containers running on this node. Using this parameter
    /// it is possible to separate the container data traffic from the
    /// management traffic of the cluster.
    pub data_path_addr: Option<String>,
    #[serde(rename = "JoinToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secret token for joining this swarm.
    pub join_token: Option<String>,
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Listen address used for inter-manager communication if the node
    /// gets promoted to manager, as well as determining the networking
    /// interface used for the VXLAN Tunnel Endpoint (VTEP).
    pub listen_addr: Option<String>,
    #[serde(rename = "RemoteAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Addresses of manager nodes already participating in the swarm.
    pub remote_addrs: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable swarm configuration.
pub struct SwarmSpec {
    #[serde(rename = "CAConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CA configuration.
    pub ca_config: Option<SwarmSpecCaConfigInlineItem>,
    #[serde(rename = "Dispatcher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Dispatcher configuration.
    pub dispatcher: Option<SwarmSpecDispatcherInlineItem>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Parameters related to encryption-at-rest.
    pub encryption_config: Option<SwarmSpecEncryptionConfigInlineItem>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the swarm.
    pub name: Option<String>,
    #[serde(rename = "Orchestration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Orchestration configuration.
    pub orchestration: Option<SwarmSpecOrchestrationInlineItem>,
    #[serde(rename = "Raft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Raft configuration.
    pub raft: Option<SwarmSpecRaftInlineItem>,
    #[serde(rename = "TaskDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defaults for creating tasks in this cluster.
    pub task_defaults: Option<SwarmSpecTaskDefaultsInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// CA configuration.
pub struct SwarmSpecCaConfigInlineItem {
    #[serde(rename = "ExternalCAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Configuration for forwarding signing requests to an external
    /// certificate authority.
    pub external_c_as: Option<Vec<SwarmSpecCaConfigInlineItemExternalCAsInlineItem>>,
    #[serde(rename = "ForceRotate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An integer whose purpose is to force swarm to generate a new
    /// signing CA certificate and key, if none have been specified in
    /// `SigningCACert` and `SigningCAKey`
    pub force_rotate: Option<u64>,
    #[serde(rename = "NodeCertExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The duration node certificates are issued for.
    pub node_cert_expiry: Option<i64>,
    #[serde(rename = "SigningCACert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The desired signing CA certificate for all swarm node TLS leaf
    /// certificates, in PEM format.
    pub signing_ca_cert: Option<String>,
    #[serde(rename = "SigningCAKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The desired signing CA key for all swarm node TLS leaf certificates,
    /// in PEM format.
    pub signing_ca_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecCaConfigInlineItemExternalCAsInlineItem {
    #[serde(rename = "CACert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The root CA certificate (in PEM format) this external CA uses
    /// to issue TLS certificates (assumed to be to the current swarm
    /// root CA certificate if not provided).
    pub ca_cert: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An object with key/value pairs that are interpreted as
    /// protocol-specific options for the external CA driver.
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Protocol for communication with the external CA (currently
    /// only `cfssl` is supported).
    pub protocol: Option<String>,
    #[serde(rename = "URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL where certificate signing requests should be sent.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Protocol for communication with the external CA (currently
/// only `cfssl` is supported).
pub enum SwarmSpecCaConfigInlineItemExternalCAsInlineItemProtocolInlineItem {
    #[serde(rename = "cfssl")]
    Cfssl,
}

impl AsRef<str> for SwarmSpecCaConfigInlineItemExternalCAsInlineItemProtocolInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            SwarmSpecCaConfigInlineItemExternalCAsInlineItemProtocolInlineItem::Cfssl => "cfssl",
        }
    }
}

impl std::fmt::Display for SwarmSpecCaConfigInlineItemExternalCAsInlineItemProtocolInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Dispatcher configuration.
pub struct SwarmSpecDispatcherInlineItem {
    #[serde(rename = "HeartbeatPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The delay for an agent to send a heartbeat to the dispatcher.
    pub heartbeat_period: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Parameters related to encryption-at-rest.
pub struct SwarmSpecEncryptionConfigInlineItem {
    #[serde(rename = "AutoLockManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, generate a key and use it to lock data stored on the
    /// managers.
    pub auto_lock_managers: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Orchestration configuration.
pub struct SwarmSpecOrchestrationInlineItem {
    #[serde(rename = "TaskHistoryRetentionLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of historic tasks to keep per instance or node. If
    /// negative, never remove completed or failed tasks.
    pub task_history_retention_limit: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Raft configuration.
pub struct SwarmSpecRaftInlineItem {
    #[serde(rename = "ElectionTick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of ticks that a follower will wait for a message from
    /// the leader before becoming a candidate and starting an election.
    /// `ElectionTick` must be greater than `HeartbeatTick`.
    ///
    /// A tick currently defaults to one second, so these translate
    /// directly to seconds currently, but this is NOT guaranteed.
    pub election_tick: Option<isize>,
    #[serde(rename = "HeartbeatTick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of ticks between heartbeats. Every HeartbeatTick ticks,
    /// the leader will send a heartbeat to the followers.
    ///
    /// A tick currently defaults to one second, so these translate
    /// directly to seconds currently, but this is NOT guaranteed.
    pub heartbeat_tick: Option<isize>,
    #[serde(rename = "KeepOldSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of snapshots to keep beyond the current snapshot.
    pub keep_old_snapshots: Option<u64>,
    #[serde(rename = "LogEntriesForSlowFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of log entries to keep around to sync up slow followers
    /// after a snapshot is created.
    pub log_entries_for_slow_followers: Option<u64>,
    #[serde(rename = "SnapshotInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of log entries between snapshots.
    pub snapshot_interval: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Defaults for creating tasks in this cluster.
pub struct SwarmSpecTaskDefaultsInlineItem {
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The log driver to use for tasks created in the orchestrator if
    /// unspecified by a service.
    ///
    /// Updating this value only affects new tasks. Existing tasks continue
    /// to use their previously configured log driver until recreated.
    pub log_driver: Option<SwarmSpecTaskDefaultsInlineItemLogDriverInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The log driver to use for tasks created in the orchestrator if
/// unspecified by a service.
///
/// Updating this value only affects new tasks. Existing tasks continue
/// to use their previously configured log driver until recreated.
pub struct SwarmSpecTaskDefaultsInlineItemLogDriverInlineItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The log driver to use as a default for new tasks.
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Driver-specific options for the selectd log driver, specified
    /// as key/value pairs.
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwarmUnlockBodyParam {
    #[serde(rename = "UnlockKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The swarm's unlock key.
    pub unlock_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// no error
pub struct SwarmUnlockkey200Response {
    #[serde(rename = "UnlockKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The swarm's unlock key.
    pub unlock_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// An identity token was generated successfully.
pub struct SystemAuth200Response {
    #[serde(rename = "IdentityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An opaque token used to authenticate a user after a successful login
    pub identity_token: Option<String>,
    #[serde(rename = "Status")]
    /// The status of the authentication
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// no error
pub struct SystemDataUsage200Response {
    #[serde(rename = "BuildCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_cache: Option<Vec<BuildCache>>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerSummary>>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageSummary>>,
    #[serde(rename = "LayersSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers_size: Option<i64>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hardware architecture of the host, as returned by the Go runtime
    /// (`GOARCH`).
    ///
    /// A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment).
    pub architecture: Option<String>,
    #[serde(rename = "BridgeNfIp6tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if `bridge-nf-call-ip6tables` is available on the host.
    pub bridge_nf_ip_6_tables: Option<bool>,
    #[serde(rename = "BridgeNfIptables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if `bridge-nf-call-iptables` is available on the host.
    pub bridge_nf_iptables: Option<bool>,
    #[serde(rename = "CPUSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.
    ///
    /// See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt)
    pub cpu_set: Option<bool>,
    #[serde(rename = "CPUShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if CPU Shares limiting is supported by the host.
    pub cpu_shares: Option<bool>,
    #[serde(rename = "CgroupDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The driver to use for managing cgroups.
    pub cgroup_driver: Option<String>,
    #[serde(rename = "CgroupVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version of the cgroup.
    pub cgroup_version: Option<String>,
    #[serde(rename = "ClusterAdvertise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The network endpoint that the Engine advertises for the purpose of
    /// node discovery. ClusterAdvertise is a `host:port` combination on which
    /// the daemon is reachable by other hosts.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when using standalone Swarm
    /// > mode, and overlay networking using an external k/v store. Overlay
    /// > networks with Swarm mode enabled use the built-in raft store, and
    /// > this field will be empty.
    pub cluster_advertise: Option<String>,
    #[serde(rename = "ClusterStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URL of the distributed storage backend.
    ///
    ///
    /// The storage backend is used for multihost networking (to store
    /// network and endpoint information) and by the node discovery mechanism.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is only propagated when using standalone Swarm
    /// > mode, and overlay networking using an external k/v store. Overlay
    /// > networks with Swarm mode enabled use the built-in raft store, and
    /// > this field will be empty.
    pub cluster_store: Option<String>,
    #[serde(rename = "ContainerdCommit")]
    pub containerd_commit: Option<Commit>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of containers on the host.
    pub containers: Option<isize>,
    #[serde(rename = "ContainersPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"paused"`.
    pub containers_paused: Option<isize>,
    #[serde(rename = "ContainersRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"running"`.
    pub containers_running: Option<isize>,
    #[serde(rename = "ContainersStopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"stopped"`.
    pub containers_stopped: Option<isize>,
    #[serde(rename = "CpuCfsPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if CPU CFS(Completely Fair Scheduler) period is supported by
    /// the host.
    pub cpu_cfs_period: Option<bool>,
    #[serde(rename = "CpuCfsQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by
    /// the host.
    pub cpu_cfs_quota: Option<bool>,
    #[serde(rename = "Debug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the daemon is running in debug-mode / with debug-level
    /// logging enabled.
    pub debug: Option<bool>,
    #[serde(rename = "DefaultAddressPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of custom default address pools for local networks, which can be
    /// specified in the daemon.json file or dockerd option.
    ///
    /// Example: a Base "10.10.0.0/16" with Size 24 will define the set of 256
    /// 10.10.[0-255].0/24 address pools.
    pub default_address_pools: Option<Vec<SystemInfoDefaultAddressPoolsInlineItem>>,
    #[serde(rename = "DefaultRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the default OCI runtime that is used when starting containers.
    ///
    /// The default can be overridden per-container at create time.
    pub default_runtime: Option<String>,
    #[serde(rename = "DockerRootDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Root directory of persistent Docker state.
    ///
    /// Defaults to `/var/lib/docker` on Linux, and `C:\ProgramData\docker`
    /// on Windows.
    pub docker_root_dir: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the storage driver in use.
    pub driver: Option<String>,
    #[serde(rename = "DriverStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information specific to the storage driver, provided as
    /// "label" / "value" pairs.
    ///
    /// This information is provided by the storage driver, and formatted
    /// in a way consistent with the output of `docker info` on the command
    /// line.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: The information returned in this field, including the
    /// > formatting of values and labels, should not be considered stable,
    /// > and may change without notice.
    pub driver_status: Option<Vec<Vec<String>>>,
    #[serde(rename = "ExperimentalBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if experimental features are enabled on the daemon.
    pub experimental_build: Option<bool>,
    #[serde(rename = "GenericResources")]
    pub generic_resources: Option<GenericResources>,
    #[serde(rename = "HttpProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTP-proxy configured for the daemon. This value is obtained from the
    /// [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.
    /// Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL
    /// are masked in the API response.
    ///
    /// Containers do not automatically inherit this configuration.
    pub http_proxy: Option<String>,
    #[serde(rename = "HttpsProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// HTTPS-proxy configured for the daemon. This value is obtained from the
    /// [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.
    /// Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL
    /// are masked in the API response.
    ///
    /// Containers do not automatically inherit this configuration.
    pub https_proxy: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Unique identifier of the daemon.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: The format of the ID itself is not part of the API, and
    /// > should not be considered stable.
    pub id: Option<String>,
    #[serde(rename = "IPv4Forwarding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates IPv4 forwarding is enabled.
    pub i_pv_4_forwarding: Option<bool>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of images on the host.
    ///
    /// Both _tagged_ and _untagged_ (dangling) images are counted.
    pub images: Option<isize>,
    #[serde(rename = "IndexServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Address / URL of the index server that is used for image search,
    /// and as a default for user authentication for Docker Hub and Docker Cloud.
    pub index_server_address: Option<String>,
    #[serde(rename = "InitBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name and, optional, path of the `docker-init` binary.
    ///
    /// If the path is omitted, the daemon searches the host's `$PATH` for the
    /// binary and uses the first result.
    pub init_binary: Option<String>,
    #[serde(rename = "InitCommit")]
    pub init_commit: Option<Commit>,
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Represents the isolation technology to use as a default for containers.
    /// The supported values are platform-specific.
    ///
    /// If no isolation value is specified on daemon start, on Windows client,
    /// the default is `hyperv`, and on Windows server, the default is `process`.
    ///
    /// This option is currently not used on other platforms.
    pub isolation: Option<String>,
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the host has kernel memory limit support enabled.
    ///
    /// <p><br /></p>
    ///
    /// > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated
    /// > `kmem.limit_in_bytes`.
    pub kernel_memory: Option<bool>,
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the host has kernel memory TCP limit support enabled.
    ///
    /// Kernel memory TCP limits are not supported when using cgroups v2, which
    /// does not support the corresponding `memory.kmem.tcp.limit_in_bytes` cgroup.
    pub kernel_memory_tcp: Option<bool>,
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Kernel version of the host.
    ///
    /// On Linux, this information obtained from `uname`. On Windows this
    /// information is queried from the <kbd>HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\</kbd>
    /// registry value, for example _"10.0 14393 (14393.1198.amd64fre.rs1_release_sec.170427-1353)"_.
    pub kernel_version: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined labels (key/value metadata) as set on the daemon.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: When part of a Swarm, nodes can both have _daemon_ labels,
    /// > set through the daemon configuration, and _node_ labels, set from a
    /// > manager node in the Swarm. Node labels are not included in this
    /// > field. Node labels can be retrieved using the `/nodes/(id)` endpoint
    /// > on a manager node in the Swarm.
    pub labels: Option<Vec<String>>,
    #[serde(rename = "LiveRestoreEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if live restore is enabled.
    ///
    /// If enabled, containers are kept running when the daemon is shutdown
    /// or upon daemon start if running containers are detected.
    pub live_restore_enabled: Option<bool>,
    #[serde(rename = "LoggingDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The logging driver to use as a default for new containers.
    pub logging_driver: Option<String>,
    #[serde(rename = "MemTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total amount of physical memory available on the host, in bytes.
    pub mem_total: Option<i64>,
    #[serde(rename = "MemoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the host has memory limit support enabled.
    pub memory_limit: Option<bool>,
    #[serde(rename = "NCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of logical CPUs usable by the daemon.
    ///
    /// The number of available CPUs is checked by querying the operating
    /// system when the daemon starts. Changes to operating system CPU
    /// allocation after the daemon is started are not reflected.
    pub ncpu: Option<isize>,
    #[serde(rename = "NEventsListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of event listeners subscribed.
    pub n_events_listener: Option<isize>,
    #[serde(rename = "NFd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total number of file Descriptors in use by the daemon process.
    ///
    /// This information is only returned if debug-mode is enabled.
    pub n_fd: Option<isize>,
    #[serde(rename = "NGoroutines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The  number of goroutines that currently exist.
    ///
    /// This information is only returned if debug-mode is enabled.
    pub n_goroutines: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hostname of the host.
    pub name: Option<String>,
    #[serde(rename = "NoProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Comma-separated list of domain extensions for which no proxy should be
    /// used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html)
    /// environment variable.
    ///
    /// Containers do not automatically inherit this configuration.
    pub no_proxy: Option<String>,
    #[serde(rename = "OSType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Generic type of the operating system of the host, as returned by the
    /// Go runtime (`GOOS`).
    ///
    /// Currently returned values are "linux" and "windows". A full list of
    /// possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment).
    pub os_type: Option<String>,
    #[serde(rename = "OSVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Version of the host's operating system
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: The information returned in this field, including its
    /// > very existence, and the formatting of values, should not be considered
    /// > stable, and may change without notice.
    pub os_version: Option<String>,
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if OOM killer disable is supported on the host.
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the host's operating system, for example: "Ubuntu 16.04.2 LTS"
    /// or "Windows Server 2016 Datacenter"
    pub operating_system: Option<String>,
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the host kernel has PID limit support enabled.
    pub pids_limit: Option<bool>,
    #[serde(rename = "Plugins")]
    pub plugins: Option<PluginsInfo>,
    #[serde(rename = "ProductLicense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reports a summary of the product license on the daemon.
    ///
    /// If a commercial license has been applied to the daemon, information
    /// such as number of nodes, and expiration are included.
    pub product_license: Option<String>,
    #[serde(rename = "RegistryConfig")]
    pub registry_config: Option<RegistryServiceConfig>,
    #[serde(rename = "RuncCommit")]
    pub runc_commit: Option<Commit>,
    #[serde(rename = "Runtimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of [OCI compliant](https://github.com/opencontainers/runtime-spec)
    /// runtimes configured on the daemon. Keys hold the "name" used to
    /// reference the runtime.
    ///
    /// The Docker daemon relies on an OCI compliant runtime (invoked via the
    /// `containerd` daemon) as its interface to the Linux kernel namespaces,
    /// cgroups, and SELinux.
    ///
    /// The default runtime is `runc`, and automatically configured. Additional
    /// runtimes can be configured by the user and will be listed here.
    pub runtimes: Option<HashMap<String, Runtime>>,
    #[serde(rename = "SecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of security features that are enabled on the daemon, such as
    /// apparmor, seccomp, SELinux, user-namespaces (userns), and rootless.
    ///
    /// Additional configuration options for each security feature may
    /// be present, and are included as a comma-separated list of key/value
    /// pairs.
    pub security_options: Option<Vec<String>>,
    #[serde(rename = "ServerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Version string of the daemon.
    ///
    /// > **Note**: the [standalone Swarm API](/swarm/swarm-api/)
    /// > returns the Swarm version instead of the daemon  version, for example
    /// > `swarm/1.2.8`.
    pub server_version: Option<String>,
    #[serde(rename = "SwapLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the host has memory swap limit support enabled.
    pub swap_limit: Option<bool>,
    #[serde(rename = "Swarm")]
    pub swarm: Option<SwarmInfo>,
    #[serde(rename = "SystemTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Current system-time in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt)
    /// format with nano-seconds.
    pub system_time: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of warnings / informational messages about missing features, or
    /// issues related to the daemon configuration.
    ///
    /// These messages can be printed by the client as information to the user.
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The driver to use for managing cgroups.
pub enum SystemInfoCgroupDriverInlineItem {
    #[serde(rename = "cgroupfs")]
    Cgroupfs,
    #[serde(rename = "systemd")]
    Systemd,
    #[serde(rename = "none")]
    None,
}

impl AsRef<str> for SystemInfoCgroupDriverInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoCgroupDriverInlineItem::Cgroupfs => "cgroupfs",
            SystemInfoCgroupDriverInlineItem::Systemd => "systemd",
            SystemInfoCgroupDriverInlineItem::None => "none",
        }
    }
}

impl std::fmt::Display for SystemInfoCgroupDriverInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The version of the cgroup.
pub enum SystemInfoCgroupVersionInlineItem {
    #[serde(rename = "1")]
    Value1,
    #[serde(rename = "2")]
    Value2,
}

impl AsRef<str> for SystemInfoCgroupVersionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoCgroupVersionInlineItem::Value1 => "1",
            SystemInfoCgroupVersionInlineItem::Value2 => "2",
        }
    }
}

impl std::fmt::Display for SystemInfoCgroupVersionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoDefaultAddressPoolsInlineItem {
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The network address in CIDR format
    pub base: Option<String>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The network pool size
    pub size: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents the isolation technology to use as a default for containers.
/// The supported values are platform-specific.
///
/// If no isolation value is specified on daemon start, on Windows client,
/// the default is `hyperv`, and on Windows server, the default is `process`.
///
/// This option is currently not used on other platforms.
pub enum SystemInfoIsolationInlineItem {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "process")]
    Process,
}

impl AsRef<str> for SystemInfoIsolationInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoIsolationInlineItem::Default => "default",
            SystemInfoIsolationInlineItem::Hyperv => "hyperv",
            SystemInfoIsolationInlineItem::Process => "process",
        }
    }
}

impl std::fmt::Display for SystemInfoIsolationInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// no error
pub type SystemPing200Response = String;

/// no error
pub type SystemPingHead200Response = String;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of Engine API: GET "/version"
pub struct SystemVersion {
    #[serde(rename = "ApiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The default (and highest) API version that is supported by the daemon
    pub api_version: Option<String>,
    #[serde(rename = "Arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The architecture that the daemon is running on
    pub arch: Option<String>,
    #[serde(rename = "BuildTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The date and time that the daemon was compiled.
    pub build_time: Option<String>,
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about system components
    pub components: Option<Vec<ComponentVersion>>,
    #[serde(rename = "Experimental")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if the daemon is started with experimental features enabled.
    ///
    /// This field is omitted when empty / false.
    pub experimental: Option<bool>,
    #[serde(rename = "GitCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The Git commit of the source code that was used to build the daemon
    pub git_commit: Option<String>,
    #[serde(rename = "GoVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version Go used to compile the daemon, and the version of the Go
    /// runtime in use.
    pub go_version: Option<String>,
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kernel version (`uname -r`) that the daemon is running on.
    ///
    /// This field is omitted when empty.
    pub kernel_version: Option<String>,
    #[serde(rename = "MinAPIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The minimum API version that is supported by the daemon
    pub min_api_version: Option<String>,
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The operating system that the daemon is running on ("linux" or "windows")
    pub os: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SystemVersionPlatformInlineItem>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version of the daemon
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemVersionPlatformInlineItem {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the issuer of leaf TLS certificates and the trusted root
/// CA certificate.
pub struct TlsInfo {
    #[serde(rename = "CertIssuerPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The base64-url-safe-encoded raw public key bytes of the issuer.
    pub cert_issuer_public_key: Option<String>,
    #[serde(rename = "CertIssuerSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The base64-url-safe-encoded raw subject bytes of the issuer.
    pub cert_issuer_subject: Option<String>,
    #[serde(rename = "TrustRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The root CA certificate(s) that are used to validate leaf TLS
    /// certificates.
    pub trust_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "AssignedGenericResources")]
    pub assigned_generic_resources: Option<GenericResources>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "DesiredState")]
    pub desired_state: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the task.
    pub id: Option<String>,
    #[serde(rename = "JobIteration")]
    pub job_iteration: Option<ObjectVersion>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the task.
    pub name: Option<String>,
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the node that this task is on.
    pub node_id: Option<String>,
    #[serde(rename = "ServiceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the service this task is part of.
    pub service_id: Option<String>,
    #[serde(rename = "Slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<isize>,
    #[serde(rename = "Spec")]
    pub spec: Option<TaskSpec>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatusInlineItem>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: Option<ObjectVersion>,
}

/// no error
pub type TaskList200Response = Vec<Task>;

/// logs returned as a stream in response body
pub type TaskLogs200Response = Vec<u8>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable task configuration.
pub struct TaskSpec {
    #[serde(rename = "ContainerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Container spec for the service.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
    /// > mutually exclusive. PluginSpec is only used when the Runtime field
    /// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
    /// > field is set to `attachment`.
    pub container_spec: Option<TaskSpecContainerSpecInlineItem>,
    #[serde(rename = "ForceUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A counter that triggers an update even if no relevant parameters have
    /// been changed.
    pub force_update: Option<isize>,
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies the log driver to use for tasks created from this spec. If
    /// not present, the default one for the swarm will be used, finally
    /// falling back to the engine default if not specified.
    pub log_driver: Option<TaskSpecLogDriverInlineItem>,
    #[serde(rename = "NetworkAttachmentSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Read-only spec type for non-swarm containers attached to swarm overlay
    /// networks.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
    /// > mutually exclusive. PluginSpec is only used when the Runtime field
    /// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
    /// > field is set to `attachment`.
    pub network_attachment_spec: Option<TaskSpecNetworkAttachmentSpecInlineItem>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies which networks the service should attach to.
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "Placement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<TaskSpecPlacementInlineItem>,
    #[serde(rename = "PluginSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Plugin spec for the service.  *(Experimental release only.)*
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
    /// > mutually exclusive. PluginSpec is only used when the Runtime field
    /// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
    /// > field is set to `attachment`.
    pub plugin_spec: Option<TaskSpecPluginSpecInlineItem>,
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Resource requirements which apply to each individual container created
    /// as part of the service.
    pub resources: Option<TaskSpecResourcesInlineItem>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the restart policy which applies to containers
    /// created as part of this service.
    pub restart_policy: Option<TaskSpecRestartPolicyInlineItem>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runtime is the type of runtime specified for the task executor.
    pub runtime: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Container spec for the service.
///
/// <p><br /></p>
///
/// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
/// > mutually exclusive. PluginSpec is only used when the Runtime field
/// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
/// > field is set to `attachment`.
pub struct TaskSpecContainerSpecInlineItem {
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments to the command.
    pub args: Option<Vec<String>>,
    #[serde(rename = "CapabilityAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of kernel capabilities to add to the default set
    /// for the container.
    pub capability_add: Option<Vec<String>>,
    #[serde(rename = "CapabilityDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of kernel capabilities to drop from the default set
    /// for the container.
    pub capability_drop: Option<Vec<String>>,
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The command to be run in the image.
    pub command: Option<Vec<String>>,
    #[serde(rename = "Configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Configs contains references to zero or more configs that will be
    /// exposed to the service.
    pub configs: Option<Vec<TaskSpecContainerSpecInlineItemConfigsInlineItem>>,
    #[serde(rename = "DNSConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for DNS related configurations in resolver configuration
    /// file (`resolv.conf`).
    pub dns_config: Option<TaskSpecContainerSpecInlineItemDnsConfigInlineItem>,
    #[serde(rename = "Dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for commands to run in.
    pub dir: Option<String>,
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of environment variables in the form `VAR=value`.
    pub env: Option<Vec<String>>,
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of additional groups that the container process will run as.
    pub groups: Option<Vec<String>>,
    #[serde(rename = "HealthCheck")]
    pub health_check: Option<HealthConfig>,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The hostname to use for the container, as a valid
    /// [RFC 1123](https://tools.ietf.org/html/rfc1123) hostname.
    pub hostname: Option<String>,
    #[serde(rename = "Hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of hostname/IP mappings to add to the container's `hosts`
    /// file. The format of extra hosts is specified in the
    /// [hosts(5)](http://man7.org/linux/man-pages/man5/hosts.5.html)
    /// man page:
    ///
    ///     IP_address canonical_hostname [aliases...]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The image name to use for the container
    pub image: Option<String>,
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Run an init inside the container that forwards signals and reaps
    /// processes. This field is omitted if empty, and the default (as
    /// configured on the daemon) is used.
    pub init: Option<bool>,
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Isolation technology of the containers running the service.
    /// (Windows only)
    pub isolation: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value data.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for mounts to be added to containers created as part
    /// of the service.
    pub mounts: Option<Vec<Mount>>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Open `stdin`
    pub open_stdin: Option<bool>,
    #[serde(rename = "Privileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Security options for the container
    pub privileges: Option<TaskSpecContainerSpecInlineItemPrivilegesInlineItem>,
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mount the container's root filesystem as read only.
    pub read_only: Option<bool>,
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Secrets contains references to zero or more secrets that will be
    /// exposed to the service.
    pub secrets: Option<Vec<TaskSpecContainerSpecInlineItemSecretsInlineItem>>,
    #[serde(rename = "StopGracePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of time to wait for the container to terminate before
    /// forcefully killing it.
    pub stop_grace_period: Option<i64>,
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Signal to stop the container.
    pub stop_signal: Option<String>,
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Set kernel namedspaced parameters (sysctls) in the container.
    /// The Sysctls option on services accepts the same sysctls as the
    /// are supported on containers. Note that while the same sysctls are
    /// supported, no guarantees or checks are made about their
    /// suitability for a clustered environment, and it's up to the user
    /// to determine whether a given sysctl will work properly in a
    /// Service.
    pub sysctls: Option<HashMap<String, String>>,
    #[serde(rename = "TTY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether a pseudo-TTY should be allocated.
    pub tty: Option<bool>,
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of resource limits to set in the container. For example: `{"Name": "nofile", "Soft": 1024, "Hard": 2048}`"
    pub ulimits: Option<Vec<TaskSpecContainerSpecInlineItemUlimitsInlineItem>>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The user inside the container.
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecInlineItemConfigsInlineItem {
    #[serde(rename = "ConfigID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ConfigID represents the ID of the specific config that we're
    /// referencing.
    pub config_id: Option<String>,
    #[serde(rename = "ConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ConfigName is the name of the config that this references,
    /// but this is just provided for lookup/display purposes. The
    /// config in the reference will be identified by its ID.
    pub config_name: Option<String>,
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// File represents a specific target that is backed by a file.
    ///
    /// <p><br /><p>
    ///
    /// > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
    pub file: Option<TaskSpecContainerSpecInlineItemConfigsInlineItemFileInlineItem>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runtime represents a target that is not mounted into the
    /// container but is used by the task
    ///
    /// <p><br /><p>
    ///
    /// > **Note**: `Configs.File` and `Configs.Runtime` are mutually
    /// > exclusive
    pub runtime: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// File represents a specific target that is backed by a file.
///
/// <p><br /><p>
///
/// > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
pub struct TaskSpecContainerSpecInlineItemConfigsInlineItemFileInlineItem {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GID represents the file GID.
    pub gid: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode represents the FileMode of the file.
    pub mode: Option<u32>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name represents the final filename in the filesystem.
    pub name: Option<String>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID represents the file UID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for DNS related configurations in resolver configuration
/// file (`resolv.conf`).
pub struct TaskSpecContainerSpecInlineItemDnsConfigInlineItem {
    #[serde(rename = "Nameservers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The IP addresses of the name servers.
    pub nameservers: Option<Vec<String>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of internal resolver variables to be modified (e.g.,
    /// `debug`, `ndots:3`, etc.).
    pub options: Option<Vec<String>>,
    #[serde(rename = "Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A search list for host-name lookup.
    pub search: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Isolation technology of the containers running the service.
/// (Windows only)
pub enum TaskSpecContainerSpecInlineItemIsolationInlineItem {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
}

impl AsRef<str> for TaskSpecContainerSpecInlineItemIsolationInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            TaskSpecContainerSpecInlineItemIsolationInlineItem::Default => "default",
            TaskSpecContainerSpecInlineItemIsolationInlineItem::Process => "process",
            TaskSpecContainerSpecInlineItemIsolationInlineItem::Hyperv => "hyperv",
        }
    }
}

impl std::fmt::Display for TaskSpecContainerSpecInlineItemIsolationInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Security options for the container
pub struct TaskSpecContainerSpecInlineItemPrivilegesInlineItem {
    #[serde(rename = "CredentialSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CredentialSpec for managed service account (Windows only)
    pub credential_spec:
        Option<TaskSpecContainerSpecInlineItemPrivilegesInlineItemCredentialSpecInlineItem>,
    #[serde(rename = "SELinuxContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SELinux labels of the container
    pub se_linux_context:
        Option<TaskSpecContainerSpecInlineItemPrivilegesInlineItemSeLinuxContextInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// CredentialSpec for managed service account (Windows only)
pub struct TaskSpecContainerSpecInlineItemPrivilegesInlineItemCredentialSpecInlineItem {
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Load credential spec from a Swarm Config with the given ID.
    /// The specified config must also be present in the Configs
    /// field with the Runtime property set.
    ///
    /// <p><br /></p>
    ///
    ///
    /// > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`,
    /// > and `CredentialSpec.Config` are mutually exclusive.
    pub config: Option<String>,
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Load credential spec from this file. The file is read by
    /// the daemon, and must be present in the `CredentialSpecs`
    /// subdirectory in the docker data directory, which defaults
    /// to `C:\ProgramData\Docker\` on Windows.
    ///
    /// For example, specifying `spec.json` loads
    /// `C:\ProgramData\Docker\CredentialSpecs\spec.json`.
    ///
    /// <p><br /></p>
    ///
    /// > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`,
    /// > and `CredentialSpec.Config` are mutually exclusive.
    pub file: Option<String>,
    #[serde(rename = "Registry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Load credential spec from this value in the Windows
    /// registry. The specified registry value must be located in:
    ///
    /// `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Virtualization\Containers\CredentialSpecs`
    ///
    /// <p><br /></p>
    ///
    ///
    /// > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`,
    /// > and `CredentialSpec.Config` are mutually exclusive.
    pub registry: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// SELinux labels of the container
pub struct TaskSpecContainerSpecInlineItemPrivilegesInlineItemSeLinuxContextInlineItem {
    #[serde(rename = "Disable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable SELinux
    pub disable: Option<bool>,
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SELinux level label
    pub level: Option<String>,
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SELinux role label
    pub role: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SELinux type label
    pub type_: Option<String>,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SELinux user label
    pub user: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecInlineItemSecretsInlineItem {
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// File represents a specific target that is backed by a file.
    pub file: Option<TaskSpecContainerSpecInlineItemSecretsInlineItemFileInlineItem>,
    #[serde(rename = "SecretID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecretID represents the ID of the specific secret that we're
    /// referencing.
    pub secret_id: Option<String>,
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SecretName is the name of the secret that this references,
    /// but this is just provided for lookup/display purposes. The
    /// secret in the reference will be identified by its ID.
    pub secret_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// File represents a specific target that is backed by a file.
pub struct TaskSpecContainerSpecInlineItemSecretsInlineItemFileInlineItem {
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// GID represents the file GID.
    pub gid: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mode represents the FileMode of the file.
    pub mode: Option<u32>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name represents the final filename in the filesystem.
    pub name: Option<String>,
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// UID represents the file UID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecInlineItemUlimitsInlineItem {
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Hard limit
    pub hard: Option<isize>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of ulimit
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Soft limit
    pub soft: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specifies the log driver to use for tasks created from this spec. If
/// not present, the default one for the swarm will be used, finally
/// falling back to the engine default if not specified.
pub struct TaskSpecLogDriverInlineItem {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Read-only spec type for non-swarm containers attached to swarm overlay
/// networks.
///
/// <p><br /></p>
///
/// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
/// > mutually exclusive. PluginSpec is only used when the Runtime field
/// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
/// > field is set to `attachment`.
pub struct TaskSpecNetworkAttachmentSpecInlineItem {
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID of the container represented by this task
    pub container_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacementInlineItem {
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An array of constraint expressions to limit the set of nodes where
    /// a task can be scheduled. Constraint expressions can either use a
    /// _match_ (`==`) or _exclude_ (`!=`) rule. Multiple constraints find
    /// nodes that satisfy every expression (AND match). Constraints can
    /// match node or Docker Engine labels as follows:
    ///
    /// node attribute       | matches                        | example
    /// ---------------------|--------------------------------|-----------------------------------------------
    /// `node.id`            | Node ID                        | `node.id==2ivku8v2gvtg4`
    /// `node.hostname`      | Node hostname                  | `node.hostname!=node-2`
    /// `node.role`          | Node role (`manager`/`worker`) | `node.role==manager`
    /// `node.platform.os`   | Node operating system          | `node.platform.os==windows`
    /// `node.platform.arch` | Node architecture              | `node.platform.arch==x86_64`
    /// `node.labels`        | User-defined node labels       | `node.labels.security==high`
    /// `engine.labels`      | Docker Engine's labels         | `engine.labels.operatingsystem==ubuntu-14.04`
    ///
    /// `engine.labels` apply to Docker Engine labels like operating system,
    /// drivers, etc. Swarm administrators add `node.labels` for operational
    /// purposes by using the [`node update endpoint`](#operation/NodeUpdate).
    pub constraints: Option<Vec<String>>,
    #[serde(rename = "MaxReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of replicas for per node (default value is 0, which
    /// is unlimited)
    pub max_replicas: Option<i64>,
    #[serde(rename = "Platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Platforms stores all the platforms that the service's image can
    /// run on. This field is used in the platform filter for scheduling.
    /// If empty, then the platform filter is off, meaning there are no
    /// scheduling restrictions.
    pub platforms: Option<Vec<Platform>>,
    #[serde(rename = "Preferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Preferences provide a way to make the scheduler aware of factors
    /// such as topology. They are provided in order from highest to
    /// lowest precedence.
    pub preferences: Option<Vec<TaskSpecPlacementInlineItemPreferencesInlineItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacementInlineItemPreferencesInlineItem {
    #[serde(rename = "Spread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<TaskSpecPlacementInlineItemPreferencesInlineItemSpreadInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacementInlineItemPreferencesInlineItemSpreadInlineItem {
    #[serde(rename = "SpreadDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// label descriptor, such as `engine.labels.az`.
    pub spread_descriptor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Plugin spec for the service.  *(Experimental release only.)*
///
/// <p><br /></p>
///
/// > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are
/// > mutually exclusive. PluginSpec is only used when the Runtime field
/// > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime
/// > field is set to `attachment`.
pub struct TaskSpecPluginSpecInlineItem {
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disable the plugin once scheduled.
    pub disabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name or 'alias' to use for the plugin.
    pub name: Option<String>,
    #[serde(rename = "PluginPrivilege")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_privilege: Option<Vec<PluginPrivilege>>,
    #[serde(rename = "Remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The plugin image reference to use.
    pub remote: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Resource requirements which apply to each individual container created
/// as part of the service.
pub struct TaskSpecResourcesInlineItem {
    #[serde(rename = "Limits")]
    pub limits: Option<Limit>,
    #[serde(rename = "Reservations")]
    pub reservations: Option<ResourceObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Specification for the restart policy which applies to containers
/// created as part of this service.
pub struct TaskSpecRestartPolicyInlineItem {
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Condition for restart.
    pub condition: Option<String>,
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Delay between restart attempts.
    pub delay: Option<i64>,
    #[serde(rename = "MaxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum attempts to restart a given container before giving up
    /// (default value is 0, which is ignored).
    pub max_attempts: Option<i64>,
    #[serde(rename = "Window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Windows is the time window used to evaluate the restart policy
    /// (default value is 0, which is unbounded).
    pub window: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Condition for restart.
pub enum TaskSpecRestartPolicyInlineItemConditionInlineItem {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "any")]
    Any,
}

impl AsRef<str> for TaskSpecRestartPolicyInlineItemConditionInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            TaskSpecRestartPolicyInlineItemConditionInlineItem::None => "none",
            TaskSpecRestartPolicyInlineItemConditionInlineItem::OnFailure => "on-failure",
            TaskSpecRestartPolicyInlineItemConditionInlineItem::Any => "any",
        }
    }
}

impl std::fmt::Display for TaskSpecRestartPolicyInlineItemConditionInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "allocated")]
    Allocated,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "orphaned")]
    Orphaned,
}

impl AsRef<str> for TaskState {
    fn as_ref(&self) -> &str {
        match self {
            TaskState::New => "new",
            TaskState::Allocated => "allocated",
            TaskState::Pending => "pending",
            TaskState::Assigned => "assigned",
            TaskState::Accepted => "accepted",
            TaskState::Preparing => "preparing",
            TaskState::Ready => "ready",
            TaskState::Starting => "starting",
            TaskState::Running => "running",
            TaskState::Complete => "complete",
            TaskState::Shutdown => "shutdown",
            TaskState::Failed => "failed",
            TaskState::Rejected => "rejected",
            TaskState::Remove => "remove",
            TaskState::Orphaned => "orphaned",
        }
    }
}

impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskStatusInlineItem {
    #[serde(rename = "ContainerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_status: Option<TaskStatusInlineItemContainerStatusInlineItem>,
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskStatusInlineItemContainerStatusInlineItem {
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<isize>,
    #[serde(rename = "PID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Device path
    pub path: Option<String>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rate
    pub rate: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date/Time the volume was created.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Driver")]
    /// Name of the volume driver used by the volume.
    pub driver: String,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// User-defined key/value metadata.
    pub labels: HashMap<String, String>,
    #[serde(rename = "Mountpoint")]
    /// Mount path of the volume on the host.
    pub mountpoint: String,
    #[serde(rename = "Name")]
    /// Name of the volume.
    pub name: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    /// The driver specific options used when creating the volume.
    pub options: HashMap<String, String>,
    #[serde(rename = "Scope")]
    /// The level at which the volume exists. Either `global` for cluster-wide,
    /// or `local` for machine level.
    pub scope: String,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Low-level details about the volume, provided by the volume driver.
    /// Details are returned as a map with key/value pairs:
    /// `{"key":"value","key2":"value2"}`.
    ///
    /// The `Status` field is optional, and is omitted if the volume driver
    /// does not support this feature.
    pub status: Option<HashMap<String, Value>>,
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Usage details about the volume. This information is used by the
    /// `GET /system/df` endpoint, and omitted in other endpoints.
    pub usage_data: Option<VolumeUsageDataInlineItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Volume configuration
pub struct VolumeCreateOptions {
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Name of the volume driver to use.
    pub driver: Option<String>,
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A mapping of driver options and values. These options are
    /// passed directly to the driver and are driver specific.
    pub driver_opts: Option<HashMap<String, String>>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The new volume's name. If not specified, Docker generates a name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Summary volume data that matches the query
pub struct VolumeList200Response {
    #[serde(rename = "Volumes")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// List of volumes
    pub volumes: Vec<Volume>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    /// Warnings that occurred when fetching the list of volumes.
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// No error
pub struct VolumePrune200Response {
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Disk space reclaimed in bytes
    pub space_reclaimed: Option<i64>,
    #[serde(rename = "VolumesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Volumes that were deleted
    pub volumes_deleted: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// The level at which the volume exists. Either `global` for cluster-wide,
/// or `local` for machine level.
pub enum VolumeScopeInlineItem {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "global")]
    Global,
}

impl AsRef<str> for VolumeScopeInlineItem {
    fn as_ref(&self) -> &str {
        match self {
            VolumeScopeInlineItem::Local => "local",
            VolumeScopeInlineItem::Global => "global",
        }
    }
}

impl std::fmt::Display for VolumeScopeInlineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Usage details about the volume. This information is used by the
/// `GET /system/df` endpoint, and omitted in other endpoints.
pub struct VolumeUsageDataInlineItem {
    #[serde(rename = "RefCount")]
    /// The number of containers referencing this volume. This field
    /// is set to `-1` if the reference-count is not available.
    pub ref_count: isize,
    #[serde(rename = "Size")]
    /// Amount of disk space used by the volume (in bytes). This information
    /// is only available for volumes created with the `"local"` volume
    /// driver. For volumes created with other volume drivers, this field
    /// is set to `-1` ("not available")
    pub size: isize,
}

pub type ConfigUpdateBodyParam = ConfigSpec;

/// Configuration for a container that is portable between hosts.
///
/// When used as `ContainerConfig` field in an image, `ContainerConfig` is an
/// optional field containing the configuration of the container that was last
/// committed when creating the image.
///
/// Previous versions of Docker builder used this field to store build cache,
/// and it is not in active use anymore.
pub type ImageCommitContainerConfigParam = ContainerConfig;

pub type NodeUpdateBodyParam = NodeSpec;

pub type SecretUpdateBodyParam = SecretSpec;

/// User modifiable swarm configuration.
pub type SwarmUpdateBodyParam = SwarmSpec;

pub type SystemAuthAuthConfigParam = AuthConfig;

/// Volume configuration
pub type VolumeCreateVolumeConfigParam = VolumeCreateOptions;
