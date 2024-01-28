use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::admin::resources::user_id::UserId;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomPreset {
    PrivateChat,
    PublicChat,
    TrustedPrivateChat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRoomCreationContent {
    #[serde(rename = "m.federate")]
    pub m_federate: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRoomRequestBody {
    pub creation_content: CreateRoomCreationContent,
    pub name: String,
    pub preset: RoomPreset,
    pub room_alias_name: String,
    pub topic: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRoomResponseBody {
    pub room_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoomApiError {
    pub errcode: String,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    pub device_id: String,
    pub is_guest: bool,
    pub user_id: UserId,
}

impl Room {
    /// Create a new room with various configuration options.
    ///
    /// Refer: https://spec.matrix.org/v1.9/client-server-api/#creation
    #[instrument(skip(client, access_token))]
    pub async fn create(
        client: &crate::http::Client,
        access_token: impl Into<String>,
        creation_content: CreateRoomRequestBody,
    ) -> Result<CreateRoomResponseBody> {
        let mut tmp = (*client).clone();
        tmp.set_token(access_token)?;

        let resp = tmp
            .post_json("/_matrix/client/v3/createRoom", &creation_content)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<RoomApiError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
