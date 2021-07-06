use crate::api::Filter;

impl_opts_builder!(json => VolumeCreate);

impl VolumeCreateOptsBuilder {
    impl_str_field!(
        /// The new volume's name. If not specified, Docker generates a name.
        name: N => "Name"
    );

    impl_str_field!(
        /// Name of the volume driver to use.
        driver: D => "Driver"
    );

    impl_map_field!(json
        /// A mapping of driver options and values.
        /// These options are passed directly to the driver and are driver specific.
        driver_opts: O => "DriverOpts");

    impl_map_field!(json
        /// User-defined key/value metadata.
        labels: L => "Labels"
    );
}

impl_opts_builder!(url => VolumePrune);

impl_opts_builder!(url => VolumeList);

/// Filter type used to filter volumes by one of the variants.
pub enum VolumeFilter {
    /// When set to `true`, returns all volumes that are not in use by a container.
    /// When set to `false`, only volumes that are in use by one or more containers are returned.
    Dangling(bool),
    /// Matches volumes based on their driver.
    Driver(String),
    /// Label in the form of `label=key`.
    LabelKey(String),
    /// Label in the form of `label=key=val`.
    Label { key: String, val: String },
    /// Matches all or part of a volume name.
    Name(String),
}

impl Filter for VolumeFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use VolumeFilter::*;
        match &self {
            Dangling(dangling) => ("dangling", dangling.to_string()),
            Driver(driver) => ("driver", driver.to_owned()),
            LabelKey(label) => ("label", label.to_owned()),
            Label { key, val } => ("label", format!("{}:{}", key, val)),
            Name(name) => ("name", name.to_owned()),
        }
    }
}

impl VolumePruneOptsBuilder {
    impl_filter_func!(
        /// Filter pruned volumes by one of the variants of the filter enum.
        VolumeFilter
    );
}

impl VolumeListOptsBuilder {
    impl_filter_func!(
        /// Filter listed volumes by one of the variants of the filter enum.
        VolumeFilter
    );
}
