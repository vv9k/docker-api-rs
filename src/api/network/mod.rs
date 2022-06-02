//! Create and manage user-defined networks that containers can be attached to.

pub mod models;
pub mod opts;

pub use models::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Network => id);

impl Network {
    impl_api_ep! { net: Network, resp
        Inspect -> &format!("/networks/{}", net.id)
        Delete -> &format!("/networks/{}", net.id)
    }

    api_doc! { Network => Connect
    /// Connect a container to a network.
    |
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.docker
            .post(
                &format!("/networks/{}/connect", self.id),
                Payload::Json(opts.serialize()?),
            )
            .await.map(|_| ())
    }}

    api_doc! { Network => Disconnect
    /// Disconnect a container from a network.
    |
    pub async fn disconnect(&self, opts: &ContainerDisconnectionOpts) -> Result<()> {
        self.docker
            .post(
                &format!("/networks/{}/disconnect", &self.id),
                Payload::Json(opts.serialize()?),
            )
            .await
            .map(|_| ())
    }}
}

impl Networks {
    impl_api_ep! { __: Network, resp
        List -> "/networks"
        Create -> "/networks/create", resp.id
        Prune -> "/networks/prune"
    }
}
