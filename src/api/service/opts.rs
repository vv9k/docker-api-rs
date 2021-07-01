use super::data::*;
use crate::{api::image::RegistryAuth, Error, Result};

use std::collections::HashMap;
use std::hash::Hash;

use serde::Serialize;
use serde_json::{json, Value};

/// Filter Opts for services listings
pub enum ServiceFilter {
    Id(String),
    Label(String),
    ReplicatedMode,
    GlobalMode,
    Name(String),
}

impl_url_opts_builder!(List);

impl ListOptsBuilder {
    pub fn filter(&mut self, filters: Vec<ServiceFilter>) -> &mut Self {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                ServiceFilter::Id(i) => param.insert("id", vec![i]),
                ServiceFilter::Label(l) => param.insert("label", vec![l]),
                ServiceFilter::ReplicatedMode => {
                    param.insert("mode", vec!["replicated".to_string()])
                }
                ServiceFilter::GlobalMode => param.insert("mode", vec!["global".to_string()]),
                ServiceFilter::Name(n) => param.insert("name", vec![n.to_string()]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }

    pub fn enable_status(&mut self) -> &mut Self {
        self.params.insert("status", "true".to_owned());
        self
    }
}

#[derive(Default, Debug)]
pub struct ServiceOpts {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, Value>,
}

impl ServiceOpts {
    /// return a new instance of a builder for Opts
    pub fn builder() -> ServiceOptsBuilder {
        ServiceOptsBuilder::default()
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    pub(crate) fn auth_header(&self) -> Option<String> {
        self.auth.clone().map(|a| a.serialize())
    }
}

#[derive(Default)]
pub struct ServiceOptsBuilder {
    auth: Option<RegistryAuth>,
    params: HashMap<&'static str, Result<Value>>,
}

impl ServiceOptsBuilder {
    pub fn name<S>(&mut self, name: S) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.params.insert("Name", Ok(json!(name.as_ref())));
        self
    }

    pub fn labels<L, K, V>(&mut self, labels: L) -> &mut Self
    where
        L: IntoIterator<Item = (K, V)>,
        K: AsRef<str> + Serialize + Eq + Hash,
        V: AsRef<str> + Serialize,
    {
        self.params.insert(
            "Labels",
            Ok(json!(labels.into_iter().collect::<HashMap<_, _>>())),
        );
        self
    }

    pub fn task_template(&mut self, spec: &TaskSpec) -> &mut Self {
        self.params.insert("TaskTemplate", to_value_result(spec));
        self
    }

    pub fn mode(&mut self, mode: &Mode) -> &mut Self {
        self.params.insert("Mode", to_value_result(mode));
        self
    }

    pub fn update_config(&mut self, conf: &UpdateConfig) -> &mut Self {
        self.params.insert("UpdateConfig", to_value_result(conf));
        self
    }

    pub fn rollback_config(&mut self, conf: &RollbackConfig) -> &mut Self {
        self.params.insert("RollbackConfig", to_value_result(conf));
        self
    }

    pub fn networks<N>(&mut self, networks: N) -> &mut Self
    where
        N: IntoIterator<Item = NetworkAttachmentConfig>,
    {
        self.params.insert(
            "Networks",
            to_value_result(
                networks
                    .into_iter()
                    .collect::<Vec<NetworkAttachmentConfig>>(),
            ),
        );
        self
    }

    pub fn endpoint_spec(&mut self, spec: &EndpointSpec) -> &mut Self {
        self.params.insert("EndpointSpec", to_value_result(spec));
        self
    }

    pub fn auth(&mut self, auth: RegistryAuth) -> &mut Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(&mut self) -> Result<ServiceOpts> {
        let params = std::mem::take(&mut self.params);
        let mut new_params = HashMap::new();
        for (k, v) in params.into_iter() {
            new_params.insert(k, v?);
        }
        Ok(ServiceOpts {
            auth: self.auth.take(),
            params: new_params,
        })
    }
}

fn to_value_result<T>(value: T) -> Result<Value>
where
    T: Serialize,
{
    Ok(serde_json::to_value(value)?)
}
