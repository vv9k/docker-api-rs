//! Create and manage containers.
//!
//! API Reference: <https://docs.docker.com/engine/api/v1.41/#tag/Container>

use std::{collections::HashMap, hash::Hash, io, iter::Peekable, path::Path, str, time::Duration};

use futures_util::{
    io::{AsyncRead, AsyncWrite},
    stream::Stream,
    TryStreamExt,
};
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::{
    api::{
        exec::{Exec, ExecContainerOpts},
        image::ContainerConfig,
        network::NetworkSettings,
    },
    conn::{tty, Multiplexer as TtyMultiPlexer, Payload, TtyChunk},
    docker::Docker,
    errors::{Error, Result},
    util::url::encoded_pair,
};

#[cfg(feature = "chrono")]
use crate::util::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

impl_api_ty!(Container => id: I);

impl<'docker> Container<'docker> {
    /// Inspects the current docker container instance's details
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerInspect>
    pub async fn inspect(&self) -> Result<ContainerDetails> {
        self.docker
            .get_json::<ContainerDetails>(&format!("/containers/{}/json", self.id)[..])
            .await
    }

    /// Returns a `top` view of information about the container process.
    /// On Unix systems, this is done by running the ps command. This endpoint is not supported on Windows.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerTop>
    pub async fn top(&self, psargs: Option<&str>) -> Result<Top> {
        let mut path = vec![format!("/containers/{}/top", self.id)];
        if let Some(ref args) = psargs {
            let encoded = encoded_pair("ps_args", args);
            path.push(encoded)
        }
        self.docker.get_json(&path.join("?")).await
    }

    /// Returns a stream of logs emitted but the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerLogs>
    pub fn logs(&self, opts: &LogsOpts) -> impl Stream<Item = Result<TtyChunk>> + Unpin + 'docker {
        let mut path = vec![format!("/containers/{}/logs", self.id)];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }

        let stream = Box::pin(self.docker.stream_get(path.join("?")));

        Box::pin(tty::decode(stream))
    }

    /// Attaches a multiplexed TCP stream to the container that can be used to read Stdout, Stderr and write Stdin.
    async fn attach_raw(&self) -> Result<impl AsyncRead + AsyncWrite + Send + 'docker> {
        self.docker
            .stream_post_upgrade(
                format!(
                    "/containers/{}/attach?stream=1&stdout=1&stderr=1&stdin=1",
                    self.id
                ),
                Payload::empty(),
            )
            .await
    }

    /// Attaches a `[TtyMultiplexer]` to the container.
    ///
    /// The `[TtyMultiplexer]` implements Stream for returning Stdout and Stderr chunks. It also implements `[AsyncWrite]` for writing to Stdin.
    ///
    /// The multiplexer can be split into its read and write halves with the `[split](TtyMultiplexer::split)` method
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerAttach>
    pub async fn attach(&self) -> Result<TtyMultiPlexer<'docker>> {
        self.attach_raw().await.map(TtyMultiPlexer::new)
    }

    /// Returns a set of changes made to the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerChanges>
    pub async fn changes(&self) -> Result<Vec<Change>> {
        self.docker
            .get_json::<Vec<Change>>(&format!("/containers/{}/changes", self.id)[..])
            .await
    }

    /// Exports the current docker container into a tarball
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerExport>
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        self.docker
            .stream_get(format!("/containers/{}/export", self.id))
            .map_ok(|c| c.to_vec())
    }

    /// Returns a stream of stats specific to this container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerStats>
    pub fn stats(&self) -> impl Stream<Item = Result<Stats>> + Unpin + 'docker {
        let codec = futures_codec::LinesCodec {};

        let reader = Box::pin(
            self.docker
                .stream_get(format!("/containers/{}/stats", self.id))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            futures_codec::FramedRead::new(reader, codec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }

    /// Start the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerStart>
    pub async fn start(&self) -> Result<()> {
        self.docker
            .post(
                &format!("/containers/{}/start", self.id)[..],
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Stop the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerStop>
    pub async fn stop(&self, wait: Option<Duration>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/stop", self.id)];
        if let Some(w) = wait {
            let encoded = encoded_pair("t", w.as_secs());

            path.push(encoded)
        }
        self.docker
            .post(&path.join("?"), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Restart the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerRestart>
    pub async fn restart(&self, wait: Option<Duration>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/restart", self.id)];
        if let Some(w) = wait {
            let encoded = encoded_pair("t", w.as_secs());
            path.push(encoded)
        }
        self.docker
            .post(&path.join("?"), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Kill the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerKill>
    pub async fn kill(&self, signal: Option<&str>) -> Result<()> {
        let mut path = vec![format!("/containers/{}/kill", self.id)];
        if let Some(sig) = signal {
            let encoded = encoded_pair("signal", sig);
            path.push(encoded)
        }
        self.docker
            .post(&path.join("?"), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Rename the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerRename>
    pub async fn rename(&self, name: &str) -> Result<()> {
        let query = encoded_pair("name", name);
        self.docker
            .post(
                &format!("/containers/{}/rename?{}", self.id, query)[..],
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Pause the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerPause>
    pub async fn pause(&self) -> Result<()> {
        self.docker
            .post(
                &format!("/containers/{}/pause", self.id)[..],
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Unpause the container instance
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerUnpause>
    pub async fn unpause(&self) -> Result<()> {
        self.docker
            .post(
                &format!("/containers/{}/unpause", self.id)[..],
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Wait until the container stops
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerWait>
    pub async fn wait(&self) -> Result<Exit> {
        self.docker
            .post_json(format!("/containers/{}/wait", self.id), Payload::empty())
            .await
    }

    /// Delete the container instance
    ///
    /// Use remove instead to use the force/v Opts.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/containers/{}", self.id)[..])
            .await
            .map(|_| ())
    }

    /// Delete the container instance (todo: force/v)
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerRemove>
    pub async fn remove(&self, opts: &RmContainerOpts) -> Result<()> {
        let mut path = vec![format!("/containers/{}", self.id)];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }
        self.docker.delete(&path.join("?")).await.map(|_| ())
    }

    /// Execute a command in this container
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#tag/Exec>
    pub fn exec(
        &'docker self,
        opts: &ExecContainerOpts,
    ) -> impl Stream<Item = Result<tty::TtyChunk>> + Unpin + 'docker {
        Exec::create_and_start(self.docker, &self.id, opts)
    }

    /// Copy a file/folder from the container.  The resulting stream is a tarball of the extracted
    /// files.
    ///
    /// If `path` is not an absolute path, it is relative to the container’s root directory. The
    /// resource specified by `path` must exist. To assert that the resource is expected to be a
    /// directory, `path` should end in `/` or `/`. (assuming a path separator of `/`). If `path`
    /// ends in `/.`  then this indicates that only the contents of the path directory should be
    /// copied.  A symlink is always resolved to its target.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerArchive>
    pub fn copy_from(&self, path: &Path) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        let path_arg = encoded_pair("path", path.to_string_lossy());

        let endpoint = format!("/containers/{}/archive?{}", self.id, path_arg);
        self.docker.stream_get(endpoint).map_ok(|c| c.to_vec())
    }

    /// Copy a byte slice as file into (see `bytes`) the container.
    ///
    /// The file will be copied at the given location (see `path`) and will be owned by root
    /// with access mask 644.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PutContainerArchive>
    pub async fn copy_file_into<P: AsRef<Path>>(&self, path: P, bytes: &[u8]) -> Result<()> {
        let path = path.as_ref();

        let mut ar = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o0644);
        ar.append_data(
            &mut header,
            path.to_path_buf()
                .iter()
                .skip(1)
                .collect::<std::path::PathBuf>(),
            bytes,
        )?;
        let data = ar.into_inner()?;

        self.copy_to(Path::new("/"), data.into()).await.map(|_| ())
    }

    /// Copy a tarball (see `body`) to the container.
    ///
    /// The tarball will be copied to the container and extracted at the given location (see `path`).
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PutContainerArchive>
    pub async fn copy_to(&self, path: &Path, body: Body) -> Result<()> {
        let path_arg = encoded_pair("path", path.to_string_lossy());

        self.docker
            .put(
                &format!("/containers/{}/archive?{}", self.id, path_arg),
                Payload::XTar(body),
            )
            .await
            .map(|_| ())
    }

    /// Get information about files in a container.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerArchiveInfo>
    pub async fn stat_file<P>(&self, path: P) -> Result<String>
    where
        P: AsRef<Path>,
    {
        static PATH_STAT_HEADER: &str = "X-Docker-Container-Path-Stat";
        let path_arg = encoded_pair("path", path.as_ref().to_string_lossy());

        let resp = self
            .docker
            .head_response(&format!("/containers/{}/archive?{}", self.id, path_arg))
            .await?;
        if let Some(header) = resp.headers().get(PATH_STAT_HEADER) {
            let header = header.to_str().map_err(|e| {
                Error::InvalidResponse(format!("response header was invalid - {}", e))
            })?;

            base64::decode(header)
                .map_err(|e| {
                    Error::InvalidResponse(format!("expected header to be valid base64 - {}", e))
                })
                .and_then(|s| {
                    str::from_utf8(s.as_slice())
                        .map(str::to_string)
                        .map_err(|e| {
                            Error::InvalidResponse(format!(
                                "expected header to be valid utf8 - {}",
                                e
                            ))
                        })
                })
        } else {
            Err(Error::InvalidResponse(format!(
                "missing `{}` header",
                PATH_STAT_HEADER
            )))
        }
    }
}

impl<'docker> Containers<'docker> {
    /// Lists the container instances on the docker host
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ContainerList>
    pub async fn list(&self, opts: &ContainerListOpts) -> Result<Vec<ContainerInfo>> {
        let mut path = vec!["/containers/json".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }
        self.docker
            .get_json::<Vec<ContainerInfo>>(&path.join("?"))
            .await
    }

    /// Returns a builder interface for creating a new container instance
    pub async fn create(&self, opts: &ContainerOpts) -> Result<ContainerCreateInfo> {
        let body: Body = opts.serialize()?.into();
        let mut path = vec!["/containers/create".to_owned()];

        if let Some(ref name) = opts.name {
            path.push(encoded_pair("name", name));
        }

        self.docker
            .post_json(&path.join("?"), Payload::Json(body))
            .await
    }
}

/// Filter Opts for container listings
pub enum ContainerFilter {
    ExitCode(u64),
    Status(String),
    LabelName(String),
    Label(String, String),
}

impl_url_opts_builder!(ContainerList);

impl ContainerListOptsBuilder {
    pub fn filter<F>(&mut self, filters: F) -> &mut Self
    where
        F: IntoIterator<Item = ContainerFilter>,
    {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                ContainerFilter::ExitCode(c) => param.insert("exit", vec![c.to_string()]),
                ContainerFilter::Status(s) => param.insert("status", vec![s]),
                ContainerFilter::LabelName(n) => param.insert("label", vec![n]),
                ContainerFilter::Label(n, v) => param.insert("label", vec![format!("{}={}", n, v)]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.insert("all", "true".to_owned());
        self
    }

    pub fn since<S>(&mut self, since: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.params.insert("since", since.into());
        self
    }

    pub fn before<B>(&mut self, before: B) -> &mut Self
    where
        B: Into<String>,
    {
        self.params.insert("before", before.into());
        self
    }

    pub fn sized(&mut self) -> &mut Self {
        self.params.insert("size", "true".to_owned());
        self
    }
}

/// Interface for building a new docker container from an existing image
#[derive(Serialize, Debug)]
pub struct ContainerOpts {
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

impl ContainerOpts {
    /// return a new instance of a builder for Opts
    pub fn builder<N>(name: N) -> ContainerOptsBuilder
    where
        N: AsRef<str>,
    {
        ContainerOptsBuilder::new(name.as_ref())
    }

    /// serialize Opts as a string. returns None if no Opts are defined
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

    pub fn parse_from<'a, K, V>(&self, params: &'a HashMap<K, V>, body: &mut Value)
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

impl ContainerOptsBuilder {
    pub(crate) fn new(image: &str) -> Self {
        let mut params = HashMap::new();

        params.insert("Image", Value::String(image.to_owned()));
        ContainerOptsBuilder { name: None, params }
    }

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

    pub fn expose<P>(&mut self, srcport: u32, protocol: P, hostport: u32) -> &mut Self
    where
        P: AsRef<str>,
    {
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
        port_bindings.insert(
            format!("{}/{}", srcport, protocol.as_ref()),
            json!(vec![exposedport]),
        );

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
    pub fn publish<P>(&mut self, srcport: u32, protocol: P) -> &mut Self
    where
        P: AsRef<str>,
    {
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
        exposed_port_bindings.insert(format!("{}/{}", srcport, protocol.as_ref()), json!({}));

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
    "Specify the working dir (corresponds to the `-w` docker cli argument)"
    working_dir: W => "WorkingDir");

    impl_vec_field!(
        "Specify any bind mounts, taking the form of `/some/host/path:/some/container/path`"
        volumes: V => "HostConfig.Binds"
    );

    impl_vec_field!(links: L => "HostConfig.Links");

    impl_field!(memory: u64 => "HostConfig.Memory");

    impl_field!(
    "Total memory limit (memory + swap) in bytes. Set to -1 (default) to enable unlimited swap."
    memory_swap: i64 => "HostConfig.MemorySwap");

    impl_field!(
    "CPU quota in units of 10<sup>-9</sup> CPUs. Set to 0 (default) for there to be no limit."
    ""
    "For example, setting `nano_cpus` to `500_000_000` results in the container being allocated"
    "50% of a single CPU, while `2_000_000_000` results in the container being allocated 2 CPUs."
    nano_cpus: u64 => "HostConfig.NanoCpus");

    /// CPU quota in units of CPUs. This is a wrapper around `nano_cpus` to do the unit conversion.
    ///
    /// See [`nano_cpus`](#method.nano_cpus).
    pub fn cpus(&mut self, cpus: f64) -> &mut Self {
        self.nano_cpus((1_000_000_000.0 * cpus) as u64)
    }

    impl_field!(
    "Sets an integer value representing the container's relative CPU weight versus other"
    "containers."
    cpu_shares: u32 => "HostConfig.CpuShares");

    impl_map_field!(labels: L => "Labels");

    /// Whether to attach to `stdin`.
    pub fn attach_stdin(&mut self, attach: bool) -> &mut Self {
        self.params.insert("AttachStdin", json!(attach));
        self.params.insert("OpenStdin", json!(attach));
        self
    }

    impl_field!(
    "Whether to attach to `stdout`."
    attach_stdout: bool => "AttachStdout");

    impl_field!(
    "Whether to attach to `stderr`."
    attach_stderr: bool => "AttachStderr");

    impl_field!(
    "Whether standard streams should be attached to a TTY."
    tty: bool => "Tty");

    impl_vec_field!(extra_hosts: H => "HostConfig.ExtraHosts");

    impl_vec_field!(volumes_from: V => "HostConfig.VolumesFrom");

    impl_str_field!(network_mode: M => "HostConfig.NetworkMode");

    impl_vec_field!(env: E => "Env");

    impl_vec_field!(cmd: C => "Cmd");

    impl_vec_field!(entrypoint: E => "Entrypoint");

    impl_vec_field!(capabilities: C => "HostConfig.CapAdd");

    pub fn devices(&mut self, devices: Vec<HashMap<String, String>>) -> &mut Self {
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
    "Signal to stop a container as a string. Default is \"SIGTERM\""
    stop_signal: S => "StopSignal");

    impl_field!(
    "Signal to stop a container as an integer. Default is 15 (SIGTERM)."
    stop_signal_num: u64 => "StopSignal");

    impl_field!(
    "Timeout to stop a container. Only seconds are counted. Default is 10s"
    stop_timeout: Duration => "StopTimeout");

    impl_str_field!(userns_mode: M => "HostConfig.UsernsMode");

    impl_field!(privileged: bool => "HostConfig.Privileged");

    impl_str_field!(user: U => "User");

    pub fn build(&self) -> ContainerOpts {
        ContainerOpts {
            name: self.name.clone(),
            params: self.params.clone(),
        }
    }
}

impl_url_opts_builder!(Logs);

impl LogsOptsBuilder {
    pub fn follow(&mut self, f: bool) -> &mut Self {
        self.params.insert("follow", f.to_string());
        self
    }

    pub fn stdout(&mut self, s: bool) -> &mut Self {
        self.params.insert("stdout", s.to_string());
        self
    }

    pub fn stderr(&mut self, s: bool) -> &mut Self {
        self.params.insert("stderr", s.to_string());
        self
    }

    pub fn timestamps(&mut self, t: bool) -> &mut Self {
        self.params.insert("timestamps", t.to_string());
        self
    }

    /// how_many can either be "all" or a to_string() of the number
    pub fn tail(&mut self, how_many: &str) -> &mut Self {
        self.params.insert("tail", how_many.to_owned());
        self
    }

    #[cfg(feature = "chrono")]
    pub fn since<Tz>(&mut self, timestamp: &chrono::DateTime<Tz>) -> &mut Self
    where
        Tz: chrono::TimeZone,
    {
        self.params
            .insert("since", timestamp.timestamp().to_string());
        self
    }

    #[cfg(not(feature = "chrono"))]
    pub fn since(&mut self, timestamp: i64) -> &mut Self {
        self.params.insert("since", timestamp.to_string());
        self
    }
}

impl_url_opts_builder!(RmContainer);

impl RmContainerOptsBuilder {
    pub fn force(&mut self, f: bool) -> &mut Self {
        self.params.insert("force", f.to_string());
        self
    }

    pub fn volumes(&mut self, s: bool) -> &mut Self {
        self.params.insert("v", s.to_string());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub command: String,
    pub id: String,
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    pub labels: HashMap<String, String>,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    pub state: String,
    pub status: String,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerDetails {
    pub id: String,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub path: String,
    pub args: Vec<String>,
    pub state: State,
    pub image: String,
    pub resolv_conf_path: String,
    pub hostname_path: String,
    pub hosts_path: String,
    pub log_path: String,
    pub name: String,
    pub restart_count: i64,
    pub driver: String,
    pub platform: String,
    pub mount_label: String,
    pub process_label: String,
    pub app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,
    pub host_config: HostConfig,
    pub graph_driver: GraphDriverData,
    pub mounts: Vec<Mount>,
    pub config: ContainerConfig,
    pub network_settings: NetworkSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GraphDriverData {
    pub name: String,
    pub data: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub error: String,
    pub exit_code: u64,
    #[cfg(feature = "chrono")]
    pub finished_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub finished_at: String,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub paused: bool,
    pub pid: u64,
    pub restarting: bool,
    pub running: bool,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    pub status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub cpu_shares: Option<i64>,
    pub memory: Option<i64>,
    pub cgroup_parent: Option<String>,
    pub blkio_weight_device: Option<Vec<ThrottleDevice>>,
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,
    pub cpu_period: Option<i64>,
    pub cpu_quota: Option<i64>,
    pub cpu_realtime_period: Option<i64>,
    pub cpu_realtime_runtime: Option<i64>,
    pub cpuset_cpus: Option<String>,
    pub cpuset_mems: Option<String>,
    pub devices: Option<Vec<DeviceMapping>>,
    pub device_cgroup_rules: Option<String>,
    pub device_requests: Option<Vec<DeviceRequest>>,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: i64,
    pub memory_reservation: Option<i64>,
    pub memory_swap: Option<i64>,
    pub memory_swappiness: Option<i64>,
    #[serde(rename = "NanoCPUs")]
    pub nano_cpus: Option<i64>,
    pub oom_kill_disable: Option<bool>,
    pub init: Option<bool>,
    pub pids_limit: Option<i64>,
    pub ulimits: Option<Vec<Ulimit>>,
    pub cpu_count: i64,
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: u64,
    #[serde(rename = "IOMaximumBandwith")]
    pub io_maximum_bandwith: Option<u64>,
    pub binds: Option<Vec<String>>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: String,
    pub log_config: LogConfig,
    pub network_mode: String,
    pub port_bindings: Option<PortMap>,
    pub restart_policy: RestartPolicy,
    pub auto_remove: bool,
    pub volume_driver: String,
    pub volumes_from: Option<Vec<String>>,
    pub mounts: Option<Vec<Mount>>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub dns: Option<Vec<String>>,
    pub dns_options: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub extra_hosts: Option<Vec<String>>,
    pub group_add: Option<Vec<String>>,
    pub ipc_mode: String,
    pub cgroup: String,
    pub links: Option<Vec<String>>,
    pub oom_score_adj: i64,
    pub pid_mode: Option<String>,
    pub privileged: bool,
    pub publish_all_ports: bool,
    pub readonly_rootfs: Option<bool>,
    pub security_opt: Option<Vec<String>>,
    pub storage_opt: Option<HashMap<String, String>>,
    pub tmpfs: Option<HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: String,
    pub userns_mode: String,
    pub shm_size: u64,
    pub sysctls: Option<HashMap<String, String>>,
    pub runtime: String,
    pub console_size: Option<Vec<u64>>,
    pub isolation: String,
    pub masked_paths: Option<Vec<String>>,
    pub readonly_paths: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrottleDevice {
    pub path: String,
    pub rate: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub name: String,
    pub maximum_retry_count: u64,
}

pub type PortMap = HashMap<String, Option<Vec<PortBinding>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "Config")]
    pub config: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: String,
    pub soft: u64,
    pub hard: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
    pub path_on_host: Option<String>,
    pub path_in_container: Option<String>,
    pub cgroup_permissions: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: String,
    pub count: u64,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    pub capabilities: Vec<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub read: String,
    pub networks: HashMap<String, NetworkStats>,
    pub memory_stats: MemoryStats,
    pub blkio_stats: BlkioStats,
    pub cpu_stats: CpuStats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkStats {
    pub rx_dropped: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
    pub tx_packets: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub tx_errors: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStats {
    pub max_usage: u64,
    pub usage: u64,
    pub failcnt: Option<u64>,
    pub limit: u64,
    pub stats: MemoryStat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryStat {
    pub total_pgmajfault: u64,
    pub cache: u64,
    pub mapped_file: u64,
    pub total_inactive_file: u64,
    pub pgpgout: u64,
    pub rss: u64,
    pub total_mapped_file: u64,
    pub writeback: u64,
    pub unevictable: u64,
    pub pgpgin: u64,
    pub total_unevictable: u64,
    pub pgmajfault: u64,
    pub total_rss: u64,
    pub total_rss_huge: u64,
    pub total_writeback: u64,
    pub total_inactive_anon: u64,
    pub rss_huge: u64,
    pub hierarchical_memory_limit: u64,
    pub hierarchical_memsw_limit: u64,
    pub total_pgfault: u64,
    pub total_active_file: u64,
    pub active_anon: u64,
    pub total_active_anon: u64,
    pub total_pgpgout: u64,
    pub total_cache: u64,
    pub inactive_anon: u64,
    pub active_file: u64,
    pub pgfault: u64,
    pub inactive_file: u64,
    pub total_pgpgin: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuStats {
    pub cpu_usage: CpuUsage,
    pub system_cpu_usage: u64,
    pub throttling_data: ThrottlingData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    pub percpu_usage: Vec<u64>,
    pub usage_in_usermode: u64,
    pub total_usage: u64,
    pub usage_in_kernelmode: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThrottlingData {
    pub periods: u64,
    pub throttled_periods: u64,
    pub throttled_time: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStats {
    pub io_service_bytes_recursive: Vec<BlkioStat>,
    pub io_serviced_recursive: Vec<BlkioStat>,
    pub io_queue_recursive: Vec<BlkioStat>,
    pub io_service_time_recursive: Vec<BlkioStat>,
    pub io_wait_time_recursive: Vec<BlkioStat>,
    pub io_merged_recursive: Vec<BlkioStat>,
    pub io_time_recursive: Vec<BlkioStat>,
    pub sectors_recursive: Vec<BlkioStat>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlkioStat {
    pub major: u64,
    pub minor: u64,
    pub op: String,
    pub value: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Change {
    pub kind: u8,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Top {
    pub titles: Vec<String>,
    pub processes: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerCreateInfo {
    pub id: String,
    pub warnings: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Exit {
    pub status_code: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_options_simple() {
        let builder = ContainerOptsBuilder::new("test_image");
        let options = builder.build();

        assert_eq!(
            r#"{"HostConfig":{},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_env() {
        let options = ContainerOptsBuilder::new("test_image")
            .env(vec!["foo", "bar"])
            .build();

        assert_eq!(
            r#"{"Env":["foo","bar"],"HostConfig":{},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_env_dynamic() {
        let env: Vec<String> = ["foo", "bar", "baz"]
            .iter()
            .map(|s| String::from(*s))
            .collect();

        let options = ContainerOptsBuilder::new("test_image").env(&env).build();

        assert_eq!(
            r#"{"Env":["foo","bar","baz"],"HostConfig":{},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_user() {
        let options = ContainerOptsBuilder::new("test_image")
            .user("alice")
            .build();

        assert_eq!(
            r#"{"HostConfig":{},"Image":"test_image","User":"alice"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_host_config() {
        let options = ContainerOptsBuilder::new("test_image")
            .network_mode("host")
            .auto_remove(true)
            .privileged(true)
            .build();

        assert_eq!(
            r#"{"HostConfig":{"AutoRemove":true,"NetworkMode":"host","Privileged":true},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_expose() {
        let options = ContainerOptsBuilder::new("test_image")
            .expose(80, "tcp", 8080)
            .build();
        assert_eq!(
            r#"{"ExposedPorts":{"80/tcp":{}},"HostConfig":{"PortBindings":{"80/tcp":[{"HostPort":"8080"}]}},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
        // try exposing two
        let options = ContainerOptsBuilder::new("test_image")
            .expose(80, "tcp", 8080)
            .expose(81, "tcp", 8081)
            .build();
        assert_eq!(
            r#"{"ExposedPorts":{"80/tcp":{},"81/tcp":{}},"HostConfig":{"PortBindings":{"80/tcp":[{"HostPort":"8080"}],"81/tcp":[{"HostPort":"8081"}]}},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[test]
    fn container_options_publish() {
        let options = ContainerOptsBuilder::new("test_image")
            .publish(80, "tcp")
            .build();
        assert_eq!(
            r#"{"ExposedPorts":{"80/tcp":{}},"HostConfig":{},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
        // try exposing two
        let options = ContainerOptsBuilder::new("test_image")
            .publish(80, "tcp")
            .publish(81, "tcp")
            .build();
        assert_eq!(
            r#"{"ExposedPorts":{"80/tcp":{},"81/tcp":{}},"HostConfig":{},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    /// Test container option PublishAllPorts
    #[test]
    fn container_options_publish_all_ports() {
        let options = ContainerOptsBuilder::new("test_image")
            .publish_all_ports()
            .build();

        assert_eq!(
            r#"{"HostConfig":{"PublishAllPorts":true},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    /// Test container Opts that are nested 3 levels deep.
    #[test]
    fn container_options_nested() {
        let options = ContainerOptsBuilder::new("test_image")
            .log_driver("fluentd")
            .build();

        assert_eq!(
            r#"{"HostConfig":{"LogConfig":{"Type":"fluentd"}},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    /// Test the restart policy settings
    #[test]
    fn container_options_restart_policy() {
        let mut options = ContainerOptsBuilder::new("test_image")
            .restart_policy("on-failure", 10)
            .build();

        assert_eq!(
            r#"{"HostConfig":{"RestartPolicy":{"MaximumRetryCount":10,"Name":"on-failure"}},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );

        options = ContainerOptsBuilder::new("test_image")
            .restart_policy("always", 0)
            .build();

        assert_eq!(
            r#"{"HostConfig":{"RestartPolicy":{"Name":"always"}},"Image":"test_image"}"#,
            options.serialize().unwrap()
        );
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn logs_options() {
        let timestamp = chrono::NaiveDateTime::from_timestamp(2_147_483_647, 0);
        let since = chrono::DateTime::<chrono::Utc>::from_utc(timestamp, chrono::Utc);

        let options = LogsOptsBuilder::default()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .tail("all")
            .since(&since)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));
    }

    #[cfg(not(feature = "chrono"))]
    #[test]
    fn logs_Opts() {
        let options = LogsOptsBuilder::default()
            .follow(true)
            .stdout(true)
            .stderr(true)
            .timestamps(true)
            .tail("all")
            .since(2_147_483_647)
            .build();

        let serialized = options.serialize().unwrap();

        assert!(serialized.contains("follow=true"));
        assert!(serialized.contains("stdout=true"));
        assert!(serialized.contains("stderr=true"));
        assert!(serialized.contains("timestamps=true"));
        assert!(serialized.contains("tail=all"));
        assert!(serialized.contains("since=2147483647"));
    }
}
