//! The CLI arguments parser.

use clap::Parser;

use crate::constants::TIMEOUT;

/// An implementation of the Ethereum handshake procedure
#[derive(Parser)]
#[command(name = "Ethereum Handshake")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target node's enode in the following form:
    /// enode://<node_id>@<ip_address>:<port>
    pub target_enode: String,

    /// Handshake timeout in milliseconds, from 100 to 10000
    #[arg(short, long, default_value_t = TIMEOUT,
    value_parser = clap::value_parser!(u64).range(100..=10*TIMEOUT))]
    pub timeout: u64,
}
