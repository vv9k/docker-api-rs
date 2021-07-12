#![cfg(feature = "swarm")]
//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::{conn::Payload, Result};

impl_api_ty!(Secret => name: N);

impl<'docker> Secret<'docker> {
    impl_api_ep! { secret: Secret, resp
        Inspect -> &format!("/secrets/{}", secret.name)
        Delete -> &format!("/secrets/{}", secret.name)
    }
    // TODO: add Secret::update
}

impl<'docker> Secrets<'docker> {
    impl_api_ep! { __: Secret, resp
        List -> "/secrets"
        Create -> "/secrets/create", resp.id
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
    pub struct SecretCreateOpts {
        name: String,
        labels: Labels,
        data: String,
        driver: Driver,
        templating: Driver,
    }

    impl SecretCreateOpts {
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

        pub fn serialize(&self) -> Result<String> {
            serde_json::to_string(&self).map_err(Error::from)
        }
    }

    #[derive(Deserialize)]
    pub(crate) struct SecretCreateInfo {
        #[serde(rename = "Id")]
        pub id: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretReference {
        pub file: Option<SecretReferenceFileTarget>,
        #[serde(rename = "SecretID")]
        pub secret_id: String,
        pub secret_name: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct SecretReferenceFileTarget {
        pub name: String,
        pub uid: String,
        pub gid: String,
        pub mode: u32,
    }
}

pub use data::*;

pub mod opts {
    use crate::api::Filter;

    impl_opts_builder!(url => SecretList);

    pub enum SecretFilter {
        /// The ID of the secret.
        Id(String),
        /// Label in the form of `label=key`
        LabelKey(String),
        /// Label in the form of `label=key=val`
        Label(String, String),
        /// The name of the secret.
        Name(String),
        Names(String),
    }

    impl Filter for SecretFilter {
        fn query_key_val(&self) -> (&'static str, String) {
            use SecretFilter::*;
            match &self {
                Id(id) => ("id", id.to_owned()),
                LabelKey(label) => ("label", label.to_owned()),
                Label(key, val) => ("label", format!("{}={}", key, val)),
                Name(name) => ("name", name.to_owned()),
                Names(names) => ("names", names.to_owned()),
            }
        }
    }

    impl SecretListOptsBuilder {
        impl_filter_func!(
            /// Filter the list of filters by one of the variants of the enum.
            SecretFilter
        );
    }
}

pub use opts::*;
