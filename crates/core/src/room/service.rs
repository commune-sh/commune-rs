use std::sync::Arc;

use tracing::instrument;

use matrix::{
    client::resources::room::{
        CreateRoomBody, Room as MatrixRoom, RoomCreationContent, RoomPreset,
    },
    Client as MatrixAdminClient,
};
use validator::Validate;

use crate::{util::secret::Secret, Error, Result};

use super::model::Room;

#[derive(Debug, Default, Validate)]
pub struct CreateRoomDto {
    pub name: String,
    pub topic: String,
    pub alias: String,
}
pub struct RoomService {
    admin: Arc<MatrixAdminClient>,
}

impl RoomService {
    pub fn new(admin: Arc<MatrixAdminClient>) -> Self {
        Self { admin }
    }

    /// Creates a Public Chat Room
    #[instrument(skip(self, dto))]
    pub async fn create_public_room(
        &self,
        access_token: &Secret,
        dto: CreateRoomDto,
    ) -> Result<Room> {
        match MatrixRoom::create(
            &self.admin,
            access_token.to_string(),
            CreateRoomBody {
                creation_content: Some(RoomCreationContent { federate: false }),
                preset: Some(RoomPreset::PublicChat),
                name: dto.name,
                room_alias_name: dto.alias,
                topic: dto.topic,
                ..Default::default()
            },
        )
        .await
        {
            Ok(room) => Ok(Room {
                room_id: room.room_id.to_string(),
            }),
            Err(err) => {
                tracing::error!("Failed to create public room: {}", err);
                Err(Error::Unknown)
            }
        }
    }
}
