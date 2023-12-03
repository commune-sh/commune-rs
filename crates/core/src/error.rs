use http::StatusCode;
use thiserror::Error;

use crate::account::error::AccountErrorCode;

pub type Result<T> = std::result::Result<T, Error>;

pub trait HttpStatusCode {
    fn status_code(&self) -> StatusCode;
    fn error_code(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("User Error. {0}")]
    User(AccountErrorCode),
    #[error("Unknown Error Occured")]
    Unknown,
}

impl From<AccountErrorCode> for Error {
    fn from(err: AccountErrorCode) -> Self {
        Error::User(err)
    }
}

impl HttpStatusCode for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::User(err) => err.status_code(),
            Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Error::User(err) => err.error_code(),
            Error::Unknown => "UNKNOWN_ERROR",
        }
    }
}
