use anyhow::Result;
use ruma_common::{OwnedRoomId, OwnedUserId};
use ruma_events::TimelineEventType;
use serde::{Deserialize, Serialize};

use crate::{error::MatrixError, Client};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum EventFormat {
    #[default]
    Client,
    Federation,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "account_data")]
    account: Option<EventFilter>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    event_fields: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    event_format: Option<EventFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    presence: Option<EventFilter>,

    #[serde(rename = "room")]
    room: Option<RoomFilter>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_senders: Vec<OwnedUserId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_types: Vec<TimelineEventType>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub senders: Vec<OwnedUserId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub types: Vec<TimelineEventType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoomFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_data: Option<RoomEventFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<RoomEventFilter>,

    pub include_leave: bool,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StateFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline: Option<RoomEventFilter>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RoomEventFilter {
    #[serde(flatten)]
    pub inner: EventFilter,

    #[serde(skip_serializing_if = "Option::is_none", rename = "contains_url")]
    pub include_urls: Option<bool>,

    pub include_redundant_members: bool,

    pub lazy_load_members: bool,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub rooms: Vec<OwnedRoomId>,

    pub unread_thread_notifications: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StateFilter {
    #[serde(flatten)]
    pub inner: EventFilter,

    #[serde(skip_serializing_if = "Option::is_none", rename = "contains_url")]
    pub include_urls: Option<bool>,

    pub include_redundant_members: bool,

    pub lazy_load_members: bool,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub not_rooms: Vec<OwnedRoomId>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub rooms: Vec<OwnedRoomId>,

    pub unread_thread_notifications: bool,
}

pub struct FilterService;

impl FilterService {
    pub async fn create(
        client: &Client,
        access_token: impl Into<String>,
        body: Filter,
    ) -> Result<Filter> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/user/@admin:matrix.localhost/filter",),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    pub async fn get(
        client: &Client,
        access_token: impl Into<String>,
        filter_id: String,
    ) -> Result<String> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .get(format!(
                "/_matrix/client/v3/user/@admin:matrix.localhost/filter/{filter_id}"
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
