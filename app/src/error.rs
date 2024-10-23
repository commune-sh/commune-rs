use axum::{extract::rejection::PathRejection, response::IntoResponse};
use thiserror::Error;

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[allow(clippy::error_impl_error)]
#[non_exhaustive]
pub(crate) enum Error {
    #[error("{0}")]
    Path(#[from] PathRejection),

    #[error("")]
    Todo(()),
}

/// Handlers are required to have `Infallible` as their error type,
/// this implementation allow the compiler to cast `Error` into `Response` automatically.
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
