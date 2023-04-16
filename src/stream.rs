use containers_api::conn::tty;
use containers_api::conn::Payload;
use futures_util::{AsyncRead, AsyncWrite};
use hyper::Body;

use crate::{Docker, Result};

/// Attaches a multiplexed TCP stream to the container that can be used to read Stdout, Stderr and write Stdin.
async fn attach_raw(
    docker: &Docker,
    endpoint: String,
    payload: Payload<Body>,
) -> Result<impl AsyncRead + AsyncWrite + Send + '_> {
    docker.post_upgrade_stream(endpoint, payload).await
}

pub async fn attach(
    docker: &Docker,
    endpoint: String,
    payload: Payload<Body>,
    is_tty: bool,
) -> Result<tty::Multiplexer<'_>> {
    attach_raw(docker, endpoint, payload).await.map(|s| {
        if is_tty {
            tty::Multiplexer::new(s, tty::decode_raw)
        } else {
            tty::Multiplexer::new(s, tty::decode_chunk)
        }
    })
}
