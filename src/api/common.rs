use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Allows easier construction of filter functions for multiple api endpoints
pub(crate) trait Filter {
    fn query_key_val(&self) -> (&'static str, String);
}

pub type Labels = HashMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TlsInfo {
    pub trust_root: String,
    pub cert_issuer_subject: String,
    pub cert_issuer_public_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub name: String,
    pub data: HashMap<String, String>,
}
