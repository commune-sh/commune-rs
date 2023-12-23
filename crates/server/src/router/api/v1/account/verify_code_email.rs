use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use commune::util::secret::Secret;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use commune::account::service::VerifyCodeDto;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<AccountVerifyCodeEmailPayload>,
) -> Response {
    let dto = VerifyCodeDto::from(payload);

    match services.commune.account.verify_code(dto).await {
        Ok(valid) => {
            let mut response = Json(VerifyCodeResponse { valid }).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => {
            tracing::warn!(?err, "Failed to register user");
            ApiError::from(err).into_response()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountVerifyCodeEmailPayload {
    pub email: String,
    pub session: Uuid,
    pub code: Secret,
}

impl From<AccountVerifyCodeEmailPayload> for VerifyCodeDto {
    fn from(payload: AccountVerifyCodeEmailPayload) -> Self {
        Self {
            email: payload.email,
            session: payload.session,
            code: payload.code,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeResponse {
    pub valid: bool,
}
