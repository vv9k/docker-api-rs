//! Auto detect the docker endpoint the same way docker-cli does
//!
//! Reference: <https://github.com/docker/cli/blob/v24.0.5/opts/hosts.go#L11-L33>

use dirs::home_dir;
use env_vars::{DOCKER_CONFIG, DOCKER_CONTEXT, DOCKER_HOST};
use serde::de::Error as SerdeError;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};

#[cfg(unix)]
pub const DEFAULT_DOCKER_ENDPOINT: &str = "unix:///var/run/docker.sock";

/// For windows the default endpoint is "npipe:////./pipe/docker_engine"
/// But currently this is not supported by docker-api, using to default tcp endpoint instead
/// https://github.com/vv9k/docker-api-rs/issues/57
#[cfg(not(unix))]
pub const DEFAULT_DOCKER_ENDPOINT: &str = "tcp://127.0.0.1:2375";

/// List of environment variables supported by the `docker` command
pub(crate) mod env_vars {
    use std::ffi::OsStr;

    /// The location of your client configuration files.
    pub const DOCKER_CONFIG: &str = "DOCKER_CONFIG";

    /// Name of the `docker context` to use (overrides `DOCKER_HOST` env var and default context set with `docker context use`)
    pub const DOCKER_CONTEXT: &str = "DOCKER_CONTEXT";

    /// Daemon socket to connect to.
    pub const DOCKER_HOST: &str = "DOCKER_HOST";

    /// Load an environment variable and verify if it's not empty
    pub fn non_empty_var<K: AsRef<OsStr>>(key: K) -> Option<String> {
        let value = std::env::var(key).ok()?;
        if value.trim().is_empty() {
            None
        } else {
            Some(value)
        }
    }
}

enum EndpointError {
    InvalidEndpoint,
    CannotFindUserHomeDir,
    InvalidJson {
        filepath: PathBuf,
        error: serde_json::Error,
    },
    IOError(std::io::Error),
}

/// Find the docker host the same way as docker-cli does
///
/// # Steps
/// 1. Try to load the endpoint from the `DOCKER_CONTEXT` environment variable
/// 2. Try to load the endpoint from the `DOCKER_HOST` environment variable
/// 3. Try to load the endpoint from the `config.json` file
/// 4. Return the default endpoint
///
/// # Fails when
/// * Cannot find docker config directory
/// * `DOCKER_CONTEXT` is defined and is invalid
/// * `config.js` file exists and fails to read or parse it
/// * `config.js` have the `currentContext` property defined, but fails to find the context endpoint
pub fn find_docker_host() -> Result<String, EndpointError> {
    // If defined, Load the endpoint from the `DOCKER_CONTEXT` environment variable
    if let Some(context) = env_vars::non_empty_var(DOCKER_CONTEXT) {
        let config_directory = docker_config_dir()?;
        return host_from_context(&context, config_directory);
    }

    // If defined, return the host from the `DOCKER_HOST` environment variable
    if let Some(host) = env_vars::non_empty_var(DOCKER_HOST) {
        return Ok(host);
    }

    // If the config.json file exists, try to load the endpoint from it
    let config_file = docker_config_dir().map(|config_dir| config_dir.join("config.json"))?;
    if config_file.exists() {
        let maybe_host = host_from_config_file(config_file)?;
        return maybe_host.ok_or_else(|| EndpointError::InvalidEndpoint);
    }

    // otherwise return the default endpoint
    Ok(DEFAULT_DOCKER_ENDPOINT.to_string())
}

/// By default, the Docker-cli stores its configuration files in a directory called
/// `.docker` within your `$HOME` directory. the default location can be overridden by
/// the `DOCKER_CONFIG` environment variable.
///
/// Reference:
/// https://github.com/docker/cli/blob/v24.0.5/man/docker-config-json.5.md
///
/// Fails if the user home directory cannot be found
pub fn docker_config_dir() -> Result<PathBuf, EndpointError> {
    // Try to load the config directory from the `DOCKER_CONFIG` environment variable
    if let Some(config_directory) = env_vars::non_empty_var(DOCKER_CONFIG).map(PathBuf::from) {
        return Ok(config_directory);
    }

    // Use the default config directory at $HOME/.docker/
    let Some(config_directory) = home_dir().map(|path| path.join(".docker/")) else {
        return Err(EndpointError::CannotFindUserHomeDir);
    };
    Ok(config_directory)
}

/// Attempts to load the endpoint from the `.docker/config.json` file
///
/// # Returns
/// * Ok(Some(host)) - if the config.js exists and contains currentContext field
/// * Ok(None) - if the config.js exists and not contain currentContext field
///
/// # Fails when
/// * config.js doesn't exists
/// * cannot read or parse the config.js file
/// * the currentContext is defined, but fails to load the context endpoint
pub fn host_from_config_file(config_file: PathBuf) -> Result<Option<String>, EndpointError> {
    // Read the config.json file and extract the current context
    let config_file_json = file_to_json(&config_file)?;

    // Check if the config file has the property currentContext
    let current_context = config_file_json
        .get("currentContext")
        .and_then(|value| value.as_str())
        .map(str::to_string);

    if let Some(context) = current_context {
        let config_directory = config_file
            .parent()
            .map(|config_dir| config_dir.to_path_buf())
            .unwrap_or_default();
        let endpoint = host_from_context(&context, config_directory)?;
        Ok(Some(endpoint))
    } else {
        Ok(None)
    }
}

/// Load the Host of a given context, the context's host is located at:
/// UNIX:
///  - $HOME/.docker/contexts/meta/<sha256 context>/meta.json
/// Windows:
/// - %USERPROFILE%\.docker\contexts\meta\<sha256 context>\meta.json
///
/// Is possible to list contexts by running `docker context ls`
pub fn host_from_context(
    context: &str,
    mut config_directory: PathBuf,
) -> Result<String, EndpointError> {
    let metadata_filepath = {
        // $HOME/.docker/contexts/meta/<sha256 context>/meta.json
        let digest = sha256_digest(context);
        config_directory.extend(["contexts", "meta", digest.as_str(), "meta.json"]);
        config_directory
    };

    host_from_metadata_file(metadata_filepath)
}

/// Parses the `meta.json` file and extract the docker endpoint
/// The endpoint is located at: `meta.Endpoints.Endpoints.docker.Host`
pub fn host_from_metadata_file(meta_filepath: PathBuf) -> Result<String, EndpointError> {
    let meta_json = file_to_json(&meta_filepath)?;

    let host = meta_json
        .get("Endpoints")
        .and_then(|value| value.get("docker"))
        .and_then(|value| value.get("Host"))
        .and_then(|value| value.as_str())
        .ok_or_else(|| EndpointError::InvalidJson {
            filepath: meta_filepath,
            error: SerdeError::missing_field("Endpoints.docker.Host"),
        })?
        .to_string();

    Ok(host)
}

/// Parsers a file to a json value
fn file_to_json(filepath: &PathBuf) -> Result<serde_json::Value, EndpointError> {
    fs::read_to_string(filepath)
        .map_err(|error| EndpointError::IOError(error))?
        .parse::<serde_json::Value>()
        .map_err(|error| EndpointError::InvalidJson {
            filepath: filepath.clone(),
            error,
        })
}

/// Returns the sha256 hex-digest of a given string
fn sha256_digest(name: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
