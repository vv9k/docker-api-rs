//! Create and manage user-defined networks that containers can be attached to.

pub mod models;
pub mod opts;

pub use models::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Network => id);

impl<'docker> Network<'docker> {
    impl_api_ep! { net: Network, resp
        Inspect -> &format!("/networks/{}", net.id)
        Delete -> &format!("/network/{}", net.id)
    }

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
    impl_api_ep! { __: Network, resp
        List -> "/networks"
        Create -> "/networks/create", resp.id
        Prune -> "/networks/prune"
    }
}
