use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};

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

#[response(error = crate::Error)]
pub struct Response {
    pub available: bool,
}
