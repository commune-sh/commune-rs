use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::auth::service::{LoginCredentials, LoginCredentialsResponse};

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<AccountLoginPayload>,
) -> Response {
    let login_credentials = LoginCredentials::from(payload);

    match services.commune.auth.login(login_credentials).await {
        Ok(tokens) => {
            let mut response = Json(AccountLoginResponse::from(tokens)).into_response();

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

#[derive(Deserialize, Serialize)]
pub struct AccountLoginResponse {
    pub access_token: String,
}

impl From<LoginCredentialsResponse> for AccountLoginResponse {
    fn from(tokens: LoginCredentialsResponse) -> Self {
        Self {
            access_token: tokens.access_token.to_string(),
        }
    }
}
