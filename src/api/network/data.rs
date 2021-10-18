use crate::api::{ConfigMap, Labels, Options};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub addr: String,
    pub prefix_len: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub bridge: String,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    pub hairpin_mode: bool,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_addr: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: isize,
    pub ports: Option<PortMap>,
    pub sandbox_key: String,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_ipv6_addresses: Option<Vec<Address>>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_addr: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: isize,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    pub mac_address: String,
    pub networks: HashMap<String, NetworkEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkEntry {
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u64,
    pub mac_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ipam {
    pub driver: Option<String>,
    pub config: Option<Vec<ConfigMap>>,
    pub options: Option<Options>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkInfo {
    pub name: Option<String>,
    pub labels: Labels,
    pub id: String,
    pub scope: Option<String>,
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    pub internal: Option<bool>,
    pub attachable: Option<bool>,
    pub containers: Option<HashMap<String, NetworkContainerDetails>>,
    pub options: Option<Options>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkContainerDetails {
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub mac_address: String,
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: String,
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkCreateInfo {
    pub id: String,
    pub warning: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworksPruneInfo {
    pub networks_deleted: Vec<String>,
}
