use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/profile/:user_id/displayname",
    }
};

#[request(error = crate::Error)]
#[derive(Deserialize, Serialize)]
pub struct Request {
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    #[serde(rename = "displayname")]
    pub display_name: String,
}

impl Request {
    pub fn new(user_id: OwnedUserId, display_name: String) -> Self {
        Self {
            user_id,
            display_name,
        }
    }
}

#[response(error = crate::Error)]
pub struct Response {}
