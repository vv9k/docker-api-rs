use crate::{api::Filter, Error, Result};

use std::{collections::HashMap, convert::AsRef};

use serde::Serialize;
use serde_json::{json, Value};

impl_url_opts_builder!(
    derives = Default
        | /// Options for filtering networks list results"
        NetworkList
);
// TODO: implement `filters` field on network list

/// Used for [`NetworkFilter::Scope`](NetworkFilter::Scope).
pub enum Scope {
    Swarm,
    Global,
    Local,
}

impl AsRef<str> for Scope {
    fn as_ref(&self) -> &str {
        match &self {
            Scope::Swarm => "swarm",
            Scope::Global => "global",
            Scope::Local => "local",
        }
    }
}

pub enum NetworkType {
    Custom,
    Builtin,
}

impl AsRef<str> for NetworkType {
    fn as_ref(&self) -> &str {
        match &self {
            NetworkType::Custom => "custom",
            NetworkType::Builtin => "builtin",
        }
    }
}

/// A single filter item used to filter the output of listing the networks.
pub enum NetworkFilter {
    /// When set to true (or 1), returns all networks that are not in use by a container.
    /// When set to false (or 0), only networks that are in use by one or more containers are returned.
    Dangling(bool),
    /// Matches a network's driver.
    Driver(String),
    /// Matches all or part of a network ID.
    Id(String),
    /// Label in the form of `label=key`
    LabelKey(String),
    /// Label in the form of `label=key=val`
    LabelKeyVal(String, String),
    /// Matches all or part of a network name.
    Name(String),
    Scope(Scope),
    Type(NetworkType),
}

// impl Filter for NetworkPruneFilter {
//     fn query_key_val(&self) -> (&'static str, String) {
//         use NetworkPruneFilter::*;
//         match &self {
//             Until(until) => ("until", until.to_owned()),
//             LabelKey(label) => ("label", label.to_owned()),
//             LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
//         }
//     }
// }

impl Filter for NetworkFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use NetworkFilter::*;

        match &self {
            Dangling(dangling) => ("dangling", dangling.to_string()),
            Driver(driver) => ("driver", driver.to_owned()),
            Id(id) => ("id", id.to_owned()),
            LabelKey(key) => ("label", key.to_owned()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
            Name(name) => ("name", name.to_owned()),
            Scope(scope) => ("scope", scope.as_ref().to_owned()),
            Type(type_) => ("type", type_.as_ref().to_owned()),
        }
    }
}

impl NetworkListOptsBuilder {
    impl_filter_func!(
        /// Filter the list of networks by one of the variants of the filter.
        NetworkFilter
    );
}

/// Interface for creating new docker network
#[derive(Serialize, Debug)]
pub struct NetworkCreateOpts {
    params: HashMap<&'static str, Value>,
}

impl NetworkCreateOpts {
    /// Return a new instance of a opts-builder for creating a network.
    pub fn builder<N>(name: N) -> NetworkCreateOptsBuilder
    where
        N: AsRef<str>,
    {
        NetworkCreateOptsBuilder::new(name.as_ref())
    }

    /// Serializes the options as a JSON string.
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

    impl_map_field!(json labels: L => "Labels");

    pub fn build(&self) -> NetworkCreateOpts {
        NetworkCreateOpts {
            params: self.params.clone(),
        }
    }
}

#[derive(Serialize, Debug)]
/// Interface for connect container to network
pub struct ContainerConnectionOpts {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOpts {
    /// Serializes the options as a JSON string.
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    /// Return a new instance of a builder for connectiong to container.
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

    // TODO: more connection options

    pub fn build(&self) -> ContainerConnectionOpts {
        ContainerConnectionOpts {
            params: self.params.clone(),
        }
    }
}

impl_url_opts_builder!(derives = Default | NetworkPrune);

pub enum NetworkPruneFilter {
    /// Prune networks created before this timestamp. The <timestamp> can be Unix timestamps,
    /// date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m) computed relative
    /// to the daemon machine’s time.
    Until(String),
    #[cfg(feature = "chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
    /// Prune networks created before this timestamp. Same as `Until` but takes a datetime object.
    UntilDate(chrono::DateTime<chrono::Utc>),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
}

impl Filter for NetworkPruneFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use NetworkPruneFilter::*;
        match &self {
            Until(until) => ("until", until.to_owned()),
            #[cfg(feature = "chrono")]
            UntilDate(until) => ("until", until.timestamp().to_string()),
            LabelKey(label) => ("label", label.to_owned()),
            Label(key, val) => ("label", format!("{}={}", key, val)),
        }
    }
}

impl NetworkPruneOptsBuilder {
    impl_filter_func!(
        /// Filter the networks to prune by one of the variants of the enum.
        NetworkPruneFilter
    );
}
