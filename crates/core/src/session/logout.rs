
use matrix::client::session::logout::*;

use crate::error::Result;

pub async fn service(handle: &matrix::Handle) -> Result<Response> {
    let req = Request::new();

    let resp: Response = handle.dispatch(None, req).await?;

    Ok(resp)
}
