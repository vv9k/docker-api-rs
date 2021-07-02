//! Create and manage persistent storage that can be attached to containers.

use crate::{
    api::{Labels, Options},
    conn::Payload,
    Result,
};

use hyper::Body;
use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

impl_api_ty!(Volume => name: N);

impl<'docker> Volumes<'docker> {
    /// Creates a new docker volume.
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeCreate>
    pub async fn create(&self, opts: &VolumeCreateOpts) -> Result<VolumeCreateInfo> {
        let body: Body = opts.serialize()?.into();
        let path = vec!["/volumes/create".to_owned()];

        self.docker
            .post_json(&path.join("?"), Payload::Json(body))
            .await
    }

    /// Lists the docker volumes on the current docker host
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeList>
    pub async fn list(&self) -> Result<Vec<VolumeInfo>> {
        let path = vec!["/volumes".to_owned()];

        let volumes_rep = self.docker.get_json::<VolumesInfo>(&path.join("?")).await?;
        Ok(match volumes_rep.volumes {
            Some(volumes) => volumes,
            None => vec![],
        })
    }
}

impl<'docker> Volume<'docker> {
    /// Deletes a volume
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/VolumeDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/volumes/{}", self.name)[..])
            .await?;
        Ok(())
    }
}

impl_json_opts_builder!(VolumeCreate);

impl VolumeCreateOptsBuilder {
    impl_str_field!(name: N => "Name");

    impl_map_field!(labels: L => "Labels");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeCreateInfo {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumesInfo {
    pub volumes: Option<Vec<VolumeInfo>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeInfo {
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    pub driver: String,
    pub labels: Option<Labels>,
    pub name: String,
    pub mountpoint: String,
    pub options: Option<Options>,
    pub scope: String,
}
