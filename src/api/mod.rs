//! All api endpoints like containers, images, networks...

pub mod common;
pub mod container;
pub mod exec;
pub mod image;
pub mod network;
pub mod system;
pub mod volume;

#[cfg(feature = "swarm")]
pub mod config;
#[cfg(feature = "swarm")]
pub mod node;
#[cfg(feature = "swarm")]
pub mod plugin;
#[cfg(feature = "swarm")]
pub mod secret;
#[cfg(feature = "swarm")]
pub mod service;
#[cfg(feature = "swarm")]
pub mod swarm;
#[cfg(feature = "swarm")]
pub mod task;

pub use {common::*, container::*, exec::*, image::*, network::*, system::*, volume::*};

#[cfg(feature = "swarm")]
pub use {config::*, node::*, plugin::*, secret::*, service::*, swarm::*, task::*};
