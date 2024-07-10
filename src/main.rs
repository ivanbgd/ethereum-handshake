//! # Ethereum Handshake
//!
//! An implementation of the Ethereum handshake procedure, as specified at:
//!
//! [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
//!
//! The binary (executable) crate.

use std::time::Instant;

use clap::Parser;

use ethereum_handshake::cli::Args;

/// The program's entry point.
///
/// Parses CLI arguments, calls the handshake algorithm,
/// and in the end prints the total execution time.
fn main() {
    let start = Instant::now();

    let args = Args::parse();

    println!("\nTook {:.3?} to complete.", start.elapsed());
}
