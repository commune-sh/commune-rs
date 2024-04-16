use matrix::{
    client::membership::join::{Request, Response},
    ruma_common::OwnedRoomId,
};

use crate::{commune, error::Error, util::opaque_id::OpaqueId};

pub async fn service(
    access_token: impl AsRef<str>,
    space_id: impl Into<OpaqueId>,
    reason: Option<String>,
) -> Result<Response, Error> {
    let server_name = &commune().config.matrix.server_name;

    let space_id = space_id.into();
    let room_id = OwnedRoomId::try_from(format!("!{space_id}:{server_name}"))
        .expect("Parsing space identifier should never panic");

    let req = Request::new(room_id.into(), reason);

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
