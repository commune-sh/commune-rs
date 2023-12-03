use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::auth::service::LoginCredentials;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<AccountLoginPayload>,
) -> Response {
    let login_credentials = LoginCredentials::from(payload);
    todo!()

    // match services.commune.account.issue_token(login_credentials).await {
    //     Ok(account) => {
    //         let mut response = Json(AccountRegisterResponse::from(account)).into_response();

    //         *response.status_mut() = StatusCode::CREATED;
    //         response
    //     }
    //     Err(err) => {
    //         tracing::warn!(?err, "Failed to register user");
    //         ApiError::from(err).into_response()
    //     }
    // }
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
    pub username: String,
}
