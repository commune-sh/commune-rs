use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedMxcUri, OwnedUserId,
};
use serde::{Deserialize, Serialize};

use crate::client::uiaa::UserIdentifier;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/login",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[serde(flatten, rename = "type")]
    pub kind: LoginType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<UserIdentifier>,

    #[serde(
        rename = "initial_device_display_name",
        skip_serializing_if = "String::is_empty"
    )]
    pub device_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<bool>,
}

impl Request {
    pub fn new(
        kind: LoginType,
        identifier: Option<UserIdentifier>,
        device_name: String,
        refresh_token: Option<bool>,
    ) -> Self {
        Self {
            kind,
            identifier,
            device_name,
            refresh_token,
        }
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize, Serialize)]
pub struct Response {
    pub access_token: String,

    pub device_id: OwnedDeviceId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in_ms: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    pub user_id: OwnedUserId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub well_known: Option<WellKnown>,
}

// impl Response {
//     pub fn new<S: Into<String>>(
//         access_token: S,
//         refresh_token: Option<S>,
//         expires_in_ms: Option<u64>,
//         user_id: impl Into<OwnedUserId>,
//         device_id: impl Into<OwnedDeviceId>,
//         well_known: Option<WellKnown>,
//     ) -> Self {
//         Self {
//             access_token: access_token.into(),
//             refresh_token: refresh_token.map(Into::into),
//             expires_in_ms,
//             device_id: device_id.into(),
//             user_id: user_id.into(),
//             well_known,
//         }
//     }
// }

#[derive(Clone, Debug, Serialize)]
pub struct IdentityProvider {
    pub id: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BaseUrl {
    pub base_url: url::Url,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WellKnown {
    #[serde(rename = "m.homeserver")]
    pub homeserver: BaseUrl,

    #[serde(rename = "m.identity_server")]
    pub identity_server: BaseUrl,
}
