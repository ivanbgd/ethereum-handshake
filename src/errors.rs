//! The errors that are used in the library and that can be used in a binary crate.

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Invalid recipient's enode. See help for the correct format.")]
    InvalidRecipientEnode,

    #[error("Invalid recipient's public key: {0}")]
    InvalidRecipientUserName(String),

    #[error("Invalid recipient's host name: {0}")]
    InvalidRecipientHostName(String),

    #[error("Timeout error: {0}")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
}
