//! # Ethereum Handshake
//!
//! An implementation of the Ethereum handshake procedure, as specified at:
//!
//! [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
//!
//! The binary (executable) crate.

use std::time::Instant;

use ethereum_handshake::cli::parse_cli_args;
use ethereum_handshake::interface::dial;
use ethereum_handshake::telemetry::init_tracing;

/// The program's entry point
///
/// Currently, our program works only as a client.
///
/// This means that it can only dial another node,
/// initiating a handshake procedure.
///
/// - Sets up a tracing subscriber
/// - Parses CLI arguments
/// - Calls the handshake algorithm
/// - Prints the total execution time
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let start = Instant::now();

    init_tracing();

    let parsed_args = parse_cli_args()?;

    dial(parsed_args).await?;

    println!("\nTook {:.3?} to complete.", start.elapsed());

    Ok(())
}
