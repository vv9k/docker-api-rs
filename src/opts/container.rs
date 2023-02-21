use crate::models::Labels;
use crate::opts::ImageName;
use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field,
    impl_url_bool_field, impl_url_str_field, impl_vec_field,
};

use std::{
    collections::HashMap,
    hash::Hash,
    iter::Peekable,
    str::{self, FromStr},
    string::ToString,
    time::Duration,
};

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::{Error, Result};

pub enum Health {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

impl AsRef<str> for Health {
    fn as_ref(&self) -> &str {
        match &self {
            Health::Starting => "starting",
            Health::Healthy => "healthy",
            Health::Unhealthy => "unhealthy",
            Health::None => "none",
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Isolation {
    #[serde(alias = "")]
    #[default]
    Default,
    Process,
    HyperV,
}

impl AsRef<str> for Isolation {
    fn as_ref(&self) -> &str {
        match &self {
            Isolation::Default => "default",
            Isolation::Process => "process",
            Isolation::HyperV => "hyperv",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerStatus {
    Created,
    Configured,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl AsRef<str> for ContainerStatus {
    fn as_ref(&self) -> &str {
        use ContainerStatus::*;
        match &self {
            Created => "created",
            Configured => "configured",
            Restarting => "restarting",
            Running => "running",
            Removing => "removing",
            Paused => "paused",
            Exited => "exited",
            Dead => "dead",
        }
    }
}

/// Filter Opts for container listings
pub enum ContainerFilter {
    Ancestor(ImageName),
    /// Container ID or name.
    Before(String),
    /// Containers with the specified exit code.
    ExitCode(u64),
    Health(Health),
    /// The container's ID.
    Id(String),
    /// Applies only to Windows daemon.
    Isolation(Isolation),
    IsTask(bool),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
    /// The container's name.
    Name(String),
    Publish(PublishPort),
    /// Network ID or name.
    Network(String),
    /// Container ID or name.
    Since(String),
    Status(ContainerStatus),
    /// Volume name or mount point destination.
    Volume(String),
}

impl Filter for ContainerFilter {
    fn query_item(&self) -> FilterItem {
        use ContainerFilter::*;
        match &self {
            Ancestor(name) => FilterItem::new("ancestor", name.to_string()),
            Before(before) => FilterItem::new("before", before.to_owned()),
            ExitCode(c) => FilterItem::new("exit", c.to_string()),
            Health(health) => FilterItem::new("health", health.as_ref().to_string()),
            Id(id) => FilterItem::new("id", id.to_owned()),
            Isolation(isolation) => FilterItem::new("isolation", isolation.as_ref().to_string()),
            IsTask(is_task) => FilterItem::new("is-task", is_task.to_string()),
            LabelKey(key) => FilterItem::new("label", key.to_owned()),
            Label(key, val) => FilterItem::new("label", format!("{key}={val}")),
            Name(name) => FilterItem::new("name", name.to_owned()),
            Publish(port) => FilterItem::new("publsh", port.to_string()),
            Network(net) => FilterItem::new("net", net.to_owned()),
            Since(since) => FilterItem::new("since", since.to_owned()),
            Status(s) => FilterItem::new("status", s.as_ref().to_string()),
            Volume(vol) => FilterItem::new("volume", vol.to_owned()),
        }
    }
}

impl_opts_builder!(url => ContainerList);

impl ContainerListOptsBuilder {
    impl_filter_func!(
        /// Filter the list of containers by one of the enum variants.
        ContainerFilter
    );

    impl_url_bool_field!(
        /// If set to true all containers will be returned
        all => "all"
    );

    impl_url_str_field!(since => "since");

    impl_url_str_field!(before => "before");

    impl_url_bool_field!(
        /// If set to true the sizes of the containers will be returned
        sized => "size"
    );
}

/// Interface for building a new docker container from an existing image
#[derive(Serialize, Debug, Clone)]
pub struct ContainerCreateOpts {
    name: Option<String>,
    params: HashMap<&'static str, Value>,
}

/// Function to insert a JSON value into a tree where the desired
/// location of the value is given as a path of JSON keys.
fn insert<'a, I, V>(key_path: &mut Peekable<I>, value: &V, parent_node: &mut Value)
where
    V: Serialize,
    I: Iterator<Item = &'a str>,
{
    if let Some(local_key) = key_path.next() {
        if key_path.peek().is_some() {
            if let Some(node) = parent_node.as_object_mut() {
                let node = node
                    .entry(local_key.to_string())
                    .or_insert(Value::Object(Map::new()));

                insert(key_path, value, node);
            }
        } else if let Some(node) = parent_node.as_object_mut() {
            node.insert(
                local_key.to_string(),
                serde_json::to_value(value).unwrap_or_default(),
            );
        }
    }
}

impl ContainerCreateOpts {
    /// Returns a builder for creating a new container.
    pub fn builder() -> ContainerCreateOptsBuilder {
        ContainerCreateOptsBuilder::default()
    }

    /// Serialize options as a JSON string.
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.to_json()).map_err(Error::from)
    }

    fn to_json(&self) -> Value {
        let mut body_members = Map::new();
        // The HostConfig element gets initialized to an empty object,
        // for backward compatibility.
        body_members.insert("HostConfig".to_string(), Value::Object(Map::new()));
        let mut body = Value::Object(body_members);
        self.parse_from(&self.params, &mut body);
        body
    }

    fn parse_from<'a, K, V>(&self, params: &'a HashMap<K, V>, body: &mut Value)
    where
        &'a HashMap<K, V>: IntoIterator,
        K: ToString + Eq + Hash,
        V: Serialize,
    {
        for (k, v) in params.iter() {
            let key_string = k.to_string();
            insert(&mut key_string.split('.').peekable(), v, body)
        }
    }

    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[derive(Default)]
pub struct ContainerCreateOptsBuilder {
    name: Option<String>,
    params: HashMap<&'static str, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Network protocol on which a port can be exposed.
pub enum Protocol {
    Tcp,
    Udp,
    Sctp,
}

impl AsRef<str> for Protocol {
    fn as_ref(&self) -> &str {
        match &self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Sctp => "sctp",
        }
    }
}

impl FromStr for Protocol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            "sctp" => Ok(Protocol::Sctp),
            proto => Err(Error::InvalidProtocol(proto.into())),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// Structure used to expose a port on a container with [`expose`](ContainerCreateOptsBuilder::expose) or
/// [`publish`](ContainerCreateOptsBuilder::publish).
pub struct PublishPort {
    port: u32,
    protocol: Protocol,
}

impl PublishPort {
    /// Expose a TCP port.
    pub fn tcp(port: u32) -> Self {
        Self {
            port,
            protocol: Protocol::Tcp,
        }
    }

    /// Expose a UDP port.
    pub fn udp(port: u32) -> Self {
        Self {
            port,
            protocol: Protocol::Udp,
        }
    }

    // Expose a SCTP port.
    pub fn sctp(port: u32) -> Self {
        Self {
            port,
            protocol: Protocol::Sctp,
        }
    }
}

impl FromStr for PublishPort {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut elems = s.split('/');
        let port = elems
            .next()
            .ok_or_else(|| Error::InvalidPort("missing port number".into()))
            .and_then(|port| {
                port.parse::<u32>()
                    .map_err(|e| Error::InvalidPort(format!("expected port number - {e}")))
            })?;

        let protocol = elems
            .next()
            .ok_or_else(|| Error::InvalidPort("missing protocol".into()))
            .and_then(Protocol::from_str)?;

        Ok(PublishPort { port, protocol })
    }
}

impl ToString for PublishPort {
    fn to_string(&self) -> String {
        format!("{}/{}", self.port, self.protocol.as_ref())
    }
}

impl ContainerCreateOptsBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            params: Default::default(),
            name: Some(name.into()),
        }
    }

    /// Set the name of the container.
    pub fn name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// enable all exposed ports on the container to be mapped to random, available, ports on the host
    pub fn publish_all_ports(mut self) -> Self {
        self.params
            .insert("HostConfig.PublishAllPorts", Value::Bool(true));
        self
    }

    pub fn expose(mut self, srcport: PublishPort, hostport: u32) -> Self {
        let mut exposedport: HashMap<String, String> = HashMap::new();
        exposedport.insert("HostPort".to_string(), hostport.to_string());

        // The idea here is to go thought the 'old' port binds and to apply them to the local
        // 'port_bindings' variable, add the bind we want and replace the 'old' value
        let mut port_bindings: HashMap<String, Value> = HashMap::new();
        for (key, val) in self
            .params
            .get("HostConfig.PortBindings")
            .unwrap_or(&json!(null))
            .as_object()
            .unwrap_or(&Map::new())
            .iter()
        {
            port_bindings.insert(key.to_string(), json!(val));
        }
        port_bindings.insert(srcport.to_string(), json!(vec![exposedport]));

        self.params
            .insert("HostConfig.PortBindings", json!(port_bindings));

        // Replicate the port bindings over to the exposed ports config
        let mut exposed_ports: HashMap<String, Value> = HashMap::new();
        let empty_config: HashMap<String, Value> = HashMap::new();
        for key in port_bindings.keys() {
            exposed_ports.insert(key.to_string(), json!(empty_config));
        }

        self.params.insert("ExposedPorts", json!(exposed_ports));

        self
    }

    /// Publish a port in the container without assigning a port on the host
    pub fn publish(mut self, port: PublishPort) -> Self {
        /* The idea here is to go thought the 'old' port binds
         * and to apply them to the local 'exposedport_bindings' variable,
         * add the bind we want and replace the 'old' value */
        let mut exposed_port_bindings: HashMap<String, Value> = HashMap::new();
        for (key, val) in self
            .params
            .get("ExposedPorts")
            .unwrap_or(&json!(null))
            .as_object()
            .unwrap_or(&Map::new())
            .iter()
        {
            exposed_port_bindings.insert(key.to_string(), json!(val));
        }
        exposed_port_bindings.insert(port.to_string(), json!({}));

        // Replicate the port bindings over to the exposed ports config
        let mut exposed_ports: HashMap<String, Value> = HashMap::new();
        let empty_config: HashMap<String, Value> = HashMap::new();
        for key in exposed_port_bindings.keys() {
            exposed_ports.insert(key.to_string(), json!(empty_config));
        }

        self.params.insert("ExposedPorts", json!(exposed_ports));

        self
    }

    impl_str_field!(
        /// Specify the working dir (corresponds to the `-w` docker cli argument)
        working_dir => "WorkingDir"
    );

    impl_str_field!(
        /// The name (or reference) of the image to use when creating the container
        image => "Image"
    );

    impl_vec_field!(
        /// Specify a Vec of string values to customize labels for MLS systems, such as SELinux.
        security_options => "HostConfig.SecurityOpt"
    );

    impl_vec_field!(
        /// Specify any bind mounts, taking the form of `/some/host/path:/some/container/path`
        volumes => "HostConfig.Binds"
    );

    impl_vec_field!(links => "HostConfig.Links");

    impl_field!(memory: u64 => "HostConfig.Memory");

    impl_field!(
        /// Total memory limit (memory + swap) in bytes. Set to -1 (default) to enable unlimited swap.
        memory_swap: i64 => "HostConfig.MemorySwap"
    );

    impl_field!(
        /// CPU quota in units of 10<sup>-9</sup> CPUs. Set to 0 (default) for there to be no limit.
        ///
        /// For example, setting `nano_cpus` to `500_000_000` results in the container being allocated
        /// 50% of a single CPU, while `2_000_000_000` results in the container being allocated 2 CPUs.
        nano_cpus: u64 => "HostConfig.NanoCpus"
    );

    /// CPU quota in units of CPUs. This is a wrapper around `nano_cpus` to do the unit conversion.
    ///
    /// See [`nano_cpus`](#method.nano_cpus).
    pub fn cpus(self, cpus: f64) -> Self {
        self.nano_cpus((1_000_000_000.0 * cpus) as u64)
    }

    impl_field!(
    /// Sets an integer value representing the container's relative CPU weight versus other containers.
    cpu_shares: u32 => "HostConfig.CpuShares");

    impl_map_field!(json labels => "Labels");

    /// Whether to attach to `stdin`.
    pub fn attach_stdin(mut self, attach: bool) -> Self {
        self.params.insert("AttachStdin", json!(attach));
        self.params.insert("OpenStdin", json!(attach));
        self
    }

    impl_field!(
    /// Whether to attach to `stdout`.
    attach_stdout: bool => "AttachStdout");

    impl_field!(
    /// Whether to attach to `stderr`.
    attach_stderr: bool => "AttachStderr");

    impl_field!(
    /// Whether standard streams should be attached to a TTY.
    tty: bool => "Tty");

    impl_vec_field!(extra_hosts => "HostConfig.ExtraHosts");

    impl_vec_field!(volumes_from => "HostConfig.VolumesFrom");

    impl_str_field!(network_mode => "HostConfig.NetworkMode");

    impl_vec_field!(env => "Env");

    impl_vec_field!(command => "Cmd");

    impl_vec_field!(entrypoint => "Entrypoint");

    impl_vec_field!(capabilities => "HostConfig.CapAdd");

    pub fn devices(mut self, devices: Vec<Labels>) -> Self {
        self.params.insert("HostConfig.Devices", json!(devices));
        self
    }

    impl_str_field!(log_driver => "HostConfig.LogConfig.Type");

    pub fn restart_policy(mut self, name: &str, maximum_retry_count: u64) -> Self {
        self.params
            .insert("HostConfig.RestartPolicy.Name", json!(name));
        if name == "on-failure" {
            self.params.insert(
                "HostConfig.RestartPolicy.MaximumRetryCount",
                json!(maximum_retry_count),
            );
        }
        self
    }

    impl_field!(auto_remove: bool => "HostConfig.AutoRemove");

    impl_str_field!(
    /// Signal to stop a container as a string. Default is \"SIGTERM\"
    stop_signal => "StopSignal");

    impl_field!(
    /// Signal to stop a container as an integer. Default is 15 (SIGTERM).
    stop_signal_num: u64 => "StopSignal");

    impl_field!(
    /// Timeout to stop a container. Only seconds are counted. Default is 10s
    stop_timeout: Duration => "StopTimeout");

    impl_str_field!(userns_mode => "HostConfig.UsernsMode");

    impl_field!(privileged: bool => "HostConfig.Privileged");

    impl_str_field!(user => "User");

    pub fn build(&self) -> ContainerCreateOpts {
        ContainerCreateOpts {
            name: self.name.clone(),
            params: self.params.clone(),
        }
    }
}

impl_opts_builder!(url => ContainerRemove);

impl ContainerRemoveOptsBuilder {
    impl_url_bool_field!(
        /// If the container is running, kill it before removing it.
        force => "force"
    );

    impl_url_bool_field!(
        /// Remove anonymous volumes associated with the container.
        volumes => "v"
    );

    impl_url_bool_field!(
        /// Remove the specified link associated with the container.
        link => "link"
    );
}

impl_opts_builder!(url => ContainerPrune);

pub enum ContainerPruneFilter {
    /// Prune containers created before this timestamp. The <timestamp> can be Unix timestamps,
    /// date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m) computed relative to
    /// the daemon machine’s time.
    Until(String),
    #[cfg(feature = "chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
    /// Prune containers created before this timestamp. Same as `Until` but takes a datetime object.
    UntilDate(chrono::DateTime<chrono::Utc>),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
}

impl Filter for ContainerPruneFilter {
    fn query_item(&self) -> FilterItem {
        use ContainerPruneFilter::*;
        match &self {
            Until(until) => FilterItem::new("until", until.to_owned()),
            #[cfg(feature = "chrono")]
            UntilDate(until) => FilterItem::new("until", until.timestamp().to_string()),
            LabelKey(label) => FilterItem::new("label", label.to_owned()),
            Label(key, val) => FilterItem::new("label", format!("{key}={val}")),
        }
    }
}

impl ContainerPruneOptsBuilder {
    impl_filter_func!(ContainerPruneFilter);
}

impl_opts_builder!(url => ContainerCommit);

impl ContainerCommitOpts {
    pub(crate) fn with_container(&self, id: &str) -> Self {
        // not exactly a nice solution but temporary
        let mut s = self.clone();
        s.params.insert("container", id.to_owned());
        s
    }
}

impl ContainerCommitOptsBuilder {
    impl_url_str_field!(
        /// Repository name for the created image
        repo => "repo"
    );
    impl_url_str_field!(
        /// Tag name for the created image
        tag => "tag"
    );
    impl_url_str_field!(
        /// Commit message
        comment => "comment"
    );
    impl_url_str_field!(
        /// Author of the image (e.g., John Hannibal Smith <hannibal@a-team.com>)
        author => "author"
    );
    impl_url_bool_field!(
        /// Whether to pause the container before committing
        pause => "pause"
    );
    impl_url_str_field!(
        /// Dockerfile instructions to apply while committing
        changes => "changes"
    );
}

impl_opts_builder!(url => ContainerStop);

impl ContainerStopOptsBuilder {
    impl_url_str_field!(
        /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
        signal => "signal"
    );

    /// Duration to wait before stopping the container
    pub fn wait(mut self, duration: Duration) -> Self {
        self.params.insert("t", duration.as_secs().to_string());
        self
    }
}

impl_opts_builder!(url => ContainerRestart);

impl ContainerRestartOptsBuilder {
    impl_url_str_field!(
        /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
        signal => "signal"
    );

    /// Duration to wait before restarting the container
    pub fn wait(mut self, duration: Duration) -> Self {
        self.params.insert("t", duration.as_secs().to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_case {
        ($opts:expr, $want:expr) => {
            let opts = $opts.build();

            pretty_assertions::assert_eq!($want, opts.serialize().unwrap())
        };
    }

    #[test]
    fn create_container_opts() {
        test_case!(
            ContainerCreateOptsBuilder::default().image("test_image"),
            r#"{"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .env(vec!["foo", "bar"]),
            r#"{"Env":["foo","bar"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .env(["foo", "bar", "baz"]),
            r#"{"Env":["foo","bar","baz"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .env(std::iter::once("test")),
            r#"{"Env":["test"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .user("alice"),
            r#"{"HostConfig":{},"Image":"test_image","User":"alice"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .network_mode("host")
                .auto_remove(true)
                .privileged(true),
            r#"{"HostConfig":{"AutoRemove":true,"NetworkMode":"host","Privileged":true},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .expose(PublishPort::tcp(80), 8080),
            r#"{"ExposedPorts":{"80/tcp":{}},"HostConfig":{"PortBindings":{"80/tcp":[{"HostPort":"8080"}]}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .expose(PublishPort::udp(80), 8080)
                .expose(PublishPort::sctp(81), 8081),
            r#"{"ExposedPorts":{"80/udp":{},"81/sctp":{}},"HostConfig":{"PortBindings":{"80/udp":[{"HostPort":"8080"}],"81/sctp":[{"HostPort":"8081"}]}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .publish(PublishPort::udp(80))
                .publish(PublishPort::sctp(6969))
                .publish(PublishPort::tcp(1337)),
            r#"{"ExposedPorts":{"1337/tcp":{},"6969/sctp":{},"80/udp":{}},"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .publish_all_ports(),
            r#"{"HostConfig":{"PublishAllPorts":true},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .log_driver("fluentd"),
            r#"{"HostConfig":{"LogConfig":{"Type":"fluentd"}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .restart_policy("on-failure", 10),
            r#"{"HostConfig":{"RestartPolicy":{"MaximumRetryCount":10,"Name":"on-failure"}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerCreateOptsBuilder::default()
                .image("test_image")
                .restart_policy("always", 0),
            r#"{"HostConfig":{"RestartPolicy":{"Name":"always"}},"Image":"test_image"}"#
        );
    }
}
