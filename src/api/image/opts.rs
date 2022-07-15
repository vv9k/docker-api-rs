use containers_api::opts::Filter;
use containers_api::url::encoded_pairs;
use containers_api::{
    impl_filter_func, impl_map_field, impl_opts_builder, impl_str_field, impl_url_bool_field,
    impl_url_field, impl_url_str_field,
};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    string::ToString,
};

use serde::Serialize;

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
    /// The username used for authentication.
    pub fn username<U>(mut self, username: U) -> Self
    where
        U: Into<String>,
    {
        self.username = Some(username.into());
        self
    }

    /// The password used for authentication.
    pub fn password<P>(mut self, password: P) -> Self
    where
        P: Into<String>,
    {
        self.password = Some(password.into());
        self
    }

    /// The email addres used for authentication.
    pub fn email<E>(mut self, email: E) -> Self
    where
        E: Into<String>,
    {
        self.email = Some(email.into());
        self
    }

    /// The server address of registry, should be a domain/IP without a protocol.
    /// Example: `10.92.0.1`, `docker.corp.local`
    pub fn server_address<A>(mut self, server_address: A) -> Self
    where
        A: Into<String>,
    {
        self.server_address = Some(server_address.into());
        self
    }

    /// Create the final authentication object.
    pub fn build(&self) -> RegistryAuth {
        RegistryAuth::Password {
            username: self.username.clone().unwrap_or_default(),
            password: self.password.clone().unwrap_or_default(),
            email: self.email.clone(),
            server_address: self.server_address.clone(),
        }
    }
}

impl_opts_builder!(url => Tag);

impl TagOptsBuilder {
    impl_url_str_field!(repo => "repo");

    impl_url_str_field!(tag => "tag");
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
    /// Name of the image to pull. The name may include a tag or digest.
    /// This parameter may only be used when pulling an image.
    /// If an untagged value is provided and no `tag` is provided, _all_
    /// tags will be pulled
    /// The pull is cancelled if the HTTP connection is closed.
    image => "fromImage");

    impl_str_field!(src => "fromSrc");

    impl_str_field!(
    /// Repository name given to an image when it is imported. The repo may include a tag.
    /// This parameter may only be used when importing an image.
    /// 
    /// By default a `latest` tag is added when calling
    /// [PullOptsBuilder::default](PullOptsBuilder::default).
    repo => "repo");

    impl_str_field!(
    /// Tag or digest. If empty when pulling an image,
    /// this causes all tags for the given image to be pulled.
    tag => "tag");

    pub fn auth(mut self, auth: RegistryAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> PullOpts {
        PullOpts {
            auth: self.auth,
            params: self.params,
        }
    }
}

#[derive(Default, Debug)]
pub struct BuildOpts {
    pub path: PathBuf,
    params: HashMap<&'static str, String>,
}

impl BuildOpts {
    /// return a new instance of a builder for Opts
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub fn builder<P>(path: P) -> BuildOptsBuilder
    where
        P: AsRef<Path>,
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
    path: PathBuf,
    params: HashMap<&'static str, String>,
}

impl BuildOptsBuilder {
    /// path is expected to be a file path to a directory containing a Dockerfile
    /// describing how to build a Docker image
    pub(crate) fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        BuildOptsBuilder {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    impl_url_str_field!(
        /// Set the name of the docker file. defaults to `DockerFile`.
        dockerfile => "dockerfile"
    );

    impl_url_str_field!(
        /// Tag this image with a name after building it.
        tag => "t"
    );

    impl_url_str_field!(
        /// Extra hosts to add to /etc/hosts.
        extra_hosts => "extrahosts"
    );

    impl_url_str_field!(remote => "remote");

    impl_url_bool_field!(
        /// Suppress verbose build output.
        quiet => "q"
    );

    impl_url_bool_field!(
        /// Don't use the image cache when building image.
        nocahe => "nocache"
    );

    impl_url_str_field!(
        /// Attempt to pull the image even if an older image exists locally.
        pull => "pull"
    );

    impl_url_bool_field!(rm => "rm");

    impl_url_bool_field!(forcerm => "forcerm");

    impl_url_field!(
        /// Set memory limit for build.
        memory: usize => "memory"
    );

    impl_url_field!(
        /// Total memory (memory + swap). Set as -1 to disable swap.
        memswap: usize => "memswap"
    );

    impl_url_field!(
        /// CPU shares (relative weight).
        cpu_shares: usize => "cpushares"
    );

    impl_url_str_field!(
        /// CPUs in which to allow execution (eg. `0-3`, `0,1`)
        cpu_set_cpus => "cpusetcpus"
    );

    impl_url_field!(
        /// The length of a CPU period in microseconds.
        cpu_period: usize => "cpuperiod"
    );

    impl_url_field!(
        /// Microseconds of CPU time that the container can get in a CPU period.
        cpu_quota: usize => "cpuquota"
    );

    // TODO: buildargs

    impl_url_field!(
        /// Size of /dev/shm in bytes. The size must be greater than 0. If omitted the system uses 64MB.
        shm_size: usize => "shmsize"
    );

    impl_url_bool_field!(
        /// Squash the resulting images layers into a single layer. (Experimental release only.)
        squash => "squash"
    );

    // TODO: use an enum?
    impl_url_str_field!(
        /// bridge`, `host`, `none`, `container:<name|id>`, or a custom network name.
        network_mode => "networkmode"
    );

    impl_url_str_field!(
        /// Platform in the format os[/arch[/variant]].
        platform => "platform"
    );

    impl_url_str_field!(
        /// Target build stage.
        target => "target"
    );

    impl_url_str_field!(
        /// BuildKit output configuration.
        outputs => "outputs"
    );

    impl_map_field!(url
        /// Add labels to this image.
        labels => "labels"
    );

    pub fn build(&self) -> BuildOpts {
        BuildOpts {
            path: self.path.clone(),
            params: self.params.clone(),
        }
    }
}

/// All forms that the image identifier can take.
pub enum ImageName {
    /// `<image>[:<tag>]`
    Tag { image: String, tag: Option<String> },
    /// `<image-id>`
    Id(String),
    /// `<image@digest>`
    Digest { image: String, digest: String },
}

impl ToString for ImageName {
    fn to_string(&self) -> String {
        match &self {
            ImageName::Tag { image, tag } => match tag {
                Some(tag) => format!("{}:{}", image, tag),
                None => image.to_owned(),
            },
            ImageName::Id(id) => id.to_owned(),
            ImageName::Digest { image, digest } => format!("{}@{}", image, digest),
        }
    }
}

impl ImageName {
    /// Create a [`Tag`](ImageName::Tag) variant of image name.
    pub fn tag<I, T>(image: I, tag: Option<T>) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        Self::Tag {
            image: image.into(),
            tag: tag.map(|t| t.into()),
        }
    }

    /// Create a [`Id`](ImageName::Id) variant of image name.
    pub fn id<I>(id: I) -> Self
    where
        I: Into<String>,
    {
        Self::Id(id.into())
    }

    /// Create a [`Digest`](ImageName::Digest) variant of image name.
    pub fn digest<I, D>(image: I, digest: D) -> Self
    where
        I: Into<String>,
        D: Into<String>,
    {
        Self::Digest {
            image: image.into(),
            digest: digest.into(),
        }
    }
}

/// Filter type used to filter listed images.
pub enum ImageFilter {
    Before(ImageName),
    Dangling,
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
    Since(ImageName),
}

impl Filter for ImageFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ImageFilter::*;
        match &self {
            Before(name) => ("before", name.to_string()),
            Dangling => ("dangling", true.to_string()),
            LabelKey(n) => ("label", n.to_owned()),
            Label(n, v) => ("label", format!("{}={}", n, v)),
            Since(name) => ("since", name.to_string()),
        }
    }
}

impl_opts_builder!(url => ImageList);

impl ImageListOptsBuilder {
    impl_url_bool_field!(
        /// Show all images. Only images from a final layer (no children) are shown by default.
        all => "all"
    );
    impl_url_bool_field!(
        /// Show digest information as a RepoDigests field on each image.
        digests => "digests"
    );
    impl_filter_func!(
        /// Filter the listed images by one of the variants of the enum.
        ImageFilter
    );
}

impl_opts_builder!(url => RmImage);

impl RmImageOptsBuilder {
    impl_url_bool_field!(
        /// Remove the image even if it is being used by stopped containers or has other tags.
        force => "force"
    );
    impl_url_bool_field!(
        /// Do not delete untagged parent images.
        noprune => "noprune"
    );
}

impl_opts_builder!(url => ImagePrune);

pub enum ImagesPruneFilter {
    /// When set to `true`, prune only unused and untagged images.
    /// When set to `false`, all unused images are pruned.
    Dangling(bool),
    #[cfg(feature = "chrono")]
    #[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
    /// Prune images created before this timestamp. Same as `Until` but takes a datetime object.
    UntilDate(chrono::DateTime<chrono::Utc>),
    /// Prune images created before this timestamp. The <timestamp> can be Unix timestamps,
    /// date formatted timestamps, or Go duration strings (e.g. 10m, 1h30m)
    /// computed relative to the daemon machine’s time.
    Until(String),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label(String, String),
}

impl Filter for ImagesPruneFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ImagesPruneFilter::*;
        match &self {
            Dangling(dangling) => ("dangling", dangling.to_string()),
            Until(until) => ("until", until.to_owned()),
            #[cfg(feature = "chrono")]
            UntilDate(until) => ("until", until.timestamp().to_string()),
            LabelKey(label) => ("label", label.to_owned()),
            Label(key, val) => ("label", format!("{}={}", key, val)),
        }
    }
}

impl ImagePruneOptsBuilder {
    impl_filter_func!(ImagesPruneFilter);
}

impl_opts_builder!(url => ClearCache);

pub enum CacheFilter {
    /// Duration relative to daemon's time, during which build cache was not used,
    /// in Go's duration format (e.g., '24h').
    Until(String),
    Id(String),
    // ID of the parent.
    Parent(String),
    Type(String),
    Description(String),
    InUse,
    Shared,
    Private,
}

impl Filter for CacheFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use CacheFilter::*;
        match &self {
            Until(until) => ("until", until.to_owned()),
            Id(id) => ("id", id.to_owned()),
            Parent(parent) => ("parent", parent.to_owned()),
            Type(type_) => ("type_", type_.to_owned()),
            Description(description) => ("description", description.to_owned()),
            InUse => ("inuse", "".to_owned()),
            Shared => ("shared", "".to_owned()),
            Private => ("private", "".to_owned()),
        }
    }
}

impl ClearCacheOptsBuilder {
    impl_url_field!(
        /// Amount of disk space in bytes to keep for cache.
        keep_storage: i64 => "keep-storage"
    );
    impl_url_bool_field!(
        /// Remove all types of build cache
        all => "all"
    );
    impl_filter_func!(
        /// Filter the builder cache with variants of the enum.
        CacheFilter
    );
}

pub struct ImagePushOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl ImagePushOpts {
    pub fn builder() -> ImagePushOptsBuilder {
        ImagePushOptsBuilder::default()
    }

    pub fn serialize(&self) -> Option<String> {
        if self.params.is_empty() {
            None
        } else {
            Some(encoded_pairs(self.params.iter()))
        }
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

pub struct ImagePushOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, String>,
}

impl Default for ImagePushOptsBuilder {
    fn default() -> Self {
        Self {
            auth: None,
            params: [("tag", "latest".into())].into(),
        }
    }
}

impl ImagePushOptsBuilder {
    impl_url_str_field!(
        /// The tag to associate with the image on the registry.
        tag => "tag"
    );

    pub fn auth(mut self, auth: RegistryAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> ImagePushOpts {
        ImagePushOpts {
            auth: self.auth,
            params: self.params,
        }
    }
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
