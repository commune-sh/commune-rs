use http::StatusCode;
use thiserror::Error;

use crate::{auth::error::AuthErrorCode, mail::error::MailErrorCode, room::error::RoomErrorCode};

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
    #[error("An error occured while starting up. {0}")]
    Startup(String),
    #[error("Unknown Error Occured")]
    Unknown,
}
