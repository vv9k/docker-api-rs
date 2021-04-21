# 0.2.0
* PullOptionsBuilder now adds a `latest` tag by default
* `ContainerOptionsBuilder::entrypoint` now correctly takes an `IntoIterator<Item = AsRef<str>>` instead of `&str`
* `HostConfig::init` field is now an Option
* `Container::remove` now takes a reference to `RmContainerOptions` instead of owned value
* All API handles like `Containers` or `Image` now implement `Debug`
* Add missing fields to `ExecContainerOptions`
