//! reference: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html
//!
//! This module contains handlers for managing rooms.

use ruma_common::{
    room::RoomType, EventEncryptionAlgorithm, OwnedMxcUri, OwnedRoomAliasId, OwnedRoomId,
    OwnedUserId, RoomVersionId,
};
use ruma_events::room::{history_visibility::HistoryVisibility, join_rules::JoinRule};
use serde::Deserialize;

mod delete_room;
mod get_members;
pub mod get_room;
pub mod get_rooms;
mod get_state;

#[derive(Clone, Debug, Deserialize)]
pub struct Room {
    pub room_id: OwnedRoomId,

    pub canonical_alias: Option<OwnedRoomAliasId>,

    pub avatar: Option<OwnedMxcUri>,

    pub name: Option<String>,

    pub joined_members: u64,

    pub joined_local_members: u64,

    pub version: RoomVersionId,

    pub creator: OwnedUserId,

    pub encryption: Option<EventEncryptionAlgorithm>,

    pub federatable: bool,

    pub public: bool,

    pub join_rules: Option<JoinRule>,

    pub history_visibility: Option<HistoryVisibility>,

    pub state_events: u64,

    pub room_type: Option<RoomType>,

    #[serde(flatten)]
    pub details: Option<RoomDetails>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RoomDetails {
    pub topic: Option<String>,

    pub forgotten: bool,
}
