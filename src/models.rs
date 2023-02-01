//! Generated Docker models

pub use docker_api_stubs::models::*;

use crate::errors::{Error, Result};

use hyper::header::HeaderMap;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

#[derive(Serialize, Debug)]
pub struct PingInfo {
    pub api_version: String,
    pub builder_version: Option<String>,
    pub docker_experimental: bool,
    pub cache_control: String,
    pub pragma: String,
    pub os_type: String,
    pub server: String,
    pub date: String,
}

impl TryFrom<&HeaderMap> for PingInfo {
    type Error = Error;

    fn try_from(value: &HeaderMap) -> Result<Self> {
        macro_rules! extract_str {
            ($id:literal) => {{
                if let Some(val) = value.get($id) {
                    val.to_str().map(ToString::to_string).map_err(|e| {
                        Error::InvalidResponse(format!(
                            "failed to convert header to string - {}",
                            e
                        ))
                    })?
                } else {
                    return Err(Error::InvalidResponse(format!(
                        "expected `{}` field in headers",
                        $id
                    )));
                }
            }};
        }

        Ok(PingInfo {
            api_version: extract_str!("api-version"),
            builder_version: value
                .get("builder-version")
                .and_then(|v| v.to_str().map(ToString::to_string).ok()),
            docker_experimental: extract_str!("docker-experimental").parse().map_err(|e| {
                Error::InvalidResponse(format!("expected header value to be bool - {e}"))
            })?,
            cache_control: extract_str!("cache-control"),
            pragma: extract_str!("pragma"),
            os_type: extract_str!("ostype"),
            date: extract_str!("date"),
            server: extract_str!("server"),
        })
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Aux {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ProgressDetail {
    pub current: Option<u64>,
    pub total: Option<u64>,
}

pub type Labels = std::collections::HashMap<String, String>;
