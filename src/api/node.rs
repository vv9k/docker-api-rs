//! Manage Docker nodes
//!
//! Nodes are instances of the Engine participating in a swarm.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{
    conn::Payload,
    errors::{Error, Result},
    util::url::encoded_pair,
    Docker,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

impl_api_ty!(Node => name: N);

impl<'docker> Node<'docker> {
    /// Inspects a named node's details.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeInspect)
    pub async fn inspect(&self) -> Result<NodeInfo> {
        self.docker
            .get_json(&format!("/nodes/{}", self.name)[..])
            .await
    }

    pub async fn update(&self, opts: &NodeUpdateOpts) -> Result<()> {
        let query = encoded_pair("version", opts.version.clone());
        let path = format!("/nodes/{}/update?{}", self.name, query);
        self.docker
            .post(&path, Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }

    async fn _delete(&self, force: bool) -> Result<()> {
        let mut path = format!("/nodes/{}", self.name);
        if force {
            let query = encoded_pair("force", force);
            path.push('?');
            path.push_str(&query);
        }
        self.docker.delete(&path[..]).await.map(|_| ())
    }

    /// Delete a node.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeDelete)
    pub async fn delete(&self) -> Result<()> {
        self._delete(false).await.map(|_| ())
    }

    /// Forcefully delete a node
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeDelete)
    pub async fn force_delete(&self) -> Result<()> {
        self._delete(true).await.map(|_| ())
    }
}

impl<'docker> Nodes<'docker> {
    /// Returns information about installed plugins.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/NodeList)
    pub async fn list(&self, opts: &NodeListOpts) -> Result<Vec<NodeInfo>> {
        let mut path = vec!["/nodes".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker.get_json::<Vec<NodeInfo>>(&path.join("?")).await
    }
}

#[derive(Serialize, Debug)]
pub struct NodeUpdateOpts {
    version: String,
    params: HashMap<&'static str, serde_json::Value>,
}

impl NodeUpdateOpts {
    /// return a new instance of a builder for Opts
    pub fn builder<V: Into<String>>(version: V) -> NodeUpdateOptsBuilder {
        NodeUpdateOptsBuilder::new(version)
    }

    impl_map_field!(labels: L => "Labels");

    impl_str_field!(name: N => "Name");

    impl_str_enum_field!(role: Role => "Role");

    impl_str_enum_field!(availability: Availability => "Availability");

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }
}

#[derive(Serialize, Debug)]
pub struct NodeUpdateOptsBuilder {
    version: String,
    params: HashMap<&'static str, serde_json::Value>,
}

impl NodeUpdateOptsBuilder {
    pub fn new<V: Into<String>>(version: V) -> Self {
        Self {
            version: version.into(),
            params: HashMap::new(),
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Membership {
    Accepted,
    Pending,
}

impl AsRef<str> for Membership {
    fn as_ref(&self) -> &str {
        match &self {
            Membership::Accepted => "accepted",
            Membership::Pending => "pending",
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Availability {
    Active,
    Pause,
    Drain,
}

impl AsRef<str> for Availability {
    fn as_ref(&self) -> &str {
        match &self {
            Availability::Active => "active",
            Availability::Pause => "pause",
            Availability::Drain => "drain",
        }
    }
}

#[derive(Serialize, Debug)]
pub enum Role {
    Manager,
    Worker,
}

impl AsRef<str> for Role {
    fn as_ref(&self) -> &str {
        match &self {
            Role::Manager => "manager",
            Role::Worker => "worker",
        }
    }
}

pub enum NodeFilter {
    Id(String),
    Label(String),
    Membership(Membership),
    Name(String),
    NodeLabel(String),
    Role(Role),
}

impl_url_opts_builder!(NodeList);

impl NodeListOptsBuilder {
    pub fn filter<F>(&mut self, filters: F) -> &mut Self
    where
        F: IntoIterator<Item = NodeFilter>,
    {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                NodeFilter::Id(id) => param.insert("id", vec![id]),
                NodeFilter::Label(label) => param.insert("label", vec![label]),
                NodeFilter::Membership(membership) => {
                    param.insert("membership", vec![membership.as_ref().to_string()])
                }
                NodeFilter::Name(name) => param.insert("name", vec![name]),
                NodeFilter::NodeLabel(node) => param.insert("node.label", vec![node]),
                NodeFilter::Role(role) => param.insert("role", vec![role.as_ref().to_string()]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeInfo {
    #[serde(rename = "ID")]
    pub id: String,
    pub version: ObjectVersion,
    #[cfg(feature = "chrono")]
    pub created_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    #[cfg(feature = "chrono")]
    pub updated_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub updated_at: String,
    pub spec: NodeSpec,
    pub description: NodeDescription,
    pub status: NodeStatus,
    pub manager_status: Option<ManagerStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    // TODO: use an enum here
    pub state: String,
    pub message: String,
    pub addr: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    pub name: String,
    pub labels: serde_json::Value,
    // TODO: use an enum here
    pub role: String,
    // TODO: use an enum here
    pub availability: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub hostname: String,
    pub platform: Platform,
    pub resources: ResourceObject,
    pub engine: EngineDescription,
    #[serde(rename = "TLSInfo")]
    pub tls_info: TlsInfo,
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
pub struct EngineDescription {
    pub engine_version: String,
    pub labels: serde_json::Value,
    pub plugins: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs")]
    pub nano_cpus: i64,
    pub memory_bytes: i64,
    pub generic_resources: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    architecture: String,
    os: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerStatus {
    pub leader: bool,
    // TODO: use an enum here
    pub reachability: String,
    pub addr: String,
}
