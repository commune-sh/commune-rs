use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedUserId,
};
use serde::{Deserialize, Serialize};

use crate::client::uiaa::{Auth, UiaaResponse};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/register",
    }
};

#[request(error = UiaaResponse)]
pub struct Request {
    pub username: String,

    pub password: String,

    #[serde(
        rename = "initial_device_display_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub device_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<bool>,

    /// Note that this information is not used to define how the registered user
    /// should be authenticated, but is instead used to authenticate the
    /// register call itself. It should be left empty, or omitted, unless an
    /// earlier call returned an response with status code 401.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
}

impl Request {
    pub fn new(
        username: String,
        password: String,
        device_name: Option<String>,
        refresh_token: Option<bool>,
        auth: Option<Auth>,
    ) -> Self {
        Self {
            username,
            password,
            device_name,
            refresh_token,
            auth,
        }
    }
}

#[response(error = UiaaResponse)]
#[derive(Deserialize, Serialize)]
pub struct Response {
    #[serde(default)]
    pub access_token: Option<String>,

    #[serde(default)]
    pub device_id: Option<OwnedDeviceId>,

    #[serde(default)]
    pub expires_in_ms: Option<u64>,

    #[serde(default)]
    pub refresh_token: Option<String>,

    pub user_id: OwnedUserId,
}
