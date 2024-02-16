use anyhow::Result;
use ruma_common::{serde::Raw, EventId, OwnedEventId, RoomId, TransactionId};

use ruma_events::{
    relation::RelationType, AnyStateEvent, AnyStateEventContent,
    AnyTimelineEvent, MessageLikeEventContent, MessageLikeEventType, StateEventContent,
    StateEventType,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::instrument;

use crate::{admin::resources::room::Direction, filter::RoomEventFilter, Client, error::MatrixError};

pub struct EventsService;

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetMessagesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    pub dir: Direction,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub filter: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetRelationsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    pub dir: Direction,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesResponse {
    pub chunk: Vec<Raw<AnyTimelineEvent>>,
    pub start: String,
    pub end: String,
    pub state: Option<Vec<Raw<AnyStateEvent>>>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct GetStateResponse(Vec<Raw<AnyStateEvent>>);

#[derive(Debug, Deserialize)]
pub struct GetRelationsResponse {
    pub chunk: Vec<Raw<AnyTimelineEvent>>,
    pub prev_batch: Option<String>,
    pub next_batch: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct SendRedactionBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub event_id: OwnedEventId,
}

#[derive(Debug, Deserialize)]
pub struct SendStateResponse {
    pub event_id: OwnedEventId,
}

#[derive(Debug, Deserialize)]
pub struct SendRedactionResponse {
    pub event_id: OwnedEventId,
}

impl EventsService {
    #[instrument(skip(client, access_token))]
    pub async fn get_event(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        event_id: &EventId,
    ) -> Result<Raw<AnyTimelineEvent>> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .get(format!(
                "/_matrix/client/v3/rooms/{room_id}/event/{event_id}",
                room_id = room_id,
                event_id = event_id,
            ))
            .await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token))]
    pub async fn get_messages(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        query: GetMessagesQuery,
    ) -> Result<GetMessagesResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .get_query(format!(
                "/_matrix/client/v3/rooms/{room_id}/messages",
                room_id = room_id,
            ), &query)
            .await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token))]
    pub async fn get_state(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
    ) -> Result<GetStateResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .get(format!(
                "/_matrix/client/v3/rooms/{room_id}/state",
                room_id = room_id,
            ))
            .await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token))]
    pub async fn get_state_content(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        event_type: StateEventType,
        state_key: Option<String>,
    ) -> Result<Raw<AnyStateEventContent>> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let mut path = format!(
            "/_matrix/client/v3/rooms/{room_id}/state/{event_type}",
            room_id = room_id,
            event_type = event_type,
        );

        if let Some(state_key) = state_key {
            path.push_str(&format!("/{state_key}", state_key = state_key))
        }

        let resp = tmp.get(path).await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token))]
    pub async fn get_relations<M: DeserializeOwned>(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        event_id: &EventId,
        rel_type: Option<Option<RelationType>>,
        event_type: Option<MessageLikeEventType>,
        query: GetRelationsQuery,
    ) -> Result<GetRelationsResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let mut path = format!(
            "/_matrix/client/v3/rooms/{room_id}/relations/{event_id}",
            room_id = room_id,
            event_id = event_id,
        );

        if let Some(rel_type) = rel_type {
            path.push_str(&format!(
                "/{rel_type}",
                rel_type = rel_type
                    .as_ref()
                    .map_or("m.in_reply_to".into(), ToString::to_string)
            ));

            if let Some(event_type) = event_type {
                path.push_str(&format!("/{event_type}", event_type = event_type))
            }
        }

        let resp = tmp.get_query(path, &query).await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token, body))]
    pub async fn send_message<T: MessageLikeEventContent>(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        txn_id: &TransactionId,
        body: T,
    ) -> Result<SendMessageResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .put_json(
                format!(
                    "/_matrix/client/v3/rooms/{room_id}/send/{event_type}/{txn_id}",
                    room_id = room_id,
                    event_type = body.event_type(),
                    txn_id = txn_id,
                ),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    #[instrument(skip(client, access_token, body))]
    pub async fn send_state<T: StateEventContent>(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        state_key: Option<String>,
        body: T,
    ) -> Result<SendStateResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let mut path = format!(
            "/_matrix/client/v3/rooms/{room_id}/state/{event_type}",
            room_id = room_id,
            event_type = body.event_type(),
        );

        if let Some(state_key) = state_key {
            path.push_str(&format!("/{state_key}", state_key = state_key))
        }

        let resp = tmp.put_json(path, &body).await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    #[instrument(skip(client, access_token, body))]
    pub async fn send_redaction(
        client: &Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        event_id: &EventId,
        txn_id: &TransactionId,
        body: SendRedactionBody,
    ) -> Result<SendRedactionResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .put_json(
                format!(
                    "/_matrix/client/v3/rooms/{room_id}/redact/{event_id}/{txn_id}",
                    room_id = room_id,
                    event_id = event_id,
                    txn_id = txn_id,
                ),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
