//! # Ethereum Handshake
//!
//! An implementation of the Ethereum handshake procedure, as specified at:
//!
//! [The RLPx Transport Protocol (devp2p)](https://github.com/ethereum/devp2p/blob/master/rlpx.md)
//!
//! The binary (executable) crate.

use std::time::{Duration, Instant};

use clap::Parser;
use tokio::net::TcpStream;
use tracing::{error, info, warn};

use ethereum_handshake::cli::{Args, parse_target_enode};
use ethereum_handshake::handshake::handshake;
use ethereum_handshake::telemetry::init_subscriber;

/// The program's entry point.
///
/// Sets up a tracing subscriber, parses CLI arguments, calls the handshake algorithm,
/// and in the end prints the total execution time.
#[tokio::main]
async fn main() -> eyre::Result<()> {
    let start = Instant::now();

    init_subscriber();

    let args = Args::parse();
    let timeout = args.timeout;
    let recipient_enode = args.target_enode;
    let (username, address) = parse_target_enode(recipient_enode)?;

    info!("Connecting to recipient...");

    // let mut stream = TcpStream::connect(address).await?;
    // info!("Connected to recipient.");

    if let Ok(stream) =
        tokio::time::timeout(Duration::from_millis(timeout), TcpStream::connect(address)).await
    {
        info!("Connected to recipient.");
        if let Err(err) = handshake(&mut stream.unwrap(), username).await {
            warn!("{}", err);
        }
    } else {
        error!("Failed to connect to recipient.");
    }

    // if let Ok(mut stream) = TcpStream::connect(address).await {
    //     info!("Connected to recipient.");
    //     if let Err(err) = handshake(&mut stream, username).await {
    //         warn!("{}", err);
    //     }
    // } else {
    //     error!("Failed to connect to recipient.");
    // }

    println!("\nTook {:.3?} to complete.", start.elapsed());

    Ok(())
}
