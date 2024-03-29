use matrix::{
    client::create_room::{Request, Response, RoomCreationContent, RoomVisibility},
    ruma_common::{OwnedUserId, RoomVersionId},
};

use crate::{commune, error::Result};

pub async fn service(
    access_token: impl AsRef<str>,
    name: Option<String>,
    topic: Option<String>,
    invite: Vec<OwnedUserId>,
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
        invite,
        true,
        name,
        None,
        None,
        topic,
        RoomVisibility::Private,
    );

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
