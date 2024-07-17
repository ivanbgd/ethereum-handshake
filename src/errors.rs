//! The errors that are used in the library and that can be used in binary crates.

use thiserror::Error;

/// Input errors that come from the CLI
#[derive(Debug, Error, PartialEq)]
pub enum CliError {
    #[error("Invalid recipient's enode. See help for the correct format.")]
    InvalidRecipientEnode,

    #[error("Invalid recipient's public key: {0}")]
    InvalidRecipientUserName(String),

    #[error("Invalid recipient's host name: {0}")]
    InvalidRecipientHostName(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),
}

impl From<ConnError> for CliError {
    fn from(value: ConnError) -> Self {
        Self::ConnectionError(value.to_string())
    }
}

/// Connection errors
#[derive(Debug, Error)]
pub enum ConnError {
    #[error("TCP stream error: {0}")]
    TcpStreamError(String),

    #[error("Timeout error: {0}")]
    TimeoutError(#[from] tokio::time::error::Elapsed),
}

/// Errors during the handshake procedure
#[derive(Debug, Error)]
pub enum HandshakeError {
    #[error("Hex decode error: {0}")]
    HexDecodeError(String),

    #[error("Sec1 error: {0}")]
    Sec1Error(String),

    #[error("I/O Error: {0}")]
    IOError(String),
}

impl From<std::io::Error> for HandshakeError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}
