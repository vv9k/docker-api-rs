//! Manage and inspect services within a swarm.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{
    api::container::LogsOpts,
    conn::{tty, Headers, Payload},
    Docker, Result,
};

use futures_util::stream::Stream;
use hyper::Body;

impl_api_ty!(Service => name: N);

impl<'docker> Services<'docker> {
    /// Lists the docker services on the current docker host
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceList>
    pub async fn list(&self, opts: &ListOpts) -> Result<Vec<ServiceInfo>> {
        let mut path = vec!["/services".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<ServiceInfo>>(&path.join("?"))
            .await
    }
}

impl<'docker> Service<'docker> {
    /// Creates a new service from ServiceOpts
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceCreate>
    pub async fn create(&self, opts: &ServiceOpts) -> Result<ServiceCreateInfo> {
        let body: Body = opts.serialize()?.into();
        let path = vec!["/service/create".to_owned()];

        let headers = opts
            .auth_header()
            .map(|a| Headers::single("X-Registry-Auth", a));

        self.docker
            .post_json_headers(&path.join("?"), Payload::Json(body), headers)
            .await
    }

    /// Inspects a named service's details
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceInspect>
    pub async fn inspect(&self) -> Result<ServiceDetails> {
        self.docker
            .get_json(&format!("/services/{}", self.name)[..])
            .await
    }

    /// Deletes a service
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete_json(&format!("/services/{}", self.name)[..])
            .await
    }

    /// Returns a stream of logs from a service
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceLogs>
    pub fn logs(
        &self,
        opts: &LogsOpts,
    ) -> impl Stream<Item = Result<tty::TtyChunk>> + Unpin + 'docker {
        let mut path = vec![format!("/services/{}/logs", self.name)];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }

        let stream = Box::pin(self.docker.stream_get(path.join("?")));

        Box::pin(tty::decode(stream))
    }
}
