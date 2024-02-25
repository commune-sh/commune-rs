use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId, OwnedUserId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms/:room_id/members",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response(error = crate::Error)]
pub struct Response {
    pub members: Vec<OwnedUserId>,

    pub total: u64,
}
