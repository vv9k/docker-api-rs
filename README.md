# docker-api

[![GitHub Actions](https://github.com/wojciechkepka/docker-api-rs/workflows/Main/badge.svg)](https://github.com/wojciechkepka/docker-api-rs/actions) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![Released API docs](https://docs.rs/docker-api/badge.svg)](http://docs.rs/docker-api)

> a rust interface to [Docker](https://www.docker.com/) containers

## Install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
docker-api = "0.1"
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
docker-api = { version = "0.1", features = ["swarm"] }
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

## Usage

Many small runnable example programs can be found in this repository's [examples directory](https://github.com/wojciechkepka/docker-api-rs/tree/master/examples).


## Notice
This crate is a fork of [shiplift](https://github.com/softprops/shiplift).

## License
[MIT](https://raw.githubusercontent.com/wojciechkepka/docker-api-rs/master/LICENSE)