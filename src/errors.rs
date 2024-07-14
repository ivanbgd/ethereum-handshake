use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid recipient's public key: {0}")]
    InvalidUserName(String),

    #[error("Invalid recipient's host name")]
    InvalidHostName(),

    #[error("Timeout error: {0}")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
}
