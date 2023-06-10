use crate::models::{Driver, Labels};
use crate::{Error, Result};
use containers_api::opts::{Filter, FilterItem};
use containers_api::{impl_filter_func, impl_opts_builder};
use serde::{Deserialize, Serialize};

impl_opts_builder!(url => ConfigList);

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
    fn query_item(&self) -> FilterItem {
        use ConfigFilter::*;
        match &self {
            Id(id) => FilterItem::new("id", id.to_owned()),
            LabelKey(label) => FilterItem::new("label", label.to_owned()),
            Label(key, val) => FilterItem::new("label", format!("{key}={val}")),
            Name(name) => FilterItem::new("name", name.to_owned()),
            Names(names) => FilterItem::new("names", names.to_owned()),
        }
    }
}

impl ConfigListOptsBuilder {
    impl_filter_func!(
        /// Filter listed configs by variants of the enum.
        ConfigFilter
    );
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
            templating: Driver {
                name: "".into(),
                options: None,
            },
        }
    }

    /// Set the templating driver of this config.
    pub fn set_templating(mut self, driver: Driver) -> Self {
        self.templating = driver;
        self
    }

    /// Add a label to this config
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

    pub fn serialize_vec(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(&self).map_err(Error::from)
    }
}
