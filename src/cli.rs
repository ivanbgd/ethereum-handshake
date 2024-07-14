//! The CLI arguments parser.

use clap::Parser;

use crate::constants::TIMEOUT;
use crate::errors::{Error, Result};

/// An implementation of the Ethereum handshake procedure
#[derive(Parser)]
#[command(name = "Ethereum Handshake")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Handshake timeout in milliseconds, from 100 to 10000
    #[arg(short, long, default_value_t = TIMEOUT,
    value_parser = clap::value_parser!(u64).range(1..=10*TIMEOUT))]
    pub timeout: u64,

    /// Target node's enode in the following form:
    /// enode://<node_id>@<ip_address>:<port>
    pub target_enode: String,
}

pub fn parse_target_enode(recipient_enode: String) -> Result<(String, String), Error> {
    let mut split_enode = recipient_enode.split('@');

    let user = split_enode.nth(0).unwrap_or_default();
    let username = match user.get(8..) {
        Some(key) => key.to_string(),
        None => return Err(Error::InvalidUserName(user.to_string())),
    };

    let hostname = match split_enode.nth(0) {
        Some(addr) => addr.to_string(),
        None => return Err(Error::InvalidHostName()),
    };

    Ok((username, hostname))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENODE: &str = "enode://a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc\
    232016e1c51b544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff05fd\
    f9b31c15072@178.128.136.233:30303";

    #[test]
    fn test_parse_target_enode_pass() {
        let enode = ENODE.to_string();

        let result = parse_target_enode(enode);
        assert!(result.is_ok());

        let (username, hostname) = result.unwrap();
        assert_eq!(
            "a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc232016e1c51b\
            544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff\
            05fdf9b31c15072",
            username
        );
        assert_eq!("178.128.136.233:30303", hostname);
    }
}
