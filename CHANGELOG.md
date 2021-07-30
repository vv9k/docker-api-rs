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
