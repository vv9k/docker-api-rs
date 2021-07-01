use std::collections::HashMap;

use serde::Serialize;

impl_url_opts_builder!(PluginList);

pub enum PluginFilter {
    Capability(String),
    Enable,
    Disable,
}

impl PluginListOptsBuilder {
    pub fn filter<F>(&mut self, filters: F) -> &mut Self
    where
        F: IntoIterator<Item = PluginFilter>,
    {
        let mut param = HashMap::new();
        for f in filters {
            match f {
                PluginFilter::Capability(cap) => param.insert("capability", vec![cap]),
                PluginFilter::Enable => param.insert("enable", vec![true.to_string()]),
                PluginFilter::Disable => param.insert("enable", vec![false.to_string()]),
            };
        }
        // structure is a a json encoded object mapping string keys to a list
        // of string values
        self.params
            .insert("filters", serde_json::to_string(&param).unwrap_or_default());
        self
    }
}
