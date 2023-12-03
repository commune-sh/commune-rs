use http::StatusCode;
use thiserror::Error;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum AuthErrorCode {
    #[error("Provided credentials are not valid")]
    InvalidCredentials,
}

impl HttpStatusCode for AuthErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthErrorCode::InvalidCredentials => StatusCode::BAD_REQUEST,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            AuthErrorCode::InvalidCredentials => "INVALID_CREDENTIALS",
        }
    }
}
