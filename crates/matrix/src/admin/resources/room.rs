//! [Room Admin API](https://matrix-org.github.io/synapse/latest/admin_api/rooms.html)
//!
//! To use it, you will need to authenticate by providing an `access_token`
//! for a server admin: see Admin API.

use anyhow::Result;
use ruma::{events::{AnyTimelineEvent, AnyStateEvent}, OwnedRoomId, OwnedEventId, OwnedUserId};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::http::Client;

#[derive(Default)]
pub struct RoomService;

#[derive(Debug, Default, Serialize)]
pub struct ListParams {
    pub from: Option<u64>,
    pub limit: Option<u64>,
    pub order_by: Option<OrderBy>,
    pub direction: Option<Direction>,
    pub search_term: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    /// Room ID postfixed with Matrix instance Host
    /// E.g. `!room:example.com`
    pub room_id: OwnedRoomId,
    pub name: Option<String>,
    pub canonical_alias: Option<String>,
    pub joined_members: u64,
    pub joined_local_members: u64,
    pub version: Option<String>,
    pub creator: Option<String>,
    pub encryption: Option<String>,
    pub federatable: bool,
    pub public: bool,
    pub join_rules: Option<String>,
    pub guest_access: Option<String>,
    pub history_visibility: Option<String>,
    pub state_events: u64,
    pub room_type: Option<String>,
    #[serde(flatten)]
    pub details: Option<RoomExt>,
}

#[derive(Debug, Deserialize)]
pub struct RoomExt {
    pub avatar: Option<String>,
    pub topic: Option<String>,
    pub joined_local_devices: u64,
    pub forgotten: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListResponse {
    pub rooms: Vec<Room>,
    pub offset: Option<u64>,
    pub total_rooms: Option<u64>,
    pub prev_batch: Option<String>,
    pub next_batch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<String>,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct State {
    #[serde(rename = "type")]
    pub kind: String,
    pub state_key: String,
    pub etc: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct StateResponse {
    pub state: Vec<State>,
}

#[derive(Default, Debug, Serialize)]
pub struct RoomEventFilter {
    pub not_types: Vec<String>,
    pub not_rooms: Vec<OwnedRoomId>,
    pub limit: Option<u64>,
    pub rooms: Option<Vec<OwnedRoomId>>,
    pub not_senders: Vec<OwnedUserId>,
    pub senders: Option<Vec<OwnedUserId>>,
    pub types: Option<Vec<String>>,
    pub url_filter: Option<bool>,
    pub lazy_load_options: Option<bool>,
    pub unread_thread_notifications: bool,
}

#[derive(Debug, Serialize)]
pub struct MessagesParams {
    pub from: String,
    pub to: Option<String>,
    pub limit: Option<u64>,
    pub filter: Option<RoomEventFilter>,
    pub direction: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct MessagesResponse {
    pub chunk: Vec<AnyTimelineEvent>,
    pub start: String,
    pub end: String,
    pub state: Option<Vec<State>>,
}

#[derive(Debug, Default, Serialize)]
pub struct TimestampToEventParams {
    pub ts: Option<u64>,
    pub direction: Option<Direction>,
}

#[derive(Debug, Deserialize)]
pub struct TimestampToEventResponse {
    pub event_id: String,
    pub origin_server_ts: u64,
}

#[derive(Debug, Deserialize)]
pub struct ForwardExtremities {
    pub event_id: String,
    pub state_group: u64,
    pub depth: u64,
    pub received_ts: u64,
}

#[derive(Debug, Deserialize)]
pub struct CheckForwardExtremitiesResponse {
    pub count: u64,
    pub result: Vec<ForwardExtremities>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteForwardExtremitiesResponse {
    pub deleted: u64,
}

#[derive(Default, Debug, Serialize)]
pub struct EventContextParams {
    pub limit: Option<u64>,
    pub filter: Option<RoomEventFilter>,
}

#[derive(Debug, Deserialize)]
pub struct EventContextResponse {
    pub start: String,
    pub end: String,
    pub events_before: Vec<AnyTimelineEvent>,
    pub event: AnyTimelineEvent,
    pub events_after: Vec<AnyTimelineEvent>,
    pub state: Vec<AnyStateEvent>,
}

impl RoomService {
    /// Returns information about a specific room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-details-api
    #[instrument(skip(client))]
    pub async fn get_one(client: &Client, room_id: OwnedRoomId) -> Result<Room> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}",
                room_id = room_id
            ))
            .await?;
        let data: Room = resp.json().await?;

        Ok(data)
    }

    /// Returns all rooms. By default, the response is ordered alphabetically by room name
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#list-room-api
    #[instrument(skip(client))]
    pub async fn get_all(client: &Client, params: ListParams) -> Result<ListResponse> {
        let resp = client
            .get_query("/_synapse/admin/v1/rooms", &params)
            .await?;
        let data: ListResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to get a list of all members of a room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-members-api
    #[instrument(skip(client))]
    pub async fn get_members(client: &Client, room_id: OwnedRoomId) -> Result<MembersResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/members",
                room_id = room_id
            ))
            .await?;
        let data: MembersResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to get a list of all state events in a room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-state-api
    #[instrument(skip(client))]
    pub async fn get_state(client: &Client, room_id: OwnedRoomId) -> Result<StateResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/state",
                room_id = room_id
            ))
            .await?;
        let data: StateResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to get all messages sent to a room in a given timeframe
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-messages-api
    #[instrument(skip(client))]
    pub async fn get_messages(
        client: &Client,
        room_id: OwnedRoomId,
        params: MessagesParams,
    ) -> Result<MessagesResponse> {
        let resp = client
            .get_query(
                format!(
                    "/_synapse/admin/v1/rooms/{room_id}/messages",
                    room_id = room_id
                ),
                &params,
            )
            .await?;
        let data: MessagesResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to get the `event_id` of the closest event to the given timestamp
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-timestamp-to-event-api
    #[instrument(skip(client))]
    pub async fn get_timestamp_to_event(
        client: &Client,
        room_id: OwnedRoomId,
        params: TimestampToEventParams,
    ) -> Result<TimestampToEventResponse> {
        let resp = client
            .get_query(
                format!(
                    "/_synapse/admin/v1/rooms/{room_id}/timestamp_to_event",
                    room_id = room_id
                ),
                &params,
            )
            .await?;
        let data: TimestampToEventResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to check the status of forward extremities for a room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#check-for-forward-extremities
    #[instrument(skip(client))]
    pub async fn check_forward_extremities(
        client: &Client,
        room_id: OwnedRoomId,
    ) -> Result<CheckForwardExtremitiesResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/forward_extremities",
                room_id = room_id
            ))
            .await?;
        let data: CheckForwardExtremitiesResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to delete forward extremities for a room
    /// WARNING: Please ensure you know what you're doing and read the related issue [#1760](https://github.com/matrix-org/synapse/issues/1760)
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#delete-for-forward-extremities
    #[instrument(skip(client))]
    pub async fn delete_forward_extremities(
        client: &Client,
        room_id: OwnedRoomId,
    ) -> Result<DeleteForwardExtremitiesResponse> {
        let resp = client
            .delete(format!(
                "/_synapse/admin/v1/rooms/{room_id}/forward_extremities",
                room_id = room_id
            ))
            .await?;
        let data: DeleteForwardExtremitiesResponse = resp.json().await?;

        Ok(data)
    }

    /// Allows a server admin to delete forward extremities for a room
    /// WARNING: Please ensure you know what you're doing and have read the related issue [#1760](https://github.com/matrix-org/synapse/issues/1760)
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#delete-for-forward-extremities
    #[instrument(skip(client))]
    pub async fn get_event_context(
        client: &Client,
        room_id: OwnedRoomId,
        event_id: OwnedEventId,
        params: EventContextParams,
    ) -> Result<EventContextResponse> {
        let resp = client
            .get_query(format!(
                "/_synapse/admin/v1/rooms/{room_id}/context/{event_id}",
                room_id = room_id,
                event_id = event_id,
            ),&params)
            .await?;
        let data: EventContextResponse = resp.json().await?;

        Ok(data)
    }
}

#[derive(Debug, Serialize)]
pub enum Direction {
    #[serde(rename = "f")]
    Forward,
    #[serde(rename = "b")]
    Backward,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderBy {
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
