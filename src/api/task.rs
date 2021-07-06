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
    use crate::api::{Labels, ObjectVersion};
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
        // TODO: TaskSpec fields
    }
}

pub use data::*;

pub mod opts {
    use crate::api::Filter;

    impl_url_opts_builder!(TaskList);

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
