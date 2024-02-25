use ruma_common::{
    api::{request, response, Metadata},
    metadata, thirdparty::Medium,
};

use super::User;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v1/threepid/:medium/users/:address",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub medium: Medium,

    #[ruma_api(path)]
    pub address: String,
}

#[response(error = crate::Error)]
pub struct Response {
    #[ruma_api(body)]
    pub user: User,
}
