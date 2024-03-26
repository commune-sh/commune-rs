use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/profile/:user_id/displayname",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

impl Request {
    pub fn new(user_id: OwnedUserId) -> Self {
        Self { user_id }
    }
}

#[response(error = crate::Error)]
pub struct Response {
    #[serde(rename = "displayname")]
    pub display_name: String,
}
