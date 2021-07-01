//! Manage and inspect services within a swarm.

use crate::{
    api::{container::LogsOpts, image::RegistryAuth},
    conn::{tty, Headers, Payload},
    docker::Docker,
    errors::{Error, Result},
};

use futures_util::stream::Stream;
use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::hash::Hash;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

impl_api_ty!(Service => name: N);

impl<'docker> Services<'docker> {
    /// Lists the docker services on the current docker host
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceList>
    pub async fn list(&self, opts: &ListOpts) -> Result<Vec<ServiceInfo>> {
        let mut path = vec!["/services".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<ServiceInfo>>(&path.join("?"))
            .await
    }
}

impl<'docker> Service<'docker> {
    /// Creates a new service from ServiceOpts
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceCreate>
    pub async fn create(&self, opts: &ServiceOpts) -> Result<ServiceCreateInfo> {
        let body: Body = opts.serialize()?.into();
        let path = vec!["/service/create".to_owned()];

        let headers = opts
            .auth_header()
            .map(|a| Headers::single("X-Registry-Auth", a));

        self.docker
            .post_json_headers(&path.join("?"), Payload::Json(body), headers)
            .await
    }

    /// Inspects a named service's details
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceInspect>
    pub async fn inspect(&self) -> Result<ServiceDetails> {
        self.docker
            .get_json(&format!("/services/{}", self.name)[..])
            .await
    }

    /// Deletes a service
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete_json(&format!("/services/{}", self.name)[..])
            .await
    }

    /// Returns a stream of logs from a service
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/ServiceLogs>
    pub fn logs(
        &self,
        opts: &LogsOpts,
    ) -> impl Stream<Item = Result<tty::TtyChunk>> + Unpin + 'docker {
        let mut path = vec![format!("/services/{}/logs", self.name)];
        if let Some(query) = opts.serialize() {
            path.push(query)
        }

        let stream = Box::pin(self.docker.stream_get(path.join("?")));

        Box::pin(tty::decode(stream))
    }
}

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
        self.params.insert("TaskTemplate", to_json_value(spec));
        self
    }

    pub fn mode(&mut self, mode: &Mode) -> &mut Self {
        self.params.insert("Mode", to_json_value(mode));
        self
    }

    pub fn update_config(&mut self, conf: &UpdateConfig) -> &mut Self {
        self.params.insert("UpdateConfig", to_json_value(conf));
        self
    }

    pub fn rollback_config(&mut self, conf: &RollbackConfig) -> &mut Self {
        self.params.insert("RollbackConfig", to_json_value(conf));
        self
    }

    pub fn networks<N>(&mut self, networks: N) -> &mut Self
    where
        N: IntoIterator<Item = NetworkAttachmentConfig>,
    {
        self.params.insert(
            "Networks",
            to_json_value(
                networks
                    .into_iter()
                    .collect::<Vec<NetworkAttachmentConfig>>(),
            ),
        );
        self
    }

    pub fn endpoint_spec(&mut self, spec: &EndpointSpec) -> &mut Self {
        self.params.insert("EndpointSpec", to_json_value(spec));
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

fn to_json_value<T>(value: T) -> Result<Value>
where
    T: Serialize,
{
    Ok(serde_json::to_value(value)?)
}

pub type ServicesInfo = Vec<ServiceInfo>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceInfo {
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
    pub endpoint: Endpoint,
    pub update_status: Option<UpdateStatus>,
    pub service_status: Option<ServiceStatus>,
    pub job_status: Option<JobStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub index: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Endpoint {
    pub spec: EndpointSpec,
    pub ports: Option<Vec<EndpointPortConfig>>,
    #[serde(rename = "VirtualIPs")]
    pub virtual_ips: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointSpec {
    pub mode: Option<String>,
    pub ports: Option<Vec<EndpointPortConfig>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointPortConfig {
    pub name: Option<String>,
    pub protocol: String,
    pub publish_mode: String,
    pub published_port: Option<u64>,
    pub target_port: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateStatus {
    pub state: String,
    #[cfg(feature = "chrono")]
    pub started_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub started_at: String,
    #[cfg(feature = "chrono")]
    pub completed_at: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub completed_at: String,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceStatus {
    pub running_tasks: u64,
    pub desired_tasks: u64,
    pub completed_tasks: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JobStatus {
    pub job_iteration: ObjectVersion,
    #[cfg(feature = "chrono")]
    pub last_execution: DateTime<Utc>,
    #[cfg(not(feature = "chrono"))]
    pub last_execution: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceDetails {
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
    pub spec: ServiceSpec,
    pub endpoint: Endpoint,
    pub update_status: Option<UpdateStatus>,
    pub service_status: Option<ServiceStatus>,
    pub job_status: Option<JobStatus>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSpec {
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub task_template: TaskSpec,
    pub mode: Mode,
    pub update_config: Option<UpdateConfig>,
    pub rollback_config: Option<RollbackConfig>,
    pub networks: Option<Vec<NetworkAttachmentConfig>>,
    pub endpoint_spec: EndpointSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
// #TODO: Add missing fields...
pub struct TaskSpec {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mode {
    pub replicated: Option<Replicated>,
    pub global: Option<serde_json::Value>,
    pub replicated_job: Option<ReplicatedJob>,
    pub global_job: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Replicated {
    pub replicas: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReplicatedJob {
    pub max_concurrent: u64,
    pub total_completions: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateConfig {
    pub parallelism: u64,
    pub delay: u64,
    pub failure_action: String,
    pub monitor: u64,
    pub max_failure_ratio: usize,
    pub order: String,
}

pub type RollbackConfig = UpdateConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkAttachmentConfig {
    pub target: String,
    pub aliases: Vec<String>,
    pub driver_opts: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceCreateInfo {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Warning")]
    pub warning: Option<String>,
}
