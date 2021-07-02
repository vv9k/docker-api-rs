pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

/// Allows easier construction of filter functions for multiple api endpoints
pub(crate) trait Filter {
    fn query_key_val(&self) -> (&'static str, String);
}
