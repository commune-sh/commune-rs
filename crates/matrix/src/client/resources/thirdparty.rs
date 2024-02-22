use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::error::MatrixError;

pub struct ThirdPartyHandle;

#[derive(Default, Debug, Serialize)]
pub struct IdentityServer {
    #[serde(skip_serializing_if = "String::is_empty", rename = "id_access_token")]
    pub access_token: String,

    #[serde(skip_serializing_if = "String::is_empty", rename = "id_server")]
    pub server_name: String,
}

#[derive(Debug, Serialize)]
pub struct RequestTokenBody {
    email: String,

    client_secret: String,

    #[serde(flatten)]
    id_server: IdentityServer,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_link: Option<url::Url>,

    send_attempt: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RequestTokenResponse {
    #[serde(rename = "sid")]
    pub session_id: String,

    pub submit_url: Option<url::Url>,
}

impl ThirdPartyHandle {
    #[instrument(skip(client, email))]
    pub async fn request_email_token(
        client: &crate::http::Client,
        email: impl Into<String>,
        next_link: Option<url::Url>,
    ) -> Result<Option<bool>> {
        // Clones the client in order to temporally set a token for the `GET`
        // request

        let client_secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
        let resp = client
            .post_json(
                "/_matrix/client/v3/account/password/email/requestToken",
                &RequestTokenBody {
                    email: email.into(),
                    client_secret,
                    id_server: IdentityServer::default(),
                    next_link,
                    send_attempt: 1,
                },
            )
            .await?;

        if resp.status().is_success() {
            return Ok(Some(true));
        }

        let error = resp.json::<MatrixError>().await?;

        match error.errcode.as_str() {
            "M_THREEPID_IN_USE" => Ok(Some(false)),
            "M_THREEPID_NOT_FOUND" => Ok(Some(true)),
            "M_THREEPID_DENIED" => Ok(None),
            _ => Err(anyhow::anyhow!(error.error)),
        }
    }
}
