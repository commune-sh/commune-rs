use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedUserId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/account/whoami",
    }
};

#[request(error = crate::Error)]
pub struct Request {}

#[response(error = crate::Error)]
pub struct Response {
    pub device_id: OwnedDeviceId,

    pub user_id: OwnedUserId,
}
