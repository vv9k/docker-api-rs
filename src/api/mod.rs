//! All api endpoints like containers, images, networks...

pub mod common;
pub mod config;
pub mod container;
pub mod event;
pub mod exec;
pub mod image;
pub mod network;
pub mod volume;

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

pub use {common::*, config::*, container::*, event::*, exec::*, image::*, network::*, volume::*};

#[cfg(feature = "swarm")]
pub use {node::*, plugin::*, secret::*, service::*, swarm::*, task::*};
