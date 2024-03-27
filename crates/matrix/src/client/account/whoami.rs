use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedDeviceId, OwnedUserId,
};
use serde::{Serialize, Deserialize};

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

impl Request {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize, Serialize)]
pub struct Response {
    pub device_id: OwnedDeviceId,
    pub user_id: OwnedUserId,
}
