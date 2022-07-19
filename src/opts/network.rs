use crate::{models::Ipam, Error, Result};
use containers_api::opts::Filter;
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field, impl_vec_field,
};

use std::{collections::HashMap, convert::AsRef};

use serde::Serialize;
use serde_json::{json, Value};

impl_opts_builder!(url =>
    /// Options for filtering networks list results"
    NetworkList
);

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

    impl_field!(
        /// Check for networks with duplicate names. Since Network is primarily keyed based on a
        /// random ID and not on the name, and network name is strictly a user-friendly alias to
        /// the network which is uniquely identified using ID, there is no guaranteed way to check
        /// for duplicates. CheckDuplicate is there to provide a best effort checking of any
        /// networks which has the same name but it is not guaranteed to catch all name collisions.
        check_duplicate: bool => "CheckDuplicate"
    );

    impl_str_field!(
        /// Name of the network driver plugin to use.
        driver => "Driver"
    );

    impl_field!(
        /// Restrict external access to the network.
        internal: bool => "Internal"
    );

    impl_field!(
        /// Globally scoped network is manually attachable by regular containers from workers
        /// in swarm mode.
        attachable: bool => "Attachable"
    );

    impl_field!(
        /// Ingress network is the network which provides the routing-mesh in swarm mode.
        ingress: bool => "Ingress"
    );

    impl_field!(
        /// Enable IPv6 on the network.
        enable_ipv6: bool => "EnableIPv6"
    );

    impl_map_field!(json
        /// Network specific options to be used by the drivers.
        options => "Options"
    );

    impl_map_field!(json
        /// User-defined key/value metadata.
        labels => "Labels"
    );

    impl_field!(
        /// IP Address Management configuration
        ipam: Ipam => "IPAM"
    );

    pub fn build(&self) -> NetworkCreateOpts {
        NetworkCreateOpts {
            params: self.params.clone(),
        }
    }
}
#[derive(Serialize, Debug)]
/// Interface for disconnecting a container from a network.
pub struct ContainerDisconnectionOpts {
    params: HashMap<&'static str, Value>,
}

impl ContainerDisconnectionOpts {
    /// Serializes the options as a JSON string.
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    /// Return a new instance of a builder for disconnecting a container from a network.
    pub fn builder<I>(container_id: I) -> ContainerDisconnectionOptsBuilder
    where
        I: AsRef<str>,
    {
        ContainerDisconnectionOptsBuilder::new(container_id.as_ref())
    }
}

#[derive(Default)]
pub struct ContainerDisconnectionOptsBuilder {
    params: HashMap<&'static str, Value>,
}

impl ContainerDisconnectionOptsBuilder {
    pub(crate) fn new(container_id: &str) -> Self {
        ContainerDisconnectionOptsBuilder {
            params: [("Container", json!(container_id.to_string()))].into(),
        }
    }

    impl_field!(
        /// Force the container to disconnect from the network.
        force: bool => "Force"
    );

    pub fn build(self) -> ContainerDisconnectionOpts {
        ContainerDisconnectionOpts {
            params: self.params,
        }
    }
}

#[derive(Serialize, Debug)]
/// Interface for connecting a container to a network.
pub struct ContainerConnectionOpts {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOpts {
    /// Serializes the options as a JSON string.
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    /// Return a new instance of a builder for connecting a container to a network.
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
    container: String,
}

impl ContainerConnectionOptsBuilder {
    pub(crate) fn new(container_id: &str) -> Self {
        ContainerConnectionOptsBuilder {
            params: HashMap::new(),
            container: container_id.to_string(),
        }
    }

    /// Endpoint's IPAM configuration.
    pub fn ipam_config(mut self, config: EndpointIpamConfig) -> Self {
        self.params.insert("EndpointConfig", json!(config.params));
        self
    }

    impl_vec_field!(aliases => "Aliases");

    impl_vec_field!(links => "Links");

    impl_str_field!(
        /// Unique ID of the network.
        network_id => "NetworkID"
    );

    impl_str_field!(
        /// Unique ID for the service endpoint in a Sandbox.
        endpoint_id => "EndpointID"
    );

    impl_str_field!(
        /// Gateway address for this network.
        gateway => "Gateway"
    );

    impl_str_field!(
        /// IPv4 address.
        ipv4 => "IPAddress"
    );

    impl_field!(
        /// Mask length of the IPv4 address.
        prefix_len: isize => "IPPrefixLen"
    );

    impl_str_field!(
        /// IPv6 gateway address.
        ipv6_gateway => "IPv6Gateway"
    );

    impl_str_field!(
        /// Global IPv6 address.
        ipv6 => "GlobalIPv6Address"
    );

    impl_field!(
        /// Mask length of the global IPv6 address.
        ipv6_prefix_len: i64 => "GlobalIPv6PrefixLen"
    );

    impl_str_field!(
        /// MAC address for the endpoint on this network.
        mac => "MacAddress"
    );

    impl_map_field!(json
        /// DriverOpts is a mapping of driver options and values. These options are passed directly
        /// to the driver and are driver specific.
        driver_opts => "DriverOpts"
    );

    pub fn build(self) -> ContainerConnectionOpts {
        let mut params = HashMap::new();
        params.insert("EndpointConfig", json!(self.params));
        params.insert("Container", json!(self.container));
        ContainerConnectionOpts { params }
    }
}

#[derive(Default)]
/// Used to configure endpoint IPAM configuration when connection a container to a network.
/// See [`ipam_config`](ContainerConnectOptsBuilder::ipam_config).
pub struct EndpointIpamConfig {
    params: HashMap<&'static str, serde_json::Value>,
}

impl EndpointIpamConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ipv4<A>(mut self, address: A) -> Self
    where
        A: Into<String>,
    {
        self.params.insert("IPv4Address", json!(address.into()));
        self
    }

    pub fn ipv6<A>(mut self, address: A) -> Self
    where
        A: Into<String>,
    {
        self.params.insert("IPv6Address", json!(address.into()));
        self
    }

    pub fn link_local_ips<I>(mut self, ips: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<String>,
    {
        self.params.insert(
            "LinkLocalIPs",
            json!(ips.into_iter().map(I::Item::into).collect::<Vec<_>>()),
        );
        self
    }
}

impl_opts_builder!(url => NetworkPrune);

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
