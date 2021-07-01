use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str};

#[cfg(feature = "chrono")]
use crate::util::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

use crate::api::{image::ContainerConfig, network::NetworkSettings};

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
    pub labels: HashMap<String, String>,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub state: String,
    pub status: String,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
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
    pub state: State,
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
    pub graph_driver: GraphDriverData,
    pub mounts: Vec<Mount>,
    pub config: ContainerConfig,
    pub network_settings: NetworkSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GraphDriverData {
    pub name: String,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub error: String,
    pub exit_code: u64,
    #[cfg(feature = "chrono")]
    pub finished_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub finished_at: String,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub paused: bool,
    pub pid: u64,
    pub restarting: bool,
    pub running: bool,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub cpu_shares: Option<i64>,
    pub memory: Option<i64>,
    pub cgroup_parent: Option<String>,
    pub blkio_weight_device: Option<Vec<ThrottleDevice>>,
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
    pub device_cgroup_rules: Option<String>,
    pub device_requests: Option<Vec<DeviceRequest>>,
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
    pub mounts: Option<Vec<Mount>>,
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
    pub storage_opt: Option<HashMap<String, String>>,
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: String,
    pub userns_mode: String,
    pub shm_size: u64,
    pub sysctls: Option<HashMap<String, String>>,
    pub runtime: String,
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
    pub config: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: String,
    pub soft: u64,
    pub hard: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
    pub path_on_host: Option<String>,
    pub path_in_container: Option<String>,
    pub cgroup_permissions: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: String,
    pub count: u64,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    pub capabilities: Vec<String>,
    pub options: Option<serde_json::Value>,
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
    pub networks: HashMap<String, NetworkStats>,
    pub memory_stats: MemoryStats,
    pub blkio_stats: BlkioStats,
    pub cpu_stats: CpuStats,
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
    pub max_usage: u64,
    pub usage: u64,
    pub failcnt: Option<u64>,
    pub limit: u64,
    pub stats: MemoryStat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStat {
    pub total_pgmajfault: u64,
    pub cache: u64,
    pub mapped_file: u64,
    pub total_inactive_file: u64,
    pub pgpgout: u64,
    pub rss: u64,
    pub total_mapped_file: u64,
    pub writeback: u64,
    pub unevictable: u64,
    pub pgpgin: u64,
    pub total_unevictable: u64,
    pub pgmajfault: u64,
    pub total_rss: u64,
    pub total_rss_huge: u64,
    pub total_writeback: u64,
    pub total_inactive_anon: u64,
    pub rss_huge: u64,
    pub hierarchical_memory_limit: u64,
    pub hierarchical_memsw_limit: u64,
    pub total_pgfault: u64,
    pub total_active_file: u64,
    pub active_anon: u64,
    pub total_active_anon: u64,
    pub total_pgpgout: u64,
    pub total_cache: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub pgfault: u64,
    pub inactive_file: u64,
    pub total_pgpgin: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: u64,
    pub throttling_data: ThrottlingData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub percpu_usage: Vec<u64>,
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
    pub io_service_bytes_recursive: Vec<BlkioStat>,
    pub io_serviced_recursive: Vec<BlkioStat>,
    pub io_queue_recursive: Vec<BlkioStat>,
    pub io_service_time_recursive: Vec<BlkioStat>,
    pub io_wait_time_recursive: Vec<BlkioStat>,
    pub io_merged_recursive: Vec<BlkioStat>,
    pub io_time_recursive: Vec<BlkioStat>,
    pub sectors_recursive: Vec<BlkioStat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStat {
    pub major: u64,
    pub minor: u64,
    pub op: String,
    pub value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    pub kind: u8,
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
    pub status_code: u64,
}
