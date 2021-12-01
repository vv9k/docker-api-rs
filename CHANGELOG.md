#
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
