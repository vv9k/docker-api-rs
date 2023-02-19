//! Create and manage images.

use crate::{
    models,
    opts::{
        ClearCacheOpts, ImageBuildOpts, ImageListOpts, ImagePruneOpts, ImagePushOpts,
        ImageRemoveOpts, PullOpts, TagOpts,
    },
};

use std::io::Read;

use futures_util::{stream::Stream, TryFutureExt, TryStreamExt};

use containers_api::{
    conn::{Headers, Payload, AUTH_HEADER},
    tarball,
    url::{construct_ep, encoded_pair, encoded_pairs},
};

use crate::Result;

impl_api_ty!(Image => name);

impl Image {
    impl_api_ep! {img: Image, resp
        Inspect -> &format!("/images/{}/json", img.name), models::ImageInspect
    }

    api_doc! { Image => Delete
    |
    /// Remove this image with options.
    ///
    /// Use [`delete`](Image::delete) to delete without options.
    pub async fn remove(&self, opts: &ImageRemoveOpts) -> Result<Vec<models::ImageDeleteResponseItem>> {
        let ep =
            containers_api::url::construct_ep(format!("/images/{}", self.name), opts.serialize());
        self.docker.delete_json(ep.as_ref()).await
    }}

    api_doc! { Image => Delete
    |
    /// Delete this image with force.
    ///
    /// Use [`remove`](Image::remove) to delete with options.
    pub async fn delete(&self) -> Result<Vec<models::ImageDeleteResponseItem>> {
        self.docker
            .delete_json(&format!("/images/{}", self.name))
            .await
    }}

    api_doc! { Image => History
    |
    /// Lists the history of the images set of changes.
    pub async fn history(&self) -> Result<models::ImageHistory200Response> {
        self.docker
            .get_json(&format!("/images/{}/history", self.name))
            .await
    }}

    api_doc! { Image => Get
    |
    /// Export this image to a tarball.
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + '_ {
        Box::pin(
            self.docker
                .get_stream(format!("/images/{}/get", self.name))
                .map_ok(|c| c.to_vec()),
        )
    }}

    api_doc! { Image => Tag
    |
    /// Adds a tag to an image.
    pub async fn tag(&self, opts: &TagOpts) -> Result<()> {
        let ep = construct_ep(format!("/images/{}/tag", self.name), opts.serialize());
        self.docker
            .post_string(&ep, Payload::empty(), Headers::none())
            .await
            .map(|_| ())
    }}

    api_doc! { Image => Push
    |
    /// Push an image to registry.
    pub async fn push(&self, opts: &ImagePushOpts) -> Result<()> {
        let ep = construct_ep(format!("/images/{}/push", self.name), opts.serialize());

        let headers = opts
            .auth_header()
            .map(|auth| Headers::single(AUTH_HEADER, auth))
            .unwrap_or_else(Headers::default);

        self.docker
            .post_string(&ep, Payload::empty(), Some(headers))
            .await
            .map(|_| ())
    }}

    api_doc! { Distribution => Inspect
    |
    /// Return image digest and platform information by contacting the registry.
    pub async fn distribution_inspect(&self) -> Result<models::DistributionInspect> {
        self.docker
            .post_json(
                &format!("/distribution/{}/json", self.name),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }}
}

impl Images {
    impl_api_ep! {img: Image, resp
        List -> "/images/json", models::ImageSummary
        Prune ->  "/images/prune", models::ImagePrune200Response
    }

    api_doc! { Image => Build
    |
    /// Builds a new image by reading a Dockerfile in a target directory. If speed is
    /// important consider using [`Image::build_par`](Image::build_par) that utilizes
    /// parallel compression on big directories, to use it enable `par-compression` feature.
    pub fn build<'docker>(
        &'docker self,
        opts: &ImageBuildOpts,
    ) -> impl Stream<Item = Result<models::ImageBuildChunk>> + Unpin + 'docker {
        let ep = construct_ep("/build", opts.serialize());
        let mut bytes = vec![];
        let tar_result = tarball::dir(&mut bytes, &opts.path);

        let docker = &self.docker;
        Box::pin(
            async move {
                tar_result?;

                let value_stream =
                    docker.post_into_stream(ep, Payload::Tar(bytes), Headers::none());

                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }}

    api_doc! { Image => Build
    |
    #[cfg(feature = "par-compress")]
    /// Builds a new image by reading a Dockerfile in a target directory. Uses parallel
    /// compression algorithm to speed up the execution. For a single-threaded version check
    /// [`Image::build`](Image::build).
    pub fn build_par<'docker>(
        &'docker self,
        opts: &ImageBuildOpts,
    ) -> impl Stream<Item = Result<models::ImageBuildChunk>> + Unpin + 'docker {
        let ep = construct_ep("/build", opts.serialize());

        let tar_result = tarball::dir_par(&opts.path);

        let docker = &self.docker;
        Box::pin(
            async move {
                let bytes = tar_result?;

                let value_stream =
                    docker.post_into_stream(ep, Payload::Tar(bytes), Headers::none());

                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }}

    api_doc! { Image => Search
    |
    /// Search for docker images by term.
    pub async fn search<T>(&self, term: T) -> Result<models::ImageSearch200Response>
    where
        T: AsRef<str>,
    {
        self.docker
            .get_json(&construct_ep(
                "/images/search",
                Some(encoded_pair("term", term.as_ref())),
            ))
            .await
    }}

    api_doc! { Image => Pull
    |
    /// Pull and create a new docker images from an existing image.
    pub fn pull<'docker>(
        &'docker self,
        opts: &PullOpts,
    ) -> impl Stream<Item = Result<models::ImageBuildChunk>> + Unpin + 'docker {
        let headers = opts.auth_header().map(|a| Headers::single(AUTH_HEADER, a));

        Box::pin(self.docker.post_into_stream(
            construct_ep("/images/create", opts.serialize()),
            Payload::empty(),
            headers,
        ))
    }}

    api_doc! { Image => GetAll
    |
    /// Exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball.
    pub fn export<'docker>(
        &'docker self,
        names: Vec<&str>,
    ) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        self.docker
            .get_stream(format!(
                "/images/get?{}",
                encoded_pairs(names.iter().map(|n| ("names", *n)))
            ))
            .map_ok(|c| c.to_vec())
    }}

    api_doc! { Image => Load
    |
    /// Imports an image or set of images from a given tarball source.
    /// Source can be uncompressed on compressed via gzip, bzip2 or xz.
    pub fn import<'docker, R>(
        &'docker self,
        mut tarball: R,
    ) -> impl Stream<Item = Result<models::ImageBuildChunk>> + Unpin + 'docker
    where
        R: Read + Send + 'docker,
    {
        Box::pin(
            async move {
                let mut bytes = Vec::default();

                tarball.read_to_end(&mut bytes)?;

                let value_stream = self.docker.post_into_stream(
                    "/images/load",
                    Payload::Tar(bytes),
                    Headers::none(),
                );
                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }}

    api_doc! { Image => Push
    |
    /// Push an image to registry.
    pub async fn push(&self, name: impl Into<crate::Id>, opts: &ImagePushOpts) -> Result<()> {
        let image = Image::new(self.docker.clone(), name);
        image.push(opts).await
    }}

    api_doc! { Build => Prune
    |
    /// Clear image build cache.
    pub async fn clear_cache(
        &self,
        opts: &ClearCacheOpts,
    ) -> Result<models::BuildPrune200Response> {
        self.docker
            .post_json(
                construct_ep("/build/prune", opts.serialize()),
                Payload::empty(),
                Headers::none(),
            )
            .await
    }}
}
