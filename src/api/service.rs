//! Manage and inspect services within a swarm.
use crate::{
    conn::{Headers, Payload, AUTH_HEADER},
    models,
    opts::{ServiceListOpts, ServiceOpts},
    Result,
};

impl_api_ty!(Service => name);

impl Service {
    api_doc! { Service => Create
    |
    /// Creates a new service from ServiceOpts.
    pub async fn create(&self, opts: &ServiceOpts) -> Result<models::ServiceCreate201Response> {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single(AUTH_HEADER, a));
        self.docker
            .post_json(
                "/services/create",
                Payload::Json(opts.serialize()?),
                headers,
            )
            .await
    }}

    impl_api_ep! { svc: Service, resp
        Inspect -> &format!("/services/{}", svc.name), models::Service
        Delete -> &format!("/services/{}", svc.name), models::ServiceUpdateResponse
        Logs -> &format!("/services/{}/logs", svc.name), ()
    }
}

impl Services {
    impl_api_ep! { svc: Service, resp
        List -> "/services", models::Service
    }
}
