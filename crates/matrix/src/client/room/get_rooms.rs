use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    room::RoomType,
    OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId, OwnedServerName,
};
use ruma_events::room::join_rules::JoinRule;
use serde::Deserialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/publicRooms",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(query)]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    #[ruma_api(query)]
    server: OwnedServerName,

    #[ruma_api(query)]
    #[serde(skip_serializing_if = "String::is_empty")]
    since: String,
}

#[response(error = crate::Error)]
pub struct Response {
    chunk: Vec<Room>,

    #[serde(skip_serializing_if = "Option::is_none")]
    next_batch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    prev_batch: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Room {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<OwnedMxcUri>,

    #[serde(rename = "canonical_alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<OwnedRoomAliasId>,

    pub join_rule: Option<JoinRule>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "num_joined_members")]
    pub members: Option<String>,

    pub room_id: OwnedRoomId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_type: Option<RoomType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    pub world_readable: bool,
}
