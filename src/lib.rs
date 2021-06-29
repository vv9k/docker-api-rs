//! docker-api is a multi-transport utility for maneuvering [docker](https://www.docker.com/) containers
//!
//! # examples
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

pub mod container;
pub mod docker;
pub mod event;
pub mod exec;
pub mod image;
pub mod network;
pub mod service;
pub mod volume;

mod tarball;

#[cfg(feature = "chrono")]
mod datetime;

pub use crate::{
    container::{
        Container, ContainerFilter, ContainerListOptions, ContainerOptions, Containers,
        LogsOptions, RmContainerOptions,
    },
    docker::Docker,
    errors::{Error, Result},
    exec::{Exec, ExecContainerOptions, ExecResizeOptions},
    image::{
        BuildOptions, Image, ImageFilter, ImageListOptions, Images, PullOptions, RegistryAuth,
        TagOptions,
    },
    network::{
        ContainerConnectionOptions, Network, NetworkCreateOptions, NetworkListOptions, Networks,
    },
    service::{Service, ServiceFilter, ServiceListOptions, ServiceOptions, Services},
    transport::Transport,
    volume::{Volume, VolumeCreateOptions, Volumes},
};
