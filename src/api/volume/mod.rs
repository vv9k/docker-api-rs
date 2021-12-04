//! Create and manage persistent storage that can be attached to containers.
pub mod models;
pub mod opts;

pub use models::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Volume => name);

impl<'docker> Volume<'docker> {
    impl_api_ep! {vol: Volume, resp
        Inspect -> &format!("/volumes/{}", vol.name)
        Delete -> &format!("/volumes/{}", vol.name)
    }
}

impl<'docker> Volumes<'docker> {
    impl_api_ep! {__: Volume, resp
        Create -> "/volumes/create", resp.name
        List -> "/volumes", VolumesInfo
        Prune -> "/volumes/prune"
    }
}
