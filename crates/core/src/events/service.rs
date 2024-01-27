use std::{str::FromStr};

use matrix::events::{
    exports::ruma_common::{OwnedEventId, TransactionId},
    relation::{InReplyTo},
    room::message::Relation,
    space::{BoardPostEventContent, BoardReplyEventContent},
};
use reqwest::{Client as HttpClient, Response as HttpResponse};
use serde::{Deserialize, Serialize};

use tokio_postgres::Client as PostgresClient;

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendResponse {
    pub event_id: String,
}

pub struct EventsService {
    destination: url::Url,
    postgres: PostgresClient,
    http: HttpClient,
}

impl EventsService {
    pub fn new(synapse_host: &str, postgres: PostgresClient) -> Self {
        let destination = url::Url::parse(synapse_host).unwrap();

        Self {
            destination,
            postgres,
            http: HttpClient::new(),
        }
    }

    pub async fn new_post<S: AsRef<str>>(
        &self,
        content: BoardPostEventContent,
        board_id: S,
        token: S,
    ) -> Result<SendResponse> {
        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.post/{txn_id}",
            board_id = board_id.as_ref(),
            txn_id = TransactionId::new(),
        );

        let resp = self.put(endpoint, token, content).await?;

        let body = resp.bytes().await.unwrap();
        tracing::debug!(?body);

        let data: SendResponse = serde_json::from_slice(&body).map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

    pub async fn new_reply<S: AsRef<str>>(
        &self,
        mut content: BoardReplyEventContent,
        in_reply_to: S,
        board_id: S,
        token: S,
    ) -> Result<SendResponse> {
        let endpoint = format!(
            "/_matrix/client/v3/rooms/{board_id}/send/space.board.reply/{txn_id}",
            board_id = board_id.as_ref(),
            txn_id = TransactionId::new(),
        );

        content.relates_to = Some(Relation::Reply {
            in_reply_to: InReplyTo::new(OwnedEventId::from_str(in_reply_to.as_ref()).map_err(
                |err| {
                    tracing::error!(?err, "Failed to parse in_reply_to");
                    Error::Unknown
                },
            )?),
        });

        let resp = self.put(endpoint, token, content).await?;

        let body = resp.bytes().await.unwrap();
        tracing::debug!(?body);

        let data: SendResponse = serde_json::from_slice(&body).map_err(|err| {
            tracing::error!(?err, "Failed to deserialize response");
            Error::Unknown
        })?;

        Ok(data)
    }

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
