//! Create and manage containers.
use crate::opts::{
    ContainerCommitOpts, ContainerCreateOpts, ContainerListOpts, ContainerPruneOpts,
    ContainerRemoveOpts, ContainerRestartOpts, ContainerStopOpts, ExecStartOpts,
};
use crate::{models, stream};

use std::{io, path::Path, str};

use futures_util::{Stream, TryStreamExt};
use hyper::Body;
use serde::Deserialize;

use crate::{
    api::Exec,
    conn::{tty, Headers, Payload},
    opts::ExecCreateOpts,
    Error, Result,
};
use containers_api::url::{append_query, construct_ep, encoded_pair};

impl_api_ty!(Container => id);

impl Container {
    impl_api_ep! {container: Container, resp
        Inspect -> &format!("/containers/{}/json", container.id), models::ContainerInspect200Response
        Logs -> &format!("/containers/{}/logs", container.id), ()
        DeleteWithOpts -> &format!("/containers/{}", container.id), String, delete
    }

    api_doc! { Container => Top
    |
    /// Returns a `top` view of information about the container process.
    /// On Unix systems, this is done by running the ps command. This endpoint is not supported on Windows.
    pub async fn top(&self, psargs: Option<&str>) -> Result<models::ContainerTop200Response> {
        let mut ep = format!("/containers/{}/top", self.id);
        if let Some(ref args) = psargs {
            append_query(&mut ep, encoded_pair("ps_args", args));
        }
        self.docker.get_json(&ep).await
    }}

    api_doc! { Container => Attach
    |
    /// Attaches a [`TtyMultiplexer`](TtyMultiplexer) to the container.
    ///
    /// The [`TtyMultiplexer`](TtyMultiplexer) implements Stream for returning Stdout and Stderr chunks. It also implements [`AsyncWrite`](futures_util::io::AsyncWrite) for writing to Stdin.
    ///
    /// The multiplexer can be split into its read and write halves with the [`split`](TtyMultiplexer::split) method
    pub async fn attach(&self) -> Result<tty::Multiplexer> {
        let inspect = self.inspect().await?;
        let is_tty = inspect.config.and_then(|c| c.tty).unwrap_or_default();
        stream::attach(
            self.docker.clone(),
            format!(
                "/containers/{}/attach?stream=1&stdout=1&stderr=1&stdin=1",
                self.id
            ),
            Payload::empty(),
            is_tty,
        )
        .await
    }}

    api_doc! { Container => Changes
    |
    /// Returns a set of changes made to the container instance.
    pub async fn changes(&self) -> Result<Option<models::ContainerChanges200Response>> {
        self.docker
            .get_json(&format!("/containers/{}/changes", self.id))
            .await
    }}

    api_doc! { Container => Export
    |
    /// Exports the current docker container into a tarball.
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + '_ {
        self.docker
            .get_stream(format!("/containers/{}/export", self.id))
            .map_ok(|c| c.to_vec())
    }}

    api_doc! { Container => Stats
    |
    /// Returns a stream of stats specific to this container instance.
    pub fn stats(&self) -> impl Stream<Item = Result<serde_json::Value>> + Unpin + '_ {
        let codec = asynchronous_codec::LinesCodec {};

        let reader = Box::pin(
            self.docker
                .get_stream(format!("/containers/{}/stats", self.id))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            asynchronous_codec::FramedRead::new(reader, codec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    log::trace!("{}", s);
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }}

    api_doc! { Container => Start
    |
    /// Start the container instance.
    pub async fn start(&self) -> Result<()> {
        self.docker
            .post_string(
                &format!("/containers/{}/start", self.id),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Stop
    |
    /// Stop the container instance.
    pub async fn stop(&self, opts: &ContainerStopOpts) -> Result<()> {
        let ep = construct_ep(format!("/containers/{}/stop", self.id), opts.serialize());
        self.docker
            .post_string(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Restart
    |
    /// Restart the container instance.
    pub async fn restart(&self, opts: &ContainerRestartOpts) -> Result<()> {
        let ep = construct_ep(format!("/containers/{}/restart", self.id), opts.serialize());
        self.docker
            .post_string(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Kill
    |
    /// Kill the container instance.
    pub async fn kill(&self, signal: Option<&str>) -> Result<()> {
        let mut ep = format!("/containers/{}/kill", self.id);
        if let Some(sig) = signal {
            append_query(&mut ep, encoded_pair("signal", sig));
        }
        self.docker
            .post_string(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Rename
    |
    /// Rename the container instance.
    pub async fn rename(&self, name: &str) -> Result<()> {
        self.docker
            .post_string(
                &format!(
                    "/containers/{}/rename?{}",
                    self.id,
                    encoded_pair("name", name)
                ),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Pause
    |
    /// Pause the container instance.
    pub async fn pause(&self) -> Result<()> {
        self.docker
            .post_string(
                &format!("/containers/{}/pause", self.id),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Unpause
    |
    /// Unpause the container instance.
    pub async fn unpause(&self) -> Result<()> {
        self.docker
            .post_string(
                &format!("/containers/{}/unpause", self.id),
                Payload::empty(),
                Headers::none(),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Container => Wait
    |
    /// Wait until the container stops.
    pub async fn wait(&self) -> Result<models::ContainerWaitResponse> {
        self.docker
            .post_json(
                format!("/containers/{}/wait", self.id),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }}

    api_doc! { Exec
    |
    /// Execute a command in this container.
    pub async fn exec(
        &self,
        create_opts: &ExecCreateOpts,
        start_opts: &ExecStartOpts,
    ) ->  Result<tty::Multiplexer> {
        Exec::create_and_start(self.docker.clone(), &self.id, create_opts, start_opts).await
    }}

    api_doc! { Container => Archive
    |
    /// Copy a file/folder from the container.  The resulting stream is a tarball of the extracted
    /// files.
    ///
    /// If `path` is not an absolute path, it is relative to the container’s root directory. The
    /// resource specified by `path` must exist. To assert that the resource is expected to be a
    /// directory, `path` should end in `/` or `/`. (assuming a path separator of `/`). If `path`
    /// ends in `/.`  then this indicates that only the contents of the path directory should be
    /// copied.  A symlink is always resolved to its target.
    pub fn copy_from(&self, path: impl AsRef<Path>) -> impl Stream<Item = Result<Vec<u8>>> + '_ {
        self.docker
            .get_stream(format!(
                "/containers/{}/archive?{}",
                self.id,
                encoded_pair("path", path.as_ref().to_string_lossy())
            ))
            .map_ok(|c| c.to_vec())
    }}

    api_doc! { PutContainer => Archive
    |
    /// Copy a byte slice as file into (see `bytes`) the container.
    ///
    /// The file will be copied at the given location (see `path`) and will be owned by root
    /// with access mask 644.
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
    }}

    api_doc! { PutContainer => Archive
    |
    /// Copy a tarball (see `body`) to the container.
    ///
    /// The tarball will be copied to the container and extracted at the given location (see `path`).
    pub async fn copy_to(&self, path: &Path, body: Body) -> Result<()> {
        self.docker
            .put(
                &format!(
                    "/containers/{}/archive?{}",
                    self.id,
                    encoded_pair("path", path.to_string_lossy())
                ),
                Payload::XTar(body),
            )
            .await
            .map(|_| ())
    }}

    api_doc! { Container => ArchiveInfo
    |
    /// Get information about files in a container.
    pub async fn stat_file<P>(&self, path: P) -> Result<String>
    where
        P: AsRef<Path>,
    {
        static PATH_STAT_HEADER: &str = "X-Docker-Container-Path-Stat";
        let resp = self
            .docker
            .head(&format!(
                "/containers/{}/archive?{}",
                self.id,
                encoded_pair("path", path.as_ref().to_string_lossy())
            ))
            .await?;
        if let Some(header) = resp.headers().get(PATH_STAT_HEADER) {
            let header = header.to_str().map_err(|e| {
                Error::InvalidResponse(format!("response header was invalid - {e}"))
            })?;

            base64::decode(header)
                .map_err(|e| {
                    Error::InvalidResponse(format!("expected header to be valid base64 - {e}"))
                })
                .and_then(|s| {
                    str::from_utf8(s.as_slice())
                        .map(str::to_string)
                        .map_err(|e| {
                            Error::InvalidResponse(format!(
                                "expected header to be valid utf8 - {e}"
                            ))
                        })
                })
        } else {
            Err(Error::InvalidResponse(format!("missing `{PATH_STAT_HEADER}` header")))
        }
    }}

    api_doc! { Image => Commit
    |
    /// Create a new image from this container
    pub async fn commit(&self, opts: &ContainerCommitOpts, config: Option<&models::ContainerConfig>) -> Result<String> {
        #[derive(Deserialize)]
        struct IdStruct {
            #[serde(rename = "Id")]
            id: String,
        }

        let payload = if let Some(config) = config {
            Payload::Json(serde_json::to_string(config)?)
        } else {
            Payload::Json("{}".into()) // empty json
        };

        self.docker
            .post_json(
                format!(
                    "/commit?{}",
                    opts.with_container(self.id().as_ref())
                        .serialize()
                        .unwrap_or_default()
                ),
                payload,
                Headers::none(),
            )
            .await
            .map(|id: IdStruct| id.id)
    }}
}

impl Containers {
    impl_api_ep! {__: Container, resp
        List -> "/containers/json", models::ContainerSummary
        Prune -> "/containers/prune", models::ContainerPrune200Response
    }

    api_doc! { Containers => Create
    |
    /// Create a container
    pub async fn create(&self, opts: &ContainerCreateOpts) -> Result<Container> {
        let ep = if let Some(name) = opts.name() {
            construct_ep("/containers/create", Some(encoded_pair("name", name)))
        } else {
            "/containers/create".to_owned()
        };
        self.docker
            .post_json(&ep, Payload::Json(opts.serialize_vec()?), Headers::none())
            .await
            .map(|resp: models::ContainerCreateResponse| {
                Container::new(self.docker.clone(), resp.id)
            })
    }}
}
