use anyhow::Result;
use ruma_common::{serde::Raw, OwnedUserId, RoomOrAliasId, RoomId};
use ruma_events::{AnyInitialStateEvent, room::power_levels::RoomPowerLevelsEventContent};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomPreset {
    PrivateChat,
    PublicChat,
    TrustedPrivateChat,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RoomCreationContent {
    #[serde(rename = "m.federate")]
    pub federate: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RoomVisibility {
    Public,
    #[default]
    Private,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateRoomBody {
    pub initial_state: Vec<Raw<AnyInitialStateEvent>>,
    pub creation_content: Option<RoomCreationContent>,
    pub invite: Vec<OwnedUserId>,
    pub is_direct: bool,
    pub name: Option<String>,
    pub power_override: Option<RoomPowerLevelsEventContent>,
    pub preset: Option<RoomPreset>,
    pub room_alias_name: Option<String>,
    pub topic: Option<String>,
    pub visibility: Option<RoomVisibility>,
}

#[derive(Clone, Debug, Serialize)]
pub struct JoinRoomBody {
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ForgetRoomBody {
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LeaveRoomBody {
    pub reason: Option<String>,
    pub user_id: OwnedUserId,
}

#[derive(Clone, Debug, Serialize)]
pub struct RoomKickOrBanBody {
    pub reason: Option<String>,
    pub user_id: OwnedUserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateRoomResponse {
    pub room_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinRoomResponse {
    pub room_id: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RoomApiError {
    pub errcode: String,
    pub error: String,
}

pub struct Room;

impl Room {
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

        let error = resp.json::<RoomApiError>().await?;

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
            .post_json(
                format!("/_matrix/client/v3/join/{alias_or_id}"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

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
    ) -> Result<()> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/rooms/{room_id}/forget"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(());
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Leave a particular room.
    /// They are still allowed to retrieve the history which they were previously allowed to see.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn leave(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: LeaveRoomBody,
    ) -> Result<()> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/rooms/{room_id}/leave"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Kick a user from a particular room.
    /// The caller must have the required power level in order to perform this operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn kick(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<()> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/rooms/{room_id}/kick"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Ban a user from a particular room.
    /// This will kick them too if they are still a member.
    /// The caller must have the required power level in order to perform this operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#leaving-rooms
    #[instrument(skip(client, access_token))]
    pub async fn ban(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<()> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/rooms/{room_id}/ban"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Unban a user from a particular room.
    /// This will allow them to re-join or be re-invited.
    /// The caller must have the required power level in order to perform this operation.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#banning-users-in-a-room
    #[instrument(skip(client, access_token))]
    pub async fn unban(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        room_id: &RoomId,
        body: RoomKickOrBanBody,
    ) -> Result<()> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json(
                format!("/_matrix/client/v3/rooms/{room_id}/unban"),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
