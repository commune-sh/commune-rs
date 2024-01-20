use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::account::model::Account;

use crate::router::middleware::AccessToken;

use super::root::{AccountMatrixCredentials, AccountSpace};

#[instrument(skip(account))]
pub async fn handler(
    Extension(account): Extension<Account>,
    Extension(access_token): Extension<AccessToken>,
) -> Response {
    let response = Json(AccountSessionResponse {
        credentials: AccountMatrixCredentials {
            username: account.username,
            display_name: account.display_name,
            avatar_url: account.avatar_url,
            access_token: access_token.to_string(),
            matrix_access_token: access_token.to_string(),
            matrix_user_id: account.user_id.to_string(),
            matrix_device_id: String::new(),
            user_space_id: String::new(),
            email: account.email,
            age: account.age,
            admin: account.admin,
            verified: account.verified,
        },
        rooms: vec![],
        spaces: vec![],
        valid: true,
    });

    response.into_response()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSessionResponse {
    pub credentials: AccountMatrixCredentials,
    pub rooms: Vec<String>,
    pub spaces: Vec<AccountSpace>,
    pub valid: bool,
}
