use matrix::client::session::username_available::*;

use crate::error::Result;

pub async fn service(handle: &matrix::Handle, username: &str) -> Result<Response> {
    let req = Request::new(username);

    let resp: Response = handle.dispatch(None, req).await?;

    Ok(resp)
}
