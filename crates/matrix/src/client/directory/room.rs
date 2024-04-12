use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomAliasId, OwnedRoomId, OwnedServerName,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/directory/room/{room_alias}",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_alias: OwnedRoomAliasId,
}

impl Request {
    pub fn new(room_alias: OwnedRoomAliasId) -> Self {
        Self { room_alias }
    }
}

#[response(error = crate::Error)]
pub struct Response {
    pub room_id: OwnedRoomId,
    pub servers: Vec<OwnedServerName>,
}
