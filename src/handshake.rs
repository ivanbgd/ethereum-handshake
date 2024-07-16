//! Functions for handling the handshake procedure
//!
//! - Initiate handshake
//! - Respond to a handshake request - not implemented

use hex;
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
use secrecy::{ExposeSecret, Secret};
use tokio::net::TcpStream;
use tracing::{info, instrument};

use crate::errors::HandshakeError;

/// The handshake procedure between our client as initiator and a recipient node
///
/// The procedure is defined at:
/// https://github.com/ethereum/devp2p/blob/master/rlpx.md
#[instrument(level = "trace")]
// pub async fn handshake(stream: &mut TcpStream, username: PublicKey) {
pub async fn initiate_handshake(
    _stream: &mut TcpStream,
    username: String,
    hostname: String,
) -> Result<(), HandshakeError> {
    info!("Starting handshake with {}...", hostname);

    // 1. initiator connects to recipient and sends its auth message
    let init_secret = EphemeralSecret::random(&mut OsRng);
    let init_secret = Secret::new(init_secret);
    let _init_pk_bytes = EncodedPoint::from(init_secret.expose_secret().public_key());

    let username = match hex::decode(username) {
        Ok(name) => name,
        Err(err) => Err(HandshakeError::HexDecodeError(err.to_string()))?,
    };
    let mut aux = [0; 65];
    aux[0] = 4;
    aux[1..].copy_from_slice(username.as_ref());
    let recip_public = match PublicKey::from_sec1_bytes(aux.as_ref()) {
        Ok(key) => key,
        Err(err) => Err(HandshakeError::Sec1Error(err.to_string()))?,
    };

    let _init_shared = init_secret.expose_secret().diffie_hellman(&recip_public);

    // TODO: ..., send auth msg

    // 5. initiator receives auth-ack and derives secrets
    // TODO + zeroize

    // 6. initiator sends its first encrypted frame containing initiator Hello message
    // TODO

    // 8. initiator receives and authenticates first encrypted frame
    // TODO

    // 9. cryptographic handshake is complete if MAC of first encrypted frame is valid on both sides
    // TODO

    Ok(())
}

/// The handshake procedure between our client as recipient and an initiator node
pub async fn respond_to_handshake() {
    unimplemented!()

    // 2. recipient accepts, decrypts and verifies auth (checks that recovery of signature == keccak256(ephemeral-pubk))

    // 3. recipient generates auth-ack message from remote-ephemeral-pubk and nonce

    // 4. recipient derives secrets and sends the first encrypted frame containing the Hello message

    // 7. recipient receives and authenticates first encrypted frame

    // 9. cryptographic handshake is complete if MAC of first encrypted frame is valid on both sides
}
