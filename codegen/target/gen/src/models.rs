#![allow(
    non_snake_case,
    clippy::redundant_field_names,
    clippy::new_without_default,
    clippy::too_many_arguments
)]

use serde::{
    de::{DeserializeOwned, Deserializer},
    Deserialize, Serialize,
};
use serde_json::Value;
use serde_with::{serde_as, NoneAsEmptyString};

use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;

fn deserialize_nonoptional_vec<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

fn deserialize_nonoptional_map<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<HashMap<String, T>, D::Error> {
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or_default())
}

/// Address represents an IPv4 or IPv6 IP address.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Address {
    /// IP address.
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// Mask length of the IP address.
    #[serde(rename = "PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_len: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AuthConfig {
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "serveraddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serveraddress: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BuildCache {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[serde(rename = "Shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /// Amount of disk space used by the build cache (in bytes).
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Date and time at which the build cache was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the build cache was last used in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "LastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(rename = "UsageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BuildInfo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<ErrorDetail>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<ProgressDetail>,
    #[serde(rename = "aux")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aux: Option<ImageId>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BuildPruneResponse {
    #[serde(rename = "CachesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caches_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
/// ClusterInfo represents information about the swarm as is returned by the \"/info\" endpoint. Join-tokens are not included.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ClusterInfo {
    /// The ID of the swarm.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SwarmSpec>,
    #[serde(rename = "TLSInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<TlsInfo>,
    /// Whether there is currently a root CA rotation in progress for the swarm
    #[serde(rename = "RootRotationInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_rotation_in_progress: Option<bool>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i64>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i64>,
}
/// Commit holds the Git-commit (SHA1) that a binary was built from, as reported in the version-string of external tools, such as `containerd`, or `runC`.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Commit {
    /// Actual commit ID of external tool.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// Commit ID of external tool expected by dockerd as set at build time.
    #[serde(rename = "Expected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ConfigSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConfigSpec {
    /// User-defined name of the config.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) config data.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Templating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templating: Option<Driver>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ConfigsCreateBody {
    /// User-defined name of the config.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) config data.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Templating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templating: Option<Driver>,
}
/// change item in response to ContainerChanges operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerChangeResponseItem {
    /// Path to file that has changed
    #[serde(rename = "Path")]
    pub path: String,
    /// Kind of change
    #[serde(rename = "Kind")]
    pub kind: i64,
}
/// Configuration for a container that is portable between hosts.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerConfig {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<HashMap<String, Value>>,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,
    /// Command is already escaped (Windows only)
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    /// MAC address of the container.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// Timeout to stop a container in seconds.
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<Vec<String>>,
}
/// OK response to ContainerCreate operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerCreateResponse {
    /// The ID of the created container
    #[serde(rename = "Id")]
    pub id: String,
    /// Warnings encountered when creating the container
    #[serde(rename = "Warnings")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub warnings: Vec<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerInspectResponse {
    /// The ID of the container
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The time the container was created
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The path to the command being run
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The arguments to the command being run
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ContainerState>,
    /// The container's image ID
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "ResolvConfPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "HostnamePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "LogPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RestartCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i64>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "MountLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "ProcessLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "AppArmorProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    /// IDs of exec instances that are running in the container.
    #[serde(rename = "ExecIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_i_ds: Option<Vec<String>>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "GraphDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<GraphDriverData>,
    /// The size of files that have been created or changed by this container.
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    /// The total size of all the files in this container.
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ContainerConfig>,
    #[serde(rename = "NetworkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<NetworkSettings>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerPruneResponse {
    /// Container IDs that were deleted
    #[serde(rename = "ContainersDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
/// ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerState {
    /// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\".
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContainerStateSTATUS>,
    /// Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is \"running\".
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// Whether this container is paused.
    #[serde(rename = "Paused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Whether this container is restarting.
    #[serde(rename = "Restarting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    /// Whether this container has been killed because it ran out of memory.
    #[serde(rename = "OOMKilled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Dead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    /// The process ID of this container
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// The last exit code of this container
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The time when this container was last started.
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// The time when this container last exited.
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ContainerStateSTATUS {
    #[serde(rename = "created")]
    CREATED,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "paused")]
    PAUSED,
    #[serde(rename = "restarting")]
    RESTARTING,
    #[serde(rename = "removing")]
    REMOVING,
    #[serde(rename = "exited")]
    EXITED,
    #[serde(rename = "dead")]
    DEAD,
}

impl ::std::fmt::Display for ContainerStateSTATUS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ContainerStateSTATUS::CREATED => write!(f, "created"),
            ContainerStateSTATUS::RUNNING => write!(f, "running"),
            ContainerStateSTATUS::PAUSED => write!(f, "paused"),
            ContainerStateSTATUS::RESTARTING => write!(f, "restarting"),
            ContainerStateSTATUS::REMOVING => write!(f, "removing"),
            ContainerStateSTATUS::EXITED => write!(f, "exited"),
            ContainerStateSTATUS::DEAD => write!(f, "dead"),
        }
    }
}

impl ::std::str::FromStr for ContainerStateSTATUS {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "created" => Ok(ContainerStateSTATUS::CREATED),
            "running" => Ok(ContainerStateSTATUS::RUNNING),
            "paused" => Ok(ContainerStateSTATUS::PAUSED),
            "restarting" => Ok(ContainerStateSTATUS::RESTARTING),
            "removing" => Ok(ContainerStateSTATUS::REMOVING),
            "exited" => Ok(ContainerStateSTATUS::EXITED),
            "dead" => Ok(ContainerStateSTATUS::DEAD),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ContainerStateSTATUS {
    fn as_ref(&self) -> &str {
        match self {
            ContainerStateSTATUS::CREATED => "created",
            ContainerStateSTATUS::RUNNING => "running",
            ContainerStateSTATUS::PAUSED => "paused",
            ContainerStateSTATUS::RESTARTING => "restarting",
            ContainerStateSTATUS::REMOVING => "removing",
            ContainerStateSTATUS::EXITED => "exited",
            ContainerStateSTATUS::DEAD => "dead",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerSummary {
    /// The ID of this container
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The names that this container has been given
    #[serde(rename = "Names")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// The name of the image used when creating this container
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The ID of the image that this container was created from
    #[serde(rename = "ImageID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Command to run when starting the container
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// When the container was created
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// The ports exposed by this container
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Port>>,
    /// The size of files that have been created or changed by this container
    #[serde(rename = "SizeRw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    /// The total size of all the files in this container
    #[serde(rename = "SizeRootFs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// The state of this container (e.g. `Exited`)
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Additional human-readable status of this container (e.g. `Exit 0`)
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<ContainerSummaryHostConfig>,
    #[serde(rename = "NetworkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ContainerSummaryNetworkSettings>,
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MountPoint>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerSummaryHostConfig {
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
}
/// A summary of the container's network settings
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerSummaryNetworkSettings {
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}
/// OK response to ContainerTop operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerTopResponse {
    /// The ps column titles
    #[serde(rename = "Titles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<String>>,
    /// Each process running in the container, where each is process is an array of values corresponding to the titles.
    #[serde(rename = "Processes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<Vec<String>>>,
}
/// OK response to ContainerUpdate operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerUpdateResponse {
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
/// container waiting error, if any
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerWaitExitError {
    /// Details of an error
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
/// OK response to ContainerWait operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainerWaitResponse {
    /// Exit code of the container
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
    #[serde(rename = "Error")]
    pub error: ContainerWaitExitError,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainersCreateBody {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<HashMap<String, Value>>,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,
    /// Command is already escaped (Windows only)
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    /// MAC address of the container.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// Timeout to stop a container in seconds.
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<Vec<String>>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "NetworkingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<NetworkingConfig>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ContainersCreateBody1 {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(rename = "ExposedPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<HashMap<String, Value>>,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<HealthConfig>,
    /// Command is already escaped (Windows only)
    #[serde(rename = "ArgsEscaped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<HashMap<String, Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.
    #[serde(rename = "NetworkDisabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_disabled: Option<bool>,
    /// MAC address of the container.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(rename = "OnBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_build: Option<Vec<String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// Timeout to stop a container in seconds.
    #[serde(rename = "StopTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i64>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(rename = "Shell")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<Vec<String>>,
    #[serde(rename = "HostConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_config: Option<HostConfig>,
    #[serde(rename = "NetworkingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<NetworkingConfig>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CreateImageInfo {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<ProgressDetail>,
}
/// A device mapping between the host and container
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DeviceMapping {
    #[serde(rename = "PathOnHost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
    #[serde(rename = "PathInContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_in_container: Option<String>,
    #[serde(rename = "CgroupPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_permissions: Option<String>,
}
/// A request for devices to be sent to device drivers
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DeviceRequest {
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "DeviceIDs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_i_ds: Option<Vec<String>>,
    /// A list of capabilities; an OR list of AND lists of capabilities.
    #[serde(rename = "Capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Vec<String>>>,
    /// Driver-specific options, specified as a key/value pairs. These options are passed directly to the driver.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
/// Describes the result obtained from contacting the registry to retrieve image metadata.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DistributionInspect {
    #[serde(rename = "Descriptor")]
    pub descriptor: OciDescriptor,
    /// An array containing all platforms supported by the image.
    #[serde(rename = "Platforms")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub platforms: Vec<OciPlatform>,
}
/// Driver represents a driver (network, logging, secrets).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Driver {
    /// Name of the driver.
    #[serde(rename = "Name")]
    pub name: String,
    /// Key/value map of driver-specific options.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
/// EndpointIPAMConfig represents an endpoint's IPAM configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_ps: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointPortConfig {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<EndpointPortConfigPROTOCOL>,
    /// The port inside the container.
    #[serde(rename = "TargetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i64>,
    /// The port on the swarm hosts.
    #[serde(rename = "PublishedPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_port: Option<i64>,
    /// The mode in which port is published.  <p><br /></p>  - \"ingress\" makes the target port accessible on every node,   regardless of whether there is a task for the service running on   that node or not. - \"host\" bypasses the routing mesh and publish the port directly on   the swarm node where that service is running.
    #[serde(rename = "PublishMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_mode: Option<EndpointPortConfigPUBLISH_MODE>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum EndpointPortConfigPROTOCOL {
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "udp")]
    UDP,
    #[serde(rename = "sctp")]
    SCTP,
}

impl ::std::fmt::Display for EndpointPortConfigPROTOCOL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EndpointPortConfigPROTOCOL::TCP => write!(f, "tcp"),
            EndpointPortConfigPROTOCOL::UDP => write!(f, "udp"),
            EndpointPortConfigPROTOCOL::SCTP => write!(f, "sctp"),
        }
    }
}

impl ::std::str::FromStr for EndpointPortConfigPROTOCOL {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(EndpointPortConfigPROTOCOL::TCP),
            "udp" => Ok(EndpointPortConfigPROTOCOL::UDP),
            "sctp" => Ok(EndpointPortConfigPROTOCOL::SCTP),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EndpointPortConfigPROTOCOL {
    fn as_ref(&self) -> &str {
        match self {
            EndpointPortConfigPROTOCOL::TCP => "tcp",
            EndpointPortConfigPROTOCOL::UDP => "udp",
            EndpointPortConfigPROTOCOL::SCTP => "sctp",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum EndpointPortConfigPUBLISH_MODE {
    #[serde(rename = "ingress")]
    INGRESS,
    #[serde(rename = "host")]
    HOST,
}

impl ::std::fmt::Display for EndpointPortConfigPUBLISH_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EndpointPortConfigPUBLISH_MODE::INGRESS => write!(f, "ingress"),
            EndpointPortConfigPUBLISH_MODE::HOST => write!(f, "host"),
        }
    }
}

impl ::std::str::FromStr for EndpointPortConfigPUBLISH_MODE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ingress" => Ok(EndpointPortConfigPUBLISH_MODE::INGRESS),
            "host" => Ok(EndpointPortConfigPUBLISH_MODE::HOST),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EndpointPortConfigPUBLISH_MODE {
    fn as_ref(&self) -> &str {
        match self {
            EndpointPortConfigPUBLISH_MODE::INGRESS => "ingress",
            EndpointPortConfigPUBLISH_MODE::HOST => "host",
        }
    }
}
/// Configuration for a network endpoint.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointSettings {
    #[serde(rename = "IPAMConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<EndpointIpamConfig>,
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Unique ID of the network.
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Unique ID for the service endpoint in a Sandbox.
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway address for this network.
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// IPv4 address.
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Mask length of the IPv4 address.
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    /// IPv6 gateway address.
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    /// Global IPv6 address.
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_address: Option<String>,
    /// Mask length of the global IPv6 address.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    /// MAC address for the endpoint on this network.
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,
}
/// Properties that can be configured to access and load balance a service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<EndpointSpecMODE>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if `vip` resolution mode is used.
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<EndpointPortConfig>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum EndpointSpecMODE {
    #[serde(rename = "vip")]
    VIP,
    #[serde(rename = "dnsrr")]
    DNSRR,
}

impl ::std::fmt::Display for EndpointSpecMODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EndpointSpecMODE::VIP => write!(f, "vip"),
            EndpointSpecMODE::DNSRR => write!(f, "dnsrr"),
        }
    }
}

impl ::std::str::FromStr for EndpointSpecMODE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vip" => Ok(EndpointSpecMODE::VIP),
            "dnsrr" => Ok(EndpointSpecMODE::DNSRR),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EndpointSpecMODE {
    fn as_ref(&self) -> &str {
        match self {
            EndpointSpecMODE::VIP => "vip",
            EndpointSpecMODE::DNSRR => "dnsrr",
        }
    }
}
/// EngineDescription provides information about an engine.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EngineDescription {
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<EngineDescriptionPlugins>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EngineDescriptionPlugins {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ErrorDetail {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
/// Represents an error.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ErrorResponse {
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
}
/// Actor describes something that generates events, like a container, network, or a volume.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EventActor {
    /// The ID of the object emitting the event
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// Various key/value attributes of the object, depending on its type.
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, String>>,
}
/// EventMessage represents the information an event contains.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EventMessage {
    /// The type of object emitting the event
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<EventMessageTYPE>,
    /// The type of event
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Actor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<EventActor>,
    /// Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<EventMessageSCOPE>,
    /// Timestamp of event
    #[serde(rename = "time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Timestamp of event, with nanosecond accuracy
    #[serde(rename = "timeNano")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_nano: Option<i64>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum EventMessageTYPE {
    #[serde(rename = "builder")]
    BUILDER,
    #[serde(rename = "config")]
    CONFIG,
    #[serde(rename = "container")]
    CONTAINER,
    #[serde(rename = "daemon")]
    DAEMON,
    #[serde(rename = "image")]
    IMAGE,
    #[serde(rename = "network")]
    NETWORK,
    #[serde(rename = "node")]
    NODE,
    #[serde(rename = "plugin")]
    PLUGIN,
    #[serde(rename = "secret")]
    SECRET,
    #[serde(rename = "service")]
    SERVICE,
    #[serde(rename = "volume")]
    VOLUME,
}

impl ::std::fmt::Display for EventMessageTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventMessageTYPE::BUILDER => write!(f, "builder"),
            EventMessageTYPE::CONFIG => write!(f, "config"),
            EventMessageTYPE::CONTAINER => write!(f, "container"),
            EventMessageTYPE::DAEMON => write!(f, "daemon"),
            EventMessageTYPE::IMAGE => write!(f, "image"),
            EventMessageTYPE::NETWORK => write!(f, "network"),
            EventMessageTYPE::NODE => write!(f, "node"),
            EventMessageTYPE::PLUGIN => write!(f, "plugin"),
            EventMessageTYPE::SECRET => write!(f, "secret"),
            EventMessageTYPE::SERVICE => write!(f, "service"),
            EventMessageTYPE::VOLUME => write!(f, "volume"),
        }
    }
}

impl ::std::str::FromStr for EventMessageTYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "builder" => Ok(EventMessageTYPE::BUILDER),
            "config" => Ok(EventMessageTYPE::CONFIG),
            "container" => Ok(EventMessageTYPE::CONTAINER),
            "daemon" => Ok(EventMessageTYPE::DAEMON),
            "image" => Ok(EventMessageTYPE::IMAGE),
            "network" => Ok(EventMessageTYPE::NETWORK),
            "node" => Ok(EventMessageTYPE::NODE),
            "plugin" => Ok(EventMessageTYPE::PLUGIN),
            "secret" => Ok(EventMessageTYPE::SECRET),
            "service" => Ok(EventMessageTYPE::SERVICE),
            "volume" => Ok(EventMessageTYPE::VOLUME),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EventMessageTYPE {
    fn as_ref(&self) -> &str {
        match self {
            EventMessageTYPE::BUILDER => "builder",
            EventMessageTYPE::CONFIG => "config",
            EventMessageTYPE::CONTAINER => "container",
            EventMessageTYPE::DAEMON => "daemon",
            EventMessageTYPE::IMAGE => "image",
            EventMessageTYPE::NETWORK => "network",
            EventMessageTYPE::NODE => "node",
            EventMessageTYPE::PLUGIN => "plugin",
            EventMessageTYPE::SECRET => "secret",
            EventMessageTYPE::SERVICE => "service",
            EventMessageTYPE::VOLUME => "volume",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum EventMessageSCOPE {
    #[serde(rename = "local")]
    LOCAL,
    #[serde(rename = "swarm")]
    SWARM,
}

impl ::std::fmt::Display for EventMessageSCOPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            EventMessageSCOPE::LOCAL => write!(f, "local"),
            EventMessageSCOPE::SWARM => write!(f, "swarm"),
        }
    }
}

impl ::std::str::FromStr for EventMessageSCOPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(EventMessageSCOPE::LOCAL),
            "swarm" => Ok(EventMessageSCOPE::SWARM),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for EventMessageSCOPE {
    fn as_ref(&self) -> &str {
        match self {
            EventMessageSCOPE::LOCAL => "local",
            EventMessageSCOPE::SWARM => "swarm",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExecConfig {
    /// Attach to `stdin` of the exec command.
    #[serde(rename = "AttachStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to `stdout` of the exec command.
    #[serde(rename = "AttachStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Attach to `stderr` of the exec command.
    #[serde(rename = "AttachStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// A list of environment variables in the form `[\"VAR=value\", ...]`.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// Runs the exec process with extended privileges.
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// The user, and optionally, group to run the exec process inside the container. Format is one of: `user`, `user:group`, `uid`, or `uid:gid`.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExecInspectResponse {
    #[serde(rename = "CanRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_remove: Option<bool>,
    #[serde(rename = "DetachKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Running")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "ProcessConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<ProcessConfig>,
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "OpenStderr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stderr: Option<bool>,
    #[serde(rename = "OpenStdout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdout: Option<bool>,
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The system process ID for the exec process.
    #[serde(rename = "Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExecStartConfig {
    /// Detach from the command.
    #[serde(rename = "Detach")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
}
/// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).
pub type GenericResources = Vec<GenericResourcesInner>;
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GenericResourcesInner {
    #[serde(rename = "NamedResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_resource_spec: Option<Value>,
    #[serde(rename = "DiscreteResourceSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete_resource_spec: Option<Value>,
}
/// Information about the storage driver used to store the container's and image's filesystem.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct GraphDriverData {
    /// Name of the storage driver.
    #[serde(rename = "Name")]
    pub name: String,
    /// Low-level storage metadata, provided as key/value pairs.  This information is driver-specific, and depends on the storage-driver in use, and should be used for informational purposes only.
    #[serde(rename = "Data")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub data: HashMap<String, String>,
}
/// Health stores information about the container's healthcheck results.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Health {
    /// Status is one of `none`, `starting`, `healthy` or `unhealthy`  - \"none\"      Indicates there is no healthcheck - \"starting\"  Starting indicates that the container is not yet ready - \"healthy\"   Healthy indicates that the container is running correctly - \"unhealthy\" Unhealthy indicates that the container has a problem
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HealthSTATUS>,
    /// FailingStreak is the number of consecutive failures
    #[serde(rename = "FailingStreak")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_streak: Option<i64>,
    /// Log contains the last few results (oldest first)
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<HealthcheckResult>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum HealthSTATUS {
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "starting")]
    STARTING,
    #[serde(rename = "healthy")]
    HEALTHY,
    #[serde(rename = "unhealthy")]
    UNHEALTHY,
}

impl ::std::fmt::Display for HealthSTATUS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HealthSTATUS::NONE => write!(f, "none"),
            HealthSTATUS::STARTING => write!(f, "starting"),
            HealthSTATUS::HEALTHY => write!(f, "healthy"),
            HealthSTATUS::UNHEALTHY => write!(f, "unhealthy"),
        }
    }
}

impl ::std::str::FromStr for HealthSTATUS {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(HealthSTATUS::NONE),
            "starting" => Ok(HealthSTATUS::STARTING),
            "healthy" => Ok(HealthSTATUS::HEALTHY),
            "unhealthy" => Ok(HealthSTATUS::UNHEALTHY),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for HealthSTATUS {
    fn as_ref(&self) -> &str {
        match self {
            HealthSTATUS::NONE => "none",
            HealthSTATUS::STARTING => "starting",
            HealthSTATUS::HEALTHY => "healthy",
            HealthSTATUS::UNHEALTHY => "unhealthy",
        }
    }
}
/// A test to perform to check that the container is healthy.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HealthConfig {
    /// The test to perform. Possible values are:  - `[]` inherit healthcheck from image or parent image - `[\"NONE\"]` disable healthcheck - `[\"CMD\", args...]` exec arguments directly - `[\"CMD-SHELL\", command]` run command with system's default shell
    #[serde(rename = "Test")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    /// The time to wait between checks in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "Interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// The time to wait before considering the check to have hung. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// Start period for the container to initialize before starting health-retries countdown in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
}
/// HealthcheckResult stores information about a single run of a healthcheck probe
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HealthcheckResult {
    /// Date and time at which this check started in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "Start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<DateTime<Utc>>,
    /// Date and time at which this check ended in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "End")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// ExitCode meanings:  - `0` healthy - `1` unhealthy - `2` reserved (considered unhealthy) - other values: error running probe
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// Output from last check
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}
/// individual image layer information in response to ImageHistory operation
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HistoryResponseItem {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    #[serde(rename = "Tags")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub tags: Vec<String>,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "Comment")]
    pub comment: String,
}
/// Container configuration that depends on the host we are running on
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HostConfig {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i64>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    /// Kernel memory limit in bytes.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`.
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    /// Hard limit for kernel TCP buffer memory (in bytes).
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    /// A list of volume bindings for this container. Each volume binding is a string in one of these forms:  - `host-src:container-dest[:options]` to bind-mount a host path   into the container. Both `host-src`, and `container-dest` must   be an _absolute_ path. - `volume-name:container-dest[:options]` to bind-mount a volume   managed by a volume driver into the container. `container-dest`   must be an _absolute_ path.  `options` is an optional, comma-delimited list of:  - `nocopy` disables automatic copying of data from the container   path to the volume. The `nocopy` flag only applies to named volumes. - `[ro|rw]` mounts a volume read-only or read-write, respectively.   If omitted or set to `rw`, volumes are mounted read-write. - `[z|Z]` applies SELinux labels to allow or deny multiple containers   to read and write to the same volume.     - `z`: a _shared_ content label is applied to the content. This       label indicates that multiple containers can share the volume       content, for both reading and writing.     - `Z`: a _private unshared_ label is applied to the content.       This label indicates that only the current container can use       a private volume. Labeling systems such as SELinux require       proper labels to be placed on volume content that is mounted       into a container. Without a label, the security system can       prevent a container's processes from using the content. By       default, the labels set by the host operating system are not       modified. - `[[r]shared|[r]slave|[r]private]` specifies mount   [propagation behavior](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt).   This only applies to bind-mounted volumes, not internal volumes   or named volumes. Mount propagation requires the source mount   point (the location where the source directory is mounted in the   host operating system) to have the correct propagation properties.   For shared volumes, the source mount point must be set to `shared`.   For slave volumes, the mount must be set to either `shared` or   `slave`.
    #[serde(rename = "Binds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binds: Option<Vec<String>>,
    /// Path to a file where the container ID is written
    #[serde(rename = "ContainerIDFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    #[serde(rename = "LogConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<HostConfigLogConfig>,
    /// Network mode to use for this container. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name to which this container should connect to.
    #[serde(rename = "NetworkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    #[serde(rename = "PortBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<PortMap>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,
    /// Automatically remove the container when the container's process exits. This has no effect if `RestartPolicy` is set.
    #[serde(rename = "AutoRemove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Driver that this container uses to mount volumes.
    #[serde(rename = "VolumeDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    /// A list of volumes to inherit from another container, specified in the form `<container name>[:<ro|rw>]`.
    #[serde(rename = "VolumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
    /// Specification for mounts to be added to the container.
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    /// A list of kernel capabilities to add to the container. Conflicts with option 'Capabilities'.
    #[serde(rename = "CapAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// A list of kernel capabilities to drop from the container. Conflicts with option 'Capabilities'.
    #[serde(rename = "CapDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// cgroup namespace mode for the container. Possible values are:  - `\"private\"`: the container runs in its own private cgroup namespace - `\"host\"`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `\"private\"` or `\"host\"`, depending on daemon version, kernel support and configuration.
    #[serde(rename = "CgroupnsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroupns_mode: Option<HostConfigCGROUPNS_MODE>,
    /// A list of DNS servers for the container to use.
    #[serde(rename = "Dns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    /// A list of DNS options.
    #[serde(rename = "DnsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<Vec<String>>,
    /// A list of DNS search domains.
    #[serde(rename = "DnsSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// A list of hostnames/IP mappings to add to the container's `/etc/hosts` file. Specified in the form `[\"hostname:IP\"]`.
    #[serde(rename = "ExtraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    /// A list of additional groups that the container process will run as.
    #[serde(rename = "GroupAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,
    /// IPC sharing mode for the container. Possible values are:  - `\"none\"`: own private IPC namespace, with /dev/shm not mounted - `\"private\"`: own private IPC namespace - `\"shareable\"`: own private IPC namespace, with a possibility to share it with other containers - `\"container:<name|id>\"`: join another (shareable) container's IPC namespace - `\"host\"`: use the host system's IPC namespace  If not specified, daemon default is used, which can either be `\"private\"` or `\"shareable\"`, depending on daemon version and configuration.
    #[serde(rename = "IpcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// Cgroup to use for the container.
    #[serde(rename = "Cgroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    /// A list of links for the container in the form `container_name:alias`.
    #[serde(rename = "Links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// An integer value containing the score given to the container in order to tune OOM killer preferences.
    #[serde(rename = "OomScoreAdj")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    /// Set the PID (Process) Namespace mode for the container. It can be either:  - `\"container:<name|id>\"`: joins another container's PID namespace - `\"host\"`: use the host's PID namespace inside the container
    #[serde(rename = "PidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// Gives the container full access to the host.
    #[serde(rename = "Privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocates an ephemeral host port for all of a container's exposed ports.  Ports are de-allocated when the container stops and allocated when the container starts. The allocated port might be changed when restarting the container.  The port is selected from the ephemeral port range that depends on the kernel. For example, on Linux the range is defined by `/proc/sys/net/ipv4/ip_local_port_range`.
    #[serde(rename = "PublishAllPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    /// Mount the container's root filesystem as read only.
    #[serde(rename = "ReadonlyRootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    /// A list of string values to customize labels for MLS systems, such as SELinux.
    #[serde(rename = "SecurityOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// Storage driver options for this container, in the form `{\"size\": \"120G\"}`.
    #[serde(rename = "StorageOpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_opt: Option<HashMap<String, String>>,
    /// A map of container directories which should be replaced by tmpfs mounts, and their corresponding mount options. For example:  ``` { \"/run\": \"rw,noexec,nosuid,size=65536k\" } ```
    #[serde(rename = "Tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<HashMap<String, String>>,
    /// UTS namespace to use for the container.
    #[serde(rename = "UTSMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uts_mode: Option<String>,
    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
    #[serde(rename = "UsernsMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    /// Size of `/dev/shm` in bytes. If omitted, the system uses 64MB.
    #[serde(rename = "ShmSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    /// A list of kernel parameters (sysctls) to set in the container. For example:  ``` {\"net.ipv4.ip_forward\": \"1\"} ```
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<HashMap<String, String>>,
    /// Runtime to use with this container.
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Initial console size, as an `[height, width]` array. (Windows only)
    #[serde(rename = "ConsoleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_size: Option<Vec<i64>>,
    /// Isolation technology of the container. (Windows only)
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<HostConfigISOLATION>,
    /// The list of paths to be masked inside the container (this overrides the default set of paths).
    #[serde(rename = "MaskedPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_paths: Option<Vec<String>>,
    /// The list of paths to be set as read-only inside the container (this overrides the default set of paths).
    #[serde(rename = "ReadonlyPaths")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum HostConfigCGROUPNS_MODE {
    #[serde(rename = "private")]
    PRIVATE,
    #[serde(rename = "host")]
    HOST,
}

impl ::std::fmt::Display for HostConfigCGROUPNS_MODE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HostConfigCGROUPNS_MODE::PRIVATE => write!(f, "private"),
            HostConfigCGROUPNS_MODE::HOST => write!(f, "host"),
        }
    }
}

impl ::std::str::FromStr for HostConfigCGROUPNS_MODE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "private" => Ok(HostConfigCGROUPNS_MODE::PRIVATE),
            "host" => Ok(HostConfigCGROUPNS_MODE::HOST),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for HostConfigCGROUPNS_MODE {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigCGROUPNS_MODE::PRIVATE => "private",
            HostConfigCGROUPNS_MODE::HOST => "host",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum HostConfigISOLATION {
    #[serde(rename = "default")]
    DEFAULT,
    #[serde(rename = "process")]
    PROCESS,
    #[serde(rename = "hyperv")]
    HYPERV,
}

impl ::std::fmt::Display for HostConfigISOLATION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HostConfigISOLATION::DEFAULT => write!(f, "default"),
            HostConfigISOLATION::PROCESS => write!(f, "process"),
            HostConfigISOLATION::HYPERV => write!(f, "hyperv"),
        }
    }
}

impl ::std::str::FromStr for HostConfigISOLATION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(HostConfigISOLATION::DEFAULT),
            "process" => Ok(HostConfigISOLATION::PROCESS),
            "hyperv" => Ok(HostConfigISOLATION::HYPERV),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for HostConfigISOLATION {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigISOLATION::DEFAULT => "default",
            HostConfigISOLATION::PROCESS => "process",
            HostConfigISOLATION::HYPERV => "hyperv",
        }
    }
}
/// The logging configuration for this container
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HostConfigLogConfig {
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<HostConfigLogConfigTYPE>,
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum HostConfigLogConfigTYPE {
    #[serde(rename = "json-file")]
    JSON_FILE,
    #[serde(rename = "syslog")]
    SYSLOG,
    #[serde(rename = "journald")]
    JOURNALD,
    #[serde(rename = "gelf")]
    GELF,
    #[serde(rename = "fluentd")]
    FLUENTD,
    #[serde(rename = "awslogs")]
    AWSLOGS,
    #[serde(rename = "splunk")]
    SPLUNK,
    #[serde(rename = "etwlogs")]
    ETWLOGS,
    #[serde(rename = "none")]
    NONE,
}

impl ::std::fmt::Display for HostConfigLogConfigTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            HostConfigLogConfigTYPE::JSON_FILE => write!(f, "json-file"),
            HostConfigLogConfigTYPE::SYSLOG => write!(f, "syslog"),
            HostConfigLogConfigTYPE::JOURNALD => write!(f, "journald"),
            HostConfigLogConfigTYPE::GELF => write!(f, "gelf"),
            HostConfigLogConfigTYPE::FLUENTD => write!(f, "fluentd"),
            HostConfigLogConfigTYPE::AWSLOGS => write!(f, "awslogs"),
            HostConfigLogConfigTYPE::SPLUNK => write!(f, "splunk"),
            HostConfigLogConfigTYPE::ETWLOGS => write!(f, "etwlogs"),
            HostConfigLogConfigTYPE::NONE => write!(f, "none"),
        }
    }
}

impl ::std::str::FromStr for HostConfigLogConfigTYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json-file" => Ok(HostConfigLogConfigTYPE::JSON_FILE),
            "syslog" => Ok(HostConfigLogConfigTYPE::SYSLOG),
            "journald" => Ok(HostConfigLogConfigTYPE::JOURNALD),
            "gelf" => Ok(HostConfigLogConfigTYPE::GELF),
            "fluentd" => Ok(HostConfigLogConfigTYPE::FLUENTD),
            "awslogs" => Ok(HostConfigLogConfigTYPE::AWSLOGS),
            "splunk" => Ok(HostConfigLogConfigTYPE::SPLUNK),
            "etwlogs" => Ok(HostConfigLogConfigTYPE::ETWLOGS),
            "none" => Ok(HostConfigLogConfigTYPE::NONE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for HostConfigLogConfigTYPE {
    fn as_ref(&self) -> &str {
        match self {
            HostConfigLogConfigTYPE::JSON_FILE => "json-file",
            HostConfigLogConfigTYPE::SYSLOG => "syslog",
            HostConfigLogConfigTYPE::JOURNALD => "journald",
            HostConfigLogConfigTYPE::GELF => "gelf",
            HostConfigLogConfigTYPE::FLUENTD => "fluentd",
            HostConfigLogConfigTYPE::AWSLOGS => "awslogs",
            HostConfigLogConfigTYPE::SPLUNK => "splunk",
            HostConfigLogConfigTYPE::ETWLOGS => "etwlogs",
            HostConfigLogConfigTYPE::NONE => "none",
        }
    }
}
/// Response to an API call that returns just an Id
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdResponse {
    /// The id of the newly created object.
    #[serde(rename = "Id")]
    pub id: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdUpdateBody {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i64>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    /// Kernel memory limit in bytes.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`.
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    /// Hard limit for kernel TCP buffer memory (in bytes).
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<RestartPolicy>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IdUpdateBody1 {
    /// Name of the service.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "TaskTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ServiceSpecMode>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<ServiceSpecUpdateConfig>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<ServiceSpecRollbackConfig>,
    /// Specifies which networks the service should attach to.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<EndpointSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageDeleteResponseItem {
    /// The image ID of an image that was untagged
    #[serde(rename = "Untagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untagged: Option<String>,
    /// The image ID of an image that was deleted
    #[serde(rename = "Deleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
}
/// Image ID or Digest
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageId {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
}
/// Information about an image in the local image cache.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageInspect {
    /// ID is the content-addressable ID of an image.  This identified is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same imagem and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
    #[serde(rename = "RepoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_tags: Option<Vec<String>>,
    /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
    #[serde(rename = "RepoDigests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_digests: Option<Vec<String>>,
    /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Optional message that was set when committing or importing the image.
    #[serde(rename = "Comment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The ID of the container that was used to create the image.  Depending on how the image was created, this field may be empty.
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "ContainerConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_config: Option<ContainerConfig>,
    /// The version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty.
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
    #[serde(rename = "Author")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ContainerConfig>,
    /// Hardware CPU architecture that the image runs on.
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// CPU architecture variant (presently ARM-only).
    #[serde(rename = "Variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// Operating System the image is built to run on.
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Operating System version the image is built to run on (especially for Windows).
    #[serde(rename = "OsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Total size of the image including all layers it is composed of.
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Total size of the image including all layers it is composed of.  In versions of Docker before v1.10, this field was calculated from the image itself and all of its parent images. Docker v1.10 and up store images self-contained, and no longer use a parent-chain, making this field an equivalent of the Size field.  This field is kept for backward compatibility, but may be removed in a future version of the API.
    #[serde(rename = "VirtualSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
    #[serde(rename = "GraphDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<GraphDriverData>,
    #[serde(rename = "RootFS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_fs: Option<ImageInspectRootFs>,
    #[serde(rename = "Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ImageInspectMetadata>,
}
/// Additional metadata of the image in the local cache. This information is local to the daemon, and not part of the image itself.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageInspectMetadata {
    /// Date and time at which the image was last tagged in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if the image was tagged locally, and omitted otherwise.
    #[serde(rename = "LastTagTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tag_time: Option<String>,
}
/// Information about the image's RootFS, including the layer IDs.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageInspectRootFs {
    #[serde(rename = "Type")]
    pub _type: String,
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImagePruneResponse {
    /// Images that were deleted
    #[serde(rename = "ImagesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images_deleted: Option<Vec<ImageDeleteResponseItem>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageSearchResponseItem {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "is_official")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_official: Option<bool>,
    #[serde(rename = "is_automated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automated: Option<bool>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "star_count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ImageSummary {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    #[serde(rename = "RepoTags")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub repo_tags: Vec<String>,
    #[serde(rename = "RepoDigests")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub repo_digests: Vec<String>,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: i64,
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "Containers")]
    pub containers: i64,
}
/// IndexInfo contains information about a registry.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IndexInfo {
    /// Name of the registry, such as \"docker.io\".
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of mirrors, expressed as URIs.
    #[serde(rename = "Mirrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
    /// Indicates if the registry is part of the list of insecure registries.  If `false`, the registry is insecure. Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  > **Warning**: Insecure registries can be useful when running a local > registry. However, because its use creates security vulnerabilities > it should ONLY be enabled for testing purposes. For increased > security, users should add their CA to their system's list of > trusted CAs instead of enabling this option.
    #[serde(rename = "Secure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// Indicates whether this is an official registry (i.e., Docker Hub / docker.io)
    #[serde(rename = "Official")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Ipam {
    /// Name of the IPAM driver to use.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// List of IPAM configuration options, specified as a map:  ``` {\"Subnet\": <CIDR>, \"IPRange\": <CIDR>, \"Gateway\": <IP address>, \"AuxAddress\": <device_name:IP address>} ```
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<IpamConfig>>,
    /// Driver-specific options, specified as a map.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct IpamConfig {
    #[serde(rename = "Subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[serde(rename = "IPRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "AuxiliaryAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxiliary_addresses: Option<HashMap<String, String>>,
}
/// JoinTokens contains the tokens workers and managers need to join the swarm.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct JoinTokens {
    /// The token workers can use to join the swarm.
    #[serde(rename = "Worker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,
    /// The token managers can use to join the swarm.
    #[serde(rename = "Manager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
}
/// An object describing a limit on resources which can be requested by a task.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Limit {
    #[serde(rename = "NanoCPUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cp_us: Option<i64>,
    #[serde(rename = "MemoryBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    /// Limits the maximum number of PIDs in the container. Set `0` for unlimited.
    #[serde(rename = "Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
/// Current local status of this node.
pub enum LocalNodeState {
    #[serde(rename = "")]
    /// Current local status of this node.
    EMPTY,
    #[serde(rename = "inactive")]
    /// Current local status of this node.
    INACTIVE,
    #[serde(rename = "pending")]
    /// Current local status of this node.
    PENDING,
    #[serde(rename = "active")]
    /// Current local status of this node.
    ACTIVE,
    #[serde(rename = "error")]
    /// Current local status of this node.
    ERROR,
    #[serde(rename = "locked")]
    /// Current local status of this node.
    LOCKED,
}
impl ToString for LocalNodeState {
    fn to_string(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}
/// ManagerStatus represents the status of a manager.  It provides the current status of a node's manager component, if the node is a manager.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ManagerStatus {
    #[serde(rename = "Leader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Reachability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reachability: Option<Reachability>,
    /// The IP address and port at which the manager is reachable.
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Mount {
    /// Container path.
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Mount source (e.g. a volume name, a host path).
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<MountTYPE>,
    /// Whether the mount should be read-only.
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`.
    #[serde(rename = "Consistency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
    #[serde(rename = "BindOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_options: Option<MountBindOptions>,
    #[serde(rename = "VolumeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_options: Option<MountVolumeOptions>,
    #[serde(rename = "TmpfsOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs_options: Option<MountTmpfsOptions>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum MountTYPE {
    #[serde(rename = "bind")]
    BIND,
    #[serde(rename = "volume")]
    VOLUME,
    #[serde(rename = "tmpfs")]
    TMPFS,
    #[serde(rename = "npipe")]
    NPIPE,
}

impl ::std::fmt::Display for MountTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MountTYPE::BIND => write!(f, "bind"),
            MountTYPE::VOLUME => write!(f, "volume"),
            MountTYPE::TMPFS => write!(f, "tmpfs"),
            MountTYPE::NPIPE => write!(f, "npipe"),
        }
    }
}

impl ::std::str::FromStr for MountTYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bind" => Ok(MountTYPE::BIND),
            "volume" => Ok(MountTYPE::VOLUME),
            "tmpfs" => Ok(MountTYPE::TMPFS),
            "npipe" => Ok(MountTYPE::NPIPE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for MountTYPE {
    fn as_ref(&self) -> &str {
        match self {
            MountTYPE::BIND => "bind",
            MountTYPE::VOLUME => "volume",
            MountTYPE::TMPFS => "tmpfs",
            MountTYPE::NPIPE => "npipe",
        }
    }
}
/// Optional configuration for the `bind` type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountBindOptions {
    /// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<MountBindOptionsPROPAGATION>,
    /// Disable recursive bind mount.
    #[serde(rename = "NonRecursive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_recursive: Option<bool>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum MountBindOptionsPROPAGATION {
    #[serde(rename = "private")]
    PRIVATE,
    #[serde(rename = "rprivate")]
    RPRIVATE,
    #[serde(rename = "shared")]
    SHARED,
    #[serde(rename = "rshared")]
    RSHARED,
    #[serde(rename = "slave")]
    SLAVE,
    #[serde(rename = "rslave")]
    RSLAVE,
}

impl ::std::fmt::Display for MountBindOptionsPROPAGATION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MountBindOptionsPROPAGATION::PRIVATE => write!(f, "private"),
            MountBindOptionsPROPAGATION::RPRIVATE => write!(f, "rprivate"),
            MountBindOptionsPROPAGATION::SHARED => write!(f, "shared"),
            MountBindOptionsPROPAGATION::RSHARED => write!(f, "rshared"),
            MountBindOptionsPROPAGATION::SLAVE => write!(f, "slave"),
            MountBindOptionsPROPAGATION::RSLAVE => write!(f, "rslave"),
        }
    }
}

impl ::std::str::FromStr for MountBindOptionsPROPAGATION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "private" => Ok(MountBindOptionsPROPAGATION::PRIVATE),
            "rprivate" => Ok(MountBindOptionsPROPAGATION::RPRIVATE),
            "shared" => Ok(MountBindOptionsPROPAGATION::SHARED),
            "rshared" => Ok(MountBindOptionsPROPAGATION::RSHARED),
            "slave" => Ok(MountBindOptionsPROPAGATION::SLAVE),
            "rslave" => Ok(MountBindOptionsPROPAGATION::RSLAVE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for MountBindOptionsPROPAGATION {
    fn as_ref(&self) -> &str {
        match self {
            MountBindOptionsPROPAGATION::PRIVATE => "private",
            MountBindOptionsPROPAGATION::RPRIVATE => "rprivate",
            MountBindOptionsPROPAGATION::SHARED => "shared",
            MountBindOptionsPROPAGATION::RSHARED => "rshared",
            MountBindOptionsPROPAGATION::SLAVE => "slave",
            MountBindOptionsPROPAGATION::RSLAVE => "rslave",
        }
    }
}
/// MountPoint represents a mount point configuration inside the container. This is used for reporting the mountpoints in use by a container.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountPoint {
    /// The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<MountPointTYPE>,
    /// Name is the name reference to the underlying data defined by `Source` e.g., the volume name.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty.
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Destination is the path relative to the container root (`/`) where the `Source` is mounted inside the container.
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Driver is the volume driver used to create the volume (if it is a volume).
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`\"z\"` on Linux, empty on Windows).
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Whether the mount is mounted writable (read-write).
    #[serde(rename = "RW")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RW: Option<bool>,
    /// Propagation describes how mounts are propagated from the host into the mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt) for details. This field is not used on Windows.
    #[serde(rename = "Propagation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum MountPointTYPE {
    #[serde(rename = "bind")]
    BIND,
    #[serde(rename = "volume")]
    VOLUME,
    #[serde(rename = "tmpfs")]
    TMPFS,
    #[serde(rename = "npipe")]
    NPIPE,
}

impl ::std::fmt::Display for MountPointTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MountPointTYPE::BIND => write!(f, "bind"),
            MountPointTYPE::VOLUME => write!(f, "volume"),
            MountPointTYPE::TMPFS => write!(f, "tmpfs"),
            MountPointTYPE::NPIPE => write!(f, "npipe"),
        }
    }
}

impl ::std::str::FromStr for MountPointTYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bind" => Ok(MountPointTYPE::BIND),
            "volume" => Ok(MountPointTYPE::VOLUME),
            "tmpfs" => Ok(MountPointTYPE::TMPFS),
            "npipe" => Ok(MountPointTYPE::NPIPE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for MountPointTYPE {
    fn as_ref(&self) -> &str {
        match self {
            MountPointTYPE::BIND => "bind",
            MountPointTYPE::VOLUME => "volume",
            MountPointTYPE::TMPFS => "tmpfs",
            MountPointTYPE::NPIPE => "npipe",
        }
    }
}
/// Optional configuration for the `tmpfs` type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountTmpfsOptions {
    /// The size for the tmpfs mount in bytes.
    #[serde(rename = "SizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// The permission mode for the tmpfs mount in an integer.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
}
/// Optional configuration for the `volume` type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountVolumeOptions {
    /// Populate volume with data from the target.
    #[serde(rename = "NoCopy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_copy: Option<bool>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "DriverConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_config: Option<MountVolumeOptionsDriverConfig>,
}
/// Map of driver specific options
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct MountVolumeOptionsDriverConfig {
    /// Name of the driver to use to create the volume.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// key/value map of driver specific options.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Network {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<HashMap<String, NetworkContainer>>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}
/// Specifies how a service should be attached to a particular network.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkAttachmentConfig {
    /// The target network for attachment. Must be a network name or ID.
    #[serde(rename = "Target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Discoverable alternate names for the service on this network.
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Driver attachment options for the network target.
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkConnectRequest {
    /// The ID or name of the container to disconnect from the network.
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Force the container to disconnect from the network.
    #[serde(rename = "Force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkContainer {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "IPv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreateRequest {
    /// The network's name.
    #[serde(rename = "Name")]
    pub name: String,
    /// Check for networks with duplicate names. Since Network is primarily keyed based on a random ID and not on the name, and network name is strictly a user-friendly alias to the network which is uniquely identified using ID, there is no guaranteed way to check for duplicates. CheckDuplicate is there to provide a best effort checking of any networks which has the same name but it is not guaranteed to catch all name collisions.
    #[serde(rename = "CheckDuplicate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_duplicate: Option<bool>,
    /// Name of the network driver plugin to use.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Restrict external access to the network.
    #[serde(rename = "Internal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Globally scoped network is manually attachable by regular containers from workers in swarm mode.
    #[serde(rename = "Attachable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Ingress network is the network which provides the routing-mesh in swarm mode.
    #[serde(rename = "Ingress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "IPAM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IPAM: Option<Ipam>,
    /// Enable IPv6 on the network.
    #[serde(rename = "EnableIPv6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_pv6: Option<bool>,
    /// Network specific options to be used by the drivers.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkCreateResponse {
    /// The ID of the created network.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkDisconnectRequest {
    /// The ID or name of the container to connect to the network.
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_config: Option<EndpointSettings>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkPruneResponse {
    /// Networks that were deleted
    #[serde(rename = "NetworksDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks_deleted: Option<Vec<String>>,
}
/// NetworkSettings exposes the network settings in the API
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkSettings {
    /// Name of the network'a bridge (for example, `docker0`).
    #[serde(rename = "Bridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    /// SandboxID uniquely represents a container's network stack.
    #[serde(rename = "SandboxID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    /// Indicates if hairpin NAT should be enabled on the virtual interface.
    #[serde(rename = "HairpinMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hairpin_mode: Option<bool>,
    /// IPv6 unicast address using the link-local prefix.
    #[serde(rename = "LinkLocalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_address: Option<String>,
    /// Prefix length of the IPv6 unicast address.
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_local_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<PortMap>,
    /// SandboxKey identifies the sandbox
    #[serde(rename = "SandboxKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_i_pv6_addresses: Option<Vec<Address>>,
    /// EndpointID uniquely represents a service endpoint in a Sandbox.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "EndpointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Global IPv6 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "GlobalIPv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_address: Option<String>,
    /// Mask length of the global IPv6 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "GlobalIPv6PrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_i_pv6_prefix_len: Option<i64>,
    /// IPv4 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Mask length of the IPv4 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    /// IPv6 gateway address for this network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPv6Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_gateway: Option<String>,
    /// MAC address for the container on the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "MacAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Information about all networks that the container is connected to.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}
/// NetworkingConfig represents the container's networking configuration for each of its interfaces. It is used for the networking configs specified in the `docker create` and `docker network connect` commands.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NetworkingConfig {
    /// A mapping of network name to endpoint configuration for that network.
    #[serde(rename = "EndpointsConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_config: Option<HashMap<String, EndpointSettings>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Node {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    /// Date and time at which the node was added to the swarm in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the node was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<NodeSpec>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<NodeDescription>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NodeStatus>,
    #[serde(rename = "ManagerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_status: Option<ManagerStatus>,
}
/// NodeDescription encapsulates the properties of the Node as reported by the agent.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NodeDescription {
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceObject>,
    #[serde(rename = "Engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<EngineDescription>,
    #[serde(rename = "TLSInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<TlsInfo>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NodeSpec {
    /// Name for the node.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Role of the node.
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<NodeSpecROLE>,
    /// Availability of the node.
    #[serde(rename = "Availability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<NodeSpecAVAILABILITY>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum NodeSpecROLE {
    #[serde(rename = "worker")]
    WORKER,
    #[serde(rename = "manager")]
    MANAGER,
}

impl ::std::fmt::Display for NodeSpecROLE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NodeSpecROLE::WORKER => write!(f, "worker"),
            NodeSpecROLE::MANAGER => write!(f, "manager"),
        }
    }
}

impl ::std::str::FromStr for NodeSpecROLE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "worker" => Ok(NodeSpecROLE::WORKER),
            "manager" => Ok(NodeSpecROLE::MANAGER),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for NodeSpecROLE {
    fn as_ref(&self) -> &str {
        match self {
            NodeSpecROLE::WORKER => "worker",
            NodeSpecROLE::MANAGER => "manager",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum NodeSpecAVAILABILITY {
    #[serde(rename = "active")]
    ACTIVE,
    #[serde(rename = "pause")]
    PAUSE,
    #[serde(rename = "drain")]
    DRAIN,
}

impl ::std::fmt::Display for NodeSpecAVAILABILITY {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NodeSpecAVAILABILITY::ACTIVE => write!(f, "active"),
            NodeSpecAVAILABILITY::PAUSE => write!(f, "pause"),
            NodeSpecAVAILABILITY::DRAIN => write!(f, "drain"),
        }
    }
}

impl ::std::str::FromStr for NodeSpecAVAILABILITY {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(NodeSpecAVAILABILITY::ACTIVE),
            "pause" => Ok(NodeSpecAVAILABILITY::PAUSE),
            "drain" => Ok(NodeSpecAVAILABILITY::DRAIN),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for NodeSpecAVAILABILITY {
    fn as_ref(&self) -> &str {
        match self {
            NodeSpecAVAILABILITY::ACTIVE => "active",
            NodeSpecAVAILABILITY::PAUSE => "pause",
            NodeSpecAVAILABILITY::DRAIN => "drain",
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
/// NodeState represents the state of a node.
pub enum NodeState {
    #[serde(rename = "unknown")]
    /// NodeState represents the state of a node.
    UNKNOWN,
    #[serde(rename = "down")]
    /// NodeState represents the state of a node.
    DOWN,
    #[serde(rename = "ready")]
    /// NodeState represents the state of a node.
    READY,
    #[serde(rename = "disconnected")]
    /// NodeState represents the state of a node.
    DISCONNECTED,
}
impl ToString for NodeState {
    fn to_string(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}
/// NodeStatus represents the status of a node.  It provides the current status of the node, as seen by the manager.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NodeStatus {
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<NodeState>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// IP address of the node.
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
/// The version number of the object such as node, service, etc. This is needed to avoid conflicting writes. The client must send the version number along with the modified specification when updating these objects.  This approach ensures safe concurrency and determinism in that the change on the object may not be applied if the version number has changed from the last read. In other words, if two update requests specify the same base version, only one of the requests can succeed. As a result, two separate update requests that happen at the same time will not unintentionally overwrite each other.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ObjectVersion {
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}
/// A descriptor struct containing digest, media type, and size, as defined in the [OCI Content Descriptors Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OciDescriptor {
    /// The media type of the object this schema refers to.
    #[serde(rename = "mediaType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The digest of the targeted content.
    #[serde(rename = "digest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The size in bytes of the blob.
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
/// Describes the platform which the image in the manifest runs on, as defined in the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/image-index.md).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct OciPlatform {
    /// The CPU architecture, for example `amd64` or `ppc64`.
    #[serde(rename = "architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The operating system, for example `linux` or `windows`.
    #[serde(rename = "os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Optional field specifying the operating system version, for example on Windows `10.0.19041.1165`.
    #[serde(rename = "os.version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Optional field specifying an array of strings, each listing a required OS feature (for example on Windows `win32k`).
    #[serde(rename = "os.features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_features: Option<Vec<String>>,
    /// Optional field specifying a variant of the CPU, for example `v7` to specify ARMv7 when architecture is `arm`.
    #[serde(rename = "variant")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
/// Represents a peer-node in the swarm
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PeerNode {
    /// Unique identifier of for this node in the swarm.
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// IP address and ports at which this node can be reached.
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
/// Platform represents the platform (Arch/OS).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Platform {
    /// Architecture represents the hardware architecture (for example, `x86_64`).
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// OS represents the Operating System (for example, `linux` or `windows`).
    #[serde(rename = "OS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OS: Option<String>,
}
/// A plugin for the Engine API
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Plugin {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    /// True if the plugin is running. False if the plugin is not running, only installed.
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Settings")]
    pub settings: PluginSettings,
    /// plugin remote reference used to push/pull the plugin
    #[serde(rename = "PluginReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_reference: Option<String>,
    #[serde(rename = "Config")]
    pub config: PluginConfig,
}
/// The config of a plugin.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfig {
    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Documentation")]
    pub documentation: String,
    #[serde(rename = "Interface")]
    pub interface: PluginConfigInterface,
    #[serde(rename = "Entrypoint")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<PluginConfigUser>,
    #[serde(rename = "Network")]
    pub network: PluginConfigNetwork,
    #[serde(rename = "Linux")]
    pub linux: PluginConfigLinux,
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    #[serde(rename = "Mounts")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
    #[serde(rename = "Env")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<PluginEnv>,
    #[serde(rename = "Args")]
    pub args: PluginConfigArgs,
    #[serde(rename = "rootfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<PluginConfigRootfs>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigArgs {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub value: Vec<String>,
}
/// The interface between Docker and the plugin
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigInterface {
    #[serde(rename = "Types")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub types: Vec<PluginInterfaceType>,
    #[serde(rename = "Socket")]
    pub socket: String,
    /// Protocol to use for clients connecting to the plugin.
    #[serde(rename = "ProtocolScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_scheme: Option<PluginConfigInterfacePROTOCOL_SCHEME>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum PluginConfigInterfacePROTOCOL_SCHEME {
    #[serde(rename = "")]
    EMPTY,
    #[serde(rename = "moby.plugins.http/v1")]
    MOBYPLUGINSHTTPV1,
}

impl ::std::fmt::Display for PluginConfigInterfacePROTOCOL_SCHEME {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PluginConfigInterfacePROTOCOL_SCHEME::EMPTY => write!(f, ""),
            PluginConfigInterfacePROTOCOL_SCHEME::MOBYPLUGINSHTTPV1 => {
                write!(f, "moby.plugins.http/v1")
            }
        }
    }
}

impl ::std::str::FromStr for PluginConfigInterfacePROTOCOL_SCHEME {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(PluginConfigInterfacePROTOCOL_SCHEME::EMPTY),
            "moby.plugins.http/v1" => Ok(PluginConfigInterfacePROTOCOL_SCHEME::MOBYPLUGINSHTTPV1),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for PluginConfigInterfacePROTOCOL_SCHEME {
    fn as_ref(&self) -> &str {
        match self {
            PluginConfigInterfacePROTOCOL_SCHEME::EMPTY => "",
            PluginConfigInterfacePROTOCOL_SCHEME::MOBYPLUGINSHTTPV1 => "moby.plugins.http/v1",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigLinux {
    #[serde(rename = "Capabilities")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub capabilities: Vec<String>,
    #[serde(rename = "AllowAllDevices")]
    pub allow_all_devices: bool,
    #[serde(rename = "Devices")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigNetwork {
    #[serde(rename = "Type")]
    pub _type: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigRootfs {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "diff_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_ids: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginConfigUser {
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<i64>,
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginDevice {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Path")]
    pub path: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginEnv {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    pub value: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginInterfaceType {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Capability")]
    pub capability: String,
    #[serde(rename = "Version")]
    pub version: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginMount {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub settable: Vec<String>,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Type")]
    pub _type: String,
    #[serde(rename = "Options")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub options: Vec<String>,
}
/// Describes a permission the user has to accept upon installing the plugin.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginPrivilege {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}
/// Settings that can be modified by users.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginSettings {
    #[serde(rename = "Mounts")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub mounts: Vec<PluginMount>,
    #[serde(rename = "Env")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub env: Vec<String>,
    #[serde(rename = "Args")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub args: Vec<String>,
    #[serde(rename = "Devices")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub devices: Vec<PluginDevice>,
}
/// Available plugins per type.  <p><br /></p>  > **Note**: Only unmanaged (V1) plugins are included in this list. > V1 plugins are \"lazily\" loaded, and are not returned in this list > if there is no resource using the plugin.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PluginsInfo {
    /// Names of available volume-drivers, and network-driver plugins.
    #[serde(rename = "Volume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
    /// Names of available network-drivers, and network-driver plugins.
    #[serde(rename = "Network")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    /// Names of available authorization plugins.
    #[serde(rename = "Authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Vec<String>>,
    /// Names of available logging-drivers, and logging-driver plugins.
    #[serde(rename = "Log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<String>>,
}
/// An open port on a container
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Port {
    /// Host IP address that the container's port is mapped to
    #[serde(rename = "IP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub IP: Option<String>,
    /// Port on the container
    #[serde(rename = "PrivatePort")]
    pub private_port: i64,
    /// Port exposed on the host
    #[serde(rename = "PublicPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i64>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub _type: Option<PortTYPE>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum PortTYPE {
    #[serde(rename = "tcp")]
    TCP,
    #[serde(rename = "udp")]
    UDP,
    #[serde(rename = "sctp")]
    SCTP,
}

impl ::std::fmt::Display for PortTYPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            PortTYPE::TCP => write!(f, "tcp"),
            PortTYPE::UDP => write!(f, "udp"),
            PortTYPE::SCTP => write!(f, "sctp"),
        }
    }
}

impl ::std::str::FromStr for PortTYPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tcp" => Ok(PortTYPE::TCP),
            "udp" => Ok(PortTYPE::UDP),
            "sctp" => Ok(PortTYPE::SCTP),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for PortTYPE {
    fn as_ref(&self) -> &str {
        match self {
            PortTYPE::TCP => "tcp",
            PortTYPE::UDP => "udp",
            PortTYPE::SCTP => "sctp",
        }
    }
}
/// PortBinding represents a binding between a host IP address and a host port.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PortBinding {
    /// Host IP address that the container's port is mapped to.
    #[serde(rename = "HostIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// Host port number that the container's port is mapped to.
    #[serde(rename = "HostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<String>,
}
/// PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table.
pub type PortMap = HashMap<String, String>;
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProcessConfig {
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "tty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(rename = "entrypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ProgressDetail {
    #[serde(rename = "current")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(rename = "total")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PushImageInfo {
    #[serde(rename = "error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<ProgressDetail>,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
/// Reachability represents the reachability of a node.
pub enum Reachability {
    #[serde(rename = "unknown")]
    /// Reachability represents the reachability of a node.
    UNKNOWN,
    #[serde(rename = "unreachable")]
    /// Reachability represents the reachability of a node.
    UNREACHABLE,
    #[serde(rename = "reachable")]
    /// Reachability represents the reachability of a node.
    REACHABLE,
}
impl ToString for Reachability {
    fn to_string(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}
/// RegistryServiceConfig stores daemon registry services configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RegistryServiceConfig {
    /// List of IP ranges to which nondistributable artifacts can be pushed, using the CIDR syntax [RFC 4632](https://tools.ietf.org/html/4632).  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior, and enables the daemon to push nondistributable artifacts to all registries whose resolved IP address is within the subnet described by the CIDR syntax.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[serde(rename = "AllowNondistributableArtifactsCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_nondistributable_artifacts_cid_rs: Option<Vec<String>>,
    /// List of registry hostnames to which nondistributable artifacts can be pushed, using the format `<hostname>[:<port>]` or `<IP address>[:<port>]`.  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior for the specified registries.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[serde(rename = "AllowNondistributableArtifactsHostnames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,
    /// List of IP ranges of insecure registries, using the CIDR syntax ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  By default, local registries (`127.0.0.0/8`) are configured as insecure. All other registries are secure. Communicating with an insecure registry is not possible if the daemon assumes that registry is secure.  This configuration override this behavior, insecure communication with registries whose resolved IP address is within the subnet described by the CIDR syntax.  Registries can also be marked insecure by hostname. Those registries are listed under `IndexConfigs` and have their `Secure` field set to `false`.  > **Warning**: Using this option can be useful when running a local > registry, but introduces security vulnerabilities. This option > should therefore ONLY be used for testing purposes. For increased > security, users should add their CA to their system's list of trusted > CAs instead of enabling this option.
    #[serde(rename = "InsecureRegistryCIDRs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_registry_cid_rs: Option<Vec<String>>,
    #[serde(rename = "IndexConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_configs: Option<HashMap<String, IndexInfo>>,
    /// List of registry URLs that act as a mirror for the official (`docker.io`) registry.
    #[serde(rename = "Mirrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
}
/// An object describing the resources which can be advertised by a node and requested by a task.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cp_us: Option<i64>,
    #[serde(rename = "MemoryBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    #[serde(rename = "GenericResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_resources: Option<GenericResources>,
}
/// A container's resources (cgroups config, ulimits, etc)
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Resources {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i64>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i64>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<ResourcesBlkioWeightDevice>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<DeviceRequest>>,
    /// Kernel memory limit in bytes.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`.
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<i64>,
    /// Hard limit for kernel TCP buffer memory (in bytes).
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<i64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<i64>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ResourcesBlkioWeightDevice {
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ResourcesUlimits {
    /// Name of ulimit
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Soft limit
    #[serde(rename = "Soft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft: Option<i64>,
    /// Hard limit
    #[serde(rename = "Hard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<i64>,
}
/// The behavior to apply when the container exits. The default is not to restart.  An ever increasing delay (double the previous delay, starting at 100ms) is added before each restart to prevent flooding the server.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RestartPolicy {
    /// - Empty string means not to restart - `no` Do not automatically restart - `always` Always restart - `unless-stopped` Restart always except when the user has manually stopped the container - `on-failure` Restart only when the container exit code is non-zero
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<RestartPolicyNAME>,
    /// If `on-failure` is used, the number of times to retry before giving up.
    #[serde(rename = "MaximumRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i64>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum RestartPolicyNAME {
    #[serde(rename = "")]
    EMPTY,
    #[serde(rename = "no")]
    NO,
    #[serde(rename = "always")]
    ALWAYS,
    #[serde(rename = "unless-stopped")]
    UNLESS_STOPPED,
    #[serde(rename = "on-failure")]
    ON_FAILURE,
}

impl ::std::fmt::Display for RestartPolicyNAME {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            RestartPolicyNAME::EMPTY => write!(f, ""),
            RestartPolicyNAME::NO => write!(f, "no"),
            RestartPolicyNAME::ALWAYS => write!(f, "always"),
            RestartPolicyNAME::UNLESS_STOPPED => write!(f, "unless-stopped"),
            RestartPolicyNAME::ON_FAILURE => write!(f, "on-failure"),
        }
    }
}

impl ::std::str::FromStr for RestartPolicyNAME {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(RestartPolicyNAME::EMPTY),
            "no" => Ok(RestartPolicyNAME::NO),
            "always" => Ok(RestartPolicyNAME::ALWAYS),
            "unless-stopped" => Ok(RestartPolicyNAME::UNLESS_STOPPED),
            "on-failure" => Ok(RestartPolicyNAME::ON_FAILURE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for RestartPolicyNAME {
    fn as_ref(&self) -> &str {
        match self {
            RestartPolicyNAME::EMPTY => "",
            RestartPolicyNAME::NO => "no",
            RestartPolicyNAME::ALWAYS => "always",
            RestartPolicyNAME::UNLESS_STOPPED => "unless-stopped",
            RestartPolicyNAME::ON_FAILURE => "on-failure",
        }
    }
}
/// Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec) runtime.  The runtime is invoked by the daemon via the `containerd` daemon. OCI runtimes act as an interface to the Linux kernel namespaces, cgroups, and SELinux.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Runtime {
    /// Name and, optional, path, of the OCI executable binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// List of command-line arguments to pass to the runtime when invoked.
    #[serde(rename = "runtimeArgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_args: Option<Vec<String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Secret {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SecretSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretSpec {
    /// User-defined name of the secret.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<Driver>,
    #[serde(rename = "Templating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templating: Option<Driver>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SecretsCreateBody {
    /// User-defined name of the secret.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[serde(rename = "Data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<Driver>,
    #[serde(rename = "Templating")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templating: Option<Driver>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Service {
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<ServiceSpec>,
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<ServiceEndpoint>,
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<ServiceUpdateStatus>,
    #[serde(rename = "ServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_status: Option<ServiceServiceStatus>,
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<ServiceJobStatus>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceCreateResponse {
    /// The ID of the created service.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// Optional warning message
    #[serde(rename = "Warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceEndpoint {
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<EndpointSpec>,
    #[serde(rename = "Ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<EndpointPortConfig>>,
    #[serde(rename = "VirtualIPs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_i_ps: Option<Vec<ServiceEndpointVirtualIPs>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceEndpointVirtualIPs {
    #[serde(rename = "NetworkID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "Addr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
/// The status of the service when it is in one of ReplicatedJob or GlobalJob modes. Absent on Replicated and Global mode services. The JobIteration is an ObjectVersion, but unlike the Service's version, does not need to be sent with an update request.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceJobStatus {
    #[serde(rename = "JobIteration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_iteration: Option<ObjectVersion>,
    /// The last time, as observed by the server, that this job was started.
    #[serde(rename = "LastExecution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
}
/// The status of the service's tasks. Provided only when requested as part of a ServiceList operation.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceServiceStatus {
    /// The number of tasks for the service currently in the Running state.
    #[serde(rename = "RunningTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks: Option<i64>,
    /// The number of tasks for the service desired to be running. For replicated services, this is the replica count from the service spec. For global services, this is computed by taking count of all tasks for the service with a Desired State other than Shutdown.
    #[serde(rename = "DesiredTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_tasks: Option<i64>,
    /// The number of tasks for a job that are in the Completed state. This field must be cross-referenced with the service type, as the value of 0 may mean the service is not in a job mode, or it may mean the job-mode service has no tasks yet Completed.
    #[serde(rename = "CompletedTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_tasks: Option<i64>,
}
/// User modifiable configuration for a service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpec {
    /// Name of the service.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "TaskTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ServiceSpecMode>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<ServiceSpecUpdateConfig>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<ServiceSpecRollbackConfig>,
    /// Specifies which networks the service should attach to.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<EndpointSpec>,
}
/// Scheduling mode for the service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpecMode {
    #[serde(rename = "Replicated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated: Option<ServiceSpecModeReplicated>,
    #[serde(rename = "Global")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<Value>,
    #[serde(rename = "ReplicatedJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated_job: Option<ServiceSpecModeReplicatedJob>,
    /// The mode used for services which run a task to the completed state on each valid node.
    #[serde(rename = "GlobalJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_job: Option<Value>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpecModeReplicated {
    #[serde(rename = "Replicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}
/// The mode used for services with a finite number of tasks that run to a completed state.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpecModeReplicatedJob {
    /// The maximum number of replicas to run simultaneously.
    #[serde(rename = "MaxConcurrent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<i64>,
    /// The total number of replicas desired to reach the Completed state. If unset, will default to the value of `MaxConcurrent`
    #[serde(rename = "TotalCompletions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_completions: Option<i64>,
}
/// Specification for the rollback strategy of the service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpecRollbackConfig {
    /// Maximum number of tasks to be rolled back in one iteration (0 means unlimited parallelism).
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// Amount of time between rollback iterations, in nanoseconds.
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Action to take if an rolled back task fails to run, or stops running during the rollback.
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<ServiceSpecRollbackConfigFAILURE_ACTION>,
    /// Amount of time to monitor each rolled back task for failures, in nanoseconds.
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<i64>,
    /// The fraction of tasks that may fail during a rollback before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f32>,
    /// The order of operations when rolling back a task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ServiceSpecRollbackConfigORDER>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ServiceSpecRollbackConfigFAILURE_ACTION {
    #[serde(rename = "continue")]
    _CONTINUE,
    #[serde(rename = "pause")]
    PAUSE,
}

impl ::std::fmt::Display for ServiceSpecRollbackConfigFAILURE_ACTION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ServiceSpecRollbackConfigFAILURE_ACTION::_CONTINUE => write!(f, "continue"),
            ServiceSpecRollbackConfigFAILURE_ACTION::PAUSE => write!(f, "pause"),
        }
    }
}

impl ::std::str::FromStr for ServiceSpecRollbackConfigFAILURE_ACTION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "continue" => Ok(ServiceSpecRollbackConfigFAILURE_ACTION::_CONTINUE),
            "pause" => Ok(ServiceSpecRollbackConfigFAILURE_ACTION::PAUSE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ServiceSpecRollbackConfigFAILURE_ACTION {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecRollbackConfigFAILURE_ACTION::_CONTINUE => "continue",
            ServiceSpecRollbackConfigFAILURE_ACTION::PAUSE => "pause",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ServiceSpecRollbackConfigORDER {
    #[serde(rename = "stop-first")]
    STOP_FIRST,
    #[serde(rename = "start-first")]
    START_FIRST,
}

impl ::std::fmt::Display for ServiceSpecRollbackConfigORDER {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ServiceSpecRollbackConfigORDER::STOP_FIRST => write!(f, "stop-first"),
            ServiceSpecRollbackConfigORDER::START_FIRST => write!(f, "start-first"),
        }
    }
}

impl ::std::str::FromStr for ServiceSpecRollbackConfigORDER {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stop-first" => Ok(ServiceSpecRollbackConfigORDER::STOP_FIRST),
            "start-first" => Ok(ServiceSpecRollbackConfigORDER::START_FIRST),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ServiceSpecRollbackConfigORDER {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecRollbackConfigORDER::STOP_FIRST => "stop-first",
            ServiceSpecRollbackConfigORDER::START_FIRST => "start-first",
        }
    }
}
/// Specification for the update strategy of the service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceSpecUpdateConfig {
    /// Maximum number of tasks to be updated in one iteration (0 means unlimited parallelism).
    #[serde(rename = "Parallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// Amount of time between updates, in nanoseconds.
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Action to take if an updated task fails to run, or stops running during the update.
    #[serde(rename = "FailureAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<ServiceSpecUpdateConfigFAILURE_ACTION>,
    /// Amount of time to monitor each updated task for failures, in nanoseconds.
    #[serde(rename = "Monitor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<i64>,
    /// The fraction of tasks that may fail during an update before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[serde(rename = "MaxFailureRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f32>,
    /// The order of operations when rolling out an updated task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[serde(rename = "Order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ServiceSpecUpdateConfigORDER>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ServiceSpecUpdateConfigFAILURE_ACTION {
    #[serde(rename = "continue")]
    _CONTINUE,
    #[serde(rename = "pause")]
    PAUSE,
    #[serde(rename = "rollback")]
    ROLLBACK,
}

impl ::std::fmt::Display for ServiceSpecUpdateConfigFAILURE_ACTION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ServiceSpecUpdateConfigFAILURE_ACTION::_CONTINUE => write!(f, "continue"),
            ServiceSpecUpdateConfigFAILURE_ACTION::PAUSE => write!(f, "pause"),
            ServiceSpecUpdateConfigFAILURE_ACTION::ROLLBACK => write!(f, "rollback"),
        }
    }
}

impl ::std::str::FromStr for ServiceSpecUpdateConfigFAILURE_ACTION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "continue" => Ok(ServiceSpecUpdateConfigFAILURE_ACTION::_CONTINUE),
            "pause" => Ok(ServiceSpecUpdateConfigFAILURE_ACTION::PAUSE),
            "rollback" => Ok(ServiceSpecUpdateConfigFAILURE_ACTION::ROLLBACK),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ServiceSpecUpdateConfigFAILURE_ACTION {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecUpdateConfigFAILURE_ACTION::_CONTINUE => "continue",
            ServiceSpecUpdateConfigFAILURE_ACTION::PAUSE => "pause",
            ServiceSpecUpdateConfigFAILURE_ACTION::ROLLBACK => "rollback",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ServiceSpecUpdateConfigORDER {
    #[serde(rename = "stop-first")]
    STOP_FIRST,
    #[serde(rename = "start-first")]
    START_FIRST,
}

impl ::std::fmt::Display for ServiceSpecUpdateConfigORDER {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ServiceSpecUpdateConfigORDER::STOP_FIRST => write!(f, "stop-first"),
            ServiceSpecUpdateConfigORDER::START_FIRST => write!(f, "start-first"),
        }
    }
}

impl ::std::str::FromStr for ServiceSpecUpdateConfigORDER {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "stop-first" => Ok(ServiceSpecUpdateConfigORDER::STOP_FIRST),
            "start-first" => Ok(ServiceSpecUpdateConfigORDER::START_FIRST),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ServiceSpecUpdateConfigORDER {
    fn as_ref(&self) -> &str {
        match self {
            ServiceSpecUpdateConfigORDER::STOP_FIRST => "stop-first",
            ServiceSpecUpdateConfigORDER::START_FIRST => "start-first",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceUpdateResponse {
    /// Optional warning messages
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
/// The status of a service update.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServiceUpdateStatus {
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ServiceUpdateStatusSTATE>,
    #[serde(rename = "StartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "CompletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum ServiceUpdateStatusSTATE {
    #[serde(rename = "updating")]
    UPDATING,
    #[serde(rename = "paused")]
    PAUSED,
    #[serde(rename = "completed")]
    COMPLETED,
}

impl ::std::fmt::Display for ServiceUpdateStatusSTATE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ServiceUpdateStatusSTATE::UPDATING => write!(f, "updating"),
            ServiceUpdateStatusSTATE::PAUSED => write!(f, "paused"),
            ServiceUpdateStatusSTATE::COMPLETED => write!(f, "completed"),
        }
    }
}

impl ::std::str::FromStr for ServiceUpdateStatusSTATE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "updating" => Ok(ServiceUpdateStatusSTATE::UPDATING),
            "paused" => Ok(ServiceUpdateStatusSTATE::PAUSED),
            "completed" => Ok(ServiceUpdateStatusSTATE::COMPLETED),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for ServiceUpdateStatusSTATE {
    fn as_ref(&self) -> &str {
        match self {
            ServiceUpdateStatusSTATE::UPDATING => "updating",
            ServiceUpdateStatusSTATE::PAUSED => "paused",
            ServiceUpdateStatusSTATE::COMPLETED => "completed",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ServicesCreateBody {
    /// Name of the service.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "TaskTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_template: Option<TaskSpec>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<ServiceSpecMode>,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_config: Option<ServiceSpecUpdateConfig>,
    #[serde(rename = "RollbackConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<ServiceSpecRollbackConfig>,
    /// Specifies which networks the service should attach to.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<EndpointSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Swarm {
    /// The ID of the swarm.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SwarmSpec>,
    #[serde(rename = "TLSInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<TlsInfo>,
    /// Whether there is currently a root CA rotation in progress for the swarm
    #[serde(rename = "RootRotationInProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_rotation_in_progress: Option<bool>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i64>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i64>,
    #[serde(rename = "JoinTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_tokens: Option<JoinTokens>,
}
/// Represents generic information about swarm.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmInfo {
    /// Unique identifier of for this node in the swarm.
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// IP address at which this node can be reached by other nodes in the swarm.
    #[serde(rename = "NodeAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_addr: Option<String>,
    #[serde(rename = "LocalNodeState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_node_state: Option<LocalNodeState>,
    #[serde(rename = "ControlAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_available: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// List of ID's and addresses of other managers in the swarm.
    #[serde(rename = "RemoteManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_managers: Option<Vec<PeerNode>>,
    /// Total number of nodes in the swarm.
    #[serde(rename = "Nodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<i64>,
    /// Total number of managers in the swarm.
    #[serde(rename = "Managers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<i64>,
    #[serde(rename = "Cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<ClusterInfo>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmInitRequest {
    /// Listen address used for inter-manager communication, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP). This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the default swarm listening port is used.
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other  nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. if no port is set or is set to 0, default port 4789 will be used.
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i64>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// Force creation of a new swarm.
    #[serde(rename = "ForceNewCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_cluster: Option<bool>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i64>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SwarmSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmInitRequest1 {
    /// Listen address used for inter-manager communication, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP). This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the default swarm listening port is used.
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other  nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. if no port is set or is set to 0, default port 4789 will be used.
    #[serde(rename = "DataPathPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i64>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// Force creation of a new swarm.
    #[serde(rename = "ForceNewCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_cluster: Option<bool>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i64>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<SwarmSpec>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmJoinRequest {
    /// Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same addres as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// Addresses of manager nodes already participating in the swarm.
    #[serde(rename = "RemoteAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_addrs: Option<Vec<String>>,
    /// Secret token for joining this swarm.
    #[serde(rename = "JoinToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmJoinRequest1 {
    /// Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
    #[serde(rename = "ListenAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same addres as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// Addresses of manager nodes already participating in the swarm.
    #[serde(rename = "RemoteAddrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_addrs: Option<Vec<String>>,
    /// Secret token for joining this swarm.
    #[serde(rename = "JoinToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}
/// User modifiable swarm configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpec {
    /// Name of the swarm.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Orchestration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration: Option<SwarmSpecOrchestration>,
    #[serde(rename = "Raft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raft: Option<SwarmSpecRaft>,
    #[serde(rename = "Dispatcher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispatcher: Option<SwarmSpecDispatcher>,
    #[serde(rename = "CAConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_config: Option<SwarmSpecCaConfig>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<SwarmSpecEncryptionConfig>,
    #[serde(rename = "TaskDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_defaults: Option<SwarmSpecTaskDefaults>,
}
/// CA configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecCaConfig {
    /// The duration node certificates are issued for.
    #[serde(rename = "NodeCertExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_cert_expiry: Option<i64>,
    /// Configuration for forwarding signing requests to an external certificate authority.
    #[serde(rename = "ExternalCAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_c_as: Option<Vec<SwarmSpecCaConfigExternalCAs>>,
    /// The desired signing CA certificate for all swarm node TLS leaf certificates, in PEM format.
    #[serde(rename = "SigningCACert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_ca_cert: Option<String>,
    /// The desired signing CA key for all swarm node TLS leaf certificates, in PEM format.
    #[serde(rename = "SigningCAKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_ca_key: Option<String>,
    /// An integer whose purpose is to force swarm to generate a new signing CA certificate and key, if none have been specified in `SigningCACert` and `SigningCAKey`
    #[serde(rename = "ForceRotate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_rotate: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecCaConfigExternalCAs {
    /// Protocol for communication with the external CA (currently only `cfssl` is supported).
    #[serde(rename = "Protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<SwarmSpecCaConfigExternalCAsPROTOCOL>,
    /// URL where certificate signing requests should be sent.
    #[serde(rename = "URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub URL: Option<String>,
    /// An object with key/value pairs that are interpreted as protocol-specific options for the external CA driver.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
    /// The root CA certificate (in PEM format) this external CA uses to issue TLS certificates (assumed to be to the current swarm root CA certificate if not provided).
    #[serde(rename = "CACert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_cert: Option<String>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum SwarmSpecCaConfigExternalCAsPROTOCOL {
    #[serde(rename = "cfssl")]
    CFSSL,
}

impl ::std::fmt::Display for SwarmSpecCaConfigExternalCAsPROTOCOL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SwarmSpecCaConfigExternalCAsPROTOCOL::CFSSL => write!(f, "cfssl"),
        }
    }
}

impl ::std::str::FromStr for SwarmSpecCaConfigExternalCAsPROTOCOL {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cfssl" => Ok(SwarmSpecCaConfigExternalCAsPROTOCOL::CFSSL),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for SwarmSpecCaConfigExternalCAsPROTOCOL {
    fn as_ref(&self) -> &str {
        match self {
            SwarmSpecCaConfigExternalCAsPROTOCOL::CFSSL => "cfssl",
        }
    }
}
/// Dispatcher configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecDispatcher {
    /// The delay for an agent to send a heartbeat to the dispatcher.
    #[serde(rename = "HeartbeatPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_period: Option<i64>,
}
/// Parameters related to encryption-at-rest.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecEncryptionConfig {
    /// If set, generate a key and use it to lock data stored on the managers.
    #[serde(rename = "AutoLockManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_lock_managers: Option<bool>,
}
/// Orchestration configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecOrchestration {
    /// The number of historic tasks to keep per instance or node. If negative, never remove completed or failed tasks.
    #[serde(rename = "TaskHistoryRetentionLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_history_retention_limit: Option<i64>,
}
/// Raft configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecRaft {
    /// The number of log entries between snapshots.
    #[serde(rename = "SnapshotInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_interval: Option<i64>,
    /// The number of snapshots to keep beyond the current snapshot.
    #[serde(rename = "KeepOldSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_old_snapshots: Option<i64>,
    /// The number of log entries to keep around to sync up slow followers after a snapshot is created.
    #[serde(rename = "LogEntriesForSlowFollowers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_entries_for_slow_followers: Option<i64>,
    /// The number of ticks that a follower will wait for a message from the leader before becoming a candidate and starting an election. `ElectionTick` must be greater than `HeartbeatTick`.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "ElectionTick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub election_tick: Option<i64>,
    /// The number of ticks between heartbeats. Every HeartbeatTick ticks, the leader will send a heartbeat to the followers.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "HeartbeatTick")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat_tick: Option<i64>,
}
/// Defaults for creating tasks in this cluster.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecTaskDefaults {
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<SwarmSpecTaskDefaultsLogDriver>,
}
/// The log driver to use for tasks created in the orchestrator if unspecified by a service.  Updating this value only affects new tasks. Existing tasks continue to use their previously configured log driver until recreated.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmSpecTaskDefaultsLogDriver {
    /// The log driver to use as a default for new tasks.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Driver-specific options for the selectd log driver, specified as key/value pairs.
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SwarmUnlockRequest {
    /// The swarm's unlock key.
    #[serde(rename = "UnlockKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_key: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemAuthResponse {
    /// The status of the authentication
    #[serde(rename = "Status")]
    pub status: String,
    /// An opaque token used to authenticate a user after a successful login
    #[serde(rename = "IdentityToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_token: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemDataUsageResponse {
    #[serde(rename = "LayersSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers_size: Option<i64>,
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ImageSummary>>,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerSummary>>,
    #[serde(rename = "Volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
    #[serde(rename = "BuildCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_cache: Option<Vec<BuildCache>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemInfo {
    /// Unique identifier of the daemon.  <p><br /></p>  > **Note**: The format of the ID itself is not part of the API, and > should not be considered stable.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    /// Total number of containers on the host.
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<i64>,
    /// Number of containers with status `\"running\"`.
    #[serde(rename = "ContainersRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_running: Option<i64>,
    /// Number of containers with status `\"paused\"`.
    #[serde(rename = "ContainersPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_paused: Option<i64>,
    /// Number of containers with status `\"stopped\"`.
    #[serde(rename = "ContainersStopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers_stopped: Option<i64>,
    /// Total number of images on the host.  Both _tagged_ and _untagged_ (dangling) images are counted.
    #[serde(rename = "Images")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<i64>,
    /// Name of the storage driver in use.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Information specific to the storage driver, provided as \"label\" / \"value\" pairs.  This information is provided by the storage driver, and formatted in a way consistent with the output of `docker info` on the command line.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.
    #[serde(rename = "DriverStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_status: Option<Vec<Vec<String>>>,
    /// Root directory of persistent Docker state.  Defaults to `/var/lib/docker` on Linux, and `C:\\ProgramData\\docker` on Windows.
    #[serde(rename = "DockerRootDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_root_dir: Option<String>,
    #[serde(rename = "Plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<PluginsInfo>,
    /// Indicates if the host has memory limit support enabled.
    #[serde(rename = "MemoryLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<bool>,
    /// Indicates if the host has memory swap limit support enabled.
    #[serde(rename = "SwapLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_limit: Option<bool>,
    /// Indicates if the host has kernel memory limit support enabled.  <p><br /></p>  > **Deprecated**: This field is deprecated as the kernel 5.4 deprecated > `kmem.limit_in_bytes`.
    #[serde(rename = "KernelMemory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory: Option<bool>,
    /// Indicates if the host has kernel memory TCP limit support enabled.  Kernel memory TCP limits are not supported when using cgroups v2, which does not support the corresponding `memory.kmem.tcp.limit_in_bytes` cgroup.
    #[serde(rename = "KernelMemoryTCP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) period is supported by the host.
    #[serde(rename = "CpuCfsPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_period: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by the host.
    #[serde(rename = "CpuCfsQuota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota: Option<bool>,
    /// Indicates if CPU Shares limiting is supported by the host.
    #[serde(rename = "CPUShares")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<bool>,
    /// Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.  See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt)
    #[serde(rename = "CPUSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_set: Option<bool>,
    /// Indicates if the host kernel has PID limit support enabled.
    #[serde(rename = "PidsLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<bool>,
    /// Indicates if OOM killer disable is supported on the host.
    #[serde(rename = "OomKillDisable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Indicates IPv4 forwarding is enabled.
    #[serde(rename = "IPv4Forwarding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv4_forwarding: Option<bool>,
    /// Indicates if `bridge-nf-call-iptables` is available on the host.
    #[serde(rename = "BridgeNfIptables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_nf_iptables: Option<bool>,
    /// Indicates if `bridge-nf-call-ip6tables` is available on the host.
    #[serde(rename = "BridgeNfIp6tables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_nf_ip6tables: Option<bool>,
    /// Indicates if the daemon is running in debug-mode / with debug-level logging enabled.
    #[serde(rename = "Debug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// The total number of file Descriptors in use by the daemon process.  This information is only returned if debug-mode is enabled.
    #[serde(rename = "NFd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_fd: Option<i64>,
    /// The  number of goroutines that currently exist.  This information is only returned if debug-mode is enabled.
    #[serde(rename = "NGoroutines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_goroutines: Option<i64>,
    /// Current system-time in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "SystemTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_time: Option<String>,
    /// The logging driver to use as a default for new containers.
    #[serde(rename = "LoggingDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_driver: Option<String>,
    /// The driver to use for managing cgroups.
    #[serde(rename = "CgroupDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_driver: Option<SystemInfoCGROUP_DRIVER>,
    /// The version of the cgroup.
    #[serde(rename = "CgroupVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cgroup_version: Option<SystemInfoCGROUP_VERSION>,
    /// Number of event listeners subscribed.
    #[serde(rename = "NEventsListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_events_listener: Option<i64>,
    /// Kernel version of the host.  On Linux, this information obtained from `uname`. On Windows this information is queried from the <kbd>HKEY_LOCAL_MACHINE\\\\SOFTWARE\\\\Microsoft\\\\Windows NT\\\\CurrentVersion\\\\</kbd> registry value, for example _\"10.0 14393 (14393.1198.amd64fre.rs1_release_sec.170427-1353)\"_.
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Name of the host's operating system, for example: \"Ubuntu 16.04.2 LTS\" or \"Windows Server 2016 Datacenter\"
    #[serde(rename = "OperatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Version of the host's operating system  <p><br /></p>  > **Note**: The information returned in this field, including its > very existence, and the formatting of values, should not be considered > stable, and may change without notice.
    #[serde(rename = "OSVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Generic type of the operating system of the host, as returned by the Go runtime (`GOOS`).  Currently returned values are \"linux\" and \"windows\". A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment).
    #[serde(rename = "OSType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Hardware architecture of the host, as returned by the Go runtime (`GOARCH`).  A full list of possible values can be found in the [Go documentation](https://golang.org/doc/install/source#environment).
    #[serde(rename = "Architecture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The number of logical CPUs usable by the daemon.  The number of available CPUs is checked by querying the operating system when the daemon starts. Changes to operating system CPU allocation after the daemon is started are not reflected.
    #[serde(rename = "NCPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub NCPU: Option<i64>,
    /// Total amount of physical memory available on the host, in bytes.
    #[serde(rename = "MemTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mem_total: Option<i64>,
    /// Address / URL of the index server that is used for image search, and as a default for user authentication for Docker Hub and Docker Cloud.
    #[serde(rename = "IndexServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_server_address: Option<String>,
    #[serde(rename = "RegistryConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_config: Option<RegistryServiceConfig>,
    #[serde(rename = "GenericResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_resources: Option<GenericResources>,
    /// HTTP-proxy configured for the daemon. This value is obtained from the [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[serde(rename = "HttpProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    /// HTTPS-proxy configured for the daemon. This value is obtained from the [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[serde(rename = "HttpsProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    /// Comma-separated list of domain extensions for which no proxy should be used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.  Containers do not automatically inherit this configuration.
    #[serde(rename = "NoProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,
    /// Hostname of the host.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key/value metadata) as set on the daemon.  <p><br /></p>  > **Note**: When part of a Swarm, nodes can both have _daemon_ labels, > set through the daemon configuration, and _node_ labels, set from a > manager node in the Swarm. Node labels are not included in this > field. Node labels can be retrieved using the `/nodes/(id)` endpoint > on a manager node in the Swarm.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Indicates if experimental features are enabled on the daemon.
    #[serde(rename = "ExperimentalBuild")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental_build: Option<bool>,
    /// Version string of the daemon.  > **Note**: the [standalone Swarm API](/swarm/swarm-api/) > returns the Swarm version instead of the daemon  version, for example > `swarm/1.2.8`.
    #[serde(rename = "ServerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    /// URL of the distributed storage backend.   The storage backend is used for multihost networking (to store network and endpoint information) and by the node discovery mechanism.  <p><br /></p>  > **Deprecated**: This field is only propagated when using standalone Swarm > mode, and overlay networking using an external k/v store. Overlay > networks with Swarm mode enabled use the built-in raft store, and > this field will be empty.
    #[serde(rename = "ClusterStore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_store: Option<String>,
    /// The network endpoint that the Engine advertises for the purpose of node discovery. ClusterAdvertise is a `host:port` combination on which the daemon is reachable by other hosts.  <p><br /></p>  > **Deprecated**: This field is only propagated when using standalone Swarm > mode, and overlay networking using an external k/v store. Overlay > networks with Swarm mode enabled use the built-in raft store, and > this field will be empty.
    #[serde(rename = "ClusterAdvertise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_advertise: Option<String>,
    /// List of [OCI compliant](https://github.com/opencontainers/runtime-spec) runtimes configured on the daemon. Keys hold the \"name\" used to reference the runtime.  The Docker daemon relies on an OCI compliant runtime (invoked via the `containerd` daemon) as its interface to the Linux kernel namespaces, cgroups, and SELinux.  The default runtime is `runc`, and automatically configured. Additional runtimes can be configured by the user and will be listed here.
    #[serde(rename = "Runtimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<HashMap<String, Runtime>>,
    /// Name of the default OCI runtime that is used when starting containers.  The default can be overridden per-container at create time.
    #[serde(rename = "DefaultRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime: Option<String>,
    #[serde(rename = "Swarm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swarm: Option<SwarmInfo>,
    /// Indicates if live restore is enabled.  If enabled, containers are kept running when the daemon is shutdown or upon daemon start if running containers are detected.
    #[serde(rename = "LiveRestoreEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_restore_enabled: Option<bool>,
    /// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms.
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<SystemInfoISOLATION>,
    /// Name and, optional, path of the `docker-init` binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[serde(rename = "InitBinary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_binary: Option<String>,
    #[serde(rename = "ContainerdCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containerd_commit: Option<Commit>,
    #[serde(rename = "RuncCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runc_commit: Option<Commit>,
    #[serde(rename = "InitCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_commit: Option<Commit>,
    /// List of security features that are enabled on the daemon, such as apparmor, seccomp, SELinux, user-namespaces (userns), and rootless.  Additional configuration options for each security feature may be present, and are included as a comma-separated list of key/value pairs.
    #[serde(rename = "SecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_options: Option<Vec<String>>,
    /// Reports a summary of the product license on the daemon.  If a commercial license has been applied to the daemon, information such as number of nodes, and expiration are included.
    #[serde(rename = "ProductLicense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_license: Option<String>,
    /// List of custom default address pools for local networks, which can be specified in the daemon.json file or dockerd option.  Example: a Base \"10.10.0.0/16\" with Size 24 will define the set of 256 10.10.[0-255].0/24 address pools.
    #[serde(rename = "DefaultAddressPools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_address_pools: Option<Vec<SystemInfoDefaultAddressPools>>,
    /// List of warnings / informational messages about missing features, or issues related to the daemon configuration.  These messages can be printed by the client as information to the user.
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum SystemInfoCGROUP_DRIVER {
    #[serde(rename = "cgroupfs")]
    CGROUPFS,
    #[serde(rename = "systemd")]
    SYSTEMD,
    #[serde(rename = "none")]
    NONE,
}

impl ::std::fmt::Display for SystemInfoCGROUP_DRIVER {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SystemInfoCGROUP_DRIVER::CGROUPFS => write!(f, "cgroupfs"),
            SystemInfoCGROUP_DRIVER::SYSTEMD => write!(f, "systemd"),
            SystemInfoCGROUP_DRIVER::NONE => write!(f, "none"),
        }
    }
}

impl ::std::str::FromStr for SystemInfoCGROUP_DRIVER {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cgroupfs" => Ok(SystemInfoCGROUP_DRIVER::CGROUPFS),
            "systemd" => Ok(SystemInfoCGROUP_DRIVER::SYSTEMD),
            "none" => Ok(SystemInfoCGROUP_DRIVER::NONE),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for SystemInfoCGROUP_DRIVER {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoCGROUP_DRIVER::CGROUPFS => "cgroupfs",
            SystemInfoCGROUP_DRIVER::SYSTEMD => "systemd",
            SystemInfoCGROUP_DRIVER::NONE => "none",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum SystemInfoCGROUP_VERSION {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
}

impl ::std::fmt::Display for SystemInfoCGROUP_VERSION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SystemInfoCGROUP_VERSION::_1 => write!(f, "1"),
            SystemInfoCGROUP_VERSION::_2 => write!(f, "2"),
        }
    }
}

impl ::std::str::FromStr for SystemInfoCGROUP_VERSION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(SystemInfoCGROUP_VERSION::_1),
            "2" => Ok(SystemInfoCGROUP_VERSION::_2),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for SystemInfoCGROUP_VERSION {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoCGROUP_VERSION::_1 => "1",
            SystemInfoCGROUP_VERSION::_2 => "2",
        }
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum SystemInfoISOLATION {
    #[serde(rename = "default")]
    DEFAULT,
    #[serde(rename = "hyperv")]
    HYPERV,
    #[serde(rename = "process")]
    PROCESS,
}

impl ::std::fmt::Display for SystemInfoISOLATION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            SystemInfoISOLATION::DEFAULT => write!(f, "default"),
            SystemInfoISOLATION::HYPERV => write!(f, "hyperv"),
            SystemInfoISOLATION::PROCESS => write!(f, "process"),
        }
    }
}

impl ::std::str::FromStr for SystemInfoISOLATION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(SystemInfoISOLATION::DEFAULT),
            "hyperv" => Ok(SystemInfoISOLATION::HYPERV),
            "process" => Ok(SystemInfoISOLATION::PROCESS),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for SystemInfoISOLATION {
    fn as_ref(&self) -> &str {
        match self {
            SystemInfoISOLATION::DEFAULT => "default",
            SystemInfoISOLATION::HYPERV => "hyperv",
            SystemInfoISOLATION::PROCESS => "process",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemInfoDefaultAddressPools {
    /// The network address in CIDR format
    #[serde(rename = "Base")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// The network pool size
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
/// Response of Engine API: GET \"/version\"
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemVersion {
    #[serde(rename = "Platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SystemVersionPlatform>,
    /// Information about system components
    #[serde(rename = "Components")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<SystemVersionComponents>>,
    /// The version of the daemon
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The default (and highest) API version that is supported by the daemon
    #[serde(rename = "ApiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The minimum API version that is supported by the daemon
    #[serde(rename = "MinAPIVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_api_version: Option<String>,
    /// The Git commit of the source code that was used to build the daemon
    #[serde(rename = "GitCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    /// The version Go used to compile the daemon, and the version of the Go runtime in use.
    #[serde(rename = "GoVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    /// The operating system that the daemon is running on (\"linux\" or \"windows\")
    #[serde(rename = "Os")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The architecture that the daemon is running on
    #[serde(rename = "Arch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// The kernel version (`uname -r`) that the daemon is running on.  This field is omitted when empty.
    #[serde(rename = "KernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Indicates if the daemon is started with experimental features enabled.  This field is omitted when empty / false.
    #[serde(rename = "Experimental")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    /// The date and time that the daemon was compiled.
    #[serde(rename = "BuildTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemVersionComponents {
    /// Name of the component
    #[serde(rename = "Name")]
    pub name: String,
    /// Version of the component
    #[serde(rename = "Version")]
    pub version: String,
    /// Key/value pairs of strings with additional information about the component. These values are intended for informational purposes only, and their content is not defined, and not part of the API specification.  These messages can be printed by the client as information to the user.
    #[serde(rename = "Details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SystemVersionPlatform {
    #[serde(rename = "Name")]
    pub name: String,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    /// The ID of the task.
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ID: Option<String>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ObjectVersion>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Name of the task.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<TaskSpec>,
    /// The ID of the service this task is part of.
    #[serde(rename = "ServiceID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "Slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<i64>,
    /// The ID of the node that this task is on.
    #[serde(rename = "NodeID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "AssignedGenericResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_generic_resources: Option<GenericResources>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    #[serde(rename = "DesiredState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<TaskState>,
    #[serde(rename = "JobIteration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_iteration: Option<ObjectVersion>,
}
/// User modifiable task configuration.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpec {
    #[serde(rename = "PluginSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_spec: Option<TaskSpecPluginSpec>,
    #[serde(rename = "ContainerSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_spec: Option<TaskSpecContainerSpec>,
    #[serde(rename = "NetworkAttachmentSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_attachment_spec: Option<TaskSpecNetworkAttachmentSpec>,
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<TaskSpecResources>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<TaskSpecRestartPolicy>,
    #[serde(rename = "Placement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<TaskSpecPlacement>,
    /// A counter that triggers an update even if no relevant parameters have been changed.
    #[serde(rename = "ForceUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<i64>,
    /// Runtime is the type of runtime specified for the task executor.
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Specifies which networks the service should attach to.
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<TaskSpecLogDriver>,
}
/// Container spec for the service.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpec {
    /// The image name to use for the container
    #[serde(rename = "Image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// User-defined key/value data.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
    /// The command to be run in the image.
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Arguments to the command.
    #[serde(rename = "Args")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// The hostname to use for the container, as a valid [RFC 1123](https://tools.ietf.org/html/rfc1123) hostname.
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// A list of environment variables in the form `VAR=value`.
    #[serde(rename = "Env")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// The working directory for commands to run in.
    #[serde(rename = "Dir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    /// The user inside the container.
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// A list of additional groups that the container process will run as.
    #[serde(rename = "Groups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "Privileges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileges: Option<TaskSpecContainerSpecPrivileges>,
    /// Whether a pseudo-TTY should be allocated.
    #[serde(rename = "TTY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TTY: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Mount the container's root filesystem as read only.
    #[serde(rename = "ReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Specification for mounts to be added to containers created as part of the service.
    #[serde(rename = "Mounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    /// Signal to stop the container.
    #[serde(rename = "StopSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// Amount of time to wait for the container to terminate before forcefully killing it.
    #[serde(rename = "StopGracePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<i64>,
    #[serde(rename = "HealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthConfig>,
    /// A list of hostname/IP mappings to add to the container's `hosts` file. The format of extra hosts is specified in the [hosts(5)](http://man7.org/linux/man-pages/man5/hosts.5.html) man page:      IP_address canonical_hostname [aliases...]
    #[serde(rename = "Hosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "DNSConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<TaskSpecContainerSpecDnsConfig>,
    /// Secrets contains references to zero or more secrets that will be exposed to the service.
    #[serde(rename = "Secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<TaskSpecContainerSpecSecrets>>,
    /// Configs contains references to zero or more configs that will be exposed to the service.
    #[serde(rename = "Configs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<TaskSpecContainerSpecConfigs>>,
    /// Isolation technology of the containers running the service. (Windows only)
    #[serde(rename = "Isolation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<TaskSpecContainerSpecISOLATION>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(rename = "Init")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init: Option<bool>,
    /// Set kernel namedspaced parameters (sysctls) in the container. The Sysctls option on services accepts the same sysctls as the are supported on containers. Note that while the same sysctls are supported, no guarantees or checks are made about their suitability for a clustered environment, and it's up to the user to determine whether a given sysctl will work properly in a Service.
    #[serde(rename = "Sysctls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<HashMap<String, String>>,
    /// A list of kernel capabilities to add to the default set for the container.
    #[serde(rename = "CapabilityAdd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_add: Option<Vec<String>>,
    /// A list of kernel capabilities to drop from the default set for the container.
    #[serde(rename = "CapabilityDrop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_drop: Option<Vec<String>>,
    /// A list of resource limits to set in the container. For example: `{\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048}`\"
    #[serde(rename = "Ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<ResourcesUlimits>>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum TaskSpecContainerSpecISOLATION {
    #[serde(rename = "default")]
    DEFAULT,
    #[serde(rename = "process")]
    PROCESS,
    #[serde(rename = "hyperv")]
    HYPERV,
}

impl ::std::fmt::Display for TaskSpecContainerSpecISOLATION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TaskSpecContainerSpecISOLATION::DEFAULT => write!(f, "default"),
            TaskSpecContainerSpecISOLATION::PROCESS => write!(f, "process"),
            TaskSpecContainerSpecISOLATION::HYPERV => write!(f, "hyperv"),
        }
    }
}

impl ::std::str::FromStr for TaskSpecContainerSpecISOLATION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(TaskSpecContainerSpecISOLATION::DEFAULT),
            "process" => Ok(TaskSpecContainerSpecISOLATION::PROCESS),
            "hyperv" => Ok(TaskSpecContainerSpecISOLATION::HYPERV),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for TaskSpecContainerSpecISOLATION {
    fn as_ref(&self) -> &str {
        match self {
            TaskSpecContainerSpecISOLATION::DEFAULT => "default",
            TaskSpecContainerSpecISOLATION::PROCESS => "process",
            TaskSpecContainerSpecISOLATION::HYPERV => "hyperv",
        }
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecConfigs {
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TaskSpecContainerSpecFile1>,
    /// Runtime represents a target that is not mounted into the container but is used by the task  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually > exclusive
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<Value>,
    /// ConfigID represents the ID of the specific config that we're referencing.
    #[serde(rename = "ConfigID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// ConfigName is the name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID.
    #[serde(rename = "ConfigName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
}
/// Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers.
    #[serde(rename = "Nameservers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// A search list for host-name lookup.
    #[serde(rename = "Search")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.).
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}
/// File represents a specific target that is backed by a file.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecFile {
    /// Name represents the final filename in the filesystem.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID represents the file UID.
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<String>,
    /// GID represents the file GID.
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<String>,
    /// Mode represents the FileMode of the file.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
}
/// File represents a specific target that is backed by a file.  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecFile1 {
    /// Name represents the final filename in the filesystem.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID represents the file UID.
    #[serde(rename = "UID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UID: Option<String>,
    /// GID represents the file GID.
    #[serde(rename = "GID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GID: Option<String>,
    /// Mode represents the FileMode of the file.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
}
/// Security options for the container
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecPrivileges {
    #[serde(rename = "CredentialSpec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_spec: Option<TaskSpecContainerSpecPrivilegesCredentialSpec>,
    #[serde(rename = "SELinuxContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_linux_context: Option<TaskSpecContainerSpecPrivilegesSeLinuxContext>,
}
/// CredentialSpec for managed service account (Windows only)
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from a Swarm Config with the given ID. The specified config must also be present in the Configs field with the Runtime property set.  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "Config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Load credential spec from this file. The file is read by the daemon, and must be present in the `CredentialSpecs` subdirectory in the docker data directory, which defaults to `C:\\ProgramData\\Docker\\` on Windows.  For example, specifying `spec.json` loads `C:\\ProgramData\\Docker\\CredentialSpecs\\spec.json`.  <p><br /></p>  > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Load credential spec from this value in the Windows registry. The specified registry value must be located in:  `HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Virtualization\\Containers\\CredentialSpecs`  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "Registry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
}
/// SELinux labels of the container
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[serde(rename = "Disable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// SELinux user label
    #[serde(rename = "User")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// SELinux role label
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// SELinux type label
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// SELinux level label
    #[serde(rename = "Level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecContainerSpecSecrets {
    #[serde(rename = "File")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TaskSpecContainerSpecFile>,
    /// SecretID represents the ID of the specific secret that we're referencing.
    #[serde(rename = "SecretID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
    /// SecretName is the name of the secret that this references, but this is just provided for lookup/display purposes. The secret in the reference will be identified by its ID.
    #[serde(rename = "SecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
/// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecLogDriver {
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}
/// Read-only spec type for non-swarm containers attached to swarm overlay networks.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecNetworkAttachmentSpec {
    /// ID of the container represented by this task
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecPlacement {
    /// An array of constraint expressions to limit the set of nodes where a task can be scheduled. Constraint expressions can either use a _match_ (`==`) or _exclude_ (`!=`) rule. Multiple constraints find nodes that satisfy every expression (AND match). Constraints can match node or Docker Engine labels as follows:  node attribute       | matches                        | example ---------------------|--------------------------------|----------------------------------------------- `node.id`            | Node ID                        | `node.id==2ivku8v2gvtg4` `node.hostname`      | Node hostname                  | `node.hostname!=node-2` `node.role`          | Node role (`manager`/`worker`) | `node.role==manager` `node.platform.os`   | Node operating system          | `node.platform.os==windows` `node.platform.arch` | Node architecture              | `node.platform.arch==x86_64` `node.labels`        | User-defined node labels       | `node.labels.security==high` `engine.labels`      | Docker Engine's labels         | `engine.labels.operatingsystem==ubuntu-14.04`  `engine.labels` apply to Docker Engine labels like operating system, drivers, etc. Swarm administrators add `node.labels` for operational purposes by using the [`node update endpoint`](#operation/NodeUpdate).
    #[serde(rename = "Constraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<String>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence.
    #[serde(rename = "Preferences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Vec<TaskSpecPlacementPreferences>>,
    /// Maximum number of replicas for per node (default value is 0, which is unlimited)
    #[serde(rename = "MaxReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i64>,
    /// Platforms stores all the platforms that the service's image can run on. This field is used in the platform filter for scheduling. If empty, then the platform filter is off, meaning there are no scheduling restrictions.
    #[serde(rename = "Platforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<Platform>>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecPlacementPreferences {
    #[serde(rename = "Spread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<TaskSpecPlacementSpread>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecPlacementSpread {
    /// label descriptor, such as `engine.labels.az`.
    #[serde(rename = "SpreadDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread_descriptor: Option<String>,
}
/// Plugin spec for the service.  *(Experimental release only.)*  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecPluginSpec {
    /// The name or 'alias' to use for the plugin.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plugin image reference to use.
    #[serde(rename = "Remote")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    /// Disable the plugin once scheduled.
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "PluginPrivilege")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_privilege: Option<Vec<PluginPrivilege>>,
}
/// Resource requirements which apply to each individual container created as part of the service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecResources {
    #[serde(rename = "Limits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limit>,
    #[serde(rename = "Reservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<ResourceObject>,
}
/// Specification for the restart policy which applies to containers created as part of this service.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskSpecRestartPolicy {
    /// Condition for restart.
    #[serde(rename = "Condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<TaskSpecRestartPolicyCONDITION>,
    /// Delay between restart attempts.
    #[serde(rename = "Delay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Maximum attempts to restart a given container before giving up (default value is 0, which is ignored).
    #[serde(rename = "MaxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// Windows is the time window used to evaluate the restart policy (default value is 0, which is unbounded).
    #[serde(rename = "Window")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<i64>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum TaskSpecRestartPolicyCONDITION {
    #[serde(rename = "none")]
    NONE,
    #[serde(rename = "on-failure")]
    ON_FAILURE,
    #[serde(rename = "any")]
    ANY,
}

impl ::std::fmt::Display for TaskSpecRestartPolicyCONDITION {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            TaskSpecRestartPolicyCONDITION::NONE => write!(f, "none"),
            TaskSpecRestartPolicyCONDITION::ON_FAILURE => write!(f, "on-failure"),
            TaskSpecRestartPolicyCONDITION::ANY => write!(f, "any"),
        }
    }
}

impl ::std::str::FromStr for TaskSpecRestartPolicyCONDITION {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(TaskSpecRestartPolicyCONDITION::NONE),
            "on-failure" => Ok(TaskSpecRestartPolicyCONDITION::ON_FAILURE),
            "any" => Ok(TaskSpecRestartPolicyCONDITION::ANY),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for TaskSpecRestartPolicyCONDITION {
    fn as_ref(&self) -> &str {
        match self {
            TaskSpecRestartPolicyCONDITION::NONE => "none",
            TaskSpecRestartPolicyCONDITION::ON_FAILURE => "on-failure",
            TaskSpecRestartPolicyCONDITION::ANY => "any",
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TaskState {
    #[serde(rename = "new")]
    NEW,
    #[serde(rename = "allocated")]
    ALLOCATED,
    #[serde(rename = "pending")]
    PENDING,
    #[serde(rename = "assigned")]
    ASSIGNED,
    #[serde(rename = "accepted")]
    ACCEPTED,
    #[serde(rename = "preparing")]
    PREPARING,
    #[serde(rename = "ready")]
    READY,
    #[serde(rename = "starting")]
    STARTING,
    #[serde(rename = "running")]
    RUNNING,
    #[serde(rename = "complete")]
    COMPLETE,
    #[serde(rename = "shutdown")]
    SHUTDOWN,
    #[serde(rename = "failed")]
    FAILED,
    #[serde(rename = "rejected")]
    REJECTED,
    #[serde(rename = "remove")]
    REMOVE,
    #[serde(rename = "orphaned")]
    ORPHANED,
}
impl ToString for TaskState {
    fn to_string(&self) -> String {
        serde_json::to_value(self)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    }
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskStatus {
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TaskState>,
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Err")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "ContainerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_status: Option<TaskStatusContainerStatus>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TaskStatusContainerStatus {
    #[serde(rename = "ContainerID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "PID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PID: Option<i64>,
    #[serde(rename = "ExitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ThrottleDevice {
    /// Device path
    #[serde(rename = "Path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Rate
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
}
/// Information about the issuer of leaf TLS certificates and the trusted root CA certificate.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TlsInfo {
    /// The root CA certificate(s) that are used to validate leaf TLS certificates.
    #[serde(rename = "TrustRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_root: Option<String>,
    /// The base64-url-safe-encoded raw subject bytes of the issuer.
    #[serde(rename = "CertIssuerSubject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_issuer_subject: Option<String>,
    /// The base64-url-safe-encoded raw public key bytes of the issuer.
    #[serde(rename = "CertIssuerPublicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_issuer_public_key: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct UnlockKeyResponse {
    /// The swarm's unlock key.
    #[serde(rename = "UnlockKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock_key: Option<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Volume {
    /// Name of the volume.
    #[serde(rename = "Name")]
    pub name: String,
    /// Name of the volume driver used by the volume.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// Mount path of the volume on the host.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    /// Date/Time the volume was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{\"key\":\"value\",\"key2\":\"value2\"}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Value>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub labels: HashMap<String, String>,
    /// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "NoneAsEmptyString")]
    pub scope: Option<VolumeSCOPE>,
    /// The driver specific options used when creating the volume.
    #[serde(rename = "Options")]
    #[serde(deserialize_with = "deserialize_nonoptional_map")]
    pub options: HashMap<String, String>,
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_data: Option<VolumeUsageData>,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Eq, Ord, Hash)]
pub enum VolumeSCOPE {
    #[serde(rename = "local")]
    LOCAL,
    #[serde(rename = "global")]
    GLOBAL,
}

impl ::std::fmt::Display for VolumeSCOPE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            VolumeSCOPE::LOCAL => write!(f, "local"),
            VolumeSCOPE::GLOBAL => write!(f, "global"),
        }
    }
}

impl ::std::str::FromStr for VolumeSCOPE {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(VolumeSCOPE::LOCAL),
            "global" => Ok(VolumeSCOPE::GLOBAL),
            x => Err(format!("Invalid enum type: {}", x)),
        }
    }
}

impl ::std::convert::AsRef<str> for VolumeSCOPE {
    fn as_ref(&self) -> &str {
        match self {
            VolumeSCOPE::LOCAL => "local",
            VolumeSCOPE::GLOBAL => "global",
        }
    }
}
/// Volume configuration
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeCreateOptions {
    /// The new volume's name. If not specified, Docker generates a name.
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the volume driver to use.
    #[serde(rename = "Driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// A mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<HashMap<String, String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<HashMap<String, String>>,
}
/// Volume list response
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeListResponse {
    /// List of volumes
    #[serde(rename = "Volumes")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub volumes: Vec<Volume>,
    /// Warnings that occurred when fetching the list of volumes.
    #[serde(rename = "Warnings")]
    #[serde(deserialize_with = "deserialize_nonoptional_vec")]
    pub warnings: Vec<String>,
}
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumePruneResponse {
    /// Volumes that were deleted
    #[serde(rename = "VolumesDeleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
/// Usage details about the volume. This information is used by the `GET /system/df` endpoint, and omitted in other endpoints.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VolumeUsageData {
    /// Amount of disk space used by the volume (in bytes). This information is only available for volumes created with the `\"local\"` volume driver. For volumes created with other volume drivers, this field is set to `-1` (\"not available\")
    #[serde(rename = "Size")]
    pub size: i64,
    /// The number of containers referencing this volume. This field is set to `-1` if the reference-count is not available.
    #[serde(rename = "RefCount")]
    pub ref_count: i64,
}
