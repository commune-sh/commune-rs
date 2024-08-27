//! This module contains handlers for getting and synchronizing events.
//!
//! reference: https://github.com/matrix-org/matrix-spec-proposals/pull/3575

use std::{collections::BTreeMap, time::Duration};

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::{deserialize_cow_str, duration::opt_ms, Raw},
    DeviceKeyAlgorithm, MilliSecondsSinceUnixEpoch, OwnedMxcUri, OwnedRoomId, OwnedUserId, RoomId,
};
use ruma_events::{
    receipt::SyncReceiptEvent, typing::SyncTypingEvent, AnyGlobalAccountDataEvent,
    AnyRoomAccountDataEvent, AnyStrippedStateEvent, AnySyncStateEvent, AnySyncTimelineEvent,
    AnyToDeviceEvent, StateEventType, TimelineEventType,
};
use serde::{self, de::Error as _, Deserialize, Serialize};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/unstable/org.matrix.msc3575/sync",
        // 1.4 => "/_matrix/client/v4/sync",
    }
};

#[request(error = crate::Error)]
#[derive(Default)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub pos: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub conn_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_id: Option<String>,

    #[serde(with = "opt_ms", default, skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub timeout: Option<Duration>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, SyncRequestList>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub room_subscriptions: BTreeMap<OwnedRoomId, RoomSubscription>,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub unsubscribe_rooms: Vec<OwnedRoomId>,

    #[serde(default, skip_serializing_if = "ExtensionsConfig::is_empty")]
    pub extensions: ExtensionsConfig,
}

#[response(error = crate::Error)]
pub struct Response {
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub initial: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub txn_id: Option<String>,

    pub pos: String,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub lists: BTreeMap<String, SyncList>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rooms: BTreeMap<OwnedRoomId, SlidingSyncRoom>,

    #[serde(default, skip_serializing_if = "Extensions::is_empty")]
    pub extensions: Extensions,

    pub delta_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UnreadNotificationsCount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_count: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_count: Option<usize>,
}

impl UnreadNotificationsCount {
    pub fn is_empty(&self) -> bool {
        self.highlight_count.is_none() && self.notification_count.is_none()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DeviceLists {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changed: Vec<OwnedUserId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub left: Vec<OwnedUserId>,
}

impl DeviceLists {
    pub fn is_empty(&self) -> bool {
        self.changed.is_empty() && self.left.is_empty()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SyncRequestListFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dm: Option<bool>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spaces: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_invite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tombstoned: Option<bool>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub room_types: Vec<String>,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub not_room_types: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_name_like: Option<String>,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub tags: Vec<String>,

    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub not_tags: Vec<String>,

    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SyncRequestList {
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub slow_get_all_rooms: bool,

    pub ranges: Vec<(usize, usize)>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sort: Vec<String>,

    #[serde(flatten)]
    pub room_details: RoomDetailsConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_old_rooms: Option<IncludeOldRooms>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SyncRequestListFilters>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bump_event_types: Vec<TimelineEventType>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RoomDetailsConfig {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_state: Vec<(StateEventType, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_limit: Option<usize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IncludeOldRooms {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_state: Vec<(StateEventType, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_limit: Option<usize>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RoomSubscription {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_state: Vec<(StateEventType, String)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_limit: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SlidingOp {
    Sync,

    Insert,

    Delete,

    Invalidate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ops: Vec<SyncOp>,

    pub count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncOp {
    pub op: SlidingOp,

    pub range: Option<(usize, usize)>,

    pub index: Option<usize>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub room_ids: Vec<OwnedRoomId>,

    pub room_id: Option<OwnedRoomId>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SlidingSyncRoom {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<OwnedMxcUri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dm: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_state: Option<Vec<Raw<AnyStrippedStateEvent>>>,

    #[serde(
        flatten,
        default,
        skip_serializing_if = "UnreadNotificationsCount::is_empty"
    )]
    pub unread_notifications: UnreadNotificationsCount,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timeline: Vec<Raw<AnySyncTimelineEvent>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_state: Vec<Raw<AnySyncStateEvent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_batch: Option<String>,

    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub limited: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_count: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_count: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_live: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<MilliSecondsSinceUnixEpoch>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ExtensionsConfig {
    #[serde(default, skip_serializing_if = "ToDeviceConfig::is_empty")]
    pub to_device: ToDeviceConfig,

    #[serde(default, skip_serializing_if = "E2EEConfig::is_empty")]
    pub e2ee: E2EEConfig,

    #[serde(default, skip_serializing_if = "AccountDataConfig::is_empty")]
    pub account_data: AccountDataConfig,

    #[serde(default, skip_serializing_if = "ReceiptsConfig::is_empty")]
    pub receipts: ReceiptsConfig,

    #[serde(default, skip_serializing_if = "TypingConfig::is_empty")]
    pub typing: TypingConfig,

    #[serde(flatten)]
    other: BTreeMap<String, serde_json::Value>,
}

impl ExtensionsConfig {
    pub fn is_empty(&self) -> bool {
        self.to_device.is_empty()
            && self.e2ee.is_empty()
            && self.account_data.is_empty()
            && self.receipts.is_empty()
            && self.typing.is_empty()
            && self.other.is_empty()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Extensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_device: Option<ToDevice>,

    #[serde(default, skip_serializing_if = "E2EE::is_empty")]
    pub e2ee: E2EE,

    #[serde(default, skip_serializing_if = "AccountData::is_empty")]
    pub account_data: AccountData,

    #[serde(default, skip_serializing_if = "Receipts::is_empty")]
    pub receipts: Receipts,

    #[serde(default, skip_serializing_if = "Typing::is_empty")]
    pub typing: Typing,
}

impl Extensions {
    pub fn is_empty(&self) -> bool {
        self.to_device.is_none()
            && self.e2ee.is_empty()
            && self.account_data.is_empty()
            && self.receipts.is_empty()
            && self.typing.is_empty()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ToDeviceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<OwnedRoomId>>,
}

impl ToDeviceConfig {
    pub fn is_empty(&self) -> bool {
        self.enabled.is_none() && self.limit.is_none() && self.since.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ToDevice {
    pub next_batch: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<Raw<AnyToDeviceEvent>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct E2EEConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl E2EEConfig {
    pub fn is_empty(&self) -> bool {
        self.enabled.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct E2EE {
    #[serde(default, skip_serializing_if = "DeviceLists::is_empty")]
    pub device_lists: DeviceLists,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub device_one_time_keys_count: BTreeMap<DeviceKeyAlgorithm, usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_unused_fallback_key_types: Option<Vec<DeviceKeyAlgorithm>>,
}

impl E2EE {
    pub fn is_empty(&self) -> bool {
        self.device_lists.is_empty()
            && self.device_one_time_keys_count.is_empty()
            && self.device_unused_fallback_key_types.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct AccountDataConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<OwnedRoomId>>,
}

impl AccountDataConfig {
    pub fn is_empty(&self) -> bool {
        self.enabled.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AccountData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub global: Vec<Raw<AnyGlobalAccountDataEvent>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rooms: BTreeMap<OwnedRoomId, Vec<Raw<AnyRoomAccountDataEvent>>>,
}

impl AccountData {
    pub fn is_empty(&self) -> bool {
        self.global.is_empty() && self.rooms.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RoomReceiptConfig {
    AllSubscribed,

    Room(OwnedRoomId),
}

impl Serialize for RoomReceiptConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RoomReceiptConfig::AllSubscribed => serializer.serialize_str("*"),
            RoomReceiptConfig::Room(r) => r.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RoomReceiptConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        match deserialize_cow_str(deserializer)?.as_ref() {
            "*" => Ok(RoomReceiptConfig::AllSubscribed),
            other => Ok(RoomReceiptConfig::Room(
                RoomId::parse(other).map_err(D::Error::custom)?.to_owned(),
            )),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ReceiptsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<RoomReceiptConfig>>,
}

impl ReceiptsConfig {
    pub fn is_empty(&self) -> bool {
        self.enabled.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Receipts {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rooms: BTreeMap<OwnedRoomId, Raw<SyncReceiptEvent>>,
}

impl Receipts {
    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct TypingConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lists: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rooms: Option<Vec<OwnedRoomId>>,
}

impl TypingConfig {
    pub fn is_empty(&self) -> bool {
        self.enabled.is_none()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Typing {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub rooms: BTreeMap<OwnedRoomId, Raw<SyncTypingEvent>>,
}

impl Typing {
    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use ruma_common::owned_room_id;

    use super::RoomReceiptConfig;

    #[test]
    fn serialize_room_receipt_config() {
        let entry = RoomReceiptConfig::AllSubscribed;
        assert_eq!(serde_json::to_string(&entry).unwrap().as_str(), r#""*""#);

        let entry = RoomReceiptConfig::Room(owned_room_id!("!n8f893n9:example.com"));
        assert_eq!(
            serde_json::to_string(&entry).unwrap().as_str(),
            r#""!n8f893n9:example.com""#
        );
    }

    #[test]
    fn deserialize_room_receipt_config() {
        assert_eq!(
            serde_json::from_str::<RoomReceiptConfig>(r#""*""#).unwrap(),
            RoomReceiptConfig::AllSubscribed
        );

        assert_eq!(
            serde_json::from_str::<RoomReceiptConfig>(r#""!n8f893n9:example.com""#).unwrap(),
            RoomReceiptConfig::Room(owned_room_id!("!n8f893n9:example.com"))
        );
    }
}
