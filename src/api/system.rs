use crate::{models, opts::EventsOpts, Docker, Error, Result};
use containers_api::url::construct_ep;
use futures_util::{Stream, TryStreamExt};

use std::{convert::TryFrom, io};

impl Docker {
    api_doc! { System => Version
    /// Returns the version of Docker that is running and various information about the system that Docker is running on.
    |
    pub async fn version(&self) -> Result<models::SystemVersion> {
        self.get_json("/version").await
    }}

    api_doc! { System => Info
    /// Returns system information about Docker instance that is running
    |
    pub async fn info(&self) -> Result<models::SystemInfo> {
        self.get_json("/info").await
    }}

    api_doc! { System => Ping
    /// This is a dummy endpoint you can use to test if the server is accessible
    |
    pub async fn ping(&self) -> Result<models::PingInfo> {
        self.get("/_ping")
            .await
            .and_then(|resp| models::PingInfo::try_from(resp.headers()))
    }}

    api_doc! { System => Events
    /// Returns a stream of Docker events
    |
    pub fn events<'docker>(
        &'docker self,
        opts: &EventsOpts,
    ) -> impl Stream<Item = Result<models::SystemEventsResponse>> + Unpin + 'docker {
        let ep = construct_ep("/events", opts.serialize());
        let reader = Box::pin(
            self.stream_get(ep)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        )
        .into_async_read();

        Box::pin(
            futures_codec::FramedRead::new(reader, futures_codec::LinesCodec)
                .map_err(Error::IO)
                .and_then(|s: String| async move {
                    serde_json::from_str(&s).map_err(Error::SerdeJsonError)
                }),
        )
    }}

    api_doc! { System => DataUsage
    /// Returns data usage of this Docker instance
    |
    pub async fn data_usage(&self) -> Result<models::SystemDataUsageResponse> {
        self.get_json("/system/df").await
    }}
}
