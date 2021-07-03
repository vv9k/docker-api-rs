#![cfg(feature = "swarm")]
//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::{conn::Payload, util::url::construct_ep, Result};

pub mod data {
    use crate::api::{Driver, Labels, ObjectVersion};
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "chrono")]
    use chrono::{DateTime, Utc};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretInfo {
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
        pub spec: SecretSpec,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretSpec {
        pub name: String,
        pub labels: Labels,
        pub data: String,
        pub driver: Driver,
        pub templating: Driver,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    /// Structure used to create a new secret with [`Secrets::create`](crate::Secrets::create).
    pub struct SecretCreate {
        name: String,
        labels: Labels,
        data: String,
        driver: Driver,
        templating: Driver,
    }

    impl SecretCreate {
        /// Create a new secret with name and data. This function will take care of
        /// encoding the secret's data as base64.
        pub fn new<N, D>(name: N, data: D) -> Self
        where
            N: Into<String>,
            D: AsRef<str>,
        {
            Self {
                name: name.into(),
                labels: Labels::new(),
                data: base64::encode(data.as_ref()),
                driver: Driver::default(),
                templating: Driver::default(),
            }
        }

        /// Set the driver of this secret.
        pub fn set_driver(&mut self, driver: Driver) {
            self.driver = driver;
        }

        /// Set the templating driver of this secret.
        pub fn set_templating(&mut self, driver: Driver) {
            self.templating = driver;
        }

        /// Add a label to this secret
        pub fn add_label<K, V>(&mut self, key: K, val: V) -> Option<String>
        where
            K: Into<String>,
            V: Into<String>,
        {
            self.labels.insert(key.into(), val.into())
        }
    }

    #[derive(Deserialize)]
    pub(crate) struct SecretCreateResponse {
        #[serde(rename = "Id")]
        pub id: String,
    }
}

pub mod opts {
    use crate::api::Filter;

    impl_url_opts_builder!(SecretList);

    pub enum SecretFilter {
        Id(String),
        LabelKey(String),
        LabelKeyVal(String, String),
        Name(String),
        Names(String),
    }

    impl Filter for SecretFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            use SecretFilter::*;
            match &self {
                Id(id) => ("id", id.to_owned()),
                LabelKey(label) => ("label", label.to_owned()),
                LabelKeyVal(key, val) => ("label", format!("{}={}", key, val)),
                Name(name) => ("name", name.to_owned()),
                Names(names) => ("names", names.to_owned()),
            }
        }
    }

    impl SecretListOptsBuilder {
        impl_filter_func!(SecretFilter);
    }
}

pub use data::*;
pub use opts::*;

impl_api_ty!(Secret => name: N);

impl<'docker> Secret<'docker> {
    api_doc! { Secret => Inspect
    /// Inspects a secret.
    |
    pub async fn inspect(&self) -> Result<SecretInfo> {
        self.docker
            .get_json(&format!("/secrets/{}", self.name))
            .await
    }}

    api_doc! { Secret => Delete
    /// Delete a secret.
    |
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/secrets/{}", self.name))
            .await
            .map(|_| ())
    }}

    // TODO: add Secret::update
}

impl<'docker> Secrets<'docker> {
    api_doc! { Secret => List
    /// List existing secrets.
    |
    pub async fn list(&self, opts: &SecretListOpts) -> Result<Vec<SecretInfo>> {
        self.docker
            .get_json(&construct_ep("/secrets", opts.serialize()))
            .await
    }}

    api_doc! { Secret => Create
    /// Create a new secret.
    |
    pub async fn create(&self, new_secret: &SecretCreate) -> Result<Secret<'_>> {
        self.docker
            .post_json(
                "/secrets/create",
                Payload::Json(serde_json::to_string(&new_secret)?),
            )
            .await
            .map(|resp: SecretCreateResponse| Secret::new(self.docker, resp.id))
    }}
}
