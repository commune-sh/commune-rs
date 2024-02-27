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
        unstable => "/_matrix/client/v3/rooms/{room_id}/leave",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[response(error = crate::Error)]
pub struct Response {}
