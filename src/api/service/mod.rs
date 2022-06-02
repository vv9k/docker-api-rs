#![cfg(feature = "swarm")]
//! Manage and inspect services within a swarm.
pub mod models;
pub mod opts;

pub use models::*;
pub use opts::*;

use crate::{
    conn::{Headers, Payload, AUTH_HEADER},
    Result,
};
use containers_api_conn::tty::TtyChunk;

impl_api_ty!(Service => name);

impl Service {
    api_doc! { Service => Create
    /// Creates a new service from ServiceOpts.
    |
    pub async fn create(&self, opts: &ServiceOpts) -> Result<ServiceCreateInfo> {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single(AUTH_HEADER, a));
        self.docker
            .post_json_headers(
                "/services/create",
                Payload::Json(opts.serialize()?),
                headers,
            )
            .await
    }}

    impl_api_ep! { svc: Service, resp
        Inspect -> &format!("/services/{}", svc.name)
        Delete -> &format!("/services/{}", svc.name)
        Logs -> &format!("/services/{}/logs", svc.name)
    }
}

impl Services {
    impl_api_ep! { svc: Service, resp
        List -> "/services"
    }
}
