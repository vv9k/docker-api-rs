use super::data::{Availability, Membership, Role};
use crate::{Error, Result};

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

    impl_map_field!(labels: L => "Labels");

    impl_str_field!(name: N => "Name");

    impl_str_enum_field!(role: Role => "Role");

    impl_str_enum_field!(availability: Availability => "Availability");

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
