use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId,
};
use serde::Deserialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms/:room_id/state",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,
}

#[response(error = crate::Error)]
pub struct Response {
    pub state: Vec<State>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct State {
    #[serde(rename = "type")]
    pub kind: String,

    pub state_key: String,

    pub etc: bool,
}
