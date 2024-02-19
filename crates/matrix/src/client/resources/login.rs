use anyhow::Result;
use ruma_common::{thirdparty::Medium, OwnedDeviceId, OwnedMxcUri, OwnedUserId, UserId};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{error::MatrixError, http::Client};

#[derive(Clone, Debug, Serialize)]
pub struct IdentityProvider {
    pub id: String,
    pub name: String,
    pub icon: Option<OwnedMxcUri>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum LoginType {
    #[serde(rename = "m.login.password")]
    Password { password: String },

    #[serde(rename = "m.login.token")]
    Token { token: String },

    #[serde(rename = "m.login.sso")]
    Sso {
        #[serde(skip_serializing_if = "<[_]>::is_empty")]
        identity_providers: Vec<IdentityProvider>,
    },

    #[serde(rename = "m.login.application_service")]
    ApplicationService,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum Identifier {
    #[serde(rename = "m.id.user")]
    User { user: String },

    #[serde(rename = "m.id.thirdparty")]
    ThirdParty { medium: Medium, address: String },

    #[serde(rename = "m.id.phone")]
    Phone { country: String, phone: String },
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginRequest {
    #[serde(flatten, rename = "type")]
    pub kind: LoginType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<Identifier>,

    #[serde(
        rename = "initial_device_display_name",
        skip_serializing_if = "String::is_empty"
    )]
    pub device_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum LoginTypeOnly {
    #[serde(rename = "m.login.password")]
    Password,

    #[serde(rename = "m.login.token")]
    Token,

    #[serde(rename = "m.login.sso")]
    Sso,

    #[serde(rename = "m.login.application_service")]
    ApplicationService,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginFlowResponse {
    pub get_login_token: Option<bool>,
    pub kind: LoginTypeOnly,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct BaseUrl {
    pub base_url: url::Url,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct WellKnown {
    #[serde(rename = "m.homeserver")]
    pub homeserver: BaseUrl,

    #[serde(rename = "m.identity_server")]
    pub identity_server: BaseUrl,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub access_token: String,
    pub device_id: OwnedDeviceId,
    pub expires_in_ms: Option<u64>,
    pub refresh_token: Option<String>,
    pub user_id: OwnedUserId,
    pub well_known: Option<WellKnown>,
}

pub struct LoginHandle;

impl LoginHandle {
    #[instrument(skip(client, username, password))]
    pub async fn login(
        client: &Client,
        username: &UserId,
        password: impl Into<String>,
    ) -> Result<LoginResponse> {
        let resp = client
            .post_json(
                "/_matrix/client/v3/login",
                &LoginRequest {
                    kind: LoginType::Password {
                        password: password.into(),
                    },
                    identifier: Some(Identifier::User {
                        user: username.to_string(),
                    }),
                    device_name: Default::default(),
                    refresh_token: Default::default(),
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
    pub async fn get_login_flows(client: &Client) -> Result<LoginFlowResponse> {
        let resp = client.get("/_matrix/client/v3/login").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    #[instrument(skip(client))]
    pub async fn redirect_sso(client: &Client) -> Result<()> {
        let resp = client.get("/_matrix/client/v3/login").await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}

#[cfg(test)]
mod tests {
    use ruma_common::{thirdparty::Medium, OwnedDeviceId, OwnedUserId};
    use serde_json::json;

    use crate::client::resources::login::{
        BaseUrl, Identifier, LoginResponse, LoginType, WellKnown,
    };

    use super::LoginRequest;

    #[test]
    fn serialize_password_login() {
        assert_eq!(
            serde_json::to_value(&LoginRequest {
                kind: LoginType::Password {
                    password: "ilovebananas".to_owned(),
                },
                identifier: Some(Identifier::User {
                    user: "cheeky_monkey".to_owned(),
                }),
                device_name: "Jungle Phone".to_owned(),
                refresh_token: None,
            })
            .unwrap(),
            json!(
            {
              "identifier": {
                "type": "m.id.user",
                "user": "cheeky_monkey"
              },
              "initial_device_display_name": "Jungle Phone",
              "password": "ilovebananas",
              "type": "m.login.password"
            }

            )
        );
    }

    #[test]
    fn serialize_password_login_with_3pid() {
        assert_eq!(
            serde_json::to_value(&LoginRequest {
                kind: LoginType::Password {
                    password: "ilovebananas".to_owned(),
                },
                identifier: Some(Identifier::ThirdParty {
                    medium: Medium::Msisdn,
                    address: "+12086964143".to_owned()
                }),
                device_name: Default::default(),
                refresh_token: None,
            })
            .unwrap(),
            json!(
                {
                  "type": "m.login.password",
                  "identifier": {
                   "type": "m.id.thirdparty",
                    "medium": "msisdn",
                    "address": "+12086964143"
                  },
                  "password": "ilovebananas"
                }
            )
        );
    }

    #[test]
    fn serialize_token_login() {
        assert_eq!(
            serde_json::to_value(&LoginRequest {
                kind: LoginType::Token {
                    token: "abcdef".to_owned()
                },
                identifier: None,
                device_name: Default::default(),
                refresh_token: None,
            })
            .unwrap(),
            json!(
                {
                  "type": "m.login.token",
                  "token": "abcdef"
                }

            )
        );
    }

    #[test]
    fn deserialize_token_login() {
        let json_data = json!(
        {
          "access_token": "abc123",
          "device_id": "GHTYAJCE",
          "expires_in_ms": 60000,
          "refresh_token": "def456",
          "user_id": "@cheeky_monkey:matrix.org",
          "well_known": {
            "m.homeserver": {
              "base_url": "https://example.org"
            },
            "m.identity_server": {
              "base_url": "https://id.example.org"
            }
          }
        });

        let data = serde_json::from_value::<LoginResponse>(json_data).unwrap();

        assert_eq!(
            data,
            LoginResponse {
                access_token: "abc123".to_owned(),
                device_id: OwnedDeviceId::from("GHTYAJCE"),
                expires_in_ms: Some(60000),
                refresh_token: Some("def456".to_owned()),
                user_id: OwnedUserId::try_from("@cheeky_monkey:matrix.org").unwrap(),
                well_known: Some(WellKnown {
                    homeserver: BaseUrl {
                        base_url: "https://example.org".parse().unwrap()
                    },
                    identity_server: BaseUrl {
                        base_url: "https://id.example.org".parse().unwrap()
                    }
                })
            }
        );
    }
}
