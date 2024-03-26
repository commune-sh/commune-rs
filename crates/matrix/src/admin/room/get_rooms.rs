use ruma_common::{
    api::{request, response, Direction, Metadata},
    metadata,
};
use serde::Serialize;

use super::Room;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/rooms",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[serde(default)]
    #[ruma_api(query)]
    pub from: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<u64>,

    #[ruma_api(query)]
    pub order_by: OrderBy,

    #[ruma_api(query)]
    pub direction: Direction,

    #[serde(skip_serializing_if = "String::is_empty")]
    #[ruma_api(query)]
    pub search_term: String,
}

#[response(error = crate::Error)]
pub struct Response {
    pub rooms: Vec<Room>,

    pub offset: u64,

    #[serde(rename = "total_rooms")]
    pub total: u64,

    pub next_batch: Option<String>,

    pub prev_batch: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
    #[default]
    Name,

    CanonicalAlias,

    JoinedMembers,

    JoinedLocalMembers,

    Version,

    Creator,

    Encryption,

    Federatable,

    Public,

    JoinRules,

    GuestAccess,

    HistoryVisibility,

    StateEvents,
}
