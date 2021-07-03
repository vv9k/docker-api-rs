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
    pub spec: Option<NodeSpec>,
    pub description: Option<NodeDescription>,
    pub status: Option<NodeStatus>,
    pub manager_status: Option<ManagerStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    // TODO: use an enum here
    pub state: Option<String>,
    pub message: Option<String>,
    pub addr: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    pub name: Option<String>,
    pub labels: Labels,
    // TODO: use an enum here
    pub role: Option<String>,
    // TODO: use an enum here
    pub availability: Option<String>,
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
    architecture: Option<String>,
    os: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerStatus {
    pub leader: Option<bool>,
    // TODO: use an enum here
    pub reachability: Option<String>,
    pub addr: Option<String>,
}
