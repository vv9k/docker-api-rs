use crate::api::Filter;

impl_json_opts_builder!(VolumeCreate);

impl VolumeCreateOptsBuilder {
    impl_str_field!(name: N => "Name");

    impl_map_field!(labels: L => "Labels");
}

impl_url_opts_builder!(VolumePrune);

pub enum VolumePruneFilter {
    Dangling(bool),
    Driver(String),
    LabelKey(String),
    LabelKeyVal(String, String),
    Name(String),
}

impl Filter for VolumePruneFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        use VolumePruneFilter::*;
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
    impl_filter_func!(VolumePruneFilter);
}
