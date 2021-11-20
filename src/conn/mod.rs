//! Connection related items

pub(crate) mod transport;
pub(crate) mod tty;

pub use transport::*;
pub use tty::*;

pub(crate) const AUTH_HEADER: &str = "X-Registry-Auth";

use hyper::client::HttpConnector;

#[cfg(feature = "tls")]
use {
    crate::{Error, Result},
    hyper_openssl::HttpsConnector,
    openssl::ssl::{SslConnector, SslFiletype, SslMethod},
    std::path::Path,
};

pub(crate) fn get_http_connector() -> HttpConnector {
    let mut http = HttpConnector::new();
    http.enforce_http(false);

    http
}

#[cfg(feature = "tls")]
pub(crate) fn get_https_connector(
    cert_path: &Path,
    verify: bool,
) -> Result<HttpsConnector<HttpConnector>> {
    let mut ssl = SslConnector::builder(SslMethod::tls())?;
    ssl.set_cipher_list("DEFAULT")?;
    ssl.set_certificate_file(&cert_path.join("cert.pem"), SslFiletype::PEM)?;
    ssl.set_private_key_file(&cert_path.join("key.pem"), SslFiletype::PEM)?;
    verify.then(|| ssl.set_ca_file(&cert_path.join("ca.pem")));

    HttpsConnector::with_connector(get_http_connector(), ssl).map_err(Error::from)
}

#[cfg(unix)]
pub(crate) fn get_unix_connector() -> hyperlocal::UnixConnector {
    hyperlocal::UnixConnector
}
