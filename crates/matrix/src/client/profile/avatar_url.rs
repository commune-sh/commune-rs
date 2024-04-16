use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedMxcUri, OwnedUserId,
};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/profile/:user_id/avatar_url",
    }
};

#[request(error = crate::Error)]
#[derive(Serialize)]
pub struct Request {
    #[ruma_api(path)]
    pub user_id: OwnedUserId,

    pub avatar_url: OwnedMxcUri,
}

impl Request {
    pub fn new(user_id: OwnedUserId, avatar_url: OwnedMxcUri) -> Self {
        Self {
            user_id,
            avatar_url,
        }
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize)]
pub struct Response {}
