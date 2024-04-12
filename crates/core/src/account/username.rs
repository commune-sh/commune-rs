use matrix::client::register::available::*;

use crate::{commune, error::Error};

pub async fn service(username: impl Into<String>) -> Result<Response, Error> {
    let req = Request::new(username.into());

    commune()
        .send_matrix_request(req, None)
        .await
        .map_err(Into::into)
}
