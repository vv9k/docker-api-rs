use crate::api::{ConfigMap, Driver, DriverData, Labels, NetworkSettings, Options, PublishPort};

use serde::{de, Deserialize, Serialize};
use std::{
    collections::HashMap,
    str::{self, FromStr},
};

#[cfg(feature = "chrono")]
use crate::util::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
use serde::ser::SerializeMap;
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub command: String,
    pub id: String,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub labels: Labels,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub state: ContainerStatus,
    pub status: String,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub mounts: Vec<Mount>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDetails {
    pub id: String,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub path: String,
    pub args: Vec<String>,
    pub state: ContainerState,
    pub image: String,
    pub resolv_conf_path: String,
    pub hostname_path: String,
    pub hosts_path: String,
    pub log_path: String,
    pub name: String,
    pub restart_count: i64,
    pub driver: String,
    pub platform: String,
    pub mount_label: String,
    pub process_label: String,
    pub app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,
    pub host_config: HostConfig,
    pub graph_driver: DriverData,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub mounts: Vec<MountPoint>,
    pub config: ContainerConfig,
    pub network_settings: NetworkSettings,
}

fn deserialize_exposed_ports<'de, D>(deserializer: D) -> Result<Vec<PublishPort>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let port_map = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;

    let mut ports = Vec::new();
    for (port, _) in port_map {
        ports.push(PublishPort::from_str(port.as_str()).map_err(serde::de::Error::custom)?);
    }

    Ok(ports)
}

fn serialize_exposed_ports<S>(ports: &[PublishPort], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serializer.serialize_map(Some(ports.len()))?;

    for port in ports {
        map.serialize_entry(
            &port.to_string(),
            &serde_json::Value::Object(serde_json::Map::new()),
        )?;
    }

    map.end()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub hostname: String,
    pub domainname: String,
    pub user: String,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub attach_stderr: bool,
    #[serde(deserialize_with = "deserialize_exposed_ports")]
    #[serde(serialize_with = "serialize_exposed_ports")]
    #[serde(default)]
    pub exposed_ports: Vec<PublishPort>,
    pub tty: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub env: Vec<String>,
    pub cmd: Option<Vec<String>>,
    pub healthcheck: Option<HealthConfig>,
    pub args_escaped: Option<bool>,
    pub image: String,
    pub volumes: Option<VolumesMap>,
    pub working_dir: String,
    pub entrypoint: Option<Vec<String>>,
    pub network_disabled: Option<bool>,
    pub mac_address: Option<String>,
    pub on_build: Option<Vec<String>>,
    pub labels: Option<Labels>,
    pub stop_signal: Option<String>,
    pub stop_timeout: Option<isize>,
    pub shell: Option<Vec<String>>,
}

pub type VolumesMap = HashMap<String, Value>;

impl ContainerConfig {
    pub fn env(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for e in &self.env {
            let pair: Vec<&str> = e.split('=').collect();
            map.insert(pair[0].to_owned(), pair[1].to_owned());
        }
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountType {
    Bind,
    Volume,
    TmpFs,
    NPipe,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountConsistency {
    Default,
    Consistent,
    Cached,
    Delegated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    #[serde(rename = "Type")]
    pub type_: Option<MountType>,
    pub source: Option<String>,
    pub target: Option<String>,
    pub read_only: Option<bool>,
    pub consistency: Option<MountConsistency>,
    pub bind_options: Option<BindOptions>,
    pub volume_options: Option<VolumeOptions>,
    pub tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TmpfsOptions {
    pub size_bytes: Option<i64>,
    pub mode: Option<isize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeOptions {
    pub no_copy: Option<bool>,
    pub labels: Option<Labels>,
    pub driver_config: Option<Driver>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BindPropagation {
    Private,
    RPrivate,
    Shared,
    RShared,
    Slave,
    RSlave,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BindOptions {
    pub propagation: Option<BindPropagation>,
    pub non_recursive: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MountPoint {
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    pub name: Option<String>,
    pub source: String,
    pub destination: String,
    pub driver: Option<String>,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub propagation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Created,
    Configured,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl AsRef<str> for ContainerStatus {
    fn as_ref(&self) -> &str {
        use ContainerStatus::*;
        match &self {
            Created => "created",
            Configured => "configured",
            Restarting => "restarting",
            Running => "running",
            Removing => "removing",
            Paused => "paused",
            Exited => "exited",
            Dead => "dead",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerState {
    pub status: ContainerStatus,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub dead: bool,
    pub pid: u64,
    pub exit_code: u64,
    pub error: String,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    #[cfg(feature = "chrono")]
    pub finished_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub finished_at: String,
    pub health: Option<ContainerHealth>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerHealth {
    pub status: HealthStatus,
    pub failing_streak: isize,
    pub log: Vec<HealthcheckResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthConfig {
    pub test: Option<Vec<String>>,
    pub interval: Option<isize>,
    pub timeout: Option<isize>,
    pub retries: Option<isize>,
    pub start_period: Option<isize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    None,
    Starting,
    Healthy,
    Unhealthy,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthcheckResult {
    #[cfg(feature = "chrono")]
    pub start: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub start: String,
    #[cfg(feature = "chrono")]
    pub end: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub end: String,
    pub exit_code: isize,
    pub output: String,
}

pub type Sysctls = HashMap<String, String>;
pub type Tmpfs = HashMap<String, String>;
pub type StorageOpt = HashMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub cpu_shares: Option<i64>,
    pub memory: Option<i64>,
    pub cgroup_parent: Option<String>,
    pub blkio_weight: u16,
    pub blkio_weight_device: Option<Vec<ThrottleWeightDevice>>,
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,
    pub cpu_period: Option<i64>,
    pub cpu_quota: Option<i64>,
    pub cpu_realtime_period: Option<i64>,
    pub cpu_realtime_runtime: Option<i64>,
    pub cpuset_cpus: Option<String>,
    pub cpuset_mems: Option<String>,
    pub devices: Option<Vec<DeviceMapping>>,
    pub device_cgroup_rules: Option<Vec<String>>,
    pub device_requests: Option<Vec<DeviceRequest>>,
    pub kernel_memory: i64,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: i64,
    pub memory_reservation: Option<i64>,
    pub memory_swap: Option<i64>,
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCPUs")]
    pub nano_cpus: Option<i64>,
    pub oom_kill_disable: Option<bool>,
    pub init: Option<bool>,
    pub pids_limit: Option<i64>,
    pub ulimits: Option<Vec<Ulimit>>,
    pub cpu_count: i64,
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: u64,
    #[serde(rename = "IOMaximumBandwith")]
    pub io_maximum_bandwith: Option<u64>,
    pub binds: Option<Vec<String>>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: String,
    pub log_config: LogConfig,
    pub network_mode: String,
    pub port_bindings: Option<PortMap>,
    pub restart_policy: RestartPolicy,
    pub auto_remove: bool,
    pub volume_driver: String,
    pub volumes_from: Option<Vec<String>>,
    pub mounts: Option<Vec<MountPoint>>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub dns: Option<Vec<String>>,
    pub dns_options: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub extra_hosts: Option<Vec<String>>,
    pub group_add: Option<Vec<String>>,
    pub ipc_mode: String,
    pub cgroup: String,
    pub links: Option<Vec<String>>,
    pub oom_score_adj: i64,
    pub pid_mode: Option<String>,
    pub privileged: bool,
    pub publish_all_ports: bool,
    pub readonly_rootfs: Option<bool>,
    pub security_opt: Option<Vec<String>>,
    pub storage_opt: Option<StorageOpt>,
    pub tmpfs: Option<Tmpfs>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: String,
    pub userns_mode: String,
    pub shm_size: u64,
    pub sysctls: Option<Sysctls>,
    pub runtime: Option<String>,
    pub console_size: Option<Vec<u64>>,
    pub isolation: String,
    pub masked_paths: Option<Vec<String>>,
    pub readonly_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrottleDevice {
    pub path: String,
    pub rate: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrottleWeightDevice {
    pub path: String,
    pub weight: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub name: String,
    pub maximum_retry_count: u64,
}

pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Config")]
    pub config: ConfigMap,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: String,
    pub soft: isize,
    pub hard: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
    pub path_on_host: String,
    pub path_in_container: String,
    pub cgroup_permissions: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: String,
    pub count: u64,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    pub capabilities: Vec<String>,
    pub options: Options,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub read: String,
    pub num_procs: u32,
    pub memory_stats: Option<MemoryStats>,
    pub blkio_stats: Option<BlkioStats>,
    pub cpu_stats: Option<CpuStats>,
    pub precpu_stats: Option<CpuStats>,
    pub pids_stats: Option<PidsStats>,
    pub storage_stats: Option<StorageStats>,
    pub networks: Option<HashMap<String, NetworkStats>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageStats {
    pub read_count_normalized: Option<u64>,
    pub read_size_bytes: Option<u64>,
    pub write_count_normalized: Option<u64>,
    pub write_size_bytes: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PidsStats {
    pub current: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub rx_dropped: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub tx_errors: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub max_usage: Option<u64>,
    pub usage: Option<u64>,
    pub failcnt: Option<u64>,
    pub limit: Option<u64>,
    pub stats: Option<MemoryStat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStat {
    pub total_pgmajfault: Option<u64>,
    pub cache: Option<u64>,
    pub mapped_file: Option<u64>,
    pub total_inactive_file: Option<u64>,
    pub pgpgout: Option<u64>,
    pub rss: Option<u64>,
    pub total_mapped_file: Option<u64>,
    pub writeback: Option<u64>,
    pub unevictable: Option<u64>,
    pub pgpgin: Option<u64>,
    pub total_unevictable: Option<u64>,
    pub pgmajfault: Option<u64>,
    pub total_rss: Option<u64>,
    pub total_rss_huge: Option<u64>,
    pub total_writeback: Option<u64>,
    pub total_inactive_anon: Option<u64>,
    pub rss_huge: Option<u64>,
    pub hierarchical_memory_limit: Option<u64>,
    pub hierarchical_memsw_limit: Option<u64>,
    pub total_pgfault: Option<u64>,
    pub total_active_file: Option<u64>,
    pub active_anon: Option<u64>,
    pub total_active_anon: Option<u64>,
    pub total_pgpgout: Option<u64>,
    pub total_cache: Option<u64>,
    pub inactive_anon: Option<u64>,
    pub active_file: Option<u64>,
    pub pgfault: Option<u64>,
    pub inactive_file: Option<u64>,
    pub total_pgpgin: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: Option<u64>,
    pub throttling_data: Option<ThrottlingData>,
    pub online_cpus: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub percpu_usage: Option<Vec<u64>>,
    pub usage_in_usermode: u64,
    pub total_usage: u64,
    pub usage_in_kernelmode: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThrottlingData {
    pub periods: u64,
    pub throttled_periods: u64,
    pub throttled_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStats {
    pub io_service_bytes_recursive: Option<Vec<BlkioStat>>,
    pub io_serviced_recursive: Option<Vec<BlkioStat>>,
    pub io_queue_recursive: Option<Vec<BlkioStat>>,
    pub io_service_time_recursive: Option<Vec<BlkioStat>>,
    pub io_wait_time_recursive: Option<Vec<BlkioStat>>,
    pub io_merged_recursive: Option<Vec<BlkioStat>>,
    pub io_time_recursive: Option<Vec<BlkioStat>>,
    pub sectors_recursive: Option<Vec<BlkioStat>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStat {
    pub major: u64,
    pub minor: u64,
    pub op: String,
    pub value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ChangeKind {
    Modified,
    Added,
    Deleted,
}

fn deserialize_change_kind<'de, D>(deserializer: D) -> Result<ChangeKind, D::Error>
where
    D: de::Deserializer<'de>,
{
    use de::Error;
    let num: u8 = de::Deserialize::deserialize(deserializer)?;
    match num {
        0 => Ok(ChangeKind::Modified),
        1 => Ok(ChangeKind::Added),
        2 => Ok(ChangeKind::Deleted),
        _ => Err(D::Error::custom("invalid change kind")),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    #[serde(deserialize_with = "deserialize_change_kind")]
    pub kind: ChangeKind,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
    pub titles: Vec<String>,
    pub processes: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateInfo {
    pub id: String,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exit {
    pub status_code: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainersPruneInfo {
    pub containers_deleted: Vec<String>,
    pub space_reclaimed: u64,
}
