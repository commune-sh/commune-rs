use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{error::MatrixError, http::Client};

#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginCredentialsPayload {
    pub r#type: &'static str,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginFlow {
    pub get_login_token: Option<bool>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoginFlows {
    pub flows: Vec<LoginFlow>,
}

pub struct Login;

impl Login {
    /// Retrieves an access token by logging in with Username and Password
    ///
    /// This is equivalent to executing:
    ///
    /// ```ignore
    /// curl -sS -d '{"type":"m.login.password", "user":"X", "password":"Y"}' http://server:port/_matrix/client/v3/login
    /// ```
    #[instrument(skip(client, username, password))]
    pub async fn login_credentials(
        client: &Client,
        username: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<LoginCredentials> {
        let resp = client
            .post_json(
                "/_matrix/client/v3/login",
                &LoginCredentialsPayload {
                    r#type: "m.login.password",
                    user: username.as_ref().to_string(),
                    password: password.as_ref().to_string(),
                },
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    #[instrument(skip(client))]
    pub async fn get_login_flows(client: &Client) -> Result<LoginFlows> {
        let resp = client.get("/_matrix/client/v3/login").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    #[instrument(skip(client))]
    pub async fn redirect_sso(client: &Client) -> Result<LoginFlows> {
        let resp = client.get("/_matrix/client/v3/login").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
