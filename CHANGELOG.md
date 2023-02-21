#
- Fix Container::attach output when TTY is enabled on container
- Add `Image::build_par` that uses multithreaded compression algorithm for creating context directory archive. This method is behind `par-compression` feature flag.
- Add `ImageListOptsBuilder::shared_size` parameter
- Add `SystemDataUsageOpts` parameter to `Docker::data_usage`
- `Container::stop` and `Container::restart` now take `ContainerStopOpts` and `ContainerRestartOpts` as paramaters respectively

# 0.12.0
- Fix some integer fields that could be negative but previously were a usize like `ImageSummary::containers`
- Fix deserialization of nullable map types like `ImageSummary::labels`
- Rename `ExecContainerOpts` -> `ExecCreateOpts`
- Rename `ExecCreateOpts::cmd` -> `ExecCreateOpts::command`
- Rename `ContainerCreateOpts::cmd` -> `ContainerCreateOpts::command`
- Change `Container::copy_from` argument type from `&Path` to a more generic `impl AsRef<Path>`
- Change `ContainerCreateOpts::builder` to not require a name
- Add a missing `ContainerOptsBuilder::image` method to set the image of container
- Rename `ContainerOptsBuilder` -> `ContainerCreateOptsBuilder`
- Rename `RmContainerOpts` -> `ContainerRemoveOpts`
- Rename `RmImageOpts` -> `ImageRemoveOpts`
- Add `Clone` implementation to `ContainerCreateOpts`
- Add `Clone` implementation to `ImageBuildOpts`
- Add `Clone` implementation to `NetworkCreateOpts`

# 0.11.0
- *BREAKING* Rename `BuildOpts` -> `ImageBuildOpts` and `BuildOptsBuilder` -> `ImageBuildOptsBuilder`
- Fix `Configs::create` endpoint

# 0.10.0
- *BREAKING* Use models generated from swagger schema
- Remove `*Id` and `*IdRef` type aliases and use a common `Id` type for object IDs

# 0.9.0
- Fix `ContainersPruneInfo` deserialization
- Logs endpoint now correctly returns `TtyChunk` instead of `Bytes`
- *BREAKING* All API structs no longer have a `'docker` lifetime. This change makes it easier to create self working objects without the lifetime hell and according to
  hyper client documentation it is cheap to clone and cloning is the recommended way to share a client.
- Add `Container::commit` that creates a image based on an existing container
- Use rust 2021 edition

# 0.8.0
- Make `ContainerInfo::state` and `ContainerSummary::state` strongly typed.
- Fix `Docker::info` response deserialization
- Fix `Docker::data_usage` response deserialization
- Add missing `ContainerStatus::Configured`
- Fix `ContainerInfo::mounts` field deserialization
- Fix `ContainerDetails::config` field deserialization
- Fix `LogConfig::config` field deserialization
- Fix network creation [#11](https://github.com/vv9k/docker-api-rs/pull/11)
- Add missing `NetworkInfo::created` field
- Fix `Network::delete` endpoint
- Fix `Network::disconnect` endpoint. It now uses correct `ContainerDisconnectionOpts`.
- Add `ContainerCreateOptsBuilder::security_opts` function to specify container security options [#12](https://github.com/vv9k/docker-api-rs/pull/12)
- Add `NetworkCreateOptsBuilder::ipam` function to specify ip address management config of a network

# 0.7.0
- Make `PidsStats`, `Descriptor` and `DistributionInspectInfo` struct fields publicly accessible.
- Add ability to push image to registry with `Image::push` or `Images::push`.
- Add `online_cpus` field to `CpuStats`
- Fix `Image::history` response deserialization
- Fix `Container::logs` and `Service::logs` endpoints
- Fix `Stats` field name from `network_stats` -> `networks`
- Add missing clone implementations to some image api data types
- All builder pattern methods now take an owned value and consume the builder on final build
- Add a default implementation to `Isolation` to fix deserialization of `Info`
- Fix `Docker::data_usage` response deserialization - fields of `VolumeInfo`: `labels`, `options`, `status` are now an `Option`
- Add a way to initialize Docker with a different API version
- Fix `ImageSummary` deserialization - `repo_tags` field is now an option as it can be a null sometimes
- Add `Docker::new_versioned`, `Docker::unix_versioned`, `Docker::tls_versioned`, `Docker::tcp_versioned` initializers that let the user specify initially used client API version
- Add `Docker::adjust_api_version` that verifies the API version returned by the server and adjusts the client version for further requests
- Add Id and IdRef type aliases for each api type
- Fix `Images::prune` response deserialization
- Fix filter parameters serialization like `ImagePruneFilter` etc.
- Fix `Images::clear_cache` response deserialization
- Rename all `data` modules to `models`
- Make `Change::kind` field into strongly typed value
- Fix `Container::changes` response deserialization

# 0.6.0
- `name` field of `ContainerCreateOpts` is now private. Use the `ContainerCreateOpts::builder` function that takes in a `name` parameter.
- Use missing `name` parameter when creating a container [#6](https://github.com/vv9k/docker-api-rs/pull/6)
- `NetworkSettings::global_ipv6_prefix_len` is now correctly a number
- Fix return type of inspecting a container
- Add new fields to `HostConfig` - `blkio_weight`, `blkio_weight_device`, `device_cgroup_rules`, `kernel_memory`
- Fix name of `HealthcheckResult` field from `started` to `start`.

# 0.5.1
- Fix `ContainerConfig` desserialization (`cmd` field might be ommited from the response)

# 0.5.0
- Add missing `ContainerSpec` fields, use correct `TaskSpec` in `ServiceSpec`
- Fix `ContainerConfig::exposed_ports` serialization


# 0.4.0
- Fix list endpoints
- Add ability to create a image with labels
- Add lots of missing filter variants and document them
- `ContainerOptsBuilder::expose` and `ContainerOptsBuilder::publish` now take a strongly typed `PublishPort`
  as input parameter.
- Add missing fields to `NetworkCreateOptsBuilder`
- Add missing fields to `ContainerConnectionOptsBuilder`
- Rename `Mount` to `MountPoint`, add `Mount` struct  
- Add missing fields to `TaskSpec`
- Fix types on `ContainerConfig`, fix deserializing `ContainerConfig::exposed_ports`
- Add logging on transport for easier debugging
- Fix delete endpoints from failing to deserialize the return type

# 0.3.3
- Fix return type of `Image::inspect`
- Make api structs like `Container` thread safe
