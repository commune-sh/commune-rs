use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::router::api::ApiError;
use crate::services::SharedServices;

#[instrument(skip(services))]
pub async fn handler(
    Extension(services): Extension<SharedServices>,
    Path(email): Path<String>,
) -> Response {
    match services.commune.account.is_email_available(&email).await {
        Ok(available) => {
            let mut response = Json(AccountEmailExistsResponse { available }).into_response();

            *response.status_mut() = StatusCode::OK;
            response
        }
        Err(err) => {
            tracing::warn!(?err, ?email, "Failed to find email");
            ApiError::from(err).into_response()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccountEmailExistsResponse {
    pub available: bool,
}
