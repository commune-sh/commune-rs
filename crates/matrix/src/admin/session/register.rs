use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedServerName, OwnedUserId,
};

use super::Hmac;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/register",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    pub nonce: String,

    pub username: String,

    pub password: String,

    #[serde(skip_deserializing_if = "String::is_empty")]
    pub displayname: String,

    pub admin: bool,

    pub hmac: Hmac,
}

#[response(error = crate::Error)]
pub struct Response {
    pub access_token: String,

    pub user_id: OwnedUserId,

    pub home_server: OwnedServerName,

    pub device_id: OwnedDeviceId,
}
