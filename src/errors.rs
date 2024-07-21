//! The errors that are used in the library and that can be used in binary crates.

use thiserror::Error;

/// Input errors that occur during dialing
#[derive(Debug, Error, PartialEq)]
pub enum DialError {
    #[error("Enode parse error: {0}")]
    EnodeParseError(#[from] EnodeParseError),

    #[error("Connection error: {0}")]
    ConnectionError(#[from] ConnError),
}

/// Errors during parsing of an enode
#[derive(Debug, Error, PartialEq)]
pub enum EnodeParseError {
    #[error("Invalid recipient's enode. See help for the correct format.")]
    InvalidRecipientEnode,

    #[error("Invalid recipient's public key: {0}")]
    InvalidRecipientUserName(String),

    #[error("Invalid recipient's host name: {0}")]
    InvalidRecipientHostName(String),
}

/// Connection errors
#[derive(Debug, Error, PartialEq)]
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

    #[error("ECIES encrypt error: {0}")]
    EciesEncryptError(String),

    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
}
