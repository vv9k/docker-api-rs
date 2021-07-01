use crate::{Error, Result};

use std::collections::HashMap;

use serde::Serialize;
use serde_json::{json, Value};

impl_url_opts_builder!(derives = Default | "Options for filtering networks list results" NetworkList);
// TODO: implement `filters` field on network list

/// Interface for creating new docker network
#[derive(Serialize, Debug)]
pub struct NetworkCreateOpts {
    params: HashMap<&'static str, Value>,
}

impl NetworkCreateOpts {
    /// return a new instance of a builder for Opts
    pub fn builder<N>(name: N) -> NetworkCreateOptsBuilder
    where
        N: AsRef<str>,
    {
        NetworkCreateOptsBuilder::new(name.as_ref())
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }
}

#[derive(Default)]
pub struct NetworkCreateOptsBuilder {
    params: HashMap<&'static str, Value>,
}

impl NetworkCreateOptsBuilder {
    pub(crate) fn new(name: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("Name", json!(name));
        NetworkCreateOptsBuilder { params }
    }

    impl_str_field!(driver: D => "Driver");

    impl_map_field!(labels: L => "Labels");

    pub fn build(&self) -> NetworkCreateOpts {
        NetworkCreateOpts {
            params: self.params.clone(),
        }
    }
}

/// Interface for connect container to network
#[derive(Serialize, Debug)]
pub struct ContainerConnectionOpts {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOpts {
    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    /// return a new instance of a builder for Opts
    pub fn builder<I>(container_id: I) -> ContainerConnectionOptsBuilder
    where
        I: AsRef<str>,
    {
        ContainerConnectionOptsBuilder::new(container_id.as_ref())
    }
}

#[derive(Default)]
pub struct ContainerConnectionOptsBuilder {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOptsBuilder {
    pub(crate) fn new(container_id: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("Container", json!(container_id));
        ContainerConnectionOptsBuilder { params }
    }

    pub fn aliases<A, S>(&mut self, aliases: A) -> &mut Self
    where
        A: IntoIterator<Item = S>,
        S: AsRef<str> + Serialize,
    {
        self.params.insert(
            "EndpointConfig",
            json!({ "Aliases": json!(aliases.into_iter().collect::<Vec<_>>()) }),
        );
        self
    }

    pub fn force(&mut self) -> &mut Self {
        self.params.insert("Force", json!(true));
        self
    }

    pub fn build(&self) -> ContainerConnectionOpts {
        ContainerConnectionOpts {
            params: self.params.clone(),
        }
    }
}
