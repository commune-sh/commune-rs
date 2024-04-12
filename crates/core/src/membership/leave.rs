use matrix::{
    client::{
        self,
        membership::leave::{Request, Response},
    },
    ruma_common::OwnedRoomOrAliasId,
};

use crate::{commune, error::Error};

pub async fn service(
    access_token: impl AsRef<str>,
    room_or_alias_id: impl Into<OwnedRoomOrAliasId>,
    reason: Option<String>,
) -> Result<Response, Error> {
    let room_or_alias_id: OwnedRoomOrAliasId = room_or_alias_id.into();

    // this cannot error, `Result<T>` is just provided in place of an enum
    // https://github.com/ruma/ruma/issues/1761
    let room_id = match room_or_alias_id.try_into() {
        Ok(room_id) => room_id,
        Err(room_alias) => {
            let req = client::directory::room::Request::new(room_alias);

            commune()
                .send_matrix_request(req, None)
                .await
                .map(|resp| resp.room_id)?
        }
    };

    let req = Request::new(room_id, reason);

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
