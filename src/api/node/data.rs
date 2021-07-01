use crate::api::{ObjectVersion, TlsInfo};

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
pub enum Role {
    Manager,
    Worker,
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match &self {
            Role::Manager => "manager",
            Role::Worker => "worker",
        }
    }
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
    pub spec: NodeSpec,
    pub description: NodeDescription,
    pub status: NodeStatus,
    pub manager_status: Option<ManagerStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    // TODO: use an enum here
    pub state: String,
    pub message: String,
    pub addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    pub name: String,
    pub labels: serde_json::Value,
    // TODO: use an enum here
    pub role: String,
    // TODO: use an enum here
    pub availability: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub hostname: String,
    pub platform: Platform,
    pub resources: ResourceObject,
    pub engine: EngineDescription,
    #[serde(rename = "TLSInfo")]
    pub tls_info: TlsInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineDescription {
    pub engine_version: String,
    pub labels: serde_json::Value,
    pub plugins: Vec<serde_json::Value>,
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
    architecture: String,
    os: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerStatus {
    pub leader: bool,
    // TODO: use an enum here
    pub reachability: String,
    pub addr: String,
}
