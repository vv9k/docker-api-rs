use crate::models::{NodeSpecAvailabilityInlineItem, NodeSpecRoleInlineItem};
use crate::{Error, Result};
use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_filter_func, impl_map_field, impl_opts_builder, impl_str_enum_field, impl_str_field,
};

use serde::Serialize;

use std::collections::HashMap;

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
        labels => "Labels"
    );

    impl_str_field!(
        /// Name for the node.
        name => "Name"
    );

    impl_str_enum_field!(
        /// Role of the node.
        role: NodeSpecRoleInlineItem => "Role"
    );

    impl_str_enum_field!(
        /// Availability of the node.
        availability: NodeSpecAvailabilityInlineItem => "Availability"
    );

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    pub fn serialize_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self).map_err(Error::from)
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
    Role(NodeSpecRoleInlineItem),
}

impl Filter for NodeFilter {
    fn query_item(&self) -> FilterItem {
        match &self {
            NodeFilter::Id(id) => FilterItem::new("id", id.to_owned()),
            NodeFilter::Label(label) => FilterItem::new("label", label.to_owned()),
            NodeFilter::Membership(membership) => {
                FilterItem::new("membership", membership.as_ref().to_string())
            }
            NodeFilter::Name(name) => FilterItem::new("name", name.to_owned()),
            NodeFilter::NodeLabel(node) => FilterItem::new("node.label", node.to_owned()),
            NodeFilter::Role(role) => FilterItem::new("role", role.as_ref().to_string()),
        }
    }
}

impl_opts_builder!(url => NodeList);

impl NodeListOptsBuilder {
    impl_filter_func!(NodeFilter);
}
