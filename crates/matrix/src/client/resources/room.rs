use anyhow::Result;
use ruma_common::{serde::Raw, OwnedRoomId, OwnedUserId, RoomId, RoomOrAliasId};
use ruma_events::{room::power_levels::RoomPowerLevelsEventContent, AnyInitialStateEvent};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::error::MatrixError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomPreset {
    PrivateChat,
    PublicChat,
    TrustedPrivateChat,
}

#[derive(Default, Debug, Serialize)]
pub struct RoomCreationContent {
    #[serde(rename = "m.federate")]
    pub federate: bool,
}

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomVisibility {
    Public,
    #[default]
    Private,
}

#[derive(Default, Debug, Serialize)]
pub struct CreateRoomBody {
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub initial_state: Vec<Raw<AnyInitialStateEvent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_content: Option<RoomCreationContent>,

    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub invite: Vec<OwnedUserId>,

    pub is_direct: bool,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_override: Option<RoomPowerLevelsEventContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<RoomPreset>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub room_alias_name: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub topic: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<RoomVisibility>,
}

#[derive(Default, Debug, Serialize)]
pub struct JoinRoomBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[derive(Default, Debug, Serialize)]
pub struct ForgetRoomBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[derive(Default, Debug, Serialize)]
pub struct LeaveRoomBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct RoomKickOrBanBody {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,

    pub user_id: OwnedUserId,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: OwnedRoomId,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomResponse {
    pub room_id: OwnedRoomId,
}

#[derive(Debug, Deserialize)]
pub struct LeaveRoomResponse {}

#[derive(Debug, Deserialize)]
pub struct ForgetRoomResponse {}

#[derive(Debug, Deserialize)]
pub struct RoomKickOrBanResponse {}

pub struct RoomService;

impl RoomService {
    /// Create a new room with various configuration options.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#creation
    #[instrument(skip(client, access_token))]
    pub async fn create(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        body: CreateRoomBody,
    ) -> Result<CreateRoomResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json("/_matrix/client/v3/createRoom", &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Join a particular room, if we are allowed to participate.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#joining-rooms
    #[instrument(skip(client, access_token))]
    pub async fn join(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        alias_or_id: &RoomOrAliasId,
        body: JoinRoomBody,
    ) -> Result<JoinRoomResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/join/{alias_or_id}"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Leave a particular room.
    /// They are still allowed to retrieve the history which they were
    /// previously allowed to see.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn leave(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: LeaveRoomBody,
    ) -> Result<LeaveRoomResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/rooms/{room_id}/leave"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Forget a particular room.
    /// This will prevent the user from accessing the history of the room.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn forget(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: ForgetRoomBody,
    ) -> Result<ForgetRoomResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/rooms/{room_id}/forget"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Kick a user from a particular room.
    /// The caller must have the required power level in order to perform this
    /// operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn kick(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<RoomKickOrBanResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/rooms/{room_id}/kick"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Ban a user from a particular room.
    /// This will kick them too if they are still a member.
    /// The caller must have the required power level in order to perform this
    /// operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn ban(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<RoomKickOrBanResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/rooms/{room_id}/ban"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Unban a user from a particular room.
    /// This will allow them to re-join or be re-invited.
    /// The caller must have the required power level in order to perform this
    /// operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#banning-users-in-a-room
    #[instrument(skip(client, access_token))]
    pub async fn unban(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<RoomKickOrBanResponse> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(format!("/_matrix/client/v3/rooms/{room_id}/unban"), &body)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
