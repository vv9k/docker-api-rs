use crate::api::{ContainerConfig, DriverData, Labels};

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use crate::util::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub description: String,
    pub is_official: bool,
    pub is_automated: bool,
    pub name: String,
    pub star_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInfo {
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub id: String,
    pub parent_id: String,
    pub labels: Option<Labels>,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub virtual_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDetails {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub parent: String,
    pub comment: String,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub container: String,
    pub docker_version: String,
    pub author: String,
    pub config: ContainerConfig,
    pub architecture: String,
    pub os: String,
    pub os_version: Option<String>,
    pub size: i64,
    pub virtual_size: i64,
    pub graph_driver: DriverData,
    #[serde(rename = "RootFS")]
    pub root_fs: ImageRootFs,
    pub metadata: ImageMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageRootFs {
    #[serde(rename = "Type")]
    pub type_: String,
    pub layers: Option<Vec<String>>,
    pub base_layer: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageMetadata {
    #[cfg(feature = "chrono")]
    pub last_tag_timed: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub last_tag_timed: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct History {
    pub id: String,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub created_by: String,
    pub comment: String,
    pub size: i64,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Untagged(String),
    Deleted(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Represents a response chunk from Docker api when building, pulling or importing an image.
pub enum ImageBuildChunk {
    Update {
        stream: String,
    },
    Error {
        error: String,
        #[serde(rename = "errorDetail")]
        error_detail: ErrorDetail,
    },
    Digest {
        aux: Aux,
    },
    PullStatus {
        status: String,
        id: Option<String>,
        progress: Option<String>,
        #[serde(rename = "progressDetail")]
        progress_detail: Option<ProgressDetail>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aux {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressDetail {
    pub current: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Descriptor {
    media_type: String,
    digest: String,
    size: u64,
    #[serde(rename = "URLs")]
    urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DistributionInspectInfo {
    descriptor: Descriptor,
    platforms: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ImagesPruneInfo {
    pub images_deleted: Vec<ImageDeleteItem>,
    pub space_reclaimed: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDeleteItem {
    pub untagged: String,
    pub deleted: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClearCacheInfo {
    pub caches_deleted: Vec<String>,
    pub space_reclaimed: i64,
}
