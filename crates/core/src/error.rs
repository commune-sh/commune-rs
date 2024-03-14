use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("forwarding a Matrix request failed: {0}")]
    Matrix(#[from] matrix::HandleError),

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
