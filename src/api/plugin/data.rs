use serde::{Deserialize, Serialize};

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
