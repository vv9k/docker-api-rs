//! Control and manage clusters of engines also known as Swarm
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, Docker, Result};

pub struct Swarm<'docker> {
    docker: &'docker Docker,
}

impl<'docker> Swarm<'docker> {
    pub fn new(docker: &'docker Docker) -> Self {
        Self { docker }
    }

    /// Inspects swarm on this Docker host.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmInspect)
    pub async fn inspect(&self) -> Result<SwarmInfo> {
        self.docker.get_json("/swarm").await
    }

    /// Get the unlock key.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmUnlockKey)
    pub async fn get_unlock_key(&self) -> Result<UnlockKey> {
        self.docker.get_json("/swarm/unlockkey").await
    }

    /// Unlock a locked manager.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmUnlock)
    pub async fn unlock_manager(&self, key: &UnlockKey) -> Result<()> {
        self.docker
            .post("/swarm/unlock", Payload::Json(serde_json::to_string(key)?))
            .await
            .map(|_| ())
    }

    /// Initialize a new swarm.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmInit)
    pub async fn initialize(&self, opts: &SwarmInitOpts) -> Result<()> {
        self.docker
            .post("/swarm/init", Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }

    /// Join an existing swarm.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmJoin)
    pub async fn join(&self, opts: &SwarmJoinOpts) -> Result<()> {
        self.docker
            .post("/swarm/join", Payload::Json(opts.serialize()?))
            .await
            .map(|_| ())
    }

    /// Leave the current swarm.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmLeave)
    pub async fn leave(&self) -> Result<()> {
        self.docker
            .post("/swarm/leave?force=false", Payload::empty())
            .await
            .map(|_| ())
    }

    /// Leave the current swarm forcefully, even if this is the last manager or that it will break the cluster.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SwarmLeave)
    pub async fn force_leave(&self) -> Result<()> {
        self.docker
            .post("/swarm/leave?force=true", Payload::empty())
            .await
            .map(|_| ())
    }
}
