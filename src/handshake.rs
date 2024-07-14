// use secp256k1::{PublicKey, SecretKey};
use tokio::net::TcpStream;
use tracing::{info, instrument};

use crate::errors::{Error, Result};

// pub async fn handshake(stream: &mut TcpStream, public_key: PublicKey) {
#[instrument(level = "trace")]
pub async fn handshake(stream: &mut TcpStream, public_key: String) -> Result<(), Error> {
    info!("Starting handshake...");

    Ok(())
}
