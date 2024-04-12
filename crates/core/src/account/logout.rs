use matrix::client::logout::root::*;

use crate::{commune, error::Error};

pub async fn service(access_token: impl AsRef<str>) -> Result<Response, Error> {
    let req = Request::new();

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
