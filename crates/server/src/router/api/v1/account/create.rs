use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use commune::account::model::Account;
use commune::account::service::CreateAccountDto;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<AccountRegisterPayload>,
) -> Response {
    let dto = CreateAccountDto::from(payload);

    match services.commune.account.register(dto).await {
        Ok(account) => {
            let mut response = Json(AccountRegisterResponse::from(account)).into_response();

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
}

impl From<AccountRegisterPayload> for CreateAccountDto {
    fn from(payload: AccountRegisterPayload) -> Self {
        Self {
            username: payload.username,
            password: payload.password.into(),
            email: payload.email,
            // FIXME: These should be queried from somewhere
            session: "test".to_string(),
            code: "test".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountRegisterResponse {
    pub username: String,
}

impl From<Account> for AccountRegisterResponse {
    fn from(acc: Account) -> Self {
        Self {
            username: acc.username,
        }
    }
}
