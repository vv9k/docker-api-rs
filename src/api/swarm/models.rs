use crate::api::{Driver, Labels, ObjectVersion, Options, TlsInfo};

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnlockKey(#[serde(rename = "UnlockKey")] pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmInfo {
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
    pub spec: SwarmSpec,
    pub tls_info: TlsInfo,
    pub root_rotation_in_progress: bool,
    pub data_path_port: u32,
    pub default_addr_pool: Vec<String>,
    pub subnet_size: u32,
    pub join_tokens: JoinTokens,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmSpec {
    pub name: Option<String>,
    pub labels: Option<Labels>,
    pub orchestration: Option<Orchestration>,
    pub raft: Option<Raft>,
    pub dispatcher: Option<Dispatcher>,
    #[serde(rename = "CAConfig")]
    pub ca_config: Option<CaConfig>,
    pub encryption_config: Option<EncryptionConfig>,
    pub task_defaults: Option<TaskDefaults>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dispatcher {
    pub heartbeat_period: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CaConfig {
    pub node_cert_expiry: Option<i64>,
    #[serde(rename = "ExternalCAs")]
    pub external_cas: Option<Vec<ExternalCa>>,
    #[serde(rename = "SigningCACert")]
    pub signing_ca_cert: Option<String>,
    #[serde(rename = "SigningCAKey")]
    pub signing_ca_key: Option<String>,
    pub force_rotate: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalCa {
    pub protocol: String,
    #[serde(rename = "URL")]
    pub url: String,
    pub options: Option<Options>,
    #[serde(rename = "CACert")]
    pub ca_cert: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EncryptionConfig {
    pub auto_lock_managers: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskDefaults {
    pub log_driver: Option<Driver>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Raft {
    pub snapshot_interval: Option<u64>,
    pub keep_old_snapshots: Option<u64>,
    pub log_entries_for_slow_followers: Option<u64>,
    pub election_tick: isize,
    pub heartbeat_tick: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinTokens {
    pub worker: String,
    pub manager: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orchestration {
    pub task_history_retention_limit: Option<i64>,
}
