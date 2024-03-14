use matrix::client::session::login::*;

use crate::{error::Result, util::secret::Secret};

pub async fn service(
    handle: &matrix::Handle,
    username: &str,
    password: &Secret,
) -> Result<Response> {
    let req = Request::new(
        LoginType::Password {
            password: password.to_string(),
        },
        Some(Identifier::User {
            user: username.to_owned(),
        }),
        "commune".to_owned(),
        Some(true),
    );

    let resp: Response = handle.dispatch(None, req).await?;

    Ok(resp)
}
