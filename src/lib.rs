//! docker-api is a rust interface to [Docker](https://www.docker.com/) containers
//!
//! # example
//!
//! ```no_run
//! # async {
//! let docker = docker_api::Docker::new("tcp://127.0.0.1:80").unwrap();
//!
//! match docker.images().list(&Default::default()).await {
//!     Ok(images) => {
//!         for image in images {
//!             println!("{:?}", image.repo_tags);
//!         }
//!     },
//!     Err(e) => eprintln!("Something bad happened! {}", e),
//! }
//! # };
//! ```
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Latest Docker API version supported by this crate.
pub const LATEST_API_VERSION: ApiVersion = ApiVersion::new(1, Some(41), None);

/// https://github.com/rust-lang/rust/issues/53749
macro_rules! version {
    () => {
        "v1.41"
    };
}

#[macro_use]
mod builder;

pub mod api;
pub mod models;
pub mod conn {
    //! Connection related items
    pub(crate) use containers_api::conn::*;
    pub use containers_api::conn::{Error, Transport, TtyChunk};
}
pub mod docker;
pub mod errors;
pub mod opts;

pub use containers_api::id::Id;
pub use containers_api::version::ApiVersion;

pub use crate::{
    api::{
        container::{self, Container, Containers},
        exec::{self, Exec},
        image::{self, Image, Images},
        network::{self, Network, Networks},
        volume::{self, Volume, Volumes},
    },
    docker::Docker,
    errors::{Error, Result},
};

#[cfg(feature = "swarm")]
#[cfg_attr(docsrs, doc(cfg(feature = "swarm")))]
pub use crate::api::{
    config::{self, Config, Configs},
    node::{self, Node, Nodes},
    plugin::{self, Plugin, Plugins},
    secret::{self, Secret, Secrets},
    service::{self, Service, Services},
    swarm::{self, Swarm},
    task::{self, Task, Tasks},
};
