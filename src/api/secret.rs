//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::Result;

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
}

pub mod opts {
    use std::collections::HashMap;

    impl_url_opts_builder!(SecretList);

    pub enum SecretFilter {
        Id(String),
        LabelKey(String),
        LabelKeyVal(String, String),
        Name(String),
        Names(String),
    }

    impl SecretListOptsBuilder {
        pub fn filter<F>(&mut self, filters: F) -> &mut Self
        where
            F: IntoIterator<Item = SecretFilter>,
        {
            let mut param = HashMap::new();
            for f in filters {
                match f {
                    SecretFilter::Id(id) => param.insert("id", vec![id]),
                    SecretFilter::LabelKey(key) => param.insert("label", vec![key]),
                    SecretFilter::LabelKeyVal(key, val) => {
                        param.insert("label", vec![[key, val].join("=")])
                    }
                    SecretFilter::Name(name) => param.insert("name", vec![name]),
                    SecretFilter::Names(names) => param.insert("names", vec![names]),
                };
            }
            // structure is a a json encoded object mapping string keys to a list
            // of string values
            self.params
                .insert("filters", serde_json::to_string(&param).unwrap_or_default());
            self
        }
    }
}

pub use data::*;
pub use opts::*;

impl_api_ty!(Secret => name: N);

impl<'docker> Secret<'docker> {
    /// Inspects a secret.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretInspect)
    pub async fn inspect(&self) -> Result<SecretInfo> {
        self.docker
            .get_json(&format!("/secrets/{}", self.name)[..])
            .await
    }

    /// Delete a secret.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretDelete)
    pub async fn leave(&self) -> Result<()> {
        self.docker
            .delete(&format!("/secrets/{}", self.name))
            .await
            .map(|_| ())
    }
}

impl<'docker> Secrets<'docker> {
    /// List secrets.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SecretList)
    pub async fn list(&self, opts: &SecretListOpts) -> Result<Vec<SecretInfo>> {
        let mut path = vec!["/secrets".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<SecretInfo>>(&path.join("?"))
            .await
    }
}
