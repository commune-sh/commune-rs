use matrix::client::resources::events::SendResponse;
use matrix::events::exports::ruma_common::RoomId;
use matrix::events::space::board::{SpaceReactionEventContent, Vote, VoteKind};
use matrix::events::space::state::SpaceStateEventContent;
use matrix::events::Raw;
use matrix::events::{
    exports::ruma_common::{OwnedEventId, TransactionId},
    space::board::{BoardPostEventContent, BoardReplyEventContent},
};
use reqwest::{Client as HttpClient, Response as HttpResponse};
use serde::Serialize;
use serde_json::{from_str, json};

use crate::events::error::BoardErrorCode;
use crate::util::secret::Secret;
use crate::{Error, Result};

pub struct EventsService {
    destination: url::Url,
    http: HttpClient,
}

impl EventsService {
    pub fn new(synapse_host: &str) -> Self {
        let destination = url::Url::parse(synapse_host)
            .expect("`synapse_host` should have already been validated");

        Self {
            destination,
            http: HttpClient::new(),
        }
    }

    pub async fn send_post<S: AsRef<str>>(
        &self,
        content: Raw<BoardPostEventContent>,
        board_id: S,
        token: Secret,
    ) -> Result<SendResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;

        let content = content
            .deserialize()
            .map_err(|err| BoardErrorCode::MalformedContent(err))?;

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.post/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token, content).await.map_err(|err| {
            tracing::error!(?err, "Failed to forward custom event.");
            Error::Unknown
        })?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response.");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reply<S: AsRef<str>>(
        &self,
        content: Raw<BoardReplyEventContent>,
        board_id: S,
        token: Secret,
    ) -> Result<SendResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;

        let content = content
            .deserialize()
            .map_err(|_| BoardErrorCode::MalformedRelationId)?;

        match content
            .relates_to
            .clone()
            .map(|relation| relation.rel_type())
        {
            Some(None) => {}
            relation => return Err(BoardErrorCode::WrongRelation(relation.flatten()).into()),
        };

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.reply/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token, content).await.map_err(|err| {
            tracing::error!(?err, "Failed to forward custom event.");
            Error::Unknown
        })?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response.");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reaction<S: AsRef<str>>(
        &self,
        board_id: S,
        event_id: S,
        key: S,
        token: Secret,
    ) -> Result<SendResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;
        let event_id = OwnedEventId::try_from(event_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedRelationId)?;
        let key = from_str::<VoteKind>(key.as_ref())
            .map_err(|err| BoardErrorCode::MalformedContent(err))?;

        let content = SpaceReactionEventContent::new(Vote { event_id, key });

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.reaction/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token, content).await.map_err(|err| {
            tracing::error!(?err, "Failed to forward custom event.");
            Error::Unknown
        })?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response.");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_redaction<S: AsRef<str>>(
        &self,
        board_id: S,
        event_id: S,
        reason: S,
        token: Secret,
    ) -> Result<SendResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;
        let event_id = OwnedEventId::try_from(event_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedRelationId)?;

        let content = json!({"reason": reason.as_ref()});

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/redact/{event_id}/{txn_id}",
            board_id = board_id,
            event_id = event_id,
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token, content).await.map_err(|err| {
            tracing::error!(?err, "Failed to forward custom event.");
            Error::Unknown
        })?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response.");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_state<S: AsRef<str>>(
        &self,
        content: Raw<impl SpaceStateEventContent>,
        board_id: S,
        state_key: S,
        token: Secret,
    ) -> Result<SendResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;

        let content = content
            .deserialize()
            .map_err(|err| BoardErrorCode::MalformedContent(err))?;

        let event_type = content.event_type().to_string();

        let endpoint = format!(
            " /_matrix/client/v3/rooms/{board_id}/state/{event_type}/{state_key}",
            board_id = board_id,
            event_type = event_type,
            state_key = state_key.as_ref(),
        );

        let resp = self.put(endpoint, token, content).await?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

    // does this belong here?
    async fn put(
        &self,
        endpoint: impl AsRef<str>,
        _token: Secret,
        json: impl Serialize,
    ) -> Result<HttpResponse> {
        let mut url = self.destination.clone();
        url.set_path(endpoint.as_ref());

        self.http.put(url).json(&json).send().await.map_err(|err| {
            tracing::error!(?err, "Failed to complete PUT request");
            Error::Unknown
        })
    }
}
