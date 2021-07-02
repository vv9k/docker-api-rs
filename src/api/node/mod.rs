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

impl<'docker> Node<'docker> {
    /// Inspects a named node's details.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeInspect)
    pub async fn inspect(&self) -> Result<NodeInfo> {
        self.docker
            .get_json(&format!("/nodes/{}", self.name)[..])
            .await
    }

    pub async fn update(&self, opts: &NodeUpdateOpts) -> Result<()> {
        let query = encoded_pair("version", opts.version().to_string());
        let path = format!("/nodes/{}/update?{}", self.name, query);
        self.docker
            .post(&path, Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }

    async fn _delete(&self, force: bool) -> Result<()> {
        let mut path = format!("/nodes/{}", self.name);
        if force {
            let query = encoded_pair("force", force);
            path.push('?');
            path.push_str(&query);
        }
        self.docker.delete(&path[..]).await.map(|_| ())
    }

    /// Delete a node.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeDelete)
    pub async fn delete(&self) -> Result<()> {
        self._delete(false).await.map(|_| ())
    }

    /// Forcefully delete a node
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeDelete)
    pub async fn force_delete(&self) -> Result<()> {
        self._delete(true).await.map(|_| ())
    }
}

impl<'docker> Nodes<'docker> {
    /// Returns information about installed plugins.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeList)
    pub async fn list(&self, opts: &NodeListOpts) -> Result<Vec<NodeInfo>> {
        let mut path = vec!["/nodes".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker.get_json::<Vec<NodeInfo>>(&path.join("?")).await
    }
}
