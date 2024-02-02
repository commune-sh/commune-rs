use std::sync::Arc;

use matrix::admin::resources::room::{MessagesParams, RoomEventFilter, RoomService};
use matrix::client::resources::events::CreateEventResponse;
use matrix::events::exports::ruma_common::{EventId, RoomId};
use matrix::events::space::board::{
    BoardPostEvent, BoardReplyEvent, SpaceReactionEventContent, Vote,
};
use matrix::events::{
    exports::ruma_common::TransactionId,
    space::board::{BoardPostEventContent, BoardReplyEventContent},
};
use matrix::events::{Raw, StateEventContent};
use matrix::Client as MatrixAdminClient;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, json};

use crate::events::error::BoardErrorCode;
use crate::util::secret::Secret;
use crate::{Error, Result};

// TODO: find out how to serialize the full event
// #[derive(Serialize)]
pub struct GetPostResponse {
    pub post: BoardPostEvent,
    pub replies: Option<Vec<BoardReplyEvent>>,
}

pub struct EventsService {
    admin: Arc<MatrixAdminClient>,
}

impl EventsService {
    pub fn new(admin: Arc<MatrixAdminClient>) -> Self {
        Self { admin }
    }

    async fn put_json(
        &self,
        path: impl AsRef<str>,
        body: &impl Serialize,
        access_token: Secret,
    ) -> Result<Response> {
        let mut tmp = (*self.admin).clone();
        tmp.set_token(access_token.to_string()).map_err(|err| {
            tracing::error!(?err, "Failed to set access token");
            Error::Unknown
        })?;

        tmp.put_json(path, &body).await.map_err(|err| {
            tracing::error!(?err, "Failed to make http request");
            Error::Unknown
        })
    }

    pub async fn send_post<S: AsRef<str>>(
        &self,
        content: Raw<BoardPostEventContent>,
        board_id: S,
        access_token: Secret,
    ) -> Result<CreateEventResponse> {
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

        let resp = self
            .put_json(endpoint, &content, access_token)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to forward custom event");
                Error::Unknown
            })?;

        let data: CreateEventResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reply<S: AsRef<str>>(
        &self,
        content: Raw<BoardReplyEventContent>,
        board_id: S,
        access_token: Secret,
    ) -> Result<CreateEventResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;

        let content = content
            .deserialize()
            .map_err(|_| BoardErrorCode::MalformedEventId)?;

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

        let resp = self
            .put_json(endpoint, &content, access_token)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to forward custom event");
                Error::Unknown
            })?;

        let data: CreateEventResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reaction<S: AsRef<str>>(
        &self,
        board_id: S,
        event_id: S,
        key: S,
        access_token: Secret,
    ) -> Result<CreateEventResponse> {
        let board_id = <&RoomId>::try_from(board_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedBoardId)?;
        let event_id = <&EventId>::try_from(event_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedEventId)?;

        let key = from_str(key.as_ref())
            .map_err(|err| BoardErrorCode::MalformedContent(err))?;
        let content = SpaceReactionEventContent::new(Vote {
            event_id: event_id.into(),
            key,
        });

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.reaction/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let resp = self
            .put_json(endpoint, &content, access_token)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to forward custom event");
                Error::Unknown
            })?;

        let data: CreateEventResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_redaction<S: AsRef<str>>(
        &self,
        board_id: S,
        event_id: S,
        reason: S,
        access_token: Secret,
    ) -> Result<CreateEventResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;
        let event_id = <&EventId>::try_from(event_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedEventId)?;

        let content = json!({
            "reason": reason.as_ref()
        });

        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/redact/{event_id}/{txn_id}",
            board_id = board_id,
            event_id = event_id,
            txn_id = TransactionId::new(),
        );

        let resp = self
            .put_json(endpoint, &content, access_token)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to complete request");
                Error::Unknown
            })?;

        let data: CreateEventResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed deserialize Synapse response");
            Error::Unknown
        })?;

        Ok(data)
    }

    // should this only accept space state events or forward regular ones too?
    pub async fn send_state<S: AsRef<str>, C: StateEventContent + DeserializeOwned>(
        &self,
        content: Raw<C>,
        board_id: S,
        state_key: S,
        access_token: Secret,
    ) -> Result<CreateEventResponse> {
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

        let resp = self
            .put_json(endpoint, &content, access_token)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to complete request");
                Error::Unknown
            })?;

        let data: CreateEventResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn get_post<S: AsRef<str>>(
        &self,
        board_id: S,
        event_id: S,
        _access_token: Secret,
        with_replies: Option<u64>,
    ) -> Result<GetPostResponse> {
        let board_id =
            <&RoomId>::try_from(board_id.as_ref()).map_err(|_| BoardErrorCode::MalformedBoardId)?;
        let event_id = <&EventId>::try_from(event_id.as_ref())
            .map_err(|_| BoardErrorCode::MalformedEventId)?;

        let resp = RoomService::get_room_events::<BoardPostEvent>(
            &self.admin,
            board_id,
            MessagesParams {
                // according to the spec this is mandatory but when left out
                // we still receive a response
                from: String::from(""),
                limit: Some(1),
                filter: Some(RoomEventFilter {
                    types: Some(vec![String::from("space.board.post")]),
                    ..Default::default()
                }),
                to: None,
                direction: None,
            },
        )
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to get board post");
            Error::Unknown
        })?;

        let Some(post) = resp.chunk.into_iter().find(|e| e.event_id() == event_id) else {
            tracing::error!(?event_id, "Failed to find the right board post");
            return Err(Error::Unknown);
        };

        if let Some(limit) = with_replies {
            let resp = RoomService::get_room_events(
                &self.admin,
                board_id.as_ref(),
                MessagesParams {
                    from: String::from(""),
                    limit: Some(limit),
                    filter: Some(RoomEventFilter {
                        // this is not good
                        types: Some(vec![String::from("space.board.replies")]),
                        ..Default::default()
                    }),
                    to: None,
                    direction: None,
                },
            )
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to get board replies");
                Error::Unknown
            })?;

            let replies = Some(resp.chunk);

            return Ok(GetPostResponse { post, replies });
        }

        Ok(GetPostResponse {
            post,
            replies: None,
        })
    }
}
