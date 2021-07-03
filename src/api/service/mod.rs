#![cfg(feature = "swarm")]
//! Manage and inspect services within a swarm.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{
    api::LogsOpts,
    conn::{tty, Headers, Payload},
    util::url::construct_ep,
    Result,
};

use futures_util::stream::Stream;

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

    impl_inspect! {svc: Service -> format!("/services/{}", svc.name)}

    api_doc! { Service => Delete
    /// Deletes a service.
    |
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete_json(&format!("/services/{}", self.name))
            .await
    }}

    api_doc! { Service => Logs
    /// Returns a stream of logs from a service.
    |
    pub fn logs(
        &self,
        opts: &LogsOpts,
    ) -> impl Stream<Item = Result<tty::TtyChunk>> + Unpin + 'docker {
        let stream = Box::pin(self.docker.stream_get(construct_ep(
            format!("/services/{}/logs", self.name),
            opts.serialize(),
        )));
        Box::pin(tty::decode(stream))
    }}
}

impl<'docker> Services<'docker> {
    api_doc! { Service => List
    /// Lists the docker services on the current docker host.
    |
    pub async fn list(&self, opts: &ListOpts) -> Result<Vec<ServiceInfo>> {
        self.docker
            .get_json(&construct_ep("/services", opts.serialize()))
            .await
    }}
}
