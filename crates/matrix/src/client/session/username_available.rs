use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/register/available",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    pub username: String,
}

impl Request {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_owned(),
        }
    }
}

#[response(error = crate::Error)]
#[derive(Serialize)]
pub struct Response {
    pub available: bool,
}
