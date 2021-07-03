pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{Docker, Error, Result};

use futures_util::{Stream, TryStreamExt};

use std::{convert::TryFrom, io};

impl Docker {
    /// Returns the version of Docker that is running and various information about the system that
    /// Docker is running on.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemVersion)
    pub async fn version(&self) -> Result<Version> {
        self.get_json("/version").await
    }

    /// Returns system information about Docker instance that is running
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemInfo)
    pub async fn info(&self) -> Result<Info> {
        self.get_json("/info").await
    }

    /// This is a dummy endpoint you can use to test if the server is accessible.
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemPingHead)
    pub async fn ping(&self) -> Result<PingInfo> {
        self.get("/_ping")
            .await
            .and_then(|resp| PingInfo::try_from(resp.headers()))
    }

    /// Returns a stream of Docker events
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemEvents)
    pub fn events<'docker>(
        &'docker self,
        opts: &EventsOpts,
    ) -> impl Stream<Item = Result<Event>> + Unpin + 'docker {
        let mut path = vec!["/events".to_owned()];
        if let Some(query) = opts.serialize() {
            path.push(query);
        }
        let reader = Box::pin(
            self.stream_get(path.join("?"))
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        )
        .into_async_read();

        let codec = futures_codec::LinesCodec {};

        Box::pin(
            futures_codec::FramedRead::new(reader, codec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }

    /// Returns data usage of this Docker instance
    ///
    /// [Api Reference](https://docs.docker.com/engine/api/v1.41/#operation/SystemDataUsage)
    pub async fn data_usage(&self) -> Result<DataUsage> {
        self.get_json("/system/df").await
    }
}
