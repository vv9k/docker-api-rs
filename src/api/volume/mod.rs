//! Create and manage persistent storage that can be attached to containers.
pub mod data;
pub mod opts;

pub use data::*;
pub use opts::*;

use crate::{conn::Payload, Result};

impl_api_ty!(Volume => name: N);

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
