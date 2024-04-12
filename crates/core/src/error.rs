use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("forwarding Matrix request failed: {0}")]
    Matrix(#[from] matrix::ClientError),

    #[error("forwarding Matrix request requires UIA: {0}")]
    Uiaa(#[from] matrix::UiaaError),

    #[error("(de)serializing type failed: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("email host has been banned")]
    EmailDomain,

    #[error("failed to validate identifier: {0}")]
    InvalidIdentifier(#[from] matrix::ruma_identifiers_validation::Error),

    #[error("an IO operation failed: {0}")]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SMTP(#[from] mail_send::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
