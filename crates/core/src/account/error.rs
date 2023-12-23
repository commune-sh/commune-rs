use http::StatusCode;
use thiserror::Error;
use validator::ValidationErrors;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum AccountErrorCode {
    #[error("Invalid verification code")]
    InvalidVerificationCode,
    #[error("Vaildation error. {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("The username {0} is already taken")]
    UsernameTaken(String),
}

impl HttpStatusCode for AccountErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            AccountErrorCode::InvalidVerificationCode => StatusCode::UNAUTHORIZED,
            AccountErrorCode::ValidationError(_) => StatusCode::BAD_REQUEST,
            AccountErrorCode::UsernameTaken(_) => StatusCode::CONFLICT,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            AccountErrorCode::InvalidVerificationCode => "INVALID_VERIFICATION_CODE",
            AccountErrorCode::ValidationError(_) => "VALIDATION_ERROR",
            AccountErrorCode::UsernameTaken(_) => "USERNAME_TAKEN",
        }
    }
}
