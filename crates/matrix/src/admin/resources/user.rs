//! [User Admin API](https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#user-admin-api)
//!
//! To use it, you will need to authenticate by providing an `access_token`
//! for a server admin: see Admin API.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use url::Url;

use crate::{error::MatrixError, http::Client};

use super::user_id::UserId;

#[derive(Default)]
pub struct UserService;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalId {
    pub auth_provider: String,
    pub external_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreePid {
    pub medium: String,
    pub address: String,
    pub added_at: u64,
    pub validated_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User name postfixed with Matrix instance Host
    /// E.g. `@user:example.com`
    pub name: String,
    pub displayname: Option<String>,
    pub threepids: Vec<ThreePid>,
    pub avatar_url: Option<Url>,
    pub is_guest: bool,
    pub admin: bool,
    pub deactivated: bool,
    pub erased: bool,
    pub shadow_banned: bool,
    pub creation_ts: u64,
    pub appservice_id: Option<String>,
    pub consent_server_notice_sent: Option<u64>,
    pub consent_version: Option<String>,
    pub consent_ts: Option<u64>,
    pub external_ids: Vec<ExternalId>,
    pub user_type: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateBody {
    pub password: String,
    pub logout_devices: bool,
    pub displayname: Option<String>,
    pub avatar_url: Option<Url>,
    pub threepids: Vec<ThreePid>,
    pub external_ids: Vec<ExternalId>,
    pub admin: bool,
    pub deactivated: bool,
    pub user_type: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListUsersQuery {
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub guests: Option<bool>,
    pub admins: Option<bool>,
    pub deactivated: Option<bool>,
    pub limit: Option<u64>,
    pub from: Option<u64>,
}

/// Data type for the response of the `GET /_synapse/admin/v2/users` endpoint.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListUser {
    pub name: String,
    pub user_type: Option<String>,
    pub is_guest: usize,
    pub admin: usize,
    pub deactivated: usize,
    pub shadow_banned: bool,
    pub avatar_url: Option<Url>,
    pub creation_ts: u64,
    pub last_seen_ts: Option<u64>,
    pub erased: bool,
    pub locked: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<ListUser>,
    pub total: u64,
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateBody {
    pub password: String,
    pub logout_devices: bool,
    pub displayname: Option<String>,
    pub avatar_url: Option<Url>,
    pub threepids: Vec<ThreePid>,
    pub external_ids: Vec<ExternalId>,
    pub admin: bool,
    pub deactivated: bool,
    pub user_type: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginAsUserBody {
    pub valid_until_ms: Option<u64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LoginAsUserResponse {
    pub access_token: String,
}

pub struct QueryUserDataResponse {
    pub name: String,
    pub displayname: Option<String>,
    pub threepids: Vec<ThreePid>,
    pub avatar_url: Option<Url>,
    pub is_guest: bool,
    pub admin: bool,
    pub deactivated: bool,
    pub erased: bool,
    pub shadow_banned: bool,
    pub creation_ts: i64,
    pub appservice_id: Option<String>,
    pub consent_server_notice_sent: Option<bool>,
    pub consent_version: Option<bool>,
    pub consent_ts: Option<bool>,
    pub external_ids: Vec<Vec<ExternalId>>,
    pub user_type: Option<String>,
}

impl UserService {
    /// This API returns information about a specific user account.
    ///
    /// Refer: https://matrix-org.github.io/synapse/v1.88/admin_api/user_admin_api.html#query-user-account
    #[instrument(skip(client))]
    pub async fn query_user_account(client: &Client, user_id: UserId) -> Result<User> {
        let resp = client
            .get(format!(
                "/_synapse/admin/v2/users/{user_id}",
                user_id = user_id
            ))
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows an administrator to create a user account.
    ///
    /// Note that internally Synapse uses this same endpoint to modify an
    /// existing user account, so this method will modify the existing user
    /// if [`UserId`] matches.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#create-or-modify-account
    #[instrument(skip(client, body))]
    pub async fn create(client: &Client, user_id: UserId, body: UserCreateBody) -> Result<User> {
        let resp = client
            .put_json(
                format!("/_synapse/admin/v2/users/{user_id}", user_id = user_id),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Returns all local user accounts. By default, the response is ordered by
    /// ascending user ID.
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#list-accounts
    #[instrument(skip(client))]
    pub async fn list(client: &Client, query: ListUsersQuery) -> Result<ListUsersResponse> {
        let resp = client
            .get_query("/_synapse/admin/v2/users", &query)
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// Allows an administrator to modify a user account
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#create-or-modify-account
    #[instrument(skip(client))]
    pub async fn update(client: &Client, user_id: UserId, body: UserUpdateBody) -> Result<User> {
        let resp = client
            .put_json(
                format!("/_synapse/admin/v2/users/{user_id}", user_id = user_id),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }

    /// **Note: This API is disabled when MSC3861 is enabled. [See #15582][1]**
    ///
    /// Get an access token that can be used to authenticate as that user.
    /// Useful for when admins wish to do actions on behalf of a user.
    ///
    /// An optional `valid_until_ms` field can be specified in the request body
    /// as an integer timestamp that specifies when the token should expire.
    ///
    /// **By default tokens do not expire. Note that this API does not allow a
    /// user to login as themselves (to create more tokens).**
    ///
    /// Refer: https://matrix-org.github.io/synapse/latest/admin_api/user_admin_api.html#login-as-a-user
    ///
    /// [1]: https://github.com/matrix-org/synapse/pull/15582
    #[instrument(skip(client))]
    pub async fn login_as_user(
        client: &Client,
        user_id: UserId,
        body: LoginAsUserBody,
    ) -> Result<LoginAsUserResponse> {
        let resp = client
            .post_json(
                format!(
                    "/_synapse/admin/v1/users/{user_id}/login",
                    user_id = user_id
                ),
                &body,
            )
            .await?;

        if resp.status().is_success() {
            return Ok(resp.json().await?);
        }

        let error = resp.json::<MatrixError>().await?;

        Err(anyhow::anyhow!(error.error))
    }
}
