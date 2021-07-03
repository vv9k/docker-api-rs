//! Create and manage user-defined networks that containers can be attached to.

pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, util::url::construct_ep, Result};

impl_api_ty!(Network => id: I);

impl<'docker> Network<'docker> {
    impl_inspect! {net: Network -> format!("/networks/{}", net.id)}

    api_doc! { Network => Delete
    /// Delete the network instance.
    |
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/networks/{}", self.id))
            .await?;
        Ok(())
    }}

    api_doc! { Network => Connect
    /// Connect container to network.
    |
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("connect", opts).await
    }}

    api_doc! { Network => Disconnect
    /// Disconnect container to network.
    |
    pub async fn disconnect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("disconnect", opts).await
    }}

    async fn do_connection<S>(&self, segment: S, opts: &ContainerConnectionOpts) -> Result<()>
    where
        S: AsRef<str>,
    {
        self.docker
            .post(
                &format!("/networks/{}/{}", self.id, segment.as_ref()),
                Payload::Json(opts.serialize()?),
            )
            .await?;
        Ok(())
    }
}

impl<'docker> Networks<'docker> {
    api_doc! { Network => List
    /// List the docker networks on the current docker host.
    |
    pub async fn list(&self, opts: &NetworkListOpts) -> Result<Vec<NetworkInfo>> {
        self.docker
            .get_json(&construct_ep("/networks", opts.serialize()))
            .await
    }}

    api_doc! { Network => Create
    /// Create a new Network instance.
    |
    pub async fn create(&self, opts: &NetworkCreateOpts) -> Result<NetworkCreateInfo> {
        self.docker
            .post_json("/networks/create", Payload::Json(opts.serialize()?))
            .await
    }}

    api_doc! { Network => Prune
    /// Delete unused networks. Returns a list of deleted network names.
    |
    pub async fn prune(&self, opts: &NetworkPruneOpts) -> Result<Vec<String>> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct PruneResponse {
            networks_deleted: Vec<String>,
        }
        self.docker
            .post_json(
                &construct_ep("/networks/prune", opts.serialize()),
                Payload::empty(),
            )
            .await
            .map(|resp: PruneResponse| resp.networks_deleted)
    }}
}
