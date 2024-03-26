use matrix::{client::account::password::*, ruma_common::UserId};

use crate::{commune, error::Result, util::secret::Secret};

pub async fn service(
    access_token: impl AsRef<str>,
    username: impl Into<String>,
    old_password: Secret,
    new_password: Secret,
) -> Result<Response> {
    let server_name = &crate::commune().config.matrix.server_name;
    let user_id = UserId::parse_with_server_name(username.into(), server_name)?;

    let req = Request::new(new_password.inner()).with_password(user_id, old_password.inner());

    commune()
        .send_matrix_request(req, Some(access_token.as_ref()))
        .await
        .map_err(Into::into)
}
