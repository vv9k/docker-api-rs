use crate::api::Filter;

use std::collections::HashMap;

impl_url_opts_builder!(PluginList);

pub enum PluginFilter {
    Capability(String),
    Enable,
    Disable,
}

impl Filter for PluginFilter {
    fn query_key_val(&self) -> (&'static str, String) {
        match &self {
            PluginFilter::Capability(cap) => ("capability", cap.to_owned()),
            PluginFilter::Enable => ("enable", true.to_string()),
            PluginFilter::Disable => ("enable", false.to_string()),
        }
    }
}

impl PluginListOptsBuilder {
    impl_filter_func!(PluginFilter);
}
