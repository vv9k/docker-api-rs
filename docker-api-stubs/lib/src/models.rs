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
    pub prefix_len: Option<usize>,
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
pub struct BuildCache {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the build cache was created in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InUse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[serde(rename = "LastUsedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the build cache was last used in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub last_used_at: Option<DateTime<Utc>>,
    #[serde(rename = "Parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "Shared")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Amount of disk space used by the build cache (in bytes).
    pub size: Option<usize>,
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "UsageCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildInfo {
    pub aux: ImageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorDetail")]
    pub error_detail: ErrorDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    pub progress_detail: ProgressDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
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
    pub spec: SwarmSpec,
    #[serde(rename = "SubnetSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// SubnetSize specifies the subnet size of the networks created from the
    /// default subnet pool.
    pub subnet_size: Option<u32>,
    #[serde(rename = "TLSInfo")]
    pub tls_info: TlsInfo,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the swarm was last updated in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
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
pub struct Config {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: ConfigSpec,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
}

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
    pub templating: Driver,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Configuration for a container that is portable between hosts.
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
    pub exposed_ports: Option<HashMap<String, HashMap<String, Value>>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: HealthConfig,
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
    pub stop_timeout: Option<usize>,
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
    pub volumes: Option<HashMap<String, HashMap<String, Value>>>,
    #[serde(rename = "WorkingDir")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The working directory for commands to run in.
    pub working_dir: Option<String>,
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
    pub exit_code: Option<usize>,
    #[serde(rename = "FinishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The time when this container last exited.
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Health,
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
    pub pid: Option<usize>,
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
    pub host_config: Option<HashMap<String, Value>>,
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
    pub network_settings: Option<HashMap<String, Value>>,
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
    pub error: ContainerWaitExitError,
    #[serde(rename = "StatusCode")]
    /// Exit code of the container
    pub status_code: usize,
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
    pub progress_detail: ProgressDetail,
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
    pub count: Option<usize>,
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
    pub published_port: Option<usize>,
    #[serde(rename = "TargetPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The port inside the container.
    pub target_port: Option<usize>,
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
    pub ipam_config: EndpointIpamConfig,
    #[serde(rename = "IPAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv4 address.
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Mask length of the IPv4 address.
    pub ip_prefix_len: Option<usize>,
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
    pub plugins: Option<Vec<HashMap<String, Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<usize>,
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
    pub actor: EventActor,
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

/// User-defined resources can be either Integer resources (e.g, `SSD=3`) or
/// String resources (e.g, `GPU=UUID1`).
pub type GenericResources = Vec<HashMap<String, Value>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Information about the storage driver used to store the container's and
/// image's filesystem.
pub struct GraphDriverData {
    #[serde(rename = "Data")]
    #[serde(default)]
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
    pub failing_streak: Option<usize>,
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
    pub interval: Option<usize>,
    #[serde(rename = "Retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The number of consecutive failures needed to consider a container as
    /// unhealthy. 0 means inherit.
    pub retries: Option<usize>,
    #[serde(rename = "StartPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Start period for the container to initialize before starting
    /// health-retries countdown in nanoseconds. It should be 0 or at least
    /// 1000000 (1 ms). 0 means inherit.
    pub start_period: Option<usize>,
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
    pub timeout: Option<usize>,
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
    pub exit_code: Option<usize>,
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
    pub config: ContainerConfig,
    #[serde(rename = "Container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the container that was used to create the image.
    ///
    /// Depending on how the image was created, this field may be empty.
    pub container: Option<String>,
    #[serde(rename = "ContainerConfig")]
    pub container_config: ContainerConfig,
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
    pub graph_driver: GraphDriverData,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identified is a content-addressable digest calculated from the
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
    pub metadata: Option<HashMap<String, Value>>,
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
    pub root_fs: Option<HashMap<String, Value>>,
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
pub struct ImageSummary {
    #[serde(rename = "Containers")]
    pub containers: usize,
    #[serde(rename = "Created")]
    pub created: usize,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Labels")]
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    #[serde(rename = "RepoDigests")]
    #[serde(default)]
    pub repo_digests: Vec<String>,
    #[serde(rename = "RepoTags")]
    #[serde(default)]
    pub repo_tags: Vec<String>,
    #[serde(rename = "SharedSize")]
    pub shared_size: usize,
    #[serde(rename = "Size")]
    pub size: usize,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: usize,
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

/// Current local status of this node.
pub type LocalNodeState = String;

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
    pub reachability: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mount {
    #[serde(rename = "BindOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional configuration for the `bind` type.
    pub bind_options: Option<HashMap<String, Value>>,
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
    pub tmpfs_options: Option<HashMap<String, Value>>,
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
    pub volume_options: Option<HashMap<String, Value>>,
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
    pub ipam: Ipam,
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
    pub global_i_pv_6_prefix_len: Option<usize>,
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
    pub ip_prefix_len: Option<usize>,
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
    pub link_local_i_pv_6_prefix_len: Option<usize>,
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
    pub ports: PortMap,
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
    pub description: NodeDescription,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ManagerStatus")]
    pub manager_status: ManagerStatus,
    #[serde(rename = "Spec")]
    pub spec: NodeSpec,
    #[serde(rename = "Status")]
    pub status: NodeStatus,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Date and time at which the node was last updated in
    /// [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// NodeDescription encapsulates the properties of the Node as reported by the
/// agent.
pub struct NodeDescription {
    #[serde(rename = "Engine")]
    pub engine: EngineDescription,
    #[serde(rename = "Hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Platform")]
    pub platform: Platform,
    #[serde(rename = "Resources")]
    pub resources: ResourceObject,
    #[serde(rename = "TLSInfo")]
    pub tls_info: TlsInfo,
}

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

/// NodeState represents the state of a node.
pub type NodeState = String;

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
    pub state: String,
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
    #[serde(default)]
    /// The config of a plugin.
    pub config: HashMap<String, Value>,
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
    #[serde(default)]
    /// Settings that can be modified by users.
    pub settings: HashMap<String, Value>,
}

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
    pub options: Vec<String>,
    #[serde(rename = "Settable")]
    #[serde(default)]
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
    pub current: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushImageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail")]
    pub progress_detail: ProgressDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Reachability represents the reachability of a node.
pub type Reachability = String;

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
    pub generic_resources: Vec<GenericResources>,
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
    pub blkio_weight: Option<usize>,
    #[serde(rename = "BlkioWeightDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Block IO weight (relative device weight) in the form:
    ///
    /// ```
    /// [{"Path": "device_path", "Weight": weight}]
    /// ```
    pub blkio_weight_device: Option<Vec<HashMap<String, Value>>>,
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
    pub cpu_shares: Option<usize>,
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
    pub ulimits: Option<Vec<HashMap<String, Value>>>,
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
    pub maximum_retry_count: Option<usize>,
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
    pub spec: SecretSpec,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
}

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
    pub driver: Driver,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined name of the secret.
    pub name: Option<String>,
    #[serde(rename = "Templating")]
    pub templating: Driver,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<HashMap<String, Value>>,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of the service when it is in one of ReplicatedJob or
    /// GlobalJob modes. Absent on Replicated and Global mode services. The
    /// JobIteration is an ObjectVersion, but unlike the Service's version,
    /// does not need to be sent with an update request.
    pub job_status: Option<HashMap<String, Value>>,
    #[serde(rename = "ServiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of the service's tasks. Provided only when requested as
    /// part of a ServiceList operation.
    pub service_status: Option<HashMap<String, Value>>,
    #[serde(rename = "Spec")]
    pub spec: ServiceSpec,
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The status of a service update.
    pub update_status: Option<HashMap<String, Value>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable configuration for a service.
pub struct ServiceSpec {
    #[serde(rename = "EndpointSpec")]
    pub endpoint_spec: EndpointSpec,
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// User-defined key/value metadata.
    pub labels: Option<HashMap<String, String>>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Scheduling mode for the service.
    pub mode: Option<HashMap<String, Value>>,
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
    pub rollback_config: Option<HashMap<String, Value>>,
    #[serde(rename = "TaskTemplate")]
    pub task_template: TaskSpec,
    #[serde(rename = "UpdateConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the update strategy of the service.
    pub update_config: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateResponse {
    #[serde(rename = "Warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional warning messages
    pub warnings: Option<Vec<String>>,
}

pub type Swarm = Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents generic information about swarm.
pub struct SwarmInfo {
    #[serde(rename = "Cluster")]
    pub cluster: ClusterInfo,
    #[serde(rename = "ControlAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_available: Option<bool>,
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "LocalNodeState")]
    pub local_node_state: String,
    #[serde(rename = "Managers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of managers in the swarm.
    pub managers: Option<usize>,
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
    pub nodes: Option<usize>,
    #[serde(rename = "RemoteManagers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of ID's and addresses of other managers in the swarm.
    pub remote_managers: Option<Vec<PeerNode>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// User modifiable swarm configuration.
pub struct SwarmSpec {
    #[serde(rename = "CAConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CA configuration.
    pub ca_config: Option<HashMap<String, Value>>,
    #[serde(rename = "Dispatcher")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Dispatcher configuration.
    pub dispatcher: Option<HashMap<String, Value>>,
    #[serde(rename = "EncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Parameters related to encryption-at-rest.
    pub encryption_config: Option<HashMap<String, Value>>,
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
    pub orchestration: Option<HashMap<String, Value>>,
    #[serde(rename = "Raft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Raft configuration.
    pub raft: Option<HashMap<String, Value>>,
    #[serde(rename = "TaskDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defaults for creating tasks in this cluster.
    pub task_defaults: Option<HashMap<String, Value>>,
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
    pub containerd_commit: Commit,
    #[serde(rename = "Containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Total number of containers on the host.
    pub containers: Option<usize>,
    #[serde(rename = "ContainersPaused")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"paused"`.
    pub containers_paused: Option<usize>,
    #[serde(rename = "ContainersRunning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"running"`.
    pub containers_running: Option<usize>,
    #[serde(rename = "ContainersStopped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of containers with status `"stopped"`.
    pub containers_stopped: Option<usize>,
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
    pub default_address_pools: Option<Vec<HashMap<String, Value>>>,
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
    pub generic_resources: Vec<GenericResources>,
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
    pub images: Option<usize>,
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
    pub init_commit: Commit,
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
    pub ncpu: Option<usize>,
    #[serde(rename = "NEventsListener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of event listeners subscribed.
    pub n_events_listener: Option<usize>,
    #[serde(rename = "NFd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The total number of file Descriptors in use by the daemon process.
    ///
    /// This information is only returned if debug-mode is enabled.
    pub n_fd: Option<usize>,
    #[serde(rename = "NGoroutines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The  number of goroutines that currently exist.
    ///
    /// This information is only returned if debug-mode is enabled.
    pub n_goroutines: Option<usize>,
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
    pub plugins: PluginsInfo,
    #[serde(rename = "ProductLicense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reports a summary of the product license on the daemon.
    ///
    /// If a commercial license has been applied to the daemon, information
    /// such as number of nodes, and expiration are included.
    pub product_license: Option<String>,
    #[serde(rename = "RegistryConfig")]
    pub registry_config: RegistryServiceConfig,
    #[serde(rename = "RuncCommit")]
    pub runc_commit: Commit,
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
    pub swarm: SwarmInfo,
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
    pub components: Option<Vec<HashMap<String, Value>>>,
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
    pub platform: Option<HashMap<String, Value>>,
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version of the daemon
    pub version: Option<String>,
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
    pub assigned_generic_resources: Vec<GenericResources>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "DesiredState")]
    pub desired_state: String,
    #[serde(rename = "ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The ID of the task.
    pub id: Option<String>,
    #[serde(rename = "JobIteration")]
    pub job_iteration: ObjectVersion,
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
    pub slot: Option<usize>,
    #[serde(rename = "Spec")]
    pub spec: TaskSpec,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HashMap<String, Value>>,
    #[serde(rename = "UpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(rename = "Version")]
    pub version: ObjectVersion,
}

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
    pub container_spec: Option<HashMap<String, Value>>,
    #[serde(rename = "ForceUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A counter that triggers an update even if no relevant parameters have
    /// been changed.
    pub force_update: Option<usize>,
    #[serde(rename = "LogDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies the log driver to use for tasks created from this spec. If
    /// not present, the default one for the swarm will be used, finally
    /// falling back to the engine default if not specified.
    pub log_driver: Option<HashMap<String, Value>>,
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
    pub network_attachment_spec: Option<HashMap<String, Value>>,
    #[serde(rename = "Networks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies which networks the service should attach to.
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    #[serde(rename = "Placement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<HashMap<String, Value>>,
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
    pub plugin_spec: Option<HashMap<String, Value>>,
    #[serde(rename = "Resources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Resource requirements which apply to each individual container created
    /// as part of the service.
    pub resources: Option<HashMap<String, Value>>,
    #[serde(rename = "RestartPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specification for the restart policy which applies to containers
    /// created as part of this service.
    pub restart_policy: Option<HashMap<String, Value>>,
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Runtime is the type of runtime specified for the task executor.
    pub runtime: Option<String>,
}

pub type TaskState = String;

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
    pub status: Option<HashMap<String, HashMap<String, Value>>>,
    #[serde(rename = "UsageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Usage details about the volume. This information is used by the
    /// `GET /system/df` endpoint, and omitted in other endpoints.
    pub usage_data: Option<HashMap<String, Value>>,
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
