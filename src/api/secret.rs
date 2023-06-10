//! Secrets are sensitive data that can be used by services. Swarm mode must be enabled for these endpoints to work.

use crate::{
    conn::{Headers, Payload},
    models,
    opts::{SecretCreateOpts, SecretListOpts},
    Result,
};

impl_api_ty!(Secret => name);

impl Secret {
    impl_api_ep! { secret: Secret, resp
        Inspect -> &format!("/secrets/{}", secret.name), models::Secret
        Delete -> &format!("/secrets/{}", secret.name), ()
    }
    // TODO: add Secret::update
}

impl Secrets {
    impl_api_ep! { __: Secret, resp
        List -> "/secrets", models::Secret
    }

    api_doc! { Secret => Create
    |
    /// Create a new secret.
    pub async fn create(&self, opts: &SecretCreateOpts) -> Result<Secret> {
        use serde::Deserialize;
        #[derive(Deserialize)]
        struct SecretCreateResponse {
            #[serde(rename = "Id")]
            pub id: String,
        }
        self.docker
            .post_json("/secrets/create", Payload::Json(opts.serialize_vec()?), Headers::none())
            .await
            .map(|resp: SecretCreateResponse| {
                Secret::new(self.docker.clone(), resp.id)
            })
    }}
}
