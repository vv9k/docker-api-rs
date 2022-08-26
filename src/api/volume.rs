//! Create and manage persistent storage that can be attached to containers.

use crate::{
    conn::Payload,
    models,
    opts::{VolumeCreateOpts, VolumeListOpts, VolumePruneOpts},
    Result,
};

impl_api_ty!(Volume => name);

impl Volume {
    impl_api_ep! {vol: Volume, resp
        Inspect -> &format!("/volumes/{}", vol.name), models::Volume
        Delete -> &format!("/volumes/{}", vol.name), ()
    }
}

impl Volumes {
    impl_api_ep! {__: Volume, resp
        Prune -> "/volumes/prune", models::VolumePrune200Response
    }

    api_doc! { Volume => List
    /// List available volumes
    |
    pub async fn list(&self, opts: &VolumeListOpts) -> Result<models::VolumeList200Response> {
        let ep = containers_api::url::construct_ep("/volumes", opts.serialize());
        self.docker.get_json(&ep).await
    }}

    api_doc! { Volume => Create
    /// Create a new volume.
    |
    pub async fn create(&self, opts: &VolumeCreateOpts) -> Result<models::Volume> {
        // #TODO: handle missing id and return warnings (?)
        self.docker
            .post_json("/volumes/create", Payload::Json(opts.serialize()?))
            .await
    }}
}
