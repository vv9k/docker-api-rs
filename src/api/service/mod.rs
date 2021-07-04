#![cfg(feature = "swarm")]
//! Manage and inspect services within a swarm.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{
    conn::{Headers, Payload},
    Result,
};

impl_api_ty!(Service => name: N);

impl<'docker> Service<'docker> {
    api_doc! { Service => Create
    /// Creates a new service from ServiceOpts.
    |
    pub async fn create(&self, opts: &ServiceOpts) -> Result<ServiceCreateInfo> {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single("X-Registry-Auth", a));
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

impl<'docker> Services<'docker> {
    impl_api_ep! { svc: Service, resp
        List -> "/services"
    }
}
