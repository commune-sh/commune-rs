use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/register/new",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    pub token: String,

    pub uses_allowed: usize,

    pub expiry_time: usize,
}

impl Request {
    pub fn new(
        token: String,
        uses_allowed: usize,
        expiry_time: usize,
    ) -> Self {
        Self {
            token,
            uses_allowed,
            expiry_time,
        }
    }
}

// Same fields as above are returned but we only
// care about knowing whether the call was successful.
#[response(error = crate::Error)]
pub struct Response {}
