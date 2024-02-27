use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedRoomOrAliasId, OwnedRoomId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/join/{alias_or_id}",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub alias_or_id: OwnedRoomOrAliasId,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[response(error = crate::Error)]
pub struct Response {
   pub room_id: OwnedRoomId,
}
