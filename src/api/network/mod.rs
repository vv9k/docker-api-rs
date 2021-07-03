//! Create and manage user-defined networks that containers can be attached to.

pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, util::url::construct_ep, Result};

impl_api_ty!(Network => id: I);

impl<'docker> Network<'docker> {
    /// Inspects the current docker network instance's details.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkInspect>
    pub async fn inspect(&self) -> Result<NetworkDetails> {
        self.docker
            .get_json(&format!("/networks/{}", self.id))
            .await
    }

    /// Delete the network instance.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/networks/{}", self.id))
            .await?;
        Ok(())
    }

    /// Connect container to network.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkConnect>
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("connect", opts).await
    }

    /// Disconnect container to network.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkDisconnect>
    pub async fn disconnect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("disconnect", opts).await
    }

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
    /// List the docker networks on the current docker host.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkList>
    pub async fn list(&self, opts: &NetworkListOpts) -> Result<Vec<NetworkDetails>> {
        self.docker
            .get_json(&construct_ep("/networks", opts.serialize()))
            .await
    }

    /// Create a new Network instance.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkCreate>
    pub async fn create(&self, opts: &NetworkCreateOpts) -> Result<NetworkCreateInfo> {
        self.docker
            .post_json("/networks/create", Payload::Json(opts.serialize()?))
            .await
    }

    /// Delete unused networks. Returns a list of deleted network names.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NetworkPrune)
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
    }
}
