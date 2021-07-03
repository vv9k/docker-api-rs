//! Create and manage persistent storage that can be attached to containers.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Volume => name: N);

impl<'docker> Volume<'docker> {
    api_doc! { Volume => Delete
    /// Deletes this volume.
    |
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/volumes/{}", self.name))
            .await
            .map(|_| ())
    }}

    api_doc! { Volume => Inspect
    /// Inspects this volume.
    |
    pub async fn inspect(&self) -> Result<VolumeInfo> {
        self.docker
            .get_json(&format!("/volumes/{}", self.name))
            .await
    }}
}

impl<'docker> Volumes<'docker> {
    api_doc! { Volume => Create
    /// Creates a new docker volume.
    |
    pub async fn create(&self, opts: &VolumeCreateOpts) -> Result<VolumeCreateInfo> {
        self.docker
            .post_json("/volumes/create", Payload::Json(opts.serialize()?))
            .await
    }}

    api_doc! { Volume => List
    /// Lists the docker volumes on the current docker host.
    |
    pub async fn list(&self) -> Result<Vec<VolumeInfo>> {
        self.docker
            .get_json("/volumes")
            .await
            .map(|rep: VolumesInfo| rep.volumes.unwrap_or_default())
    }}

    api_doc! { Volume => Prune
    /// Delete unused volumes.
    |
    pub async fn prune(&self, opts: &VolumePruneOpts) -> Result<VolumePruneInfo> {
        self.docker
            .post_json(
                &crate::util::url::construct_ep("/volumes/prune", opts.serialize()),
                Payload::empty(),
            )
            .await
    }}
}
