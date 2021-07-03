#![cfg(feature = "swarm")]
//! Manage Docker nodes
//!
//! Nodes are instances of the Engine participating in a swarm.
//! Swarm mode must be enabled for these endpoints to work.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{
    conn::Payload,
    util::url::{construct_ep, encoded_pair},
    Result,
};

impl_api_ty!(Node => name: N);

impl<'docker> Node<'docker> {
    impl_inspect! {node: Node -> format!("/nodes/{}", node.name)}

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

    async fn _delete(&self, force: bool) -> Result<()> {
        let query = if force {
            Some(encoded_pair("force", true))
        } else {
            None
        };
        self.docker
            .delete(&construct_ep(format!("/nodes/{}", self.name), query))
            .await
            .map(|_| ())
    }

    api_doc! { Node => Delete
    /// Delete a node.
    |
    pub async fn delete(&self) -> Result<()> {
        self._delete(false).await.map(|_| ())
    }}

    api_doc! { Node => Delete
    /// Forcefully delete a node
    |
    pub async fn force_delete(&self) -> Result<()> {
        self._delete(true).await.map(|_| ())
    }}
}

impl<'docker> Nodes<'docker> {
    api_doc! { Node => List
    /// Returns information about installed plugins.
    |
    pub async fn list(&self, opts: &NodeListOpts) -> Result<Vec<NodeInfo>> {
        self.docker
            .get_json(&construct_ep("/nodes", opts.serialize()))
            .await
    }}
}
