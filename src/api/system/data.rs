use crate::{
    api::{Attributes, Isolation, Labels, Mount, NetworkEntry, Port, VolumeInfo},
    errors::{Error, Result},
};

use hyper::header::HeaderMap;
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, convert::TryFrom};

#[cfg(feature = "chrono")]
use {
    crate::util::datetime::{datetime_from_nano_timestamp, datetime_from_unix_timestamp},
    chrono::{DateTime, Utc},
};

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
    pub containers: isize,
    pub containers_running: isize,
    pub containers_paused: isize,
    pub containers_stopped: isize,
    pub images: isize,
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
    pub n_fd: isize,
    pub n_goroutines: isize,
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
    pub isolation: Isolation,
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
    size: isize,
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
    pub created: isize,
    pub size: isize,
    pub shared_size: isize,
    pub virtual_size: isize,
    pub labels: Option<Labels>,
    pub containers: isize,
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
    pub labels: Option<Labels>,
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
    pub size: i64,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub last_used_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub last_used_at: String,
    pub usage_count: isize,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "Type")]
    pub typ: String,
    #[serde(rename = "Action")]
    pub action: String,
    #[serde(rename = "Actor")]
    pub actor: Actor,
    pub status: Option<String>,
    pub id: Option<String>,
    pub from: Option<String>,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub time: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub time: u64,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_nano_timestamp", rename = "timeNano")]
    pub time_nano: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    #[serde(rename = "timeNano")]
    pub time_nano: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Actor {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Attributes")]
    pub attributes: Attributes,
}
