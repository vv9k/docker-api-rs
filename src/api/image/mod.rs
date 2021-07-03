//! Create and manage images.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use std::io::Read;

use futures_util::{stream::Stream, TryFutureExt, TryStreamExt};

use crate::{
    conn::{Headers, Payload},
    util::{
        tarball,
        url::{append_query, construct_ep, encoded_pair, encoded_pairs},
    },
    Result,
};

impl_api_ty!(Image => name: N);

impl<'docker> Image<'docker> {
    /// Inspects a named image's details.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageInspect)
    pub async fn inspect(&self) -> Result<ImageDetails> {
        self.docker
            .get_json(&format!("/images/{}/json", self.name))
            .await
    }

    /// Lists the history of the images set of changes.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageHistory)
    pub async fn history(&self) -> Result<Vec<History>> {
        self.docker
            .get_json(&format!("/images/{}/history", self.name))
            .await
    }

    /// Remove an image.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageDelete)
    pub async fn remove(&self, opts: &ImageRemoveOpts) -> Result<Vec<Status>> {
        self.docker
            .delete_json(&construct_ep(
                format!("/images/{}", self.name),
                opts.serialize(),
            ))
            .await
    }

    /// Export this image to a tarball.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageGet)
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + 'docker {
        Box::pin(
            self.docker
                .stream_get(format!("/images/{}/get", self.name))
                .map_ok(|c| c.to_vec()),
        )
    }

    /// Adds a tag to an image.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageTag)
    pub async fn tag(&self, opts: &TagOpts) -> Result<()> {
        let mut ep = format!("/images/{}/tag", self.name);
        if let Some(query) = opts.serialize() {
            append_query(&mut ep, query);
        }
        self.docker.post(&ep, Payload::empty()).await.map(|_| ())
    }

    /// Return image digest and platform information by contacting the registry.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/DistributionInspect)
    pub async fn distribution_inspect(&self) -> Result<DistributionInspectInfo> {
        self.docker
            .post_json(
                &format!("/distribution/{}/json", self.name),
                Payload::empty(),
            )
            .await
    }
}

impl<'docker> Images<'docker> {
    /// Builds a new image build by reading a Dockerfile in a target directory.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageBuild)
    pub fn build(
        &self,
        opts: &BuildOpts,
    ) -> impl Stream<Item = Result<ImageBuildChunk>> + Unpin + 'docker {
        let ep = construct_ep("/build", opts.serialize());

        // To not tie the lifetime of `opts` to the 'stream, we do the tarring work outside of the
        // stream. But for backwards compatability, we have to return the error inside of the
        // stream.
        let mut bytes = Vec::default();
        let tar_result = tarball::dir(&mut bytes, opts.path.as_str());

        // We must take ownership of the Docker reference. If we don't then the lifetime of 'stream
        // is incorrectly tied to `self`.
        let docker = self.docker;
        Box::pin(
            async move {
                // Bubble up error inside the stream for backwards compatability
                tar_result?;

                let value_stream =
                    docker.stream_post_into(ep, Payload::Tar(bytes), Headers::none());

                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }

    /// Lists the docker images on the current docker host.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageList)
    pub async fn list(&self, opts: &ImageListOpts) -> Result<Vec<ImageInfo>> {
        self.docker
            .get_json(&construct_ep("/images/json", opts.serialize()))
            .await
    }

    /// Search for docker images by term.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageSearch)
    pub async fn search(&self, term: &str) -> Result<Vec<SearchResult>> {
        self.docker
            .get_json(&format!("/images/search?{}", encoded_pair("term", term)))
            .await
    }

    /// Pull and create a new docker images from an existing image.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImagePull)
    pub fn pull(
        &self,
        opts: &PullOpts,
    ) -> impl Stream<Item = Result<ImageBuildChunk>> + Unpin + 'docker {
        let headers = opts
            .auth_header()
            .map(|a| Headers::single("X-Registry-Auth", a));

        Box::pin(self.docker.stream_post_into(
            construct_ep("/images/create", opts.serialize()),
            Payload::empty(),
            headers,
        ))
    }

    /// Exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageGetAll)
    pub fn export(&self, names: Vec<&str>) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        self.docker
            .stream_get(format!(
                "/images/get?{}",
                encoded_pairs(names.iter().map(|n| ("names", *n)))
            ))
            .map_ok(|c| c.to_vec())
    }

    /// Imports an image or set of images from a given tarball source.
    /// Source can be uncompressed on compressed via gzip, bzip2 or xz.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageLoad)
    pub fn import<R>(
        self,
        mut tarball: R,
    ) -> impl Stream<Item = Result<ImageBuildChunk>> + Unpin + 'docker
    where
        R: Read + Send + 'docker,
    {
        Box::pin(
            async move {
                let mut bytes = Vec::default();

                tarball.read_to_end(&mut bytes)?;

                let value_stream = self.docker.stream_post_into(
                    "/images/load",
                    Payload::Tar(bytes),
                    Headers::none(),
                );
                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }

    /// Delete unused images.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImagePrune)
    pub async fn prune(&self, opts: &ImagePruneOpts) -> Result<ImagePruneInfo> {
        self.docker
            .post_json(
                &construct_ep("/images/prune", opts.serialize()),
                Payload::empty(),
            )
            .await
    }
}
