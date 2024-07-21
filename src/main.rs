//! # Ethereum Handshake
//!
//! An implementation of the Ethereum handshake procedure, as specified at:
//!
//! [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
//!
//! The binary (executable) crate.

use std::time::Instant;

use k256::SecretKey;
use rand_core::OsRng;

use ethereum_handshake::input::{parse_cli_args, parse_file_enodes};
use ethereum_handshake::interface::{answer, dial_all};
use ethereum_handshake::telemetry::init_tracing;

/// The program's entry point
///
/// The application can work as an initiator of a connection, or as a recipient.
///
/// This means that it can dial another node, initiating a handshake procedure,
/// and that it can also receive a call from another node, responding to a
/// handshake procedure (not implemented).
///
/// - Sets up a tracing subscriber
/// - Parses CLI arguments
/// - Calls the handshake procedure
/// - Prints the total execution time
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let start = Instant::now();

    init_tracing();

    let static_secret_key = get_static_private_key();

    let parsed_args = parse_cli_args();
    let timeout = parsed_args.timeout;
    let cli_enodes = parsed_args.cli_enodes;
    let file_enodes = parse_file_enodes(parsed_args.file_path).unwrap_or_else(|err| {
        eprintln!("{}", err);
        Vec::new()
    });
    let mut enodes = Vec::with_capacity(cli_enodes.len() + file_enodes.len());
    enodes.extend(cli_enodes);
    enodes.extend(file_enodes);

    // TODO: tokio::select maybe, but handshakes might not be atomic in that case, or they will be?
    if !enodes.is_empty() {
        dial_all(&static_secret_key, timeout, enodes).await?;
    } else {
        answer(timeout).await?;
    }

    println!("\nTook {:.3?} to complete.", start.elapsed());

    Ok(())
}

/// Simulate reading of a static secp256k1 private key
/// from a permanent (non-volatile) storage
fn get_static_private_key() -> SecretKey {
    let static_secret_key: SecretKey = SecretKey::random(&mut OsRng);
    // let sk = Secret::new(sk.to_bytes());

    static_secret_key
}
