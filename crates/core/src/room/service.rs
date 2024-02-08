use std::sync::Arc;

use tracing::instrument;
use validator::Validate;

use matrix::client::resources::room::{
    Room as MatrixRoom, RoomPreset, RoomCreationContent, CreateRoomBody,
};
use matrix::Client as MatrixAdminClient;

use crate::util::secret::Secret;
use crate::{Error, Result};

use super::error::RoomErrorCode;
use super::model::Room;

#[derive(Debug, Clone, Validate)]
pub struct CreateRoomDto {
    #[validate(length(min = 3, max = 255))]
    pub name: String,
    #[validate(length(min = 3, max = 255))]
    pub topic: String,
    #[validate(length(min = 3, max = 255))]
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
        dto.validate()
            .map_err(|err| Error::Room(RoomErrorCode::from(err)))?;

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
                room_id: room.room_id,
            }),
            Err(err) => {
                tracing::error!("Failed to create room: {}", err);
                Err(Error::Unknown)
            }
        }
    }
}
