use ruma_common::{
    api::{request, response, Direction, Metadata},
    metadata, OwnedUserId,
};
use serde::Serialize;

use super::User;

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/users",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub user_id: Option<OwnedUserId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub admins: Option<bool>,

    #[serde(skip_serializing_if = "ruma_common::serde::is_default")]
    #[ruma_api(query)]
    pub deactivated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[ruma_api(query)]
    pub limit: Option<u64>,

    #[serde(skip_serializing_if = "ruma_common::serde::is_default")]
    #[ruma_api(query)]
    pub from: u64,

    #[serde(skip_serializing_if = "ruma_common::serde::is_default")]
    #[ruma_api(query)]
    pub order_by: OrderBy,

    #[serde(skip_serializing_if = "ruma_common::serde::is_default")]
    #[ruma_api(query)]
    pub dir: Direction,
}

#[response(error = crate::Error)]
pub struct Response {
    users: Vec<User>,

    next_token: String,

    total: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
#[allow(dead_code)]
pub enum OrderBy {
    #[default]
    Name,

    Admin,

    UserType,

    Deactivated,

    ShadowBanned,

    Displayname,

    AvatarUrl,

    CreationTs,

    LastSeenTs,
}
