use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/v3/logout",
    }
};

#[request(error = crate::Error)]
pub struct Request {}

#[allow(clippy::new_without_default)]
impl Request {
    pub fn new() -> Self {
        Self {}
    }
}

#[response(error = crate::Error)]
#[derive(Serialize)]
pub struct Response {}
