//! Manage Docker nodes
//!
//! Nodes are instances of the Engine participating in a swarm.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{
    conn::{Headers, Payload},
    models,
    opts::{NodeListOpts, NodeUpdateOpts},
    Result,
};
use containers_api::url::encoded_pair;

impl_api_ty!(Node => name);

type Void = ();

impl Node {
    impl_api_ep! {node: Node, resp
        Inspect -> &format!("/nodes/{}", node.name), models::Node
        ForceDelete -> &format!("/nodes/{}", node.name), Void
    }

    api_doc! { Node => Update
    |
    /// Update a node.
    pub async fn update(&self, opts: &NodeUpdateOpts) -> Result<()> {
        self.docker
            .post(
                &format!(
                    "/nodes/{}/update?{}",
                    self.name,
                    encoded_pair("version", opts.version().to_string())
                ),
                Payload::Json(opts.serialize_vec()?),
                Headers::none()
            )
            .await
            .map(|_| ())
    }}
}

impl Nodes {
    impl_api_ep! {node: Node, resp
        List -> "/nodes", models::Node
    }
}
