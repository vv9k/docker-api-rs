#![cfg(feature = "swarm")]
//! Configs are application configurations that can be used by services.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{conn::Payload, Result};

impl_api_ty!(Config => name: N);

impl<'docker> Config<'docker> {
    impl_api_ep! { cfg: Config, resp
        Inspect -> &format!("/configs/{}", cfg.name)
        Delete -> &format!("/configs/{}", cfg.name)
    }

    // TODO: add Config::update
}

impl<'docker> Configs<'docker> {
    impl_api_ep! { __: Config, resp
        List -> "/configs"
        Create -> "/configs/create", resp.id
    }
}

pub mod data {
    use crate::{
        api::{Driver, Labels, ObjectVersion},
        Error, Result,
    };
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
    pub struct ConfigCreateOpts {
        name: String,
        labels: Labels,
        data: String,
        templating: Driver,
    }

    impl ConfigCreateOpts {
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

        pub fn serialize(&self) -> Result<String> {
            serde_json::to_string(&self).map_err(Error::from)
        }
    }

    #[derive(Deserialize)]
    pub(crate) struct ConfigCreateInfo {
        #[serde(rename = "Id")]
        pub id: String,
    }
}

pub use data::*;

pub mod opts {
    use crate::api::Filter;

    impl_url_opts_builder!(ConfigList);

    pub enum ConfigFilter {
        /// The ID of the config.
        Id(String),
        /// Label in the form of `label=key`
        LabelKey(String),
        /// Label in the form of `label=key=val`
        Label(String, String),
        /// The name of the config.
        Name(String),
        Names(String),
    }

    impl Filter for ConfigFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            use ConfigFilter::*;
            match &self {
                Id(id) => ("id", id.to_owned()),
                LabelKey(label) => ("label", label.to_owned()),
                Label(key, val) => ("label", format!("{}={}", key, val)),
                Name(name) => ("name", name.to_owned()),
                Names(names) => ("names", names.to_owned()),
            }
        }
    }

    impl ConfigListOptsBuilder {
        impl_filter_func!(
            /// Filter listed configs by variants of the enum.
            ConfigFilter
        );
    }
}

pub use opts::*;
