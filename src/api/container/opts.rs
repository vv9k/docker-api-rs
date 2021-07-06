use crate::api::{Filter, ImageName, Labels};

use std::{
    collections::HashMap, hash::Hash, iter::Peekable, str, string::ToString, time::Duration,
};

use serde::Serialize;
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

#[cfg(windows)]
pub enum Isolation {
    Default,
    Process,
    HyperV,
}

#[cfg(windows)]
impl AsRef<str> for Isolation {
    fn as_ref(&self) -> &str {
        match &self {
            Isolation::Default => "default",
            Isolation::Process => "process",
            Isolation::HyperV => "hyperv",
        }
    }
}

pub enum ContainerStatusEnum {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl AsRef<str> for ContainerStatusEnum {
    fn as_ref(&self) -> &str {
        use ContainerStatusEnum::*;
        match &self {
            Created => "created",
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
    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    /// Applies only to Windows daemon.
    Isolation(Isolation),
    IsTask(bool),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
    /// The container's name.
    Name(String),
    // TODO: ContainerFilter::Publish
    /// Network ID or name.
    Network(String),
    /// Container ID or name.
    Since(String),
    Status(ContainerStatusEnum),
    /// Volume name or mount point destination.
    Volume(String),
}

impl Filter for ContainerFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ContainerFilter::*;
        match &self {
            Ancestor(name) => ("ancestor", name.to_string()),
            Before(before) => ("before", before.to_owned()),
            ExitCode(c) => ("exit", c.to_string()),
            Health(health) => ("health", health.as_ref().to_string()),
            Id(id) => ("id", id.to_owned()),
            #[cfg(windows)]
            Isolation(isolation) => ("isolation", isolation.as_ref().to_string()),
            IsTask(is_task) => ("is-task", is_task.to_string()),
            LabelKey(key) => ("label", key.to_owned()),
            Label(key, val) => ("label", format!("{}={}", key, val)),
            Name(name) => ("name", name.to_owned()),
            Network(net) => ("net", net.to_owned()),
            Since(since) => ("since", since.to_owned()),
            Status(s) => ("status", s.as_ref().to_string()),
            Volume(vol) => ("volume", vol.to_owned()),
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

    impl_url_str_field!(since: S => "since");

    impl_url_str_field!(before: B => "before");

    impl_url_bool_field!(
        /// If set to true the sizes of the containers will be returned
        sized => "size"
    );
}

/// Interface for building a new docker container from an existing image
#[derive(Serialize, Debug)]
pub struct ContainerCreateOpts {
    pub name: Option<String>,
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
    pub fn builder<N>(name: N) -> ContainerOptsBuilder
    where
        N: AsRef<str>,
    {
        ContainerOptsBuilder::new(name.as_ref())
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
}

#[derive(Default)]
pub struct ContainerOptsBuilder {
    name: Option<String>,
    params: HashMap<&'static str, Value>,
}

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

/// Structure used to expose a port on a container with [`expose`](ContainerOptsBuilder::expose) or
/// [`publish`](ContainerOptsBuilder::publish).
pub struct PublishPort {
    protocol: Protocol,
    port: u32,
}

impl PublishPort {
    /// Expose a TCP port.
    pub fn tcp(port: u32) -> Self {
        Self {
            protocol: Protocol::Tcp,
            port,
        }
    }

    /// Expose a UDP port.
    pub fn udp(port: u32) -> Self {
        Self {
            protocol: Protocol::Udp,
            port,
        }
    }

    // Expose a SCTP port.
    pub fn sctp(port: u32) -> Self {
        Self {
            protocol: Protocol::Sctp,
            port,
        }
    }
}

impl ToString for PublishPort {
    fn to_string(&self) -> String {
        format!("{}/{}", self.port, self.protocol.as_ref())
    }
}

impl ContainerOptsBuilder {
    pub(crate) fn new(image: &str) -> Self {
        let mut params = HashMap::new();

        params.insert("Image", Value::String(image.to_owned()));
        ContainerOptsBuilder { name: None, params }
    }

    /// Set the name of the container.
    pub fn name<N>(&mut self, name: N) -> &mut Self
    where
        N: Into<String>,
    {
        self.name = Some(name.into());
        self
    }

    /// enable all exposed ports on the container to be mapped to random, available, ports on the host
    pub fn publish_all_ports(&mut self) -> &mut Self {
        self.params
            .insert("HostConfig.PublishAllPorts", Value::Bool(true));
        self
    }

    pub fn expose(&mut self, srcport: PublishPort, hostport: u32) -> &mut Self {
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
    pub fn publish(&mut self, port: PublishPort) -> &mut Self {
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
        working_dir: W => "WorkingDir"
    );

    impl_vec_field!(
        /// Specify any bind mounts, taking the form of `/some/host/path:/some/container/path`
        volumes: V => "HostConfig.Binds"
    );

    impl_vec_field!(links: L => "HostConfig.Links");

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
    pub fn cpus(&mut self, cpus: f64) -> &mut Self {
        self.nano_cpus((1_000_000_000.0 * cpus) as u64)
    }

    impl_field!(
    /// Sets an integer value representing the container's relative CPU weight versus other containers.
    cpu_shares: u32 => "HostConfig.CpuShares");

    impl_map_field!(json labels: L => "Labels");

    /// Whether to attach to `stdin`.
    pub fn attach_stdin(&mut self, attach: bool) -> &mut Self {
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

    impl_vec_field!(extra_hosts: H => "HostConfig.ExtraHosts");

    impl_vec_field!(volumes_from: V => "HostConfig.VolumesFrom");

    impl_str_field!(network_mode: M => "HostConfig.NetworkMode");

    impl_vec_field!(env: E => "Env");

    impl_vec_field!(cmd: C => "Cmd");

    impl_vec_field!(entrypoint: E => "Entrypoint");

    impl_vec_field!(capabilities: C => "HostConfig.CapAdd");

    pub fn devices(&mut self, devices: Vec<Labels>) -> &mut Self {
        self.params.insert("HostConfig.Devices", json!(devices));
        self
    }

    impl_str_field!(log_driver: L => "HostConfig.LogConfig.Type");

    pub fn restart_policy(&mut self, name: &str, maximum_retry_count: u64) -> &mut Self {
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
    stop_signal: S => "StopSignal");

    impl_field!(
    /// Signal to stop a container as an integer. Default is 15 (SIGTERM).
    stop_signal_num: u64 => "StopSignal");

    impl_field!(
    /// Timeout to stop a container. Only seconds are counted. Default is 10s
    stop_timeout: Duration => "StopTimeout");

    impl_str_field!(userns_mode: M => "HostConfig.UsernsMode");

    impl_field!(privileged: bool => "HostConfig.Privileged");

    impl_str_field!(user: U => "User");

    pub fn build(&self) -> ContainerCreateOpts {
        ContainerCreateOpts {
            name: self.name.clone(),
            params: self.params.clone(),
        }
    }
}

impl_opts_builder!(url => RmContainer);

impl RmContainerOptsBuilder {
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
    fn query_key_val(&self) -> (&'static str, String) {
        use ContainerPruneFilter::*;
        match &self {
            Until(until) => ("until", until.to_owned()),
            #[cfg(feature = "chrono")]
            UntilDate(until) => ("until", until.timestamp().to_string()),
            LabelKey(label) => ("label", label.to_owned()),
            Label(key, val) => ("label", format!("{}={}", key, val)),
        }
    }
}

impl ContainerPruneOptsBuilder {
    impl_filter_func!(ContainerPruneFilter);
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
            ContainerOptsBuilder::new("test_image"),
            r#"{"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").env(vec!["foo", "bar"]),
            r#"{"Env":["foo","bar"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").env(&["foo", "bar", "baz"]),
            r#"{"Env":["foo","bar","baz"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").env(std::iter::once("test")),
            r#"{"Env":["test"],"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").user("alice"),
            r#"{"HostConfig":{},"Image":"test_image","User":"alice"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image")
                .network_mode("host")
                .auto_remove(true)
                .privileged(true),
            r#"{"HostConfig":{"AutoRemove":true,"NetworkMode":"host","Privileged":true},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").expose(PublishPort::tcp(80), 8080),
            r#"{"ExposedPorts":{"80/tcp":{}},"HostConfig":{"PortBindings":{"80/tcp":[{"HostPort":"8080"}]}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image")
                .expose(PublishPort::udp(80), 8080)
                .expose(PublishPort::sctp(81), 8081),
            r#"{"ExposedPorts":{"80/udp":{},"81/sctp":{}},"HostConfig":{"PortBindings":{"80/udp":[{"HostPort":"8080"}],"81/sctp":[{"HostPort":"8081"}]}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image")
                .publish(PublishPort::udp(80))
                .publish(PublishPort::sctp(6969))
                .publish(PublishPort::tcp(1337)),
            r#"{"ExposedPorts":{"1337/tcp":{},"6969/sctp":{},"80/udp":{}},"HostConfig":{},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").publish_all_ports(),
            r#"{"HostConfig":{"PublishAllPorts":true},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").log_driver("fluentd"),
            r#"{"HostConfig":{"LogConfig":{"Type":"fluentd"}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").restart_policy("on-failure", 10),
            r#"{"HostConfig":{"RestartPolicy":{"MaximumRetryCount":10,"Name":"on-failure"}},"Image":"test_image"}"#
        );

        test_case!(
            ContainerOptsBuilder::new("test_image").restart_policy("always", 0),
            r#"{"HostConfig":{"RestartPolicy":{"Name":"always"}},"Image":"test_image"}"#
        );
    }
}
