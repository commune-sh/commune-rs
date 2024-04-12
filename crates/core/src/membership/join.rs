use matrix::{
    client::membership::join::{Request, Response},
    ruma_common::OwnedRoomOrAliasId,
};

use crate::{commune, error::Error};

pub async fn service(
    access_token: impl AsRef<str>,
    room_id: impl Into<OwnedRoomOrAliasId>,
    reason: Option<String>,
) -> Result<Response, Error> {
    let req = Request::new(room_id.into(), reason);

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
