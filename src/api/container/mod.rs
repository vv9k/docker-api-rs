//! Create and manage containers.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use std::{io, path::Path, str, time::Duration};

use futures_util::{
    io::{AsyncRead, AsyncWrite},
    Stream, TryStreamExt,
};
use hyper::Body;

use crate::{
    api::{Exec, ExecContainerOpts, LogsOpts},
    conn::{tty, Multiplexer as TtyMultiplexer, Payload, TtyChunk},
    util::url::{append_query, construct_ep, encoded_pair},
    Error, Result,
};

impl_api_ty!(Container => id: I);

impl<'docker> Container<'docker> {
    /// Inspects the current docker container instance's details.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerInspect)
    pub async fn inspect(&self) -> Result<ContainerDetails> {
        self.docker
            .get_json::<ContainerDetails>(&format!("/containers/{}/json", self.id))
            .await
    }

    /// Returns a `top` view of information about the container process.
    /// On Unix systems, this is done by running the ps command. This endpoint is not supported on Windows.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerTop)
    pub async fn top(&self, psargs: Option<&str>) -> Result<Top> {
        let mut ep = format!("/containers/{}/top", self.id);
        if let Some(ref args) = psargs {
            append_query(&mut ep, encoded_pair("ps_args", args));
        }
        self.docker.get_json(&ep).await
    }

    /// Returns a stream of logs emitted by this container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerLogs)
    pub fn logs(&self, opts: &LogsOpts) -> impl Stream<Item = Result<TtyChunk>> + Unpin + 'docker {
        let stream = Box::pin(self.docker.stream_get(construct_ep(
            format!("/containers/{}/logs", self.id),
            opts.serialize(),
        )));

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

    /// Attaches a [`TtyMultiplexer`](TtyMultiplexer) to the container.
    ///
    /// The [`TtyMultiplexer`](TtyMultiplexer) implements Stream for returning Stdout and Stderr chunks. It also implements [`AsyncWrite`](futures_util::io::AsyncWrite) for writing to Stdin.
    ///
    /// The multiplexer can be split into its read and write halves with the [`split`](TtyMultiplexer::split) method
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerAttach)
    pub async fn attach(&self) -> Result<TtyMultiplexer<'docker>> {
        self.attach_raw().await.map(TtyMultiplexer::new)
    }

    /// Returns a set of changes made to the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerChanges)
    pub async fn changes(&self) -> Result<Vec<Change>> {
        self.docker
            .get_json(&format!("/containers/{}/changes", self.id))
            .await
    }

    /// Exports the current docker container into a tarball.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerExport)
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        self.docker
            .stream_get(format!("/containers/{}/export", self.id))
            .map_ok(|c| c.to_vec())
    }

    /// Returns a stream of stats specific to this container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerStats)
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

    /// Start the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerStart)
    pub async fn start(&self) -> Result<()> {
        self.docker
            .post(&format!("/containers/{}/start", self.id), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Stop the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerStop)
    pub async fn stop(&self, wait: Option<Duration>) -> Result<()> {
        let mut ep = format!("/containers/{}/stop", self.id);
        if let Some(w) = wait {
            append_query(&mut ep, encoded_pair("t", w.as_secs()));
        }
        self.docker.post(&ep, Payload::empty()).await.map(|_| ())
    }

    /// Restart the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerRestart)
    pub async fn restart(&self, wait: Option<Duration>) -> Result<()> {
        let mut ep = format!("/containers/{}/restart", self.id);
        if let Some(w) = wait {
            append_query(&mut ep, encoded_pair("t", w.as_secs()));
        }
        self.docker.post(&ep, Payload::empty()).await.map(|_| ())
    }

    /// Kill the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerKill)
    pub async fn kill(&self, signal: Option<&str>) -> Result<()> {
        let mut ep = format!("/containers/{}/kill", self.id);
        if let Some(sig) = signal {
            append_query(&mut ep, encoded_pair("signal", sig));
        }
        self.docker.post(&ep, Payload::empty()).await.map(|_| ())
    }

    /// Rename the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerRename)
    pub async fn rename(&self, name: &str) -> Result<()> {
        self.docker
            .post(
                &format!(
                    "/containers/{}/rename?{}",
                    self.id,
                    encoded_pair("name", name)
                ),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Pause the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerPause)
    pub async fn pause(&self) -> Result<()> {
        self.docker
            .post(&format!("/containers/{}/pause", self.id), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Unpause the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerUnpause)
    pub async fn unpause(&self) -> Result<()> {
        self.docker
            .post(
                &format!("/containers/{}/unpause", self.id),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }

    /// Wait until the container stops.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerWait)
    pub async fn wait(&self) -> Result<Exit> {
        self.docker
            .post_json(format!("/containers/{}/wait", self.id), Payload::empty())
            .await
    }

    /// Delete the container instance.
    ///
    /// Use remove instead to use the force/v Opts.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerDelete)
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/containers/{}", self.id))
            .await
            .map(|_| ())
    }

    /// Delete the container instance.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerRemove)
    pub async fn remove(&self, opts: &RmContainerOpts) -> Result<()> {
        self.docker
            .delete(&construct_ep(
                format!("/containers/{}", self.id),
                opts.serialize(),
            ))
            .await
            .map(|_| ())
    }

    /// Execute a command in this container.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#tag/Exec)
    pub fn exec(
        &'docker self,
        opts: &ExecContainerOpts,
    ) -> impl Stream<Item = Result<TtyChunk>> + Unpin + 'docker {
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
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerArchive)
    pub fn copy_from(&self, path: &Path) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        self.docker
            .stream_get(format!(
                "/containers/{}/archive?{}",
                self.id,
                encoded_pair("path", path.to_string_lossy())
            ))
            .map_ok(|c| c.to_vec())
    }

    /// Copy a byte slice as file into (see `bytes`) the container.
    ///
    /// The file will be copied at the given location (see `path`) and will be owned by root
    /// with access mask 644.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/PutContainerArchive)
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
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/PutContainerArchive)
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
    }

    /// Get information about files in a container.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerArchiveInfo)
    pub async fn stat_file<P>(&self, path: P) -> Result<String>
    where
        P: AsRef<Path>,
    {
        static PATH_STAT_HEADER: &str = "X-Docker-Container-Path-Stat";
        let resp = self
            .docker
            .head_response(&format!(
                "/containers/{}/archive?{}",
                self.id,
                encoded_pair("path", path.as_ref().to_string_lossy())
            ))
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
    /// Lists the container instances on the docker host.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerList)
    pub async fn list(&self, opts: &ContainerListOpts) -> Result<Vec<ContainerInfo>> {
        self.docker
            .get_json(&construct_ep("/containers/json", opts.serialize()))
            .await
    }

    /// Create a new container.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerList)
    pub async fn create(&self, opts: &ContainerOpts) -> Result<ContainerCreateInfo> {
        self.docker
            .post_json(
                &construct_ep("/containers/create", opts.name.as_ref()),
                Payload::Json(opts.serialize()?),
            )
            .await
    }

    /// Delete stopped containers.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ContainerPrune)
    pub async fn prune(&self, opts: &ContainerPruneOpts) -> Result<()> {
        self.docker
            .post(
                &construct_ep("/containers/prune", opts.serialize()),
                Payload::empty(),
            )
            .await
            .map(|_| ())
    }
}
