use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedUserId,
};
use serde::{Deserialize, Serialize};

use crate::client::uiaa::UiaaRequest;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/register",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    username: String,

    password: String,

    #[serde(
        rename = "initial_device_display_name",
        skip_serializing_if = "String::is_empty"
    )]
    device_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<bool>,

    /// Note that this information is not used to define how the registered user should be
    /// authenticated, but is instead used to authenticate the register call itself.
    /// It should be left empty, or omitted, unless an earlier call returned an response
    /// with status code 401.
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<UiaaRequest>,
}

impl Request {
    pub fn new(username: &str, password: &str, device_name: &str, auth: Option<UiaaRequest>) -> Self {
        Self {
            username: username.to_owned(),
            password: password.to_owned(),
            device_name: device_name.to_owned(),
            refresh_token: Some(true),
            auth
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
}
