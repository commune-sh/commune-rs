use http::StatusCode;
use thiserror::Error;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum AuthErrorCode {
    #[error("Provided credentials are not valid")]
    InvalidCredentials,
    #[error("Redis connection failed")]
    RedisConnectionError(#[from] redis::RedisError),
    #[error("Verification Code Marshall/Unmarshall failed")]
    VerificationCodeMarshallError(#[from] serde_json::Error),
}

impl HttpStatusCode for AuthErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthErrorCode::InvalidCredentials => StatusCode::BAD_REQUEST,
            AuthErrorCode::RedisConnectionError(_)
            | AuthErrorCode::VerificationCodeMarshallError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            AuthErrorCode::InvalidCredentials => "INVALID_CREDENTIALS",
            AuthErrorCode::RedisConnectionError(_) => "REDIS_CONNECTION_ERROR",
            AuthErrorCode::VerificationCodeMarshallError(_) => "VERIFICATION_CODE_MARSHALL_ERROR",
        }
    }
}
