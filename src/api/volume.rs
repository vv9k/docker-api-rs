//! Create and manage persistent storage that can be attached to containers.

use crate::{
    conn::{Headers, Payload},
    models,
    opts::{ClusterVolumeUpdateOpts, VolumeCreateOpts, VolumeListOpts, VolumePruneOpts},
    Result,
};
use containers_api::url;

impl_api_ty!(Volume => name);

impl Volume {
    impl_api_ep! {vol: Volume, resp
        Inspect -> &format!("/volumes/{}", vol.name), models::Volume
        Delete -> &format!("/volumes/{}", vol.name), ()
    }

    api_doc! { Volume => Update
    |
    /// Update a volume. Valid only for Swarm cluster volumes
    pub async fn update(&self, opts: &ClusterVolumeUpdateOpts) -> Result<()> {
        let mut ep = format!("/volumes/{}", self.name());
        url::append_query(&mut ep, url::encoded_pair("version", opts.version()));
        self.docker.put(&ep, Payload::Json(opts.serialize()?)).await.map(|_| ())
    }}
}

impl Volumes {
    impl_api_ep! {__: Volume, resp
        Prune -> "/volumes/prune", models::VolumePrune200Response
    }

    api_doc! { Volume => List
    |
    /// List available volumes
    pub async fn list(&self, opts: &VolumeListOpts) -> Result<models::VolumeListResponse> {
        let ep = url::construct_ep("/volumes", opts.serialize());
        self.docker.get_json(&ep).await
    }}

    api_doc! { Volume => Create
    |
    /// Create a new volume.
    pub async fn create(&self, opts: &VolumeCreateOpts) -> Result<models::Volume> {
        // #TODO: handle missing id and return warnings (?)
        self.docker
            .post_json(
                "/volumes/create",
                Payload::Json(opts.serialize()?),
                Headers::none(),
            )
            .await
    }}
}
