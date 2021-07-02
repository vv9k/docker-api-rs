use crate::api::Labels;

use std::collections::HashMap;

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
    pub architecture: String,
    pub author: String,
    pub comment: String,
    pub config: ContainerConfig,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub docker_version: String,
    pub id: String,
    pub os: String,
    pub parent: String,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub size: u64,
    pub virtual_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub cmd: Option<Vec<String>>,
    pub domainname: String,
    pub entrypoint: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub exposed_ports: Option<HashMap<String, Labels>>,
    pub hostname: String,
    pub image: String,
    pub labels: Option<Labels>,
    // pub MacAddress: String,
    pub on_build: Option<Vec<String>>,
    // pub NetworkDisabled: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub user: String,
    pub working_dir: String,
}

impl ContainerConfig {
    pub fn env(&self) -> Labels {
        let mut map = HashMap::new();
        if let Some(ref vars) = self.env {
            for e in vars {
                let pair: Vec<&str> = e.split('=').collect();
                map.insert(pair[0].to_owned(), pair[1].to_owned());
            }
        }
        map
    }
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
