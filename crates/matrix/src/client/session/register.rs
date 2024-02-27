use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedUserId,
};

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
    pub username: String,

    pub password: String,

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
}
