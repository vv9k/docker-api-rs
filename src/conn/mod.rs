pub mod transport;
pub mod tty;

pub use transport::*;
pub use tty::*;

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
    let http = get_http_connector();
    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_cipher_list("DEFAULT")?;
    let cert = cert_path.join("cert.pem");
    let key = cert_path.join("key.pem");
    connector.set_certificate_file(cert.as_path(), SslFiletype::PEM)?;
    connector.set_private_key_file(key.as_path(), SslFiletype::PEM)?;
    if verify {
        let ca = cert_path.join("ca.pem");
        connector.set_ca_file(ca.as_path())?;
    }

    HttpsConnector::with_connector(http, connector).map_err(Error::from)
}

#[cfg(unix)]
pub(crate) fn get_unix_connector() -> hyperlocal::UnixConnector {
    hyperlocal::UnixConnector
}
