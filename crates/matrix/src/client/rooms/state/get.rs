use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedRoomId,
};
use ruma_events::{AnyStateEventContent, StateEventType};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/rooms/{room_id}/state/{event_type}/{state_key}",
    }
};

#[request(error = crate::Error)]
#[derive(Serialize)]
pub struct Request {
    #[ruma_api(path)]
    pub event_type: StateEventType,

    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    #[ruma_api(path)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state_key: String,
}

impl Request {
    pub fn new(
        event_type: StateEventType,
        room_id: OwnedRoomId,
        state_key: Option<String>,
    ) -> Self {
        Self {
            event_type,
            room_id,
            state_key: state_key.unwrap_or_default(),
        }
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct Response {
    pub content: Raw<AnyStateEventContent>,
}
