use super::Room;
use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms/:room_id",
        }
};

#[request(error = crate::Error)]
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
pub struct Response {
    #[ruma_api(body)]
    pub room: Room,
}
