#![cfg(feature = "swarm")]
//! A task is a container running on a swarm. It is the atomic scheduling unit of swarm.
//! Swarm mode must be enabled for these endpoints to work.

use crate::Result;

impl_api_ty!(Task => id: I);

impl<'docker> Task<'docker> {
    impl_api_ep! { task: Task, resp
        Inspect -> &format!("/tasks/{}", task.id)
        Logs -> &format!("/tasks/{}/logs", task.id)
    }
}

impl<'docker> Tasks<'docker> {
    impl_api_ep! { task: Task, resp
        List -> "/tasks"
    }
}

pub mod data {
    use crate::api::{
        Driver, Isolation, Labels, Mount, NetworkAttachmentConfig, ObjectVersion, Platform,
        ResourceObject, Sysctls, Ulimit,
    };
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "chrono")]
    use chrono::{DateTime, Utc};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaskInfo {
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
        pub name: String,
        pub labels: Labels,
        pub spec: TaskSpec,
        pub slot: isize,
        #[serde(rename = "NodeID")]
        pub node_id: String,
        // TODO: generic resources field
        // pub assigned_generic_resources: Vec<serde_json::Value>, ??
        pub status: TaskStatus,
        pub desired_state: TaskState,
        pub job_iteration: ObjectVersion,
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum TaskState {
        New,
        Allocated,
        Pending,
        Assigned,
        Accepted,
        Preparing,
        Ready,
        Starting,
        Running,
        Complete,
        Shutdown,
        Failed,
        Rejected,
        Remove,
        Orphaned,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaskStatus {
        #[cfg(feature = "chrono")]
        pub timestamp: DateTime<Utc>,
        #[cfg(not(feature = "chrono"))]
        pub timestamp: String,
        pub state: TaskState,
        pub message: String,
        pub err: String,
        pub container_status: ContainerStatus,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ContainerStatus {
        #[serde(rename = "ContainerID")]
        pub container_id: String,
        #[serde(rename = "PID")]
        pub pid: isize,
        pub exit_code: isize,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaskSpec {
        pub plugin_spec: Option<PluginSpec>,
        pub container_spec: Option<ContainerSpec>,
        pub network_attachment_spec: Option<NetworkAttachmentSpec>,
        pub resources: Option<Resources>,
        pub restart_policy: Option<TaskRestartPolicy>,
        pub placement: Option<TaskPlacement>,
        pub force_update: Option<isize>,
        pub runtime: Option<String>,
        pub networks: Option<Vec<NetworkAttachmentConfig>>,
        pub log_driver: Option<Driver>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaskPlacement {
        pub constraints: Option<Vec<String>>,
        pub preferences: Option<Vec<PlacementPreference>>,
        pub max_replicas: Option<u64>,
        pub platforms: Option<Vec<Platform>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct PlacementPreference {
        pub spread: SpreadOver,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SpreadOver {
        pub spread_descriptor: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaskRestartPolicy {
        pub condition: Option<RestartCondition>,
        pub delay: Option<i64>,
        pub max_attempts: Option<i64>,
        pub window: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum RestartCondition {
        None,
        // TODO: figure out if this deserializes to `on-failure`
        OnFailure,
        Any,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Resources {
        pub limits: Option<Limit>,
        pub reservation: Option<ResourceObject>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct Limit {
        #[serde(rename = "NanoCPUs")]
        pub nano_cpus: Option<i64>,
        pub memory_bytes: Option<i64>,
        pub pids: Option<i64>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct NetworkAttachmentSpec {
        #[serde(rename = "ContainerID")]
        pub container_id: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct PluginSpec {
        pub name: Option<String>,
        pub remote: Option<String>,
        pub disabled: Option<bool>,
        pub plugin_privilege: Option<Vec<PluginPrivilege>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct PluginPrivilege {
        pub name: Option<String>,
        pub description: Option<String>,
        pub value: Option<Vec<String>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ContainerSpec {
        pub image: Option<String>,
        pub labels: Option<Labels>,
        pub command: Option<Vec<String>>,
        pub args: Option<Vec<String>>,
        pub hostname: Option<String>,
        pub env: Option<Vec<String>>,
        pub dir: Option<String>,
        pub user: Option<String>,
        pub groups: Option<Vec<String>>,
        pub privileges: Option<Privileges>,
        #[serde(rename = "TTY")]
        pub tty: Option<bool>,
        pub open_stdin: Option<bool>,
        pub read_only: Option<bool>,
        pub mounts: Option<Vec<Mount>>,
        pub stop_signal: Option<String>,
        pub health_check: Option<HealthConfig>,
        pub hosts: Option<Vec<String>>,
        #[serde(rename = "DNSConfig")]
        pub dns_config: Option<DnsConfig>,
        // TODO: secrets, configs
        pub isolation: Option<Isolation>,
        pub init: Option<bool>,
        pub sysctls: Option<Sysctls>,
        pub capability_add: Option<Vec<String>>,
        pub capability_drop: Option<Vec<String>>,
        pub ulimits: Option<Vec<Ulimit>>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct DnsConfig {
        pub nameservers: Option<Vec<String>>,
        pub search: Option<Vec<String>>,
        pub options: Option<Vec<String>>,
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
    #[serde(rename_all = "PascalCase")]
    pub struct Privileges {
        pub credential_spec: CredentialSpec,
        #[serde(rename = "SELinuxContext")]
        pub se_linux_context: SeLinuxContext,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CredentialSpec {
        pub config: String,
        pub file: String,
        pub registry: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SeLinuxContext {
        pub disable: bool,
        pub user: String,
        pub role: String,
        #[serde(rename = "Type")]
        pub type_: String,
        pub level: String,
    }
}

pub use data::*;

pub mod opts {
    use crate::api::Filter;

    impl_opts_builder!(url => TaskList);

    #[derive(Clone, Copy, Debug)]
    pub enum TaskStateFilter {
        Running,
        Shutdown,
        Accepted,
    }

    impl AsRef<str> for TaskStateFilter {
        fn as_ref(&self) -> &str {
            match &self {
                Self::Running => "running",
                Self::Shutdown => "shutdown",
                Self::Accepted => "accepted",
            }
        }
    }

    pub enum TaskFilter {
        /// The state that the task should be in.
        DesiredState(TaskStateFilter),
        /// The ID of the config.
        Id(String),
        /// Label in the form of `label=key`
        LabelKey(String),
        /// Label in the form of `label=key=val`
        Label(String, String),
        /// The name of the config.
        Name(String),
        /// Name of the node.
        Node(String),
        /// Name of the service.
        Service(String),
    }

    impl Filter for TaskFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            use TaskFilter::*;
            match &self {
                DesiredState(state) => ("desired-state", state.as_ref().to_string()),
                Id(id) => ("id", id.to_owned()),
                LabelKey(key) => ("label", key.to_owned()),
                Label(key, val) => ("label", format!("{}={}", key, val)),
                Name(name) => ("name", name.to_owned()),
                Node(node) => ("node", node.to_owned()),
                Service(service) => ("service", service.to_owned()),
            }
        }
    }

    impl TaskListOptsBuilder {
        impl_filter_func!(
            /// Filter listed tasks by variants of the enum.
            TaskFilter
        );
    }
}

pub use opts::*;
