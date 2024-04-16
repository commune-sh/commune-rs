pub mod avatar_url;
pub mod displayname;

use matrix::{
    client::profile::{Request, Response},
    ruma_common::OwnedUserId,
};

use crate::{commune, error::Error};

pub async fn service(user_id: impl Into<OwnedUserId>) -> Result<Response, Error> {
    let req = Request::new(user_id.into());

    commune()
        .send_matrix_request(req, None)
        .await
        .map_err(Into::into)
}
