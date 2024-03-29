use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedRoomId,
};
use ruma_events::AnyStateEvent;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/rooms/{room_id}/state",
    }
};

#[request(error = crate::Error)]
#[derive(Serialize)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

impl Request {
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id }
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Response {
    pub event_id: Vec<Raw<AnyStateEvent>>,
}
