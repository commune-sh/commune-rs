use matrix::{
    client::{
        account::whoami,
        profile::avatar_url::{Request, Response},
    },
    ruma_common::OwnedMxcUri,
};

use crate::{commune, error::Error};

pub async fn service(
    access_token: impl AsRef<str>,
    avatar_url: impl Into<OwnedMxcUri>,
) -> Result<Response, Error> {
    let req = whoami::Request::new();

    let whoami::Response { user_id, .. } = commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await?;

    let req = Request::new(user_id, avatar_url.into());

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
