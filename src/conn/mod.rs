pub mod transport;
pub mod tty;

pub use transport::*;
pub use tty::*;

use hyper::client::HttpConnector;

pub(crate) fn get_http_connector() -> HttpConnector {
    let mut http = HttpConnector::new();
    http.enforce_http(false);

    http
}
