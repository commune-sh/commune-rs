use std::str::FromStr;

use anyhow::Result;
use ruma_common::serde::Raw;
use ruma_common::{OwnedEventId, OwnedRoomId, TransactionId};

use ruma_events::room::redaction::RoomRedactionEventContent;
use ruma_events::{MessageLikeEventContent, StateEventContent, StateEventType};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::Client;

pub struct Events;

#[derive(Serialize, Deserialize)]
pub struct CreateEventResponse {
    event_id: String,
}

impl Events {
    #[instrument(skip(client, access_token, content, room_id))]
    pub async fn send_message<T: MessageLikeEventContent>(
        client: &Client,
        access_token: impl Into<String>,
        content: T,
        room_id: OwnedRoomId,
    ) -> Result<CreateEventResponse> {
        let mut client = (*client).clone();
        client.set_token(access_token)?;

        let resp = client
            .put_json(
                format!(
                    "/_matrix/client/v3/rooms/{room_id}/send/{event_type}/{txn_id}",
                    room_id = room_id,
                    event_type = content.event_type(),
                    txn_id = TransactionId::new(),
                ),
                &content,
            )
            .await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token, content, event_type, room_id, state_key))]
    pub async fn send_state<S: AsRef<str>, T: StateEventContent>(
        client: &Client,
        access_token: impl Into<String>,
        content: Raw<T>,
        room_id: OwnedRoomId,
        event_type: StateEventType,
        state_key: S,
    ) -> Result<CreateEventResponse> {
        let mut client = (*client).clone();
        client.set_token(access_token)?;

        let resp = client
            .put_json(
                format!(
                    "/_matrix/client/v3/rooms/{room_id}/state/{event_type}/{state_key}",
                    room_id = room_id,
                    event_type = event_type,
                    state_key = state_key.as_ref(),
                ),
                &content,
            )
            .await?;

        Ok(resp.json().await?)
    }

    #[instrument(skip(client, access_token, room_id, event_id))]
    pub async fn send_redaction(
        client: &Client,
        access_token: impl Into<String>,
        room_id: OwnedRoomId,
        event_id: OwnedEventId,
        reason: Option<String>,
    ) -> Result<CreateEventResponse> {
        let mut client = (*client).clone();
        client.set_token(access_token)?;

        let content =
            RoomRedactionEventContent::new_v11(OwnedEventId::from_str(event_id.as_ref())?);

        let resp = client
            .put_json(
                format!(
                    "/_matrix/client/v3/rooms/{room_id}/redact/{event_id}/{txn_id}",
                    room_id = room_id,
                    event_id = event_id,
                    txn_id = TransactionId::new(),
                ),
                &content,
            )
            .await?;

        Ok(resp.json().await?)
    }
}
