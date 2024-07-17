//! # Ethereum Handshake
//!
//! An implementation of the Ethereum handshake procedure, as specified at:
//!
//! [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
//!
//! The binary (executable) crate.

use std::time::Instant;

use ethereum_handshake::cli::parse_cli_args;
use ethereum_handshake::interface::{answer, dial};
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

    let parsed_args = parse_cli_args()?;

    if !parsed_args.hostname.is_empty() {
        dial(parsed_args).await?;
    } else {
        answer(parsed_args.timeout).await?;
    }

    println!("\nTook {:.3?} to complete.", start.elapsed());

    Ok(())
}
