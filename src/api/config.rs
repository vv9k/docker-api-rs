//! Configs are application configurations that can be used by services.
//! Swarm mode must be enabled for these endpoints to work.

use crate::{
    conn::{Headers, Payload},
    models,
    opts::{ConfigCreateOpts, ConfigListOpts},
    Result,
};

impl_api_ty!(Config => name);

impl Config {
    impl_api_ep! { cfg: Config, resp
        Inspect -> &format!("/configs/{}", cfg.name), models::Config
        Delete -> &format!("/configs/{}", cfg.name), ()
    }

    // TODO: add Config::update
}

impl Configs {
    impl_api_ep! { __: Config, resp
        List -> "/configs", models::Config
    }

    api_doc! {
    Config => Create
    |
    /// Create a new config.
    pub async fn create(&self, opts: &ConfigCreateOpts) -> Result<Config> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct ConfigCreateResponse {
            #[serde(rename = "Id")]
            pub id: String,
        }
        self.docker
            .post_json("/configs/create", Payload::Json(opts.serialize_vec()?), Headers::none())
            .await
            .map(|resp: ConfigCreateResponse| {
                Config::new(self.docker.clone(), resp.id)
            })
    }}
}
