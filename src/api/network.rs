//! Create and manage user-defined networks that containers can be attached to.

use std::collections::HashMap;

use hyper::Body;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    conn::Payload,
    docker::Docker,
    errors::{Error, Result},
};

impl_api_ty!(Network => id: I);

impl<'docker> Networks<'docker> {
    /// List the docker networks on the current docker host
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkList>
    pub async fn list(&self, opts: &NetworkListOpts) -> Result<Vec<NetworkDetails>> {
        let mut path = vec!["/networks".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        self.docker.get_json(&path.join("?")).await
    }

    /// Create a new Network instance
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkCreate>
    pub async fn create(&self, opts: &NetworkCreateOpts) -> Result<NetworkCreateInfo> {
        let body: Body = opts.serialize()?.into();
        let path = vec!["/networks/create".to_owned()];

        self.docker
            .post_json(&path.join("?"), Payload::Json(body))
            .await
    }
}

impl<'docker> Network<'docker> {
    /// Inspects the current docker network instance's details
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkInspect>
    pub async fn inspect(&self) -> Result<NetworkDetails> {
        self.docker
            .get_json(&format!("/networks/{}", self.id)[..])
            .await
    }

    /// Delete the network instance
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkDelete>
    pub async fn delete(&self) -> Result<()> {
        self.docker
            .delete(&format!("/networks/{}", self.id)[..])
            .await?;
        Ok(())
    }

    /// Connect container to network
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkConnect>
    pub async fn connect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("connect", opts).await
    }

    /// Disconnect container to network
    ///
    /// API Reference: <https://docs.docker.com/engine/api/v1.41/#operation/NetworkDisconnect>
    pub async fn disconnect(&self, opts: &ContainerConnectionOpts) -> Result<()> {
        self.do_connection("disconnect", opts).await
    }

    async fn do_connection<S>(&self, segment: S, opts: &ContainerConnectionOpts) -> Result<()>
    where
        S: AsRef<str>,
    {
        let body: Body = opts.serialize()?.into();

        self.docker
            .post(
                &format!("/networks/{}/{}", self.id, segment.as_ref())[..],
                Payload::Json(body),
            )
            .await?;
        Ok(())
    }
}

impl_url_opts_builder!(derives = Default | "Options for filtering networks list results" NetworkList);
// TODO: implement `filters` field on network list

/// Interface for creating new docker network
#[derive(Serialize, Debug)]
pub struct NetworkCreateOpts {
    params: HashMap<&'static str, Value>,
}

impl NetworkCreateOpts {
    /// return a new instance of a builder for Opts
    pub fn builder<N>(name: N) -> NetworkCreateOptsBuilder
    where
        N: AsRef<str>,
    {
        NetworkCreateOptsBuilder::new(name.as_ref())
    }

    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }
}

#[derive(Default)]
pub struct NetworkCreateOptsBuilder {
    params: HashMap<&'static str, Value>,
}

impl NetworkCreateOptsBuilder {
    pub(crate) fn new(name: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("Name", json!(name));
        NetworkCreateOptsBuilder { params }
    }

    impl_str_field!(driver: D => "Driver");

    impl_map_field!(labels: L => "Labels");

    pub fn build(&self) -> NetworkCreateOpts {
        NetworkCreateOpts {
            params: self.params.clone(),
        }
    }
}

/// Interface for connect container to network
#[derive(Serialize, Debug)]
pub struct ContainerConnectionOpts {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOpts {
    /// serialize Opts as a string. returns None if no Opts are defined
    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string(&self.params).map_err(Error::from)
    }

    /// return a new instance of a builder for Opts
    pub fn builder<I>(container_id: I) -> ContainerConnectionOptsBuilder
    where
        I: AsRef<str>,
    {
        ContainerConnectionOptsBuilder::new(container_id.as_ref())
    }
}

#[derive(Default)]
pub struct ContainerConnectionOptsBuilder {
    params: HashMap<&'static str, Value>,
}

impl ContainerConnectionOptsBuilder {
    pub(crate) fn new(container_id: &str) -> Self {
        let mut params = HashMap::new();
        params.insert("Container", json!(container_id));
        ContainerConnectionOptsBuilder { params }
    }

    pub fn aliases<A, S>(&mut self, aliases: A) -> &mut Self
    where
        A: IntoIterator<Item = S>,
        S: AsRef<str> + Serialize,
    {
        self.params.insert(
            "EndpointConfig",
            json!({ "Aliases": json!(aliases.into_iter().collect::<Vec<_>>()) }),
        );
        self
    }

    pub fn force(&mut self) -> &mut Self {
        self.params.insert("Force", json!(true));
        self
    }

    pub fn build(&self) -> ContainerConnectionOpts {
        ContainerConnectionOpts {
            params: self.params.clone(),
        }
    }
}

type PortDescription = HashMap<String, Option<Vec<HashMap<String, String>>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub bridge: String,
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u64,
    pub mac_address: String,
    pub ports: Option<PortDescription>,
    pub networks: HashMap<String, NetworkEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkEntry {
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u64,
    pub mac_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ipam {
    pub driver: String,
    pub config: Vec<HashMap<String, String>>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkDetails {
    pub name: String,
    pub id: String,
    pub scope: String,
    pub driver: String,
    #[serde(rename = "EnableIPv6")]
    pub enable_ipv6: bool,
    #[serde(rename = "IPAM")]
    pub ipam: Ipam,
    pub internal: bool,
    pub attachable: bool,
    pub containers: HashMap<String, NetworkContainerDetails>,
    pub options: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkContainerDetails {
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub mac_address: String,
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: String,
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkCreateInfo {
    pub id: String,
    pub warning: String,
}
