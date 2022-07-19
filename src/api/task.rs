//! A task is a container running on a swarm. It is the atomic scheduling unit of swarm.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{models, Result};

impl_api_ty!(Task => id);

impl Task {
    impl_api_ep! { task: Task, resp
        Inspect -> &format!("/tasks/{}", task.id), models::Task
        Logs -> &format!("/tasks/{}/logs", task.id), ()
    }
}

impl Tasks {
    impl_api_ep! { task: Task, resp
        List -> "/tasks", models::Task
    }
}

pub mod opts {
    use containers_api::opts::Filter;
    use containers_api::{impl_filter_func, impl_opts_builder};

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
