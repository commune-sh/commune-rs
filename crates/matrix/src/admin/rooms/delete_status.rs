use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/rooms/:room_id/delete_status",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response(error = crate::Error)]
pub struct Response {}
