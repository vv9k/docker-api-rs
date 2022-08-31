use containers_api::opts::{Filter, FilterItem};
use containers_api::{impl_filter_func, impl_opts_builder};

impl_opts_builder!(url => PluginList);

pub enum PluginFilter {
    Capability(String),
    Enable,
    Disable,
}

impl Filter for PluginFilter {
    fn query_item(&self) -> FilterItem {
        match &self {
            PluginFilter::Capability(cap) => FilterItem::new("capability", cap.to_owned()),
            PluginFilter::Enable => FilterItem::new("enable", true.to_string()),
            PluginFilter::Disable => FilterItem::new("enable", false.to_string()),
        }
    }
}

impl PluginListOptsBuilder {
    impl_filter_func!(
        /// Filter listed plugins by the variants of the enum.
        PluginFilter
    );
}
