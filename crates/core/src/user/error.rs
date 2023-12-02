use http::StatusCode;
use thiserror::Error;
use validator::ValidationErrors;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum UserErrorCode {
    #[error("Vaildation error. {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("The username {0} is already taken")]
    UsernameTaken(String),
}

impl HttpStatusCode for UserErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            UserErrorCode::ValidationError(_) => StatusCode::BAD_REQUEST,
            UserErrorCode::UsernameTaken(_) => StatusCode::CONFLICT,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            UserErrorCode::ValidationError(_) => "VALIDATION_ERROR",
            UserErrorCode::UsernameTaken(_) => "USERNAME_TAKEN",
        }
    }
}
