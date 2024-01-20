pub mod v1;

use axum::response::IntoResponse;
use axum::Json;
use axum::Router;
use http::StatusCode;
use serde::Serialize;

use commune::error::HttpStatusCode;

pub struct Api;

impl Api {
    pub fn routes() -> Router {
        Router::new().nest("/api", Router::new().nest("/v1", v1::V1::routes()))
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    message: String,
    code: &'static str,
    #[serde(skip)]
    status: StatusCode,
}

impl ApiError {
    pub fn new(message: String, code: &'static str, status: StatusCode) -> Self {
        Self {
            message,
            code,
            status,
        }
    }

    pub fn unauthorized() -> Self {
        Self::new(
            "You must be authenticated to access this request".to_string(),
            "UNAUTHORIZED",
            StatusCode::UNAUTHORIZED,
        )
    }

    pub fn internal_server_error() -> Self {
        Self::new(
            "Internal server error".to_string(),
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl From<commune::error::Error> for ApiError {
    fn from(err: commune::error::Error) -> Self {
        Self {
            message: err.to_string(),
            code: err.error_code(),
            status: err.status_code(),
        }
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
            code: "UNKNOWN_ERROR",
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status.as_u16();
        let mut response = Json(self).into_response();

        *response.status_mut() =
            axum::http::StatusCode::from_u16(status).expect("Invalid status code");
        response
    }
}
