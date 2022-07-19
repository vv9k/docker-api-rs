//! Run new commands inside running containers.

use futures_util::{
    stream::{Stream, TryStreamExt},
    TryFutureExt,
};
use hyper::Body;

use crate::{
    conn::{tty, Headers, Payload},
    models,
    opts::{ExecContainerOpts, ExecResizeOpts},
    Docker, Result,
};

pub type ExecId = String;
pub type ExecIdRef<'a> = &'a str;

api_doc! { Exec
/// Interface for docker exec instance
|
pub struct Exec {
    docker: Docker,
    id: ExecId,
}}

impl Exec {
    fn new<ID>(docker: Docker, id: ID) -> Self
    where
        ID: Into<ExecId>,
    {
        Exec {
            docker,
            id: id.into(),
        }
    }

    impl_api_ep! {exec: Exec, resp
        Inspect -> &format!("/exec/{}/json", exec.id), models::ExecInspectResponse
    }

    api_doc! { Exec => Create
    /// Creates a new exec instance that will be executed in a container with id == container_id.
    |
    pub async fn create<C>(
        docker: Docker,
        container_id: C,
        opts: &ExecContainerOpts,
    ) -> Result<Exec>
    where
        C: AsRef<str>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            id: String,
        }

        docker
            .post_json(
                &format!("/containers/{}/exec", container_id.as_ref()),
                Payload::Json(opts.serialize()?),
            )
            .await
            .map(|resp: Response| Exec::new(docker, resp.id))
    }}

    // This exists for Container::exec()
    //
    // We need to combine `Exec::create` and `Exec::start` into one method because otherwise you
    // needlessly tie the Stream to the lifetime of `container_id` and `opts`. This is because
    // `Exec::create` is async so it must occur inside of the `async move` block. However, this
    // means that `container_id` and `opts` are both expected to be alive in the returned stream
    // because we can't do the work of creating an endpoint from `container_id` or serializing
    // `opts`. By doing this work outside of the stream, we get owned values that we can then move
    // into the stream and have the lifetimes work out as you would expect.
    //
    // Yes, it is sad that we can't do the easy method and thus have some duplicated code.
    pub(crate) fn create_and_start<'docker, C>(
        docker: &'docker Docker,
        container_id: C,
        opts: &ExecContainerOpts,
    ) -> impl Stream<Item = crate::conn::Result<tty::TtyChunk>> + Unpin + 'docker
    where
        C: AsRef<str>,
    {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            id: String,
        }

        // To not tie the lifetime of `opts` to the stream, we do the serializing work outside of
        // the stream. But for backwards compatability, we have to return the error inside of the
        // stream.
        let body_result = opts.serialize();

        // To not tie the lifetime of `container_id` to the stream, we convert it to an (owned)
        // endpoint outside of the stream.
        let container_endpoint = format!("/containers/{}/exec", container_id.as_ref());

        Box::pin(
            async move {
                let exec_id = docker
                    .post_json(
                        &container_endpoint,
                        Payload::Json(
                            body_result.map_err(|e| crate::conn::Error::Any(Box::new(e)))?,
                        ),
                    )
                    .await
                    .map(|resp: Response| resp.id)
                    .map_err(|e| crate::conn::Error::Any(Box::new(e)))?;

                let stream = Box::pin(
                    docker
                        .stream_post(
                            format!("/exec/{}/start", exec_id),
                            Payload::Json("{}"),
                            Headers::none(),
                        )
                        .map_err(|e| crate::conn::Error::Any(Box::new(e))),
                );

                Ok(tty::decode(stream))
            }
            .try_flatten_stream(),
        )
    }

    /// Get a reference to a set of operations available to an already created exec instance.
    ///
    /// It's in callers responsibility to ensure that exec instance with specified id actually
    /// exists. Use [Exec::create](Exec::create) to ensure that the exec instance is created
    /// beforehand.
    pub fn get<ID>(docker: Docker, id: ID) -> Exec
    where
        ID: Into<ExecId>,
    {
        Exec::new(docker, id)
    }

    api_doc! { Exec => Start
    /// Starts this exec instance returning a multiplexed tty stream.
    |
    pub fn start(&self) -> impl Stream<Item = crate::conn::Result<tty::TtyChunk>> + '_ {
        // We must take ownership of the docker reference to not needlessly tie the stream to the
        // lifetime of `self`.
        let docker = &self.docker;
        // We convert `self.id` into the (owned) endpoint outside of the stream to not needlessly
        // tie the stream to the lifetime of `self`.
        let endpoint = format!("/exec/{}/start", &self.id);
        Box::pin(
            async move {
                let stream = Box::pin(
                    docker
                        .stream_post(endpoint, Payload::Json("{}"), Headers::none())
                        .map_err(|e| crate::conn::Error::Any(Box::new(e))),
                );

                Ok(tty::decode(stream))
            }
            .try_flatten_stream(),
        )
    }}

    api_doc! { Exec => Resize
    /// Resize the TTY session used by an exec instance. This only works if the exec was created
    /// with `tty` enabled.
    |
    pub async fn resize(&self, opts: &ExecResizeOpts) -> Result<()> {
        let body: Body = opts.serialize()?.into();

        self.docker
            .post_json(&format!("/exec/{}/resize", &self.id), Payload::Json(body))
            .await
    }}
}
