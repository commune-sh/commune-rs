use ruma_common::{OwnedRoomId, OwnedUserId};
use serde::Serialize;

#[derive(Default, Debug, Serialize)]
pub struct RoomEventFilter {
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_types: Vec<String>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_senders: Vec<OwnedUserId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub senders: Vec<OwnedUserId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub types: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_urls: Option<bool>,

    pub lazy_load_members: bool,

    pub unread_thread_notifications: bool,
}
