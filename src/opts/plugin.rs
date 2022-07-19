use containers_api::opts::Filter;
use containers_api::{impl_filter_func, impl_opts_builder};

impl_opts_builder!(url => PluginList);

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
    impl_filter_func!(
        /// Filter listed plugins by the variants of the enum.
        PluginFilter
    );
}
