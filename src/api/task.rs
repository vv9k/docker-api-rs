//! A task is a container running on a swarm. It is the atomic scheduling unit of swarm.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{models, opts::TaskListOpts, Result};

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
