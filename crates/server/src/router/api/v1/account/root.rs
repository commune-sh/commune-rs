use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::account::model::Account;
use commune::account::service::CreateAccountDto;
use url::Url;
use uuid::Uuid;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Json(payload): Json<AccountRegisterPayload>,
) -> Response {
    let dto = CreateAccountDto::from(payload);

    match services.commune.account.register(dto).await {
        Ok(account) => {
            let access_token = services
                .commune
                .account
                .issue_user_token(account.user_id.clone())
                .await
                .unwrap();
            let payload = AccountRegisterResponse {
                access_token: access_token.to_string(),
                created: true,
                credentials: AccountMatrixCredentials {
                    username: account.username,
                    display_name: account.display_name,
                    avatar_url: account.avatar_url,
                    access_token: access_token.to_string(),
                    matrix_access_token: access_token.to_string(),
                    matrix_user_id: account.user_id.to_string(),
                    matrix_device_id: "".to_string(),
                    user_space_id: "".to_string(),
                    email: account.email,
                    age: account.age,
                    admin: account.admin,
                    verified: account.verified,
                },
                ..Default::default()
            };

            let mut response = Json(payload).into_response();

            *response.status_mut() = StatusCode::CREATED;
            response
        }
        Err(err) => {
            tracing::warn!(?err, "Failed to register user");
            ApiError::from(err).into_response()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountRegisterPayload {
    pub username: String,
    pub password: String,
    pub email: String,
    pub session: Uuid,
    pub code: String,
}

impl From<AccountRegisterPayload> for CreateAccountDto {
    fn from(payload: AccountRegisterPayload) -> Self {
        Self {
            username: payload.username,
            password: payload.password.into(),
            email: payload.email,
            session: payload.session,
            code: payload.code.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSpace {
    pub room_id: String,
    pub alias: String,
    pub name: String,
    pub topic: Option<String>,
    pub avatar: Option<String>,
    pub header: Option<String>,
    pub is_profile: bool,
    pub is_default: bool,
    pub is_owner: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountMatrixCredentials {
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<Url>,
    pub access_token: String,
    pub matrix_access_token: String,
    pub matrix_user_id: String,
    pub matrix_device_id: String,
    pub user_space_id: String,
    pub email: String,
    pub age: i64,
    pub admin: bool,
    pub verified: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountRegisterResponse {
    pub access_token: String,
    pub created: bool,
    pub credentials: AccountMatrixCredentials,
    pub rooms: Vec<String>,
    pub spaces: Vec<AccountSpace>,
}

impl From<Account> for AccountRegisterResponse {
    fn from(_: Account) -> Self {
        Self {
            ..Default::default()
        }
    }
}
