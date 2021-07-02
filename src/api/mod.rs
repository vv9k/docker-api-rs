//! All api endpoints like containers, images, networks...

pub mod common;
pub mod container;
pub mod event;
pub mod exec;
pub mod image;
pub mod network;
pub mod node;
pub mod plugin;
pub mod secret;
pub mod service;
pub mod swarm;
pub mod task;
pub mod volume;

pub use common::*;
pub use container::*;
pub use event::*;
pub use exec::*;
pub use image::*;
pub use network::*;
pub use node::*;
pub use plugin::*;
pub use secret::*;
pub use service::*;
pub use swarm::*;
pub use task::*;
pub use volume::*;
