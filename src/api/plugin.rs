//! Install, create and manage plugins
//!
//! Api Reference: <https://docs.docker.com/engine/api/v1.41/#tag/Plugin>

use crate::{conn::Payload, errors::Result, util::url::encoded_pair, Docker};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
/// Interface for accessing and manipulating a Docker plugin.
///
/// Api Reference: <https://docs.docker.com/engine/api/v1.41/#tag/Plugin>
pub struct Plugin<'docker> {
    docker: &'docker Docker,
    name: String,
}

impl<'docker> Plugin<'docker> {
    /// Exports an interface for operations that may be performed against a named image.
    pub fn new<S>(docker: &'docker Docker, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            docker,
            name: name.into(),
        }
    }

    /// Inspects a named plugin's details.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginInspect>
    pub async fn inspect(&self) -> Result<PluginInfo> {
        self.docker
            .get_json(&format!("/plugins/{}/json", self.name)[..])
            .await
    }

    async fn _remove(&self, force: bool) -> Result<PluginInfo> {
        let mut path = format!("/plugins/{}", self.name);
        if force {
            let query = encoded_pair("force", force);
            path.push('?');
            path.push_str(&query);
        }
        self.docker.delete_json(&path[..]).await
    }

    /// Removes a plugin.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginDelete>
    pub async fn remove(&self) -> Result<PluginInfo> {
        self._remove(false).await
    }

    /// Forcefully remove a plugin. This may result in issues if the plugin is in use by a container.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginDelete>
    pub async fn force_remove(&self) -> Result<PluginInfo> {
        self._remove(true).await
    }

    /// Enable a plugin.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginEnable>
    pub async fn enable(&self, timeout: Option<u64>) -> Result<()> {
        let mut path = format!("/plugins/{}/enable", self.name);
        if let Some(timeout) = timeout {
            let query = encoded_pair("timeout", timeout);
            path.push('?');
            path.push_str(&query);
        }

        self.docker.post(&path, Payload::empty()).await.map(|_| ())
    }

    /// Disable a plugin.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginDisable>
    pub async fn disable(&self) -> Result<()> {
        self.docker
            .post(&format!("/plugins/{}/disable", self.name), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Push a plugin to the registry.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginPush>
    pub async fn push(&self) -> Result<()> {
        self.docker
            .post(&format!("/plugins/{}/push", self.name), Payload::empty())
            .await
            .map(|_| ())
    }

    /// Create a plugin from a tar archive on the file system. The `path` parameter is a path
    /// to the tar containing plugin rootfs and manifest.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginCreate>
    pub async fn create<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        self.docker
            .post(
                &format!("/plugins/{}/create", self.name),
                Payload::Text(path.as_ref().to_string_lossy().to_string()),
            )
            .await
            .map(|_| ())
    }
}

#[derive(Debug)]
/// Interface for docker plugins
pub struct Plugins<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Plugins<'docker> {
    /// Exports an interface for interacting with docker plugins
    pub fn new(docker: &'docker Docker) -> Self {
        Self { docker }
    }

    /// Returns a reference to a set of operations available for a plugin with `name`
    pub fn get<N>(&self, name: N) -> Plugin<'docker>
    where
        N: Into<String>,
    {
        Plugin::new(self.docker, name)
    }

    /// Returns information about installed plugins.
    ///
    /// Api Reference: <https://docs.docker.com/engine/api/v1.41/#operation/PluginList>
    pub async fn list(&self, opts: &PluginListOpts) -> Result<Vec<PluginInfo>> {
        let mut path = vec!["/images/json".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker
            .get_json::<Vec<PluginInfo>>(&path.join("?"))
            .await
    }
}

impl_url_opts_builder!(PluginList);

impl PluginListOptsBuilder {
    pub fn filter<F>(&mut self, filters: F) -> &mut Self
    where
        F: IntoIterator<Item = PluginFilter>,
    {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                PluginFilter::Capability(cap) => param.insert("capability", vec![cap]),
                PluginFilter::Enable => param.insert("enable", vec![true.to_string()]),
                PluginFilter::Disable => param.insert("enable", vec![false.to_string()]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginInfo {
    pub id: Option<String>,
    pub name: String,
    pub enabled: bool,
    pub settings: PluginSettings,
    pub plugin_reference: Option<String>,
    pub config: PluginConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginSettings {
    pub mounts: Vec<PluginMount>,
    pub env: Vec<String>,
    pub args: Vec<String>,
    pub devices: Vec<PluginDevice>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginMount {
    pub name: String,
    pub description: String,
    pub settable: Vec<String>,
    pub source: String,
    pub destination: String,
    #[serde(rename = "Type")]
    pub type_: String,
    pub options: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginDevice {
    pub name: String,
    pub description: String,
    pub settable: Vec<String>,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginConfig {
    pub docker_version: Option<String>,
    pub description: String,
    pub documentation: String,
    pub interface: PluginInterface,
    pub entrypoint: Vec<String>,
    pub work_dir: String,
    pub user: Option<User>,
    pub network: PluginNetwork,
    pub linux: LinuxInfo,
    pub propagated_mount: String,
    pub ipc_host: bool,
    pub pid_host: bool,
    pub mounts: Vec<PluginMount>,
    pub env: Vec<PluginEnv>,
    pub args: PluginArgs,
    pub rootfs: Option<PluginRootfs>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UID")]
    pub uid: u32,
    #[serde(rename = "GID")]
    pub gid: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinuxInfo {
    pub capabilities: Vec<String>,
    pub allow_all_devices: bool,
    pub devices: Vec<PluginDevice>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginNetwork {
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginInterface {
    pub types: Vec<PluginInterfaceType>,
    pub socket: String,
    pub protocol_scheme: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginInterfaceType {
    pub prefix: String,
    pub capability: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginEnv {
    pub name: String,
    pub description: String,
    pub settable: Vec<String>,
    pub value: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginArgs {
    pub name: String,
    pub description: String,
    pub settable: Vec<String>,
    pub value: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginRootfs {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub diff_ids: Option<Vec<String>>,
}

pub enum PluginFilter {
    Capability(String),
    Enable,
    Disable,
}
