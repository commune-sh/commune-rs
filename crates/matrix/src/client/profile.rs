pub mod avatar_url;
pub mod displayname;

use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedMxcUri, OwnedUserId,
};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/profile/:user_id",
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
#[derive(Deserialize, Serialize)]
pub struct Response {
    pub displayname: Option<String>,
    pub avatar_url: Option<OwnedMxcUri>,
}
