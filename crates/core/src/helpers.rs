use matrix::{
    client::directory::room::Request,
    ruma_common::{OwnedRoomId, OwnedRoomOrAliasId},
};

use crate::{commune, error::Error};

pub async fn get_room_id(
    room_or_alias_id: impl Into<OwnedRoomOrAliasId>,
) -> Result<OwnedRoomId, Error> {
    // this cannot error, `Result<T>` is just provided in place of an enum
    // https://github.com/ruma/ruma/issues/1761

    match room_or_alias_id.into().try_into() {
        Ok(room_id) => Ok(room_id),
        Err(room_alias) => commune()
            .send_matrix_request(Request::new(room_alias), None)
            .await
            .map(|resp| resp.room_id)
            .map_err(Into::into),
    }
}
