use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedUserId, OwnedRoomId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/rooms/{room_id}/kick",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    pub user_id: OwnedUserId,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[response(error = crate::Error)]
pub struct Response {}
