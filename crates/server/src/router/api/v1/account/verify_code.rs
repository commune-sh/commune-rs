use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use commune::account::service::SendCodeDto;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services, payload))]
pub async fn handler(
    State(services): State<SharedServices>,
    Json(payload): Json<AccountVerifyCodePayload>,
) -> Response {
    let dto = SendCodeDto::from(payload);

    match services.commune.account.send_code(dto).await {
        Ok(_) => {
            let mut response = Json(VerifyCodeResponse { sent: true }).into_response();

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
pub struct AccountVerifyCodePayload {
    pub email: String,
    pub session: Uuid,
}

impl From<AccountVerifyCodePayload> for SendCodeDto {
    fn from(payload: AccountVerifyCodePayload) -> Self {
        Self {
            email: payload.email,
            session: payload.session,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeResponse {
    pub sent: bool,
}
