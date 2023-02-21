//! Create and manage user-defined networks that containers can be attached to.

use crate::{
    conn::{Headers, Payload},
    models,
    opts::{
        ContainerConnectionOpts, ContainerDisconnectionOpts, NetworkCreateOpts, NetworkListOpts,
        NetworkPruneOpts,
    },
    Result,
};

impl_api_ty!(Network => id);

impl Network {
    impl_api_ep! { net: Network, resp
        Inspect -> &format!("/networks/{}", net.id), models::Network
        Delete -> &format!("/networks/{}", net.id), ()
    }

    api_doc! { Network => Connect
    |
    /// Connect a container to a network.
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.docker
            .post_string(
                &format!("/networks/{}/connect", self.id),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Network => Disconnect
    |
    /// Disconnect a container from a network.
    pub async fn disconnect(&self, opts: &ContainerDisconnectionOpts) -> Result<()> {
        self.docker
            .post_string(
                &format!("/networks/{}/disconnect", &self.id),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}
}

impl Networks {
    impl_api_ep! { __: Network, resp
        List -> "/networks", models::Network
        Prune -> "/networks/prune", models::NetworkPrune200Response
    }

    api_doc! { Network => Create
    |
    /// Create a new network.
    pub async fn create(&self, opts: &NetworkCreateOpts) -> Result<Network> {
        // #TODO: handle missing id and return warnings (?)
        self.docker
            .post_json(
                "/networks/create",
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|resp: models::NetworkCreate201Response| {
                Network::new(self.docker.clone(), resp.id.unwrap_or_default())
            })
    }}
}
