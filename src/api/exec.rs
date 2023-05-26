//! Run new commands inside running containers.

use hyper::Body;

use crate::{
    conn::{tty, Headers, Payload},
    models,
    opts::{ExecCreateOpts, ExecResizeOpts, ExecStartOpts},
    stream, Docker, Result,
};

api_doc! { Exec
/// Interface for docker exec instance
|
pub struct Exec {
    docker: Docker,
    id: crate::Id,
}}

impl Exec {
    fn new(docker: Docker, id: impl Into<crate::Id>) -> Self {
        Exec {
            docker,
            id: id.into(),
        }
    }

    /// Get a reference to a set of operations available to an already created exec instance.
    ///
    /// It's in callers responsibility to ensure that exec instance with specified id actually
    /// exists. Use [Exec::create](Exec::create) to ensure that the exec instance is created
    /// beforehand.
    pub fn get(docker: Docker, id: impl Into<crate::Id>) -> Exec {
        Exec::new(docker, id)
    }

    api_doc! { Exec => Inspect
    |
    /// Inspect this Exec instance
    pub async fn inspect(&self) -> Result<models::ExecInspect200Response> {
        Self::inspect_impl(&self.docker, self.id.as_ref()).await
    }}

    async fn inspect_impl(docker: &Docker, id: &str) -> Result<models::ExecInspect200Response> {
        docker.get_json(&format!("/exec/{id}/json")).await
    }

    async fn create_impl(
        docker: Docker,
        container_id: &str,
        opts: &ExecCreateOpts,
    ) -> Result<crate::Id> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Response {
            id: String,
        }

        docker
            .post_json(
                &format!("/containers/{}/exec", container_id),
                Payload::Json(opts.serialize_vec()?),
                Headers::none(),
            )
            .await
            .map(|resp: Response| resp.id.into())
    }

    api_doc! { Exec => Create
    |
    /// Creates a new exec instance that will be executed in a container with id == container_id.
    pub async fn create(
        docker: Docker,
        container_id: impl AsRef<str>,
        opts: &ExecCreateOpts,
    ) -> Result<Exec>
    {
        Self::create_impl(docker.clone(), container_id.as_ref(), opts)
        .await
            .map(|id| Exec::new(docker, id))
    }}

    async fn start_impl(
        docker: Docker,
        id: &str,
        opts: &ExecStartOpts,
    ) -> Result<tty::Multiplexer> {
        let endpoint = format!("/exec/{}/start", id);
        let inspect_data = Self::inspect_impl(&docker, id).await?;
        let is_tty = inspect_data
            .process_config
            .and_then(|c| c.tty)
            .unwrap_or_default();

        stream::attach(
            docker,
            endpoint,
            Payload::Json(opts.serialize_vec()?.into()),
            is_tty,
        )
        .await
    }

    api_doc! { Exec => Start
    |
    /// Starts this exec instance returning a multiplexed tty stream.
    pub async fn start(&self, opts: &ExecStartOpts) -> Result<tty::Multiplexer> {
        Self::start_impl(self.docker.clone(), self.id.as_ref(), opts).await
    }}

    pub(crate) async fn create_and_start(
        docker: Docker,
        container_id: impl AsRef<str>,
        create_opts: &ExecCreateOpts,
        start_opts: &ExecStartOpts,
    ) -> Result<tty::Multiplexer> {
        let container_id = container_id.as_ref();
        let id = Self::create_impl(docker.clone(), container_id, create_opts).await?;

        Self::start_impl(docker, id.as_ref(), start_opts).await
    }

    api_doc! { Exec => Resize
    |
    /// Resize the TTY session used by an exec instance. This only works if the exec was created
    /// with `tty` enabled.
    pub async fn resize(&self, opts: &ExecResizeOpts) -> Result<()> {
        let body: Body = opts.serialize()?.into();

        self.docker
            .post_json(
                &format!("/exec/{}/resize", &self.id),
                Payload::Json(body),
                Headers::none(),
            )
            .await
    }}
}
