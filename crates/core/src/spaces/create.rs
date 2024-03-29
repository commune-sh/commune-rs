use matrix::{
    client::create_room::{Request, Response, RoomCreationContent, RoomVisibility},
    ruma_common::{room::RoomType, OwnedRoomAliasId, RoomVersionId},
    ruma_events::room::power_levels::RoomPowerLevelsEventContent,
};

use crate::{commune, error::Result};

pub async fn service(
    access_token: impl AsRef<str>,
    alias: Option<String>,
    name: Option<String>,
    topic: Option<String>,
) -> Result<Response> {
    let mut power_levels = RoomPowerLevelsEventContent::new();
    power_levels.events_default = 100.into();

    let creation_content = Some(RoomCreationContent {
        kind: RoomType::Space,
        federate: true,
        room_version: RoomVersionId::V11,
        predecessor: None,
    });

    let req = Request::new(
        creation_content,
        Vec::new(),
        Vec::new(),
        false,
        name,
        Some(power_levels),
        alias,
        topic,
        RoomVisibility::Public,
    );

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
