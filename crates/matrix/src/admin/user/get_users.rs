use ruma_common::{
    api::{request, response, Metadata, Direction},
    thirdparty::ThirdPartyIdentifier,
    OwnedMxcUri, OwnedUserId, metadata,
};
use ruma_macros::{request, response};

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
    pub name: OwnedUserId,

    pub displayname: Option<String>,

    pub threepids: Vec<ThirdPartyIdentifier>,

    pub avatar_url: Option<OwnedMxcUri>,

    pub admin: bool,

    pub deactivated: bool,

    pub erased: bool,

    pub shadow_banned: bool,

    pub creation_ts: u64,

    pub locked: bool,
}

#[derive(Clone, Debug, Default)]
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
