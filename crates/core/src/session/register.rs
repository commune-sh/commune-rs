use matrix::client::session::register::*;

use crate::{error::Result, util::secret::Secret};

pub async fn service(
    handle: &matrix::Handle,
    username: &str,
    password: &Secret,
) -> Result<Response> {
    let req = Request::new(username, password.inner(), "commune");

    let resp: Response = handle.dispatch(None, req).await?;

    Ok(resp)
}

// pub async fn verify_email(
//     handle: matrix::Handle,
//     username: &str,
//     password: Secret,
//     email: &str,
// ) -> Result<AccessToken> {
//     use session::register::*;

//     let verification_code: String = rand::thread_rng()
//         .gen::<[u32; 6]>()
//         .iter()
//         .flat_map(|i| char::from_digit(i % 10, 10))
//         .collect();

//     let req = Request::new(username, password.inner(), "commune beta");

//     let resp: Response = handle.dispatch(None, req).await?;

//     Ok(AccessToken::new(resp.access_token))
// }
