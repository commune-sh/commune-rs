use std::str::FromStr;

use anyhow::Result;
use ruma_common::serde::Raw;
use ruma_common::{OwnedEventId, OwnedRoomId, TransactionId};

use ruma_events::room::redaction::RoomRedactionEventContent;
use ruma_events::{AnyStateEventContent, MessageLikeEventContent, StateEventType};
use serde::Deserialize;
use tracing::instrument;

use crate::Client;

pub struct Events;

#[derive(Debug, Clone, Deserialize)]
pub struct SendResponse {
    pub event_id: String,
}

impl Events {
    #[instrument(skip(client, content, room_id))]
    pub async fn send_message<T: MessageLikeEventContent>(
        client: &Client,
        content: T,
        room_id: OwnedRoomId,
    ) -> Result<SendResponse> {
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

    #[instrument(skip(client, content, event_type, room_id, state_key))]
    pub async fn send_state<S: AsRef<str>>(
        client: &Client,
        content: Raw<AnyStateEventContent>,
        room_id: OwnedRoomId,
        event_type: StateEventType,
        state_key: S,
    ) -> Result<SendResponse> {
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

    #[instrument(skip(client, room_id, event_id))]
    pub async fn send_redaction(
        client: &Client,
        room_id: OwnedRoomId,
        event_id: OwnedEventId,
        reason: Option<String>,
    ) -> Result<SendResponse> {
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
