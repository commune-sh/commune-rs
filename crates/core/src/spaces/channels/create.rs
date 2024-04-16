use matrix::{
    client::{
        create_room::{Request, Response, RoomCreationContent, RoomVisibility},
        rooms,
    },
    ruma_common::{OwnedRoomId, RoomVersionId},
    ruma_events::{
        space::{child::SpaceChildEventContent, parent::SpaceParentEventContent},
        EventContent,
    },
};

use crate::{commune, error::Error, util::opaque_id::OpaqueId};

pub async fn service(
    access_token: impl AsRef<str>,
    space_id: OpaqueId,
    name: Option<String>,
    topic: Option<String>,
) -> Result<Response, Error> {
    let server_name = &commune().config.matrix.server_name;
    let space_id = OwnedRoomId::try_from(format!("!{space_id}:{server_name}"))
        .expect("Parsing space identifier should never panic");

    let req = Request::new(
        Some(RoomCreationContent {
            kind: None,
            federate: true,
            room_version: RoomVersionId::V11,
            predecessor: None,
        }),
        Vec::new(),
        Vec::new(),
        false,
        name,
        None,
        None,
        topic,
        RoomVisibility::Public,
    );

    let resp = commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await?;

    let mut parent_content = SpaceParentEventContent::new(vec![server_name.to_owned()]);
    parent_content.canonical = true;

    let req = rooms::state::create::Request::new(
        parent_content.event_type(),
        resp.room_id.clone(),
        Some(resp.room_id.to_string()),
        parent_content,
    )?;

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await?;

    let mut child_content = SpaceChildEventContent::new(vec![server_name.to_owned()]);
    child_content.suggested = true;

    let req = rooms::state::create::Request::new(
        child_content.event_type(),
        space_id.clone(),
        Some(space_id.to_string()),
        child_content,
    )?;

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await?;

    Ok(resp)
}
