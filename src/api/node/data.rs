use crate::api::{Labels, ObjectVersion, TlsInfo};

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub enum Membership {
    Accepted,
    Pending,
}

impl AsRef<str> for Membership {
    fn as_ref(&self) -> &str {
        match &self {
            Membership::Accepted => "accepted",
            Membership::Pending => "pending",
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Availability {
    Active,
    Pause,
    Drain,
}

impl AsRef<str> for Availability {
    fn as_ref(&self) -> &str {
        match &self {
            Availability::Active => "active",
            Availability::Pause => "pause",
            Availability::Drain => "drain",
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeRole {
    Manager,
    Worker,
}

impl AsRef<str> for NodeRole {
    fn as_ref(&self) -> &str {
        match &self {
            NodeRole::Manager => "manager",
            NodeRole::Worker => "worker",
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeReachability {
    Unknown,
    Unreachable,
    Reachable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeInfo {
    #[serde(rename = "ID")]
    pub id: String,
    pub version: ObjectVersion,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub updated_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub updated_at: String,
    pub spec: Option<NodeSpec>,
    pub description: Option<NodeDescription>,
    pub status: Option<NodeStatus>,
    pub manager_status: Option<ManagerStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeState {
    Unknown,
    Down,
    Ready,
    Disconnected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeAvailability {
    Active,
    Pause,
    Drain,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    pub state: Option<NodeState>,
    pub message: Option<String>,
    pub addr: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    pub name: Option<String>,
    pub labels: Labels,
    pub role: Option<NodeRole>,
    pub availability: Option<NodeAvailability>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub hostname: Option<String>,
    pub platform: Option<Platform>,
    pub resources: Option<ResourceObject>,
    pub engine: Option<EngineDescription>,
    #[serde(rename = "TLSInfo")]
    pub tls_info: Option<TlsInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineDescription {
    pub engine_version: Option<String>,
    pub labels: Option<Labels>,
    pub plugins: Option<Vec<serde_json::Value>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs")]
    pub nano_cpus: i64,
    pub memory_bytes: i64,
    pub generic_resources: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    pub architecture: Option<String>,
    pub os: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerStatus {
    pub leader: Option<bool>,
    pub reachability: Option<NodeReachability>,
    pub addr: Option<String>,
}
