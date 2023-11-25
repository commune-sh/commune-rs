pub mod v1;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Router;
use serde::Serialize;

use crate::services::SharedServices;

pub struct Api;

impl Api {
    pub fn routes() -> Router<SharedServices> {
        Router::new().nest("/v1", v1::V1::routes())
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    #[serde(skip)]
    status: StatusCode,
}

impl ApiError {
    pub fn new(message: String, status: StatusCode) -> Self {
        Self { message, status }
    }
}

/// Any `anyhow::Error` can be converted into an `ApiError`.
///
/// Caveat is that given that anyhow error is generic (w/o context), the
/// error status is 500.
///
/// Perhaps in the future, a more specific error type can be used, like with
/// `thiserror`.
impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self {
            message: err.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status;
        let mut response = Json(self).into_response();

        *response.status_mut() = status;
        response
    }
}
