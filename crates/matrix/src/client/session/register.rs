use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedMxcUri, thirdparty::Medium, OwnedDeviceId, OwnedUserId,
};
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "",
    }
};

#[request(error = crate::Error)]
pub struct Request {
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

#[response(error = crate::Error)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct BaseUrl {
    pub base_url: url::Url,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WellKnown {
    #[serde(rename = "m.homeserver")]
    pub homeserver: BaseUrl,

    #[serde(rename = "m.identity_server")]
    pub identity_server: BaseUrl,
}
