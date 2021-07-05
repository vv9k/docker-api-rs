use crate::api::Filter;

impl_json_opts_builder!(VolumeCreate);

impl VolumeCreateOptsBuilder {
    impl_str_field!("The new volume's name. If not specified, Docker generates a name." name: N => "Name");

    impl_str_field!("Name of the volume driver to use." driver: D => "Driver");

    impl_map_field!(json
        "A mapping of driver options and values."
        "These options are passed directly to the driver and are driver specific."
        driver_opts: O => "DriverOpts");

    impl_map_field!(json "User-defined key/value metadata." labels: L => "Labels");
}

impl_url_opts_builder!(derives = Default | VolumePrune);

impl_url_opts_builder!(derives = Default | VolumeList);
pub enum VolumeFilter {
    Dangling(bool),
    Driver(String),
    LabelKey(String),
    LabelKeyVal(String, String),
    Name(String),
}

impl Filter for VolumeFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use VolumeFilter::*;
        match &self {
            Dangling(dangling) => ("dangling", dangling.to_string()),
            Driver(driver) => ("driver", driver.to_owned()),
            LabelKey(label) => ("label", label.to_owned()),
            LabelKeyVal(key, val) => ("label", format!("{}:{}", key, val)),
            Name(name) => ("name", name.to_owned()),
        }
    }
}

impl VolumePruneOptsBuilder {
    impl_filter_func!(VolumeFilter);
}

impl VolumeListOptsBuilder {
    impl_filter_func!(VolumeFilter);
}
