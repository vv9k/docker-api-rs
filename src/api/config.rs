#![cfg(feature = "swarm")]
//! Configs are application configurations that can be used by services.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{conn::Payload, util::url::construct_ep, Result};

pub mod data {
    use crate::api::{Driver, Labels, ObjectVersion};
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "chrono")]
    use chrono::{DateTime, Utc};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ConfigInfo {
        #[serde(rename = "ID")]
        pub id: String,
        pub version: ObjectVersion,
        #[cfg(feature = "chrono")]
        pub created_at: DateTime<Utc>,
        #[cfg(not(feature = "chrono"))]
        pub created_at: String,
        #[cfg(feature = "chrono")]
        pub updated_at: DateTime<Utc>,
        #[cfg(not(feature = "chrono"))]
        pub updated_at: String,
        pub spec: ConfigSpec,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ConfigSpec {
        pub name: String,
        pub labels: Labels,
        pub data: String,
        pub templating: Driver,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    /// Structure used to create a new config with [`Configs::create`](crate::Configs::create).
    pub struct ConfigCreate {
        name: String,
        labels: Labels,
        data: String,
        templating: Driver,
    }

    impl ConfigCreate {
        /// Create a new config with name and data. This function will take care of
        /// encoding the config's data as base64.
        pub fn new<N, D>(name: N, data: D) -> Self
        where
            N: Into<String>,
            D: AsRef<str>,
        {
            Self {
                name: name.into(),
                labels: Labels::new(),
                data: base64::encode(data.as_ref()),
                templating: Driver::default(),
            }
        }

        /// Set the templating driver of this config.
        pub fn set_templating(&mut self, driver: Driver) {
            self.templating = driver;
        }

        /// Add a label to this config
        pub fn add_label<K, V>(&mut self, key: K, val: V) -> Option<String>
        where
            K: Into<String>,
            V: Into<String>,
        {
            self.labels.insert(key.into(), val.into())
        }
    }

    #[derive(Deserialize)]
    pub(crate) struct ConfigCreateResponse {
        #[serde(rename = "Id")]
        pub id: String,
    }
}

pub mod opts {
    use crate::api::Filter;

    impl_url_opts_builder!(ConfigList);

    pub enum ConfigFilter {
        Id(String),
        LabelKey(String),
        LabelKeyVal(String, String),
        Name(String),
        Names(String),
    }

    impl Filter for ConfigFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            use ConfigFilter::*;
            match &self {
                Id(id) => ("id", id.to_owned()),
                LabelKey(label) => ("label", label.to_owned()),
                LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
                Name(name) => ("name", name.to_owned()),
                Names(names) => ("names", names.to_owned()),
            }
        }
    }

    impl ConfigListOptsBuilder {
        impl_filter_func!(ConfigFilter);
    }
}

pub use data::*;
pub use opts::*;

impl_api_ty!(Config => name: N);

impl<'docker> Config<'docker> {
    /// Inspects a config.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ConfigInspect)
    pub async fn inspect(&self) -> Result<ConfigInfo> {
        self.docker
            .get_json(&format!("/configs/{}", self.name))
            .await
    }

    /// Delete a config.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ConfigDelete)
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/configs/{}", self.name))
            .await
            .map(|_| ())
    }

    // TODO: add Config::update
}

impl<'docker> Configs<'docker> {
    /// List existing configs.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ConfigList)
    pub async fn list(&self, opts: &ConfigListOpts) -> Result<Vec<ConfigInfo>> {
        self.docker
            .get_json(&construct_ep("/configs", opts.serialize()))
            .await
    }

    /// Create a new config.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/ConfigCreate)
    pub async fn create(&self, new_config: &ConfigCreate) -> Result<Config<'_>> {
        self.docker
            .post_json(
                "/configs/create",
                Payload::Json(serde_json::to_string(&new_config)?),
            )
            .await
            .map(|resp: ConfigCreateResponse| Config::new(self.docker, resp.id))
    }
}
