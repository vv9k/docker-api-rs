use crate::{api::Filter, util::url::encoded_pairs};

use std::collections::HashMap;

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
    impl_url_str_field!(repo: R => "repo");

    impl_url_str_field!(tag: T => "tag");
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

    impl_url_str_field!("set the name of the docker file. defaults to `DockerFile`" dockerfile: P => "dockerfile");

    impl_url_str_field!("tag this image with a name after building it" tag: T => "t");

    impl_url_str_field!("Extra hosts to add to /etc/hosts." extra_hosts: H => "extrahosts");

    impl_url_str_field!(remote: R => "remote");

    impl_url_bool_field!("Suppress verbose build output." quiet => "q");

    impl_url_bool_field!("Don't use the image cache when building image." nocahe => "nocache");

    impl_url_str_field!("Attempt to pull the image even if an older image exists locally."
                        pull: I => "pull");

    impl_url_bool_field!(rm => "rm");

    impl_url_bool_field!(forcerm => "forcerm");

    impl_url_field!("Set memory limit for build." memory: usize => "memory");

    impl_url_field!("Total memory (memory + swap). Set as -1 to disable swap." memswap: usize => "memswap");

    impl_url_field!("CPU shares (relative weight)." cpu_shares: usize => "cpushares");

    impl_url_str_field!("CPUs in which to allow execution (eg. `0-3`, `0,1`)" cpu_set_cpus: C => "cpusetcpus");

    impl_url_field!("The length of a CPU period in microseconds." cpu_period: usize => "cpuperiod");

    impl_url_field!(
        "Microseconds of CPU time that the container can get in a CPU period."
        cpu_quota: usize => "cpuquota"
    );

    // TODO: buildargs

    impl_url_field!(
        "Size of /dev/shm in bytes. The size must be greater than 0. If omitted the system uses 64MB."
        shm_size: usize => "shmsize"
    );

    impl_url_bool_field!(
        "Squash the resulting images layers into a single layer. (Experimental release only.)"
        squash => "squash"
    );

    impl_url_str_field!(
        "`bridge`, `host`, `none`, `container:<name|id>`, or a custom network name."
        network_mode: T => "networkmode"
    );

    impl_url_str_field!("Platform in the format os[/arch[/variant]]." platform: P => "platform");

    impl_url_str_field!("Target build stage." target: T => "target");

    impl_url_str_field!("BuildKit output configuration." outputs: C => "outputs");

    // TODO: labels

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

impl Filter for ImageFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        match &self {
            ImageFilter::Dangling => ("dangling", true.to_string()),
            ImageFilter::LabelName(n) => ("label", n.to_owned()),
            ImageFilter::Label(n, v) => ("label", format!("{}={}", n, v)),
        }
    }
}

impl_url_opts_builder!(derives = Default | ImageList);

impl ImageListOptsBuilder {
    impl_url_bool_field!(
        "Show all images. Only images from a final layer (no children) are shown by default."
        all => "all"
    );
    impl_url_bool_field!(
        "Show digest information as a RepoDigests field on each image."
        digests => "digests"
    );
    impl_filter_func!(ImageFilter);
}

impl_url_opts_builder!(derives = Default | ImageRemove);

impl ImageRemoveOptsBuilder {
    impl_url_bool_field!(
        "Remove the image even if it is being used by stopped containers or has other tags."
        force => "force"
    );
    impl_url_bool_field!("Do not delete untagged parent images." noprune => "noprune");
}

impl_url_opts_builder!(ImagePrune);

pub enum ImagePruneFilter {
    Dangling(bool),
    Until(String),
    LabelKey(String),
    LabelKeyVal(String, String),
}

impl Filter for ImagePruneFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use ImagePruneFilter::*;
        match &self {
            Dangling(dangling) => ("dangling", dangling.to_string()),
            Until(until) => ("until", until.to_owned()),
            LabelKey(label) => ("label", label.to_owned()),
            LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
        }
    }
}

impl ImagePruneOptsBuilder {
    impl_filter_func!(ImagePruneFilter);
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
