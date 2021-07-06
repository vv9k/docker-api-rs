use super::data::{Availability, Membership, Role};
use crate::{api::Filter, Error, Result};

use serde::Serialize;

use std::collections::HashMap;

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

    impl_map_field!(json
        /// User-defined key/value metadata
        labels: L => "Labels"
    );

    impl_str_field!(
        /// Name for the node.
        name: N => "Name"
    );

    impl_str_enum_field!(
        /// Role of the node.
        role: Role => "Role"
    );

    impl_str_enum_field!(
        /// Availability of the node.
        availability: Availability => "Availability"
    );

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    pub fn version(&self) -> &str {
        &self.version
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

/// Filter type used to filter nodes by one of the variants.
pub enum NodeFilter {
    Id(String),
    /// The engine label
    Label(String),
    Membership(Membership),
    Name(String),
    NodeLabel(String),
    Role(Role),
}

impl Filter for NodeFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        match &self {
            NodeFilter::Id(id) => ("id", id.to_owned()),
            NodeFilter::Label(label) => ("label", label.to_owned()),
            NodeFilter::Membership(membership) => ("membership", membership.as_ref().to_string()),
            NodeFilter::Name(name) => ("name", name.to_owned()),
            NodeFilter::NodeLabel(node) => ("node.label", node.to_owned()),
            NodeFilter::Role(role) => ("role", role.as_ref().to_string()),
        }
    }
}

impl_url_opts_builder!(NodeList);

impl NodeListOptsBuilder {
    impl_filter_func!(NodeFilter);
}
