use containers_api::opts::{Filter, FilterItem};
use containers_api::{
    impl_field, impl_filter_func, impl_map_field, impl_opts_builder, impl_opts_required_builder,
    impl_str_field,
};

impl_opts_builder!(json => VolumeCreate);

impl VolumeCreateOptsBuilder {
    impl_str_field!(
        /// The new volume's name. If not specified, Docker generates a name.
        name => "Name"
    );

    impl_str_field!(
        /// Name of the volume driver to use.
        driver => "Driver"
    );

    impl_map_field!(json
        /// A mapping of driver options and values.
        /// These options are passed directly to the driver and are driver specific.
        driver_opts => "DriverOpts");

    impl_map_field!(json
        /// User-defined key/value metadata.
        labels => "Labels"
    );

    impl_field!(
        /// Cluster-specific options used to create the volume.
        cluster_spec: crate::models::ClusterVolumeSpec => "ClusterVolumeSpec"
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
    fn query_item(&self) -> FilterItem {
        use VolumeFilter::*;
        match &self {
            Dangling(dangling) => FilterItem::new("dangling", dangling.to_string()),
            Driver(driver) => FilterItem::new("driver", driver.to_owned()),
            LabelKey(label) => FilterItem::new("label", label.to_owned()),
            Label { key, val } => FilterItem::new("label", format!("{key}:{val}")),
            Name(name) => FilterItem::new("name", name.to_owned()),
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

impl_opts_required_builder!(json =>
    /// Update swarm cluster volume
    ClusterVolumeUpdate,
    /// The version number of the volume being updated. This is required to avoid conflicting writes. Found in the volume's ClusterVolume field.
    version: i64 => "version"
);

impl ClusterVolumeUpdateOptsBuilder {
    impl_str_field!(
        /// Group defines the volume group of this volume. Volumes belonging to the same group can be referred to by group name when creating Services.
        /// Referring to a volume by group instructs Swarm to treat volumes in that group interchangeably for the purpose of scheduling. Volumes with
        /// an empty string for a group technically all belong to the same, emptystring group.
        group => "Group"
    );

    impl_field!(
        /// Defines how the volume is used by tasks.
        access_mode: serde_json::Value => "AccessMode"
    );
}
