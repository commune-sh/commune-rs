use anyhow::Result;
use ruma_common::{OwnedUserId};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::error::MatrixError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub device_id: String,
    pub is_guest: bool,
    pub user_id: OwnedUserId,
}

impl Session {
    /// Gets information about the owner of a given access token.
    ///
    /// Note that, as with the rest of the Client-Server API, Application
    /// Services may masquerade as users within their namespace by giving a
    /// user_id query parameter. In this situation, the server should verify
    /// that the given user_id is registered by the appservice, and return it
    /// in the response body.
    ///
    /// Refer: https://playground.matrix.org/#get-/_matrix/client/v3/account/whoami
    #[instrument(skip(client, access_token))]
    pub async fn get(
        client: &crate::http::Client,
        access_token: impl Into<String>,
    ) -> Result<Self> {
        // Clones the client in order to temporally set a token for the `GET`
        // request
        let mut tmp = (*client).clone();

        tmp.set_token(access_token)?;

        let resp = tmp.get("/_matrix/client/v3/account/whoami").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
