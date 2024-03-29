use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomId, OwnedRoomOrAliasId, OwnedServerName,
};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/join/:room_id_or_alias",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub room_id_or_alias: OwnedRoomOrAliasId,

    #[ruma_api(query)]
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub server_name: Vec<OwnedServerName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl Request {
    pub fn new(room_id_or_alias: OwnedRoomOrAliasId, reason: Option<String>) -> Self {
        Self {
            room_id_or_alias,
            reason,
            server_name: Vec::new(),
        }
    }
}

#[response(error = crate::Error)]
#[derive(Serialize)]
pub struct Response {
    pub room_id: OwnedRoomId,
}
