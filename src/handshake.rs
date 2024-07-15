//! Functions for handling the handshake procedure
//!
//! - Initiate handshake
//! - Respond to a handshake request - not implemented

// use secp256k1::{PublicKey, SecretKey};
use tokio::net::TcpStream;
use tracing::{info, instrument};

use crate::errors::{Error, Result};

/// The handshake procedure between our client as initiator and a recipient node
#[instrument(level = "trace")]
// pub async fn handshake(stream: &mut TcpStream, username: PublicKey) {
pub async fn initiate_handshake(
    _stream: &mut TcpStream,
    _username: String,
    hostname: String,
) -> Result<(), Error> {
    info!("Starting handshake with {}...", hostname);

    // 1. initiator connects to recipient and sends its auth message

    Ok(())
}

/// The handshake procedure between our client as recipient and an initiator node
pub async fn respond_to_handshake() {
    unimplemented!()
}
