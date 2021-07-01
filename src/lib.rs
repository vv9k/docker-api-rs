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

#[macro_use]
mod builder;
mod util;

pub mod api;
pub mod conn;
pub mod docker;
pub mod errors;

pub use crate::{
    api::{
        container::{self, Container, Containers},
        exec::{self, Exec, ExecContainerOpts},
        image::{self, Image, Images},
        network::{self, Network, Networks},
        plugin::{self, Plugin, Plugins},
        service::{self, Service, Services},
        volume::{self, Volume, Volumes},
    },
    conn::Transport,
    docker::Docker,
    errors::{Error, Result},
};
