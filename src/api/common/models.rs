use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type Labels = HashMap<String, String>;
pub type StatusMap = HashMap<String, String>;
pub type Options = HashMap<String, String>;
pub type Data = HashMap<String, String>;
pub type ConfigMap = HashMap<String, String>;
pub type Attributes = HashMap<String, String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub index: u64,
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
pub struct DriverData {
    pub name: String,
    pub data: Data,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Driver {
    pub name: String,
    pub options: Options,
}
