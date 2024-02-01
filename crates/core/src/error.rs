use http::StatusCode;
use thiserror::Error;

use crate::account::error::AccountErrorCode;
use crate::auth::error::AuthErrorCode;
use crate::events::error::BoardErrorCode;
use crate::mail::error::MailErrorCode;
use crate::room::error::RoomErrorCode;

pub type Result<T> = std::result::Result<T, Error>;

pub trait HttpStatusCode {
    fn status_code(&self) -> StatusCode;
    fn error_code(&self) -> &'static str;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Auth(#[from] AuthErrorCode),
    #[error("{0}")]
    User(AccountErrorCode),
    #[error("{0}")]
    Room(RoomErrorCode),
    #[error("{0}")]
    Mail(#[from] MailErrorCode),
    #[error("{0}")]
    Board(BoardErrorCode),
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

impl From<BoardErrorCode> for Error {
    fn from(err: BoardErrorCode) -> Self {
        Error::Board(err)
    }
}

impl HttpStatusCode for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Auth(err) => err.status_code(),
            Error::User(err) => err.status_code(),
            Error::Mail(err) => err.status_code(),
            Error::Room(err) => err.status_code(),
            Error::Board(err) => err.status_code(),
            Error::Startup(_) | Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            Error::Auth(err) => err.error_code(),
            Error::User(err) => err.error_code(),
            Error::Mail(err) => err.error_code(),
            Error::Room(err) => err.error_code(),
            Error::Board(err) => err.error_code(),
            Error::Startup(_) => "SERVER_STARTUP_ERROR",
            Error::Unknown => "UNKNOWN_ERROR",
        }
    }
}
