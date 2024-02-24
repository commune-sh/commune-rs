//! [Room Admin API](https://matrix-org.github.io/synapse/latest/admin_api/rooms.html)
//!
//! To use it, you will need to authenticate by providing an `access_token`
//! for a server admin: see Admin API.

use anyhow::Result;
use ruma_common::{serde::Raw, EventId, OwnedRoomAliasId, OwnedRoomId, OwnedUserId, RoomId};
use ruma_events::{AnyMessageLikeEvent, AnyStateEvent, AnyTimelineEvent};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{error::MatrixError, filter::RoomEventFilter, http::Client};

#[derive(Default)]
pub struct RoomService;

#[derive(Default, Debug, Serialize)]
pub struct ListRoomQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    pub order_by: OrderBy,

    pub direction: Direction,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub search_term: String,
}

#[derive(Debug, Default, Serialize)]
pub struct MessagesQuery {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub from: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub to: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RoomEventFilter>,

    pub direction: Direction,
}

#[derive(Default, Debug, Serialize)]
pub struct TimestampToEventQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts: Option<u64>,

    pub direction: Direction,
}

#[derive(Default, Debug, Serialize)]
pub struct EventContextQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RoomEventFilter>,
}

#[derive(Debug, Serialize)]
pub struct ReplaceRoomQuery {
    #[serde(rename = "new_room_user_id")]
    pub admin: OwnedUserId,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub room_name: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Default, Debug, Serialize)]
pub struct DeleteQuery {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub new_room: Option<ReplaceRoomQuery>,

    pub block: bool,

    pub purge: bool,
}

#[derive(Debug, Deserialize)]
pub struct ListRoomResponse {
    pub rooms: Vec<Room>,
    pub offset: Option<u64>,
    pub total_rooms: Option<u64>,
    pub prev_batch: Option<String>,
    pub next_batch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<OwnedUserId>,
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
    pub details: Option<RoomDetails>,
}

#[derive(Debug, Deserialize)]
pub struct RoomDetails {
    pub avatar: Option<String>,
    pub topic: Option<String>,
    pub joined_local_devices: u64,
    pub forgotten: bool,
}

#[derive(Debug, Deserialize)]
pub struct GetEventsResponse {
    pub chunk: Raw<Vec<AnyTimelineEvent>>,
    pub start: String,
    pub end: String,
    pub state: Option<Vec<State>>,
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

#[derive(Debug, Deserialize)]
pub struct EventContextResponse {
    pub start: String,
    pub end: String,
    pub events_before: Vec<Raw<AnyMessageLikeEvent>>,
    pub event: Raw<AnyMessageLikeEvent>,
    pub events_after: Vec<Raw<AnyMessageLikeEvent>>,
    pub state: Vec<Raw<AnyStateEvent>>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteRoomResponse {
    pub kicked_users: Vec<OwnedUserId>,
    pub failed_to_kick_users: Vec<OwnedUserId>,
    pub local_aliases: Vec<OwnedRoomAliasId>,
    pub new_room_id: Option<OwnedRoomId>,
}

impl RoomService {
    /// Returns information about a specific room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-details-api
    #[instrument(skip(client))]
    pub async fn get_one(client: &Client, room_id: &RoomId) -> Result<Room> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}",
                room_id = room_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Returns all rooms. By default, the response is ordered alphabetically by
    /// room name
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#list-room-api
    #[instrument(skip(client))]
    pub async fn get_all(client: &Client, query: ListRoomQuery) -> Result<ListRoomResponse> {
        let resp = client.get_query("/_synapse/admin/v1/rooms", &query).await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows a server admin to get a list of all members of a room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-members-api
    #[instrument(skip(client))]
    pub async fn get_members(client: &Client, room_id: &RoomId) -> Result<MembersResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/members",
                room_id = room_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows a server admin to get all messages sent to a room in a given
    /// timeframe
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-messages-api
    #[instrument(skip(client))]
    pub async fn get_state(client: &Client, room_id: &RoomId) -> Result<StateResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/state",
                room_id = room_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows a server admin to get the `event_id` of the closest event to the
    /// given timestamp
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-timestamp-to-event-api
    #[instrument(skip(client))]
    pub async fn get_timestamp_to_event(
        client: &Client,
        room_id: &RoomId,
        query: TimestampToEventQuery,
    ) -> Result<TimestampToEventResponse> {
        let resp = client
            .get_query(
                format!(
                    "/_synapse/admin/v1/rooms/{room_id}/timestamp_to_event",
                    room_id = room_id
                ),
                &query,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows a server admin to check the status of forward extremities for a
    /// room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#check-for-forward-extremities
    #[instrument(skip(client))]
    pub async fn check_forward_extremities(
        client: &Client,
        room_id: &RoomId,
    ) -> Result<CheckForwardExtremitiesResponse> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v1/rooms/{room_id}/forward_extremities",
                room_id = room_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows a server admin to delete forward extremities for a room
    /// WARNING: Please ensure you know what you're doing and read the related issue [#1760](https://github.com/matrix-org/synapse/issues/1760)
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#delete-for-forward-extremities
    #[instrument(skip(client))]
    pub async fn delete_forward_extremities(
        client: &Client,
        room_id: &RoomId,
    ) -> Result<DeleteForwardExtremitiesResponse> {
        let resp = client
            .delete(format!(
                "/_synapse/admin/v1/rooms/{room_id}/forward_extremities",
                room_id = room_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// allows server admins to remove rooms from the server and block these
    /// rooms
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#delete-room-api
    #[instrument(skip(client))]
    pub async fn delete_room(
        client: &Client,
        room_id: &RoomId,
        query: DeleteQuery,
    ) -> Result<DeleteRoomResponse> {
        let resp = client
            .delete_json(
                format!("/_synapse/admin/v1/rooms/{room_id}", room_id = room_id),
                &query,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}

impl RoomService {
    /// Allows a server admin to get a list of all state events in a room
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#room-state-api
    #[instrument(skip(client))]
    pub async fn get_room_events(
        client: &Client,
        room_id: &RoomId,
        query: MessagesQuery,
    ) -> Result<GetEventsResponse> {
        let resp = client
            .get_query(
                format!(
                    "/_synapse/admin/v1/rooms/{room_id}/messages",
                    room_id = room_id
                ),
                &query,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// This API lets a client find the context of an event. This is designed
    /// primarily to investigate abuse reports.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/rooms.html#event-context-api
    #[instrument(skip(client))]
    pub async fn get_event_context(
        client: &Client,
        room_id: &RoomId,
        event_id: &EventId,
        query: EventContextQuery,
    ) -> Result<EventContextResponse> {
        let resp = client
            .get_query(
                format!(
                    "/_synapse/admin/v1/rooms/{room_id}/context/{event_id}",
                    room_id = room_id,
                    event_id = event_id,
                ),
                &query,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub enum Direction {
    #[serde(rename = "f")]
    #[default]
    Forward,
    #[serde(rename = "b")]
    Backward,
}

#[derive(Default, Debug, Serialize)]
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
