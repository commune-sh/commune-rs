use ruma_common::{
    api::{request, response, Metadata},
    metadata, OwnedUserId,
};

use super::User;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/users/:user_id",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub user_id: OwnedUserId,
}

#[response(error = crate::Error)]
pub struct Response {
    #[ruma_api(body)]
    pub user: User,
}
