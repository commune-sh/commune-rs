use http::StatusCode;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as LettreSmtpError;
use thiserror::Error;

use crate::error::HttpStatusCode;

#[derive(Debug, Error)]
pub enum MailErrorCode {
    #[error("Failed to connect to SMTP Server. {0}")]
    SmtpConnection(LettreSmtpError),
    #[error("Invalid mail payload. {0}")]
    InvalidMailPayload(LettreError),
}

impl HttpStatusCode for MailErrorCode {
    fn status_code(&self) -> StatusCode {
        match self {
            MailErrorCode::SmtpConnection(_) | MailErrorCode::InvalidMailPayload(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_code(&self) -> &'static str {
        match self {
            MailErrorCode::SmtpConnection(_) => "SMTP_CONNECTION",
            MailErrorCode::InvalidMailPayload(_) => "INVALID_MAIL_PAYLOAD",
        }
    }
}
