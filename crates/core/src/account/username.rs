use matrix::client::register::available::*;

use crate::{commune, error::Result};

pub async fn service(username: impl Into<String>) -> Result<Response> {
    let req = Request::new(username.into());

    commune()
        .send_matrix_request(req, None)
        .await
        .map_err(Into::into)
}
