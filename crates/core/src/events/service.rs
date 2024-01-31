use matrix::client::resources::events::SendResponse;
use matrix::events::exports::ruma_common::OwnedRoomId;
use matrix::events::space::board::{SpaceReactionEventContent, Vote, VoteKind};
use matrix::events::{
    exports::ruma_common::{OwnedEventId, TransactionId},
    relation::InReplyTo,
    room::message::Relation,
    space::board::{BoardPostEventContent, BoardReplyEventContent},
};
use reqwest::{Client as HttpClient, Response as HttpResponse};
use serde::Serialize;

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

    pub async fn send_post(
        &self,
        content: BoardPostEventContent,
        board_id: OwnedRoomId,
        token: Secret,
    ) -> Result<SendResponse> {
        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.post/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token.to_string(), content).await?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reply(
        &self,
        mut content: BoardReplyEventContent,
        in_reply_to: OwnedEventId,
        board_id: OwnedRoomId,
        token: Secret,
    ) -> Result<SendResponse> {
        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.reply/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        // is this our responsibility ?
        content.relates_to = Some(Relation::Reply {
            in_reply_to: InReplyTo::new(in_reply_to),
        });

        let resp = self.put(endpoint, token.to_string(), content).await?;

        let data: SendResponse = resp.json().await.map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn send_reaction(
        &self,
        board_id: OwnedRoomId,
        event_id: OwnedEventId,
        key: VoteKind,
        token: Secret,
    ) -> Result<SendResponse> {
        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.reaction/{txn_id}",
            board_id = board_id,
            txn_id = TransactionId::new(),
        );

        let content = SpaceReactionEventContent::new(Vote { event_id, key });

        let resp = self.put(endpoint, token.to_string(), content).await?;

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
        _token: impl AsRef<str>,
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
