use matrix::client::register::token::validity::*;

use crate::{commune, error::Result};

pub async fn service(access_token: impl AsRef<str>) -> Result<Response> {
    let req = Request::new(access_token.as_ref().to_owned());

    commune()
        .send_matrix_request(req, None)
        .await
        .map_err(Into::into)
}
