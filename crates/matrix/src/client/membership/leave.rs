use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/rooms/:room_id/leave",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl Request {
    pub fn new(room_id: OwnedRoomId, reason: Option<String>) -> Self {
        Self { room_id, reason }
    }
}

#[response(error = crate::Error)]
pub struct Response {
    pub room_id: OwnedRoomId,
}
