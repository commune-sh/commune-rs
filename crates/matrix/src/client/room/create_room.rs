use std::collections::BTreeMap;

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    power_levels::NotificationPowerLevels,
    serde::Raw,
    OwnedRoomId, OwnedUserId,
};
use ruma_events::{AnyInitialStateEvent, TimelineEventType};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
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

    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(
        rename = "power_level_content_override",
        skip_serializing_if = "Option::is_none"
    )]
    pub power_override: Option<RoomPowerLevels>,

    pub preset: RoomPreset,

    #[serde(rename = "room_alias_name", skip_serializing_if = "String::is_empty")]
    pub alias: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub topic: String,
}

#[response(error = crate::Error)]
pub struct Response {
    pub room_id: OwnedRoomId,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoomCreationContent {
    #[serde(rename = "m.federate")]
    pub federate: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomPreset {
    PublicChat,

    PrivateChat,

    #[default]
    TrustedPrivateChat,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoomPowerLevels {
    pub ban: u64,

    pub events: BTreeMap<TimelineEventType, u64>,

    pub events_default: u64,

    pub invite: u64,

    pub kick: u64,

    pub redact: u64,

    pub state_default: u64,

    pub users: BTreeMap<OwnedUserId, u64>,

    pub users_default: u64,

    pub notifications: NotificationPowerLevels,
}
