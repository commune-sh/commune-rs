use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};
use serde::Serialize;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v3/register/available",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(query)]
    pub username: String,
}

impl Request {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

#[response(error = crate::Error)]
#[derive(Serialize)]
pub struct Response {
    pub available: bool,
}
