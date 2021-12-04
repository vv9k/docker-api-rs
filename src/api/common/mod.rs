pub mod models;
pub mod opts;

pub use models::*;
pub use opts::*;

/// Allows easier construction of filter functions for multiple api endpoints
pub(crate) trait Filter {
    // TODO: Add a stronger return type. Not all filters are `key=val`, soma are only `key`
    fn query_key_val(&self) -> (&'static str, String);
}
