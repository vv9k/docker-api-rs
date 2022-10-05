//! Representations of various client errors

use hyper::{self, StatusCode};
use serde_json::Error as SerdeError;
use thiserror::Error as ThisError;

use futures_util::io::Error as IoError;

/// Represents the result of all docker operations
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error(transparent)]
    SerdeJsonError(#[from] SerdeError),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    #[allow(clippy::upper_case_acronyms)]
    IO(#[from] IoError),
    #[error("The response is invalid - {0}")]
    InvalidResponse(String),
    #[error("error {code} - {message}")]
    Fault { code: StatusCode, message: String },
    #[error("The HTTP connection was not upgraded by the docker host")]
    ConnectionNotUpgraded,
    #[error("Provided scheme `{0}` is not supported")]
    UnsupportedScheme(String),
    #[error("Provided URI is missing authority part after scheme")]
    MissingAuthority,
    #[error("Failed to parse url - {0}")]
    InvalidUrl(url::ParseError),
    #[error("Failed to parse uri - {0}")]
    InvalidUri(http::uri::InvalidUri),
    #[error("Invalid port - {0}")]
    InvalidPort(String),
    #[error("Invalid protocol - {0}")]
    InvalidProtocol(String),
    #[error(transparent)]
    MalformedVersion(#[from] containers_api::version::Error),
    #[error(transparent)]
    Error(#[from] containers_api::conn::Error),
    #[error(transparent)]
    Any(Box<dyn std::error::Error + 'static + Send + Sync>),
    #[error("{0}")]
    StringError(String),
}

impl Clone for Error {
    fn clone(&self) -> Self {
        match self {
            Error::SerdeJsonError(err) => Error::StringError(err.to_string()),
            Error::IO(err) => Error::StringError(err.to_string()),
            Error::Error(err) => Error::StringError(err.to_string()),
            e => e.clone(),
        }
    }
}
