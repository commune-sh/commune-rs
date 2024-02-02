use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

use crate::router::api::ApiError;
use crate::router::middleware::AccessToken;
use crate::services::SharedServices;

#[derive(Deserialize)]
pub struct RedactEventDto {
    board_id: String,
    event_id: String,
    reason: String,
}

pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Extension(access_token): Extension<AccessToken>,
    Json(payload): Json<RedactEventDto>,
) -> Response {
    match services
        .commune
        .events
        .send_redaction(
            payload.board_id,
            payload.event_id,
            payload.reason,
            access_token.into(),
        )
        .await
    {
        Ok(resp) => {
            let mut response = Json(resp).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => ApiError::from(err).into_response(),
    }
}
