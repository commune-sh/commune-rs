use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    room::RoomType,
    serde::Raw,
    OwnedRoomId, OwnedUserId, RoomVersionId,
};
use ruma_events::{
    room::{create::PreviousRoom, power_levels::RoomPowerLevelsEventContent},
    AnyInitialStateEvent,
};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/createRoom",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_content: Option<RoomCreationContent>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub initial_state: Vec<Raw<AnyInitialStateEvent>>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub invite: Vec<OwnedUserId>,

    pub is_direct: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_override: Option<RoomPowerLevelsEventContent>,

    #[serde(rename = "room_alias_name", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    pub visibility: RoomVisibility,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoomCreationContent {
    #[serde(rename = "m.federate")]
    pub federate: bool,

    #[serde(rename = "type")]
    pub room_version: RoomVersionId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub predecessor: Option<PreviousRoom>,

    #[serde(rename = "type")]
    pub kind: Option<RoomType>,
}

#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomVisibility {
    Public,

    #[default]
    Private,
}

impl Request {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        creation_content: Option<RoomCreationContent>,
        initial_state: Vec<Raw<AnyInitialStateEvent>>,
        invite: Vec<OwnedUserId>,
        is_direct: bool,
        name: Option<String>,
        power_override: Option<RoomPowerLevelsEventContent>,
        alias: Option<String>,
        topic: Option<String>,
        visibility: RoomVisibility,
    ) -> Self {
        Self {
            creation_content,
            initial_state,
            invite,
            is_direct,
            name,
            power_override,
            alias,
            topic,
            visibility,
        }
    }
}

#[response(error = crate::Error)]
#[derive(Deserialize, Serialize)]
pub struct Response {
    pub room_id: OwnedRoomId,
}
