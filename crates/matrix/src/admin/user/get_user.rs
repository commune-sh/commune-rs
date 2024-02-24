use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    thirdparty::ThirdPartyIdentifier,
    OwnedMxcUri, OwnedUserId,
};

#[allow(dead_code)]
const METADATA: Metadata = metadata! {
    method: GET,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_synapse/admin/v2/users/:name",
    }
};

#[request(error = crate::Error)]
pub struct Request {
    #[ruma_api(path)]
    pub name: OwnedUserId,
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

    pub consent_server_notice_sent: Option<u64>,

    pub consent_ts: Option<u64>,

    pub external_ids: Vec<ExternalId>,

    pub locked: bool,
}

#[derive(Clone, Debug)]
pub struct ExternalId {
    pub auth_provider: String,

    pub external_id: String,
}
