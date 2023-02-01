//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::{
    conn::{Headers, Payload},
    models, Result,
};

impl_api_ty!(Secret => name);

impl Secret {
    impl_api_ep! { secret: Secret, resp
        Inspect -> &format!("/secrets/{}", secret.name), models::Secret
        Delete -> &format!("/secrets/{}", secret.name), ()
    }
    // TODO: add Secret::update
}

impl Secrets {
    impl_api_ep! { __: Secret, resp
        List -> "/secrets", models::Secret
    }

    api_doc! { Secret => Create
    |
    /// Create a new secret.
    pub async fn create(&self, opts: &SecretCreateOpts) -> Result<Secret> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SecretCreateResponse {
            #[serde(rename = "Id")]
            pub id: String,
        }
        self.docker
            .post_json("/secrets/create", Payload::Json(opts.serialize()?), Headers::none())
            .await
            .map(|resp: SecretCreateResponse| {
                Secret::new(self.docker.clone(), resp.id)
            })
    }}
}

pub mod opts {
    use crate::models::{Driver, Labels};
    use crate::{Error, Result};
    use containers_api::opts::{Filter, FilterItem};
    use containers_api::{impl_filter_func, impl_opts_builder};
    use serde::{Deserialize, Serialize};

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
        fn query_item(&self) -> FilterItem {
            use SecretFilter::*;
            match &self {
                Id(id) => FilterItem::new("id", id.to_owned()),
                LabelKey(label) => FilterItem::new("label", label.to_owned()),
                Label(key, val) => FilterItem::new("label", format!("{key}={val}")),
                Name(name) => FilterItem::new("name", name.to_owned()),
                Names(names) => FilterItem::new("names", names.to_owned()),
            }
        }
    }

    impl SecretListOptsBuilder {
        impl_filter_func!(
            /// Filter the list of filters by one of the variants of the enum.
            SecretFilter
        );
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
                driver: Driver {
                    name: "".into(),
                    options: None,
                },
                templating: Driver {
                    name: "".into(),
                    options: None,
                },
            }
        }

        /// Set the driver of this secret.
        pub fn set_driver(mut self, driver: Driver) -> Self {
            self.driver = driver;
            self
        }

        /// Set the templating driver of this secret.
        pub fn set_templating(mut self, driver: Driver) -> Self {
            self.templating = driver;
            self
        }

        /// Add a label to this secret
        pub fn add_label<K, V>(mut self, key: K, val: V) -> Self
        where
            K: Into<String>,
            V: Into<String>,
        {
            self.labels.insert(key.into(), val.into());
            self
        }

        pub fn serialize(&self) -> Result<String> {
            serde_json::to_string(&self).map_err(Error::from)
        }
    }
}

pub use opts::*;
