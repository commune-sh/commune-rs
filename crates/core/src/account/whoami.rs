use matrix::client::account::whoami::*;

use crate::commune;

pub async fn service(access_token: impl AsRef<str>) -> Result<Response, crate::error::Error> {
    let req = Request::new();

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
