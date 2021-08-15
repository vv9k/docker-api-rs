# docker-api

[![GitHub Actions](https://github.com/wojciechkepka/docker-api-rs/workflows/Main/badge.svg)](https://github.com/wojciechkepka/docker-api-rs/actions) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![Released API docs](https://docs.rs/docker-api/badge.svg)](http://docs.rs/docker-api)

> a rust interface to [Docker](https://www.docker.com/) containers

## Install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
docker-api = "0.5"
```

## Supported API
Default endpoints include:
 - Containers
 - Images
 - Networks
 - Volumes
 - Exec
 - System

To enable swarm endpoints add a `swarm` feature to `Cargo.toml` like so:
```toml
docker-api = { version = "0.5", features = ["swarm"] }
```

Swarm endpoints include:
 - Swarm
 - Nodes
 - Services
 - Tasks
 - Secrets
 - Configs
 - Plugins

## SSL Connection

To enable HTTPS connection to docker add a `tls` flag to `Cargo.toml`.

## Default features

By default only `chrono` feature is enabled. To disable it use:
```toml
docker-api = { version = "0.5", default-features = false }
```

## Usage

Examples for most API endpoints can be found in the [examples directory](https://github.com/wojciechkepka/docker-api-rs/tree/master/examples).


## Notice
This crate is a fork of [shiplift](https://github.com/softprops/shiplift).

## License
[MIT](https://raw.githubusercontent.com/wojciechkepka/docker-api-rs/master/LICENSE)
