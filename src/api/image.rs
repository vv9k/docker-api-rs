//! Create and manage images.

use std::{collections::HashMap, io::Read};

use futures_util::{stream::Stream, TryFutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};

use crate::{
    conn::{Headers, Payload},
    docker::Docker,
    errors::Result,
    util::{
        tarball,
        url::{encoded_pair, encoded_pairs},
    },
};

#[cfg(feature = "chrono")]
use crate::util::datetime::datetime_from_unix_timestamp;
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

impl_api_ty!(Image => name: N);

impl<'docker> Image<'docker> {
    /// Inspects a named image's details
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageInspect)
    pub async fn inspect(&self) -> Result<ImageDetails> {
        self.docker
            .get_json(&format!("/images/{}/json", self.name)[..])
            .await
    }

    /// Lists the history of the images set of changes
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageHistory)
    pub async fn history(&self) -> Result<Vec<History>> {
        self.docker
            .get_json(&format!("/images/{}/history", self.name)[..])
            .await
    }

    /// Deletes an image
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImagePrune)
    pub async fn delete(&self) -> Result<Vec<Status>> {
        self.docker
            .delete_json(&format!("/images/{}", self.name)[..])
            .await
    }

    /// Export this image to a tarball
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageGet)
    pub fn export(&self) -> impl Stream<Item = Result<Vec<u8>>> + Unpin + 'docker {
        Box::pin(
            self.docker
                .stream_get(format!("/images/{}/get", self.name))
                .map_ok(|c| c.to_vec()),
        )
    }

    /// Adds a tag to an image
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageTag)
    pub async fn tag(&self, opts: &TagOpts) -> Result<()> {
        let mut path = vec![format!("/images/{}/tag", self.name)];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }
        let _ = self.docker.post(&path.join("?"), Payload::empty()).await?;
        Ok(())
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
    /// Builds a new image build by reading a Dockerfile in a target directory
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageBuild)
    pub fn build(
        &self,
        opts: &BuildOpts,
    ) -> impl Stream<Item = Result<ImageBuildChunk>> + Unpin + 'docker {
        let mut endpoint = vec!["/build".to_owned()];
        if let Some(query) = opts.serialize() {
            endpoint.push(query)
        }

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

                let value_stream = docker.stream_post_into(
                    endpoint.join("?"),
                    Payload::Tar(bytes),
                    Headers::none(),
                );

                Ok(value_stream)
            }
            .try_flatten_stream(),
        )
    }

    /// Lists the docker images on the current docker host
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageList)
    pub async fn list(&self, opts: &ImageListOpts) -> Result<Vec<ImageInfo>> {
        let mut path = vec!["/images/json".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<ImageInfo>>(&path.join("?"))
            .await
    }

    /// Search for docker images by term
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageSearch)
    pub async fn search(&self, term: &str) -> Result<Vec<SearchResult>> {
        let query = encoded_pair("term", term);
        self.docker
            .get_json::<Vec<SearchResult>>(&format!("/images/search?{}", query)[..])
            .await
    }

    /// Pull and create a new docker images from an existing image
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImagePull)
    pub fn pull(
        &self,
        opts: &PullOpts,
    ) -> impl Stream<Item = Result<ImageBuildChunk>> + Unpin + 'docker {
        let mut path = vec!["/images/create".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        let headers = opts
            .auth_header()
            .map(|a| Headers::single("X-Registry-Auth", a));

        Box::pin(
            self.docker
                .stream_post_into(path.join("?"), Payload::empty(), headers),
        )
    }

    /// exports a collection of named images,
    /// either by name, name:tag, or image id, into a tarball
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ImageGetAll)
    pub fn export(&self, names: Vec<&str>) -> impl Stream<Item = Result<Vec<u8>>> + 'docker {
        let params = names.iter().map(|n| ("names", *n));
        let query = encoded_pairs(params);
        self.docker
            .stream_get(format!("/images/get?{}", query))
            .map_ok(|c| c.to_vec())
    }

    /// imports an image or set of images from a given tarball source
    /// source can be uncompressed on compressed via gzip, bzip2 or xz
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
}

#[derive(Clone, Serialize, Debug)]
#[serde(untagged)]
pub enum RegistryAuth {
    Password {
        username: String,
        password: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,

        #[serde(rename = "serveraddress")]
        #[serde(skip_serializing_if = "Option::is_none")]
        server_address: Option<String>,
    },
    Token {
        #[serde(rename = "identitytoken")]
        identity_token: String,
    },
}

impl RegistryAuth {
    /// return a new instance with token authentication
    pub fn token<S>(token: S) -> RegistryAuth
    where
        S: Into<String>,
    {
        RegistryAuth::Token {
            identity_token: token.into(),
        }
    }

    /// return a new instance of a builder for authentication
    pub fn builder() -> RegistryAuthBuilder {
        RegistryAuthBuilder::default()
    }

    /// serialize authentication as JSON in base64
    pub fn serialize(&self) -> String {
        serde_json::to_string(self)
            .map(|c| base64::encode_config(&c, base64::URL_SAFE))
            .unwrap_or_default()
    }
}

#[derive(Default)]
pub struct RegistryAuthBuilder {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    server_address: Option<String>,
}

impl RegistryAuthBuilder {
    pub fn username<I>(&mut self, username: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.username = Some(username.into());
        self
    }

    pub fn password<I>(&mut self, password: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.password = Some(password.into());
        self
    }

    pub fn email<I>(&mut self, email: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.email = Some(email.into());
        self
    }

    pub fn server_address<I>(&mut self, server_address: I) -> &mut Self
    where
        I: Into<String>,
    {
        self.server_address = Some(server_address.into());
        self
    }

    pub fn build(&self) -> RegistryAuth {
        RegistryAuth::Password {
            username: self.username.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default(),
            email: self.email.clone(),
            server_address: self.server_address.clone(),
        }
    }
}

impl_url_opts_builder!(Tag);

impl TagOptsBuilder {
    pub fn repo<R>(&mut self, r: R) -> &mut Self
    where
        R: Into<String>,
    {
        self.params.insert("repo", r.into());
        self
    }

    pub fn tag<T>(&mut self, t: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.params.insert("tag", t.into());
        self
    }
}

#[derive(Default, Debug)]
pub struct PullOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, serde_json::Value>,
}

impl PullOpts {
    /// return a new instance of a builder for Opts
    pub fn builder() -> PullOptsBuilder {
        PullOptsBuilder::default()
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(encoded_pairs(
                self.params
                    .iter()
                    .map(|(k, v)| (k, v.as_str().unwrap_or_default())),
            ))
        }
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

pub struct PullOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, serde_json::Value>,
}

impl Default for PullOptsBuilder {
    fn default() -> Self {
        let mut params = HashMap::new();
        params.insert("tag", serde_json::Value::String("latest".into()));

        PullOptsBuilder { auth: None, params }
    }
}

impl PullOptsBuilder {
    impl_str_field!(
    " Name of the image to pull. The name may include a tag or digest."
    "This parameter may only be used when pulling an image."
    "If an untagged value is provided and no `tag` is provided, _all_"
    "tags will be pulled"
    "The pull is cancelled if the HTTP connection is closed."
    image: I => "fromImage");

    impl_str_field!(src: S => "fromSrc");

    impl_str_field!(
    "Repository name given to an image when it is imported. The repo may include a tag."
    "This parameter may only be used when importing an image."
    ""
    "By default a `latest` tag is added when calling"
    "[PullOptsBuilder::default](PullOptsBuilder::default]."
    repo: S => "repo");

    impl_str_field!(
    "Tag or digest. If empty when pulling an image,"
    "this causes all tags for the given image to be pulled."
    tag: T => "tag");

    pub fn auth(&mut self, auth: RegistryAuth) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(&mut self) -> PullOpts {
        PullOpts {
            auth: self.auth.take(),
            params: self.params.clone(),
        }
    }
}

#[derive(Default, Debug)]
pub struct BuildOpts {
    pub path: String,
    params: HashMap<&'static str, String>,
}

impl BuildOpts {
    /// return a new instance of a builder for Opts
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub fn builder<S>(path: S) -> BuildOptsBuilder
    where
        S: Into<String>,
    {
        BuildOptsBuilder::new(path)
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(encoded_pairs(&self.params))
        }
    }
}

#[derive(Default)]
pub struct BuildOptsBuilder {
    path: String,
    params: HashMap<&'static str, String>,
}

impl BuildOptsBuilder {
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub(crate) fn new<P>(path: P) -> Self
    where
        P: Into<String>,
    {
        BuildOptsBuilder {
            path: path.into(),
            ..Default::default()
        }
    }

    /// set the name of the docker file. defaults to "DockerFile"
    pub fn dockerfile<P>(&mut self, path: P) -> &mut Self
    where
        P: Into<String>,
    {
        self.params.insert("dockerfile", path.into());
        self
    }

    /// tag this image with a name after building it
    pub fn tag<T>(&mut self, t: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.params.insert("t", t.into());
        self
    }

    pub fn remote<R>(&mut self, r: R) -> &mut Self
    where
        R: Into<String>,
    {
        self.params.insert("remote", r.into());
        self
    }

    /// don't use the image cache when building image
    pub fn nocache(&mut self, nc: bool) -> &mut Self {
        self.params.insert("nocache", nc.to_string());
        self
    }

    pub fn rm(&mut self, r: bool) -> &mut Self {
        self.params.insert("rm", r.to_string());
        self
    }

    pub fn forcerm(&mut self, fr: bool) -> &mut Self {
        self.params.insert("forcerm", fr.to_string());
        self
    }

    /// `bridge`, `host`, `none`, `container:<name|id>`, or a custom network name.
    pub fn network_mode<T>(&mut self, t: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.params.insert("networkmode", t.into());
        self
    }

    pub fn memory(&mut self, memory: u64) -> &mut Self {
        self.params.insert("memory", memory.to_string());
        self
    }

    pub fn cpu_shares(&mut self, cpu_shares: u32) -> &mut Self {
        self.params.insert("cpushares", cpu_shares.to_string());
        self
    }

    // todo: memswap
    // todo: cpusetcpus
    // todo: cpuperiod
    // todo: cpuquota
    // todo: buildargs

    pub fn build(&self) -> BuildOpts {
        BuildOpts {
            path: self.path.clone(),
            params: self.params.clone(),
        }
    }
}

/// Filter Opts for image listings
pub enum ImageFilter {
    Dangling,
    LabelName(String),
    Label(String, String),
}

impl_url_opts_builder!(ImageList);

impl ImageListOptsBuilder {
    pub fn digests(&mut self, d: bool) -> &mut Self {
        self.params.insert("digests", d.to_string());
        self
    }

    pub fn all(&mut self) -> &mut Self {
        self.params.insert("all", "true".into());
        self
    }

    pub fn filter_name<F>(&mut self, name: F) -> &mut Self
    where
        F: Into<String>,
    {
        self.params.insert("filter", name.into());
        self
    }

    pub fn filter<F>(&mut self, filters: F) -> &mut Self
    where
        F: IntoIterator<Item = ImageFilter>,
    {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                ImageFilter::Dangling => param.insert("dangling", vec![true.to_string()]),
                ImageFilter::LabelName(n) => param.insert("label", vec![n]),
                ImageFilter::Label(n, v) => param.insert("label", vec![format!("{}={}", n, v)]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub description: String,
    pub is_official: bool,
    pub is_automated: bool,
    pub name: String,
    pub star_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInfo {
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub id: String,
    pub parent_id: String,
    pub labels: Option<HashMap<String, String>>,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub virtual_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDetails {
    pub architecture: String,
    pub author: String,
    pub comment: String,
    pub config: ContainerConfig,
    #[cfg(feature = "chrono")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: String,
    pub docker_version: String,
    pub id: String,
    pub os: String,
    pub parent: String,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub size: u64,
    pub virtual_size: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    pub cmd: Option<Vec<String>>,
    pub domainname: String,
    pub entrypoint: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub exposed_ports: Option<HashMap<String, HashMap<String, String>>>,
    pub hostname: String,
    pub image: String,
    pub labels: Option<HashMap<String, String>>,
    // pub MacAddress: String,
    pub on_build: Option<Vec<String>>,
    // pub NetworkDisabled: bool,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub user: String,
    pub working_dir: String,
}

impl ContainerConfig {
    pub fn env(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(ref vars) = self.env {
            for e in vars {
                let pair: Vec<&str> = e.split('=').collect();
                map.insert(pair[0].to_owned(), pair[1].to_owned());
            }
        }
        map
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct History {
    pub id: String,
    #[cfg(feature = "chrono")]
    #[serde(deserialize_with = "datetime_from_unix_timestamp")]
    pub created: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub created: u64,
    pub created_by: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Status {
    Untagged(String),
    Deleted(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
/// Represents a response chunk from Docker api when building, pulling or importing an image.
pub enum ImageBuildChunk {
    Update {
        stream: String,
    },
    Error {
        error: String,
        #[serde(rename = "errorDetail")]
        error_detail: ErrorDetail,
    },
    Digest {
        aux: Aux,
    },
    PullStatus {
        status: String,
        id: Option<String>,
        progress: Option<String>,
        #[serde(rename = "progressDetail")]
        progress_detail: Option<ProgressDetail>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Aux {
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetail {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgressDetail {
    pub current: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Descriptor {
    media_type: String,
    digest: String,
    size: u64,
    #[serde(rename = "URLs")]
    urls: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DistributionInspectInfo {
    descriptor: Descriptor,
    platforms: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test registry auth with token
    #[test]
    fn registry_auth_token() {
        let opts = RegistryAuth::token("abc");
        assert_eq!(
            base64::encode(r#"{"identitytoken":"abc"}"#),
            opts.serialize()
        );
    }

    /// Test registry auth with username and password
    #[test]
    fn registry_auth_password_simple() {
        let opts = RegistryAuth::builder()
            .username("user_abc")
            .password("password_abc")
            .build();
        assert_eq!(
            base64::encode(r#"{"username":"user_abc","password":"password_abc"}"#),
            opts.serialize()
        );
    }

    /// Test registry auth with all fields
    #[test]
    fn registry_auth_password_all() {
        let opts = RegistryAuth::builder()
            .username("user_abc")
            .password("password_abc")
            .email("email_abc")
            .server_address("https://example.org")
            .build();
        assert_eq!(
            base64::encode(
                r#"{"username":"user_abc","password":"password_abc","email":"email_abc","serveraddress":"https://example.org"}"#
            ),
            opts.serialize()
        );
    }
}