use crate::{models, opts::RegistryAuth, Error, Result};
use containers_api::opts::{Filter, FilterItem};
use containers_api::{impl_filter_func, impl_opts_builder, impl_url_bool_field};

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

impl Filter for ServiceFilter {
    fn query_item(&self) -> FilterItem {
        match &self {
            ServiceFilter::Id(i) => FilterItem::new("id", i.to_owned()),
            ServiceFilter::Label(l) => FilterItem::new("label", l.to_owned()),
            ServiceFilter::ReplicatedMode => FilterItem::new("mode", "replicated".to_string()),
            ServiceFilter::GlobalMode => FilterItem::new("mode", "global".to_string()),
            ServiceFilter::Name(n) => FilterItem::new("name", n.to_string()),
        }
    }
}

impl_opts_builder!(url => ServiceList);

impl ServiceListOptsBuilder {
    impl_filter_func!(ServiceFilter);

    impl_url_bool_field!(
        /// Include service status, with count of running and desired tasks.
        status => "status"
    );
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
    pub fn name<S>(mut self, name: S) -> Self
    where
        S: AsRef<str>,
    {
        self.params.insert("Name", Ok(json!(name.as_ref())));
        self
    }

    pub fn labels<L, K, V>(mut self, labels: L) -> Self
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

    pub fn task_template(mut self, spec: &models::TaskSpec) -> Self {
        self.params.insert("TaskTemplate", to_value_result(spec));
        self
    }

    pub fn mode(mut self, mode: &models::ServiceSpecModeInlineItem) -> Self {
        self.params.insert("Mode", to_value_result(mode));
        self
    }

    pub fn update_config(mut self, conf: &models::ServiceSpecUpdateConfigInlineItem) -> Self {
        self.params.insert("UpdateConfig", to_value_result(conf));
        self
    }

    pub fn rollback_config(mut self, conf: &models::ServiceSpecRollbackConfigInlineItem) -> Self {
        self.params.insert("RollbackConfig", to_value_result(conf));
        self
    }

    pub fn networks<N>(mut self, networks: N) -> Self
    where
        N: IntoIterator<Item = models::NetworkAttachmentConfig>,
    {
        self.params.insert(
            "Networks",
            to_value_result(
                networks
                    .into_iter()
                    .collect::<Vec<models::NetworkAttachmentConfig>>(),
            ),
        );
        self
    }

    pub fn endpoint_spec(mut self, spec: &models::EndpointSpec) -> Self {
        self.params.insert("EndpointSpec", to_value_result(spec));
        self
    }

    pub fn auth(mut self, auth: RegistryAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn build(self) -> Result<ServiceOpts> {
        let mut new_params = HashMap::new();
        for (k, v) in self.params.into_iter() {
            new_params.insert(k, v?);
        }
        Ok(ServiceOpts {
            auth: self.auth,
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
