use ruma_common::{
    api::{request, response, Metadata},
    metadata,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/register",
    }
};

#[request(error = crate::Error)]
pub struct Request {}

#[response(error = crate::Error)]
pub struct Response {
    nonce: String,
}
