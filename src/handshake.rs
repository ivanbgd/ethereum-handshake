//! Functions for handling the handshake procedure
//!
//! - Initiate handshake
//! - Respond to a handshake request - not implemented

// use ecies::{decrypt, utils::generate_keypair};
use hex;
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
// use rlp::{Rlp, RlpStream};
use rlp::RlpStream;
use secrecy::{ExposeSecret, Secret, Zeroize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info, instrument};

use crate::constants::{AUTH_VERSION, PUBLIC_KEY_UNCOMPRESSED_LEN};
use crate::errors::HandshakeError;

/// The handshake procedure between our client as initiator and a recipient node
///
/// The procedure is defined at:
/// https://github.com/ethereum/devp2p/blob/master/rlpx.md
#[instrument(level = "trace")]
pub async fn initiate_handshake(
    stream: &mut TcpStream,
    username: String,
    hostname: String,
) -> Result<(), HandshakeError> {
    info!("Starting handshake with {}...", hostname);

    // 1. initiator connects to recipient and sends its auth message
    step_1(stream, &username, &hostname).await?;

    // 5. initiator receives auth-ack and derives secrets
    step_5(stream, &username, &hostname).await?;

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

/// Step 1: initiator connects to recipient and sends its `auth` message
async fn step_1(
    stream: &mut TcpStream,
    username: &String,
    hostname: &String,
) -> Result<(), HandshakeError> {
    debug!("Begin Step 1 with {}", hostname);

    let init_secret = EphemeralSecret::random(&mut OsRng);
    let init_secret = Secret::new(init_secret);
    let _init_pk_bytes = EncodedPoint::from(init_secret.expose_secret().public_key());

    let username = match hex::decode(username) {
        Ok(name) => name,
        Err(err) => Err(HandshakeError::HexDecodeError(err.to_string()))?,
    };
    let mut aux = [0; PUBLIC_KEY_UNCOMPRESSED_LEN];
    aux[0] = 4;
    aux[1..].copy_from_slice(username.as_ref());
    let recip_public = match PublicKey::from_sec1_bytes(aux.as_ref()) {
        Ok(key) => key,
        Err(err) => Err(HandshakeError::Sec1Error(err.to_string()))?,
    };

    let _init_shared = init_secret.expose_secret().diffie_hellman(&recip_public);

    // "auth_body" is an RLP stream of 4 values
    let mut rlp_stream = RlpStream::new_list(1);
    // rlp_stream.append(sig);
    // rlp_stream.append(init_pubkey);
    // rlp_stream.append(init_nonce);
    rlp_stream.append(&AUTH_VERSION);
    let auth_body = rlp_stream.out();

    let _enc_auth_body = ecies::encrypt(username.as_ref(), auth_body.as_ref());

    let auth = "";

    // send the "auth" message to recipient
    stream.write_all(auth.as_ref()).await?;
    stream.flush().await?;

    debug!(
        "Sent the auth message to recipient {}. End of Step 1.",
        hostname
    );

    Ok(())
}

/// Step 5: initiator receives `auth-ack` and derives secrets
async fn step_5(
    stream: &mut TcpStream,
    _username: &str,
    hostname: &String,
) -> Result<(), HandshakeError> {
    debug!("Begin Step 5 with {}", hostname);

    // receive the "auth-ack" message from recipient
    let mut auth_ack = [0u8; 1024];
    let auth_ack_len = Secret::new(stream.read(&mut auth_ack).await?);
    // dbg!(auth_ack);
    dbg!(auth_ack_len.expose_secret());

    // derive secrets

    // TODO: zeroize secrets
    auth_ack.zeroize();

    Ok(())
}
