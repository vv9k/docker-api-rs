use crate::api::{Labels, Options, StatusMap};

use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeCreateInfo {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumesInfo {
    #[serde(default)]
    pub volumes: Vec<VolumeInfo>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeInfo {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    pub status: StatusMap,
    pub labels: Labels,
    pub options: Options,
    pub scope: String,
    pub usage_data: Option<UsageData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageData {
    pub size: isize,
    pub ref_count: isize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumesPruneInfo {
    pub volumes_deleted: Vec<String>,
    pub space_reclaimed: i64,
}
