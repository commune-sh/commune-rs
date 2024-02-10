use http::StatusCode;
use thiserror::Error;
use validator::ValidationErrors;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum RoomErrorCode {
    #[error("Failed to parse RoomId")]
    MalformedRoomId,
    #[error("Validation error. {0}")]
    ValidationError(#[from] ValidationErrors),
}

impl HttpStatusCode for RoomErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            RoomErrorCode::MalformedRoomId | RoomErrorCode::ValidationError(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            RoomErrorCode::MalformedRoomId => "BAD_REQUEST",
            RoomErrorCode::ValidationError(_) => "CREATION_DETAIL_INVALID",
        }
    }
}
