use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("forwarding a Matrix request failed: {0}")]
    Matrix(#[from] matrix::Error),

    #[error("an IO operation failed: {0}")]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    SMTP(#[from] mail_send::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
