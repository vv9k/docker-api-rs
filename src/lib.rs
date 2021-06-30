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

pub mod errors;
pub mod transport;
pub mod tty;
#[macro_use]
mod builder;
mod util;

pub mod container;
pub mod docker;
pub mod event;
pub mod exec;
pub mod image;
pub mod network;
pub mod plugin;
pub mod service;
pub mod volume;

mod tarball;

#[cfg(feature = "chrono")]
mod datetime;

pub use crate::{
    container::{Container, Containers},
    docker::Docker,
    errors::{Error, Result},
    exec::{Exec, ExecContainerOpts},
    image::{Image, Images},
    network::{Network, Networks},
    plugin::{Plugin, Plugins},
    service::{Service, Services},
    transport::Transport,
    volume::{Volume, Volumes},
};
