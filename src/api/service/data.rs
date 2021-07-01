use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

pub type ServicesInfo = Vec<ServiceInfo>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceInfo {
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
    pub endpoint: Endpoint,
    pub update_status: Option<UpdateStatus>,
    pub service_status: Option<ServiceStatus>,
    pub job_status: Option<JobStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    pub spec: EndpointSpec,
    pub ports: Option<Vec<EndpointPortConfig>>,
    #[serde(rename = "VirtualIPs")]
    pub virtual_ips: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointSpec {
    pub mode: Option<String>,
    pub ports: Option<Vec<EndpointPortConfig>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointPortConfig {
    pub name: Option<String>,
    pub protocol: String,
    pub publish_mode: String,
    pub published_port: Option<u64>,
    pub target_port: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStatus {
    pub state: String,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    #[cfg(feature = "chrono")]
    pub completed_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub completed_at: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceStatus {
    pub running_tasks: u64,
    pub desired_tasks: u64,
    pub completed_tasks: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStatus {
    pub job_iteration: ObjectVersion,
    #[cfg(feature = "chrono")]
    pub last_execution: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub last_execution: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDetails {
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
    pub spec: ServiceSpec,
    pub endpoint: Endpoint,
    pub update_status: Option<UpdateStatus>,
    pub service_status: Option<ServiceStatus>,
    pub job_status: Option<JobStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSpec {
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub task_template: TaskSpec,
    pub mode: Mode,
    pub update_config: Option<UpdateConfig>,
    pub rollback_config: Option<RollbackConfig>,
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    pub endpoint_spec: EndpointSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
// #TODO: Add missing fields...
pub struct TaskSpec {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    pub replicated: Option<Replicated>,
    pub global: Option<serde_json::Value>,
    pub replicated_job: Option<ReplicatedJob>,
    pub global_job: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Replicated {
    pub replicas: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicatedJob {
    pub max_concurrent: u64,
    pub total_completions: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateConfig {
    pub parallelism: u64,
    pub delay: u64,
    pub failure_action: String,
    pub monitor: u64,
    pub max_failure_ratio: usize,
    pub order: String,
}

pub type RollbackConfig = UpdateConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkAttachmentConfig {
    pub target: String,
    pub aliases: Vec<String>,
    pub driver_opts: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceCreateInfo {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Warning")]
    pub warning: Option<String>,
}
