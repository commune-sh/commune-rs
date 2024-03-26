//! This module contains handlers for managing users.
//!
//! reference: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html

use ruma_common::{thirdparty::ThirdPartyIdentifier, OwnedMxcUri, OwnedUserId};
use serde::{Deserialize, Serialize};

pub mod get_user;
pub mod get_user_by_3pid;
pub mod get_users;
pub mod set_user;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "name")]
    pub user_id: OwnedUserId,

    pub displayname: Option<String>,

    pub avatar_url: Option<OwnedMxcUri>,

    pub threepids: Vec<ThirdPartyIdentifier>,

    pub external_ids: Vec<ExternalId>,

    pub admin: bool,

    pub deactivated: bool,

    #[serde(skip_serializing)]
    pub erased: bool,

    #[serde(skip_serializing)]
    pub shadow_banned: bool,

    #[serde(skip_serializing)]
    pub creation_ts: u64,

    #[serde(skip_serializing)]
    pub consent_server_notice_sent: Option<u64>,

    #[serde(skip_serializing)]
    pub consent_ts: Option<u64>,

    pub locked: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExternalId {
    pub auth_provider: String,

    pub external_id: String,
}
