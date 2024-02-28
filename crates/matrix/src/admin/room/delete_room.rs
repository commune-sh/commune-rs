use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId, OwnedUserId,
};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: DELETE,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/rooms/:room_id",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub new_room: Option<NewRoomParams>,

    pub block: bool,

    #[serde(skip_serializing_if = "ruma_common::serde::is_true")]
    pub purge: bool,

    pub force_purge: bool,
}

#[response(error = crate::Error)]
pub struct Response {
    pub delete_id: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewRoomParams {
    pub creator: OwnedUserId,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}
