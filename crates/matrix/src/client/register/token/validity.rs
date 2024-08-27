use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: true,
    authentication: None,
    history: {
        unstable => "/_matrix/client/v1/register/m.login.registration_token/validity",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(query)]
    pub token: String,
}

impl Request {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

#[response(error = crate::Error)]
pub struct Response {
    pub valid: bool,
}
