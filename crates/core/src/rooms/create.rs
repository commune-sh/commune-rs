use matrix::{
    client::create_room::{Request, Response, RoomCreationContent, RoomVisibility},
    ruma_common::{room::RoomType, OwnedRoomOrAliasId, RoomVersionId},
    ruma_events::{
        room::power_levels::RoomPowerLevelsEventContent,
        space::parent::{InitialSpaceParentEvent, SpaceParentEventContent},
    },
};

use crate::{commune, error::Result};

pub async fn service(
    access_token: impl AsRef<str>,
    parent: OwnedRoomOrAliasId,
    alias: Option<String>,
    name: Option<String>,
    topic: Option<String>,
) -> Result<Response> {
    let creation_content = Some(RoomCreationContent {
        kind: None,
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
        None,
        alias,
        topic,
        RoomVisibility::Public,
    );

    let resp = commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await?;

    let mut content =
        SpaceParentEventContent::new(vec![commune().config.matrix.server_name.clone()]);
    content.canonical = true;

    let state_event = InitialSpaceParentEvent {
        content,
        state_key: resp.room_id.clone(),
    };

    Ok(resp)
}
