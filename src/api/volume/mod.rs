//! Create and manage persistent storage that can be attached to containers.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Volume => name: N);

impl<'docker> Volume<'docker> {
    /// Deletes this volume.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/volumes/{}", self.name))
            .await
            .map(|_| ())
    }

    /// Inspects this volume.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeInspect>
    pub async fn inspect(&self) -> Result<VolumeInfo> {
        self.docker
            .get_json(&format!("/volumes/{}", self.name))
            .await
    }
}

impl<'docker> Volumes<'docker> {
    /// Creates a new docker volume.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeCreate>
    pub async fn create(&self, opts: &VolumeCreateOpts) -> Result<VolumeCreateInfo> {
        self.docker
            .post_json("/volumes/create", Payload::Json(opts.serialize()?))
            .await
    }

    /// Lists the docker volumes on the current docker host.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeList>
    pub async fn list(&self) -> Result<Vec<VolumeInfo>> {
        self.docker
            .get_json("/volumes")
            .await
            .map(|rep: VolumesInfo| rep.volumes.unwrap_or_default())
    }

    /// Delete unused volumes.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/VolumePrune)
    pub async fn prune(&self, opts: &VolumePruneOpts) -> Result<VolumePruneInfo> {
        self.docker
            .post_json(
                &crate::util::url::construct_ep("/volumes/prune", opts.serialize()),
                Payload::empty(),
            )
            .await
    }
}
