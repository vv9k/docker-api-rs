#![cfg(feature = "swarm")]
//! Manage Docker nodes
//!
//! Nodes are instances of the Engine participating in a swarm.
//! Swarm mode must be enabled for these endpoints to work.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, util::url::encoded_pair, Result};

impl_api_ty!(Node => name: N);

type Void = ();

impl<'docker> Node<'docker> {
    impl_api_ep! {node: Node, resp
        Inspect -> &format!("/nodes/{}", node.name)
        ForceDelete -> &format!("/nodes/{}", node.name), Void
    }

    api_doc! { Node => Update
    /// Update a node.
    |
    pub async fn update(&self, opts: &NodeUpdateOpts) -> Result<()> {
        self.docker
            .post(
                &format!(
                    "/nodes/{}/update?{}",
                    self.name,
                    encoded_pair("version", opts.version().to_string())
                ),
                Payload::Json(opts.serialize()?),
            )
            .await
            .map(|_| ())
    }}
}

impl<'docker> Nodes<'docker> {
    impl_api_ep! {node: Node, resp
        List -> "/nodes"
    }
}
