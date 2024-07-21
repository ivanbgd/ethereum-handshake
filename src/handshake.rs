//! Functions for handling the handshake procedure
//!
//! - Initiate handshake
//! - Respond to a handshake request - not implemented

// use ecies::{decrypt, utils::generate_keypair};
use ethereum_types::H256;
use hex;
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey, SecretKey};
// use k256::ecdsa::{self, Signature, signature::Signer, SigningKey};
// use k256::ecdsa::signature::SignerMut;
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
#[instrument(level = "trace", skip_all)]
pub async fn initiate_handshake(
    static_secret_key: &SecretKey,
    stream: &mut TcpStream,
    username: String,
    hostname: String,
) -> Result<(), HandshakeError> {
    info!("Starting handshake with {}...", hostname);

    // 1. initiator connects to recipient and sends its auth message
    step_1(static_secret_key, stream, &username, &hostname).await?;

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
#[instrument(level = "trace", skip_all)]
pub async fn respond_to_handshake() {
    unimplemented!()

    // 2. recipient accepts, decrypts and verifies auth (checks that recovery of signature == keccak256(ephemeral-pubk))

    // 3. recipient generates auth-ack message from remote-ephemeral-pubk and nonce

    // 4. recipient derives secrets and sends the first encrypted frame containing the Hello message

    // 7. recipient receives and authenticates first encrypted frame

    // 9. cryptographic handshake is complete if MAC of first encrypted frame is valid on both sides
}

/// Step 1: initiator connects to recipient and sends its `auth` message
#[instrument(level = "trace", skip_all)]
async fn step_1(
    static_secret_key: &SecretKey,
    stream: &mut TcpStream,
    username: &String,
    hostname: &String,
) -> Result<(), HandshakeError> {
    debug!("Begin Step 1 with {}", hostname);

    let _static_public_key = EncodedPoint::from(static_secret_key.public_key());
    let initiator_public_key = static_secret_key.public_key();
    let initiator_public_key = &(*(initiator_public_key.to_sec1_bytes()))[1..];

    let username = match hex::decode(username) {
        Ok(name) => name,
        Err(err) => Err(HandshakeError::HexDecodeError(err.to_string()))?,
    };
    let mut aux = [0; PUBLIC_KEY_UNCOMPRESSED_LEN];
    aux[0] = 4;
    aux[1..].copy_from_slice(username.as_ref());
    let recipient_public_key = match PublicKey::from_sec1_bytes(aux.as_ref()) {
        Ok(key) => key,
        Err(err) => Err(HandshakeError::Sec1Error(err.to_string()))?,
    };

    let init_ephemeral_secret = EphemeralSecret::random(&mut OsRng);
    let init_ephemeral_secret = Secret::new(init_ephemeral_secret);
    let _init_ephemeral_pubkey_bytes =
        EncodedPoint::from(init_ephemeral_secret.expose_secret().public_key());

    let init_shared = init_ephemeral_secret
        .expose_secret()
        .diffie_hellman(&recipient_public_key);
    let init_shared = H256::from_slice(&init_shared.raw_secret_bytes()[..32]);

    let initiator_nonce = H256::random();

    let _init_ephemeral_pubkey = init_ephemeral_secret.expose_secret().public_key();

    // TODO: compute signature
    let message = init_shared ^ initiator_nonce;
    let signature = message;
    // let signature: Signature = init_ephemeral_secret
    //     .expose_secret()
    //     .sign(message.as_fixed_bytes());

    // "auth_body" is an RLP stream of 4 values
    let mut rlp_stream = RlpStream::new_list(4);
    rlp_stream.append(&signature);
    // rlp_stream.append(&_init_ephemeral_pubkey.to_sec1_bytes().deref());
    rlp_stream.append(&initiator_public_key);
    rlp_stream.append(&initiator_nonce);
    rlp_stream.append(&AUTH_VERSION);
    let auth_body = rlp_stream.out();

    let _enc_auth_body = ecies::encrypt(username.as_ref(), auth_body.as_ref());
    let _enc_auth_body = ecies::encrypt(aux.as_ref(), auth_body.as_ref());

    // TODO: add auth-padding, auth-size?
    let enc_auth_body = ecies::encrypt(&recipient_public_key.to_sec1_bytes(), auth_body.as_ref())
        .map_err(|err| HandshakeError::EciesEncryptError(err.to_string()))?;

    let _auth_size = enc_auth_body.len();

    // TODO: prepend with auth_size
    let auth = enc_auth_body;

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
#[instrument(level = "trace", skip_all)]
async fn step_5(
    stream: &mut TcpStream,
    _username: &str,
    hostname: &String,
) -> Result<(), HandshakeError> {
    debug!("Begin Step 5 with {}", hostname);

    // receive the "auth-ack" message from recipient

    // pin the buffer for security reasons so that it can't be relocated and copied over,
    // so that we're certain we've manually zeroized the only instance at the end
    let mut auth_ack = Box::pin([0u8; 1024]);
    let auth_ack_len = Secret::new(stream.read(&mut *auth_ack).await?);
    eprintln!("{:?}", *auth_ack);
    dbg!(auth_ack_len.expose_secret());

    // derive secrets

    // TODO: zeroize secrets
    auth_ack.zeroize();

    Ok(())
}
