use http::StatusCode;
use thiserror::Error;

use crate::account::error::AccountErrorCode;
use crate::auth::error::AuthErrorCode;
use crate::mail::error::MailErrorCode;

pub type Result<T> = std::result::Result<T, Error>;

pub trait HttpStatusCode {
    fn status_code(&self) -> StatusCode;
    fn error_code(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Auth Error. {0}")]
    Auth(#[from] AuthErrorCode),
    #[error("User Error. {0}")]
    User(AccountErrorCode),
    #[error("Mail Error. {0}")]
    Mail(#[from] MailErrorCode),
    #[error("An error occured while starting up. {0}")]
    Startup(String),
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
            Error::Auth(err) => err.status_code(),
            Error::User(err) => err.status_code(),
            Error::Mail(err) => err.status_code(),
            Error::Startup(_) | Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Error::Auth(err) => err.error_code(),
            Error::User(err) => err.error_code(),
            Error::Mail(err) => err.error_code(),
            Error::Startup(_) => "SERVER_STARTUP_ERROR",
            Error::Unknown => "UNKNOWN_ERROR",
        }
    }
}
