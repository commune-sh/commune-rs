use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use commune::Error;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::auth::service::LoginCredentials;

use crate::{router::api::ApiError, services::SharedServices};

use super::root::{AccountMatrixCredentials, AccountSpace};

#[instrument(skip(services))]
pub async fn get(Extension(services): Extension<SharedServices>) -> Response {
    match services.commune.auth.get_login_flows().await {
        Ok(flows) => Json(flows).into_response(),
        Err(err) => {
            tracing::warn!(?err, "Failed to retrieve login flows");
            ApiError::from(err).into_response()
        }
    }
}

#[instrument(skip(services, payload))]
pub async fn post(
    Extension(services): Extension<SharedServices>,
    Json(payload): Json<AccountLoginPayload>,
) -> Response {
    let login_credentials = LoginCredentials::from(payload);

    let Ok(tokens) = services.commune.auth.login(login_credentials).await else {
        tracing::warn!("Failed to authenticate user");
        return ApiError::from(Error::Auth(
            commune::auth::error::AuthErrorCode::InvalidCredentials,
        ))
        .into_response();
    };

    match services.commune.account.whoami(&tokens.access_token).await {
        Ok(account) => {
            let mut response = Json(AccountLoginResponse {
                access_token: tokens.access_token.to_string(),
                credentials: AccountMatrixCredentials {
                    username: account.username,
                    display_name: account.display_name,
                    avatar_url: account.avatar_url,
                    access_token: tokens.access_token.to_string(),
                    matrix_access_token: tokens.access_token.to_string(),
                    matrix_user_id: account.user_id.to_string(),
                    matrix_device_id: String::new(),
                    user_space_id: String::new(),
                    email: account.email,
                    age: account.age,
                    admin: account.admin,
                    verified: account.verified,
                },
                ..Default::default()
            })
            .into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => {
            tracing::warn!(?err, "Failed to authenticate user");
            ApiError::from(err).into_response()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountLoginPayload {
    pub username: String,
    pub password: String,
}

impl From<AccountLoginPayload> for LoginCredentials {
    fn from(payload: AccountLoginPayload) -> Self {
        Self {
            username: payload.username,
            password: payload.password.into(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLoginResponse {
    pub access_token: String,
    pub credentials: AccountMatrixCredentials,
    pub rooms: Vec<String>,
    pub spaces: Vec<AccountSpace>,
}
