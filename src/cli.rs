//! The CLI arguments parser.

use clap::Parser;

use crate::constants::TIMEOUT;
use crate::errors::CliError;

/// An implementation of the Ethereum handshake procedure
#[derive(Parser)]
#[command(name = "Ethereum Handshake")]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Handshake timeout in milliseconds, from 100 to 10000
    #[arg(short, long, default_value_t = TIMEOUT,
    value_parser = clap::value_parser!(u64).range(1..=10*TIMEOUT))]
    pub timeout: u64,

    /// Recipient node's enode in the following form:
    /// enode://<node_id>@<ipv4_address>:<port>
    pub recipient_enode: String,
}

/// Parsed CLI arguments
/// - timeout
/// - username (part of enode - public key)
/// - hostname (part of enode - address:port
#[derive(Debug)]
pub struct ParsedArgs {
    pub timeout: u64,
    pub username: String,
    pub hostname: String,
}

/// Parse CLI arguments for a single recipient node
///
/// # Returns
/// [`ParsedArgs`]
pub fn parse_cli_args() -> Result<ParsedArgs, CliError> {
    let args = CliArgs::parse();
    let timeout = args.timeout;
    let recipient_enode = args.recipient_enode;
    let (username, hostname) = parse_recipient_enode(recipient_enode)?;

    Ok(ParsedArgs {
        timeout,
        username,
        hostname,
    })
}

/// Parses recipient's `enode` into `username` and `hostname`
///
/// Provides some basic, exemplary, validation.
///
/// Expects full `enode` as input, which means it should begin
/// with `enode://`.
///
/// The expected `enode` format is:
///
/// `enode://<node_id>@<ip_address>:<port>`
///
/// This function doesn't require the IP address to necessarily be IPv4.
///
/// # Returns
/// - `username` is the recipient's public key.
/// - `hostname` is the recipient's IP address and port, separated by `:`.
///
/// # Errors
/// - [`CliError::InvalidRecipientUserName`], if it can't parse it
/// - [`CliError::InvalidRecipientEnode`], if `enode` doesn't contain `@`
pub fn parse_recipient_enode(recipient_enode: String) -> Result<(String, String), CliError> {
    let mut split_enode = recipient_enode.split('@');

    let user = split_enode.next().unwrap_or_default();
    let username = match user.get(8..) {
        Some(key) => key.to_string(),
        None => return Err(CliError::InvalidRecipientUserName(user.to_string())),
    };

    let hostname = match split_enode.next() {
        Some(addr) => addr.to_string(),
        None => return Err(CliError::InvalidRecipientEnode),
    };

    Ok((username, hostname))
}

#[cfg(test)]
mod tests {
    use crate::constants::{TEST_ENODE, TEST_HOSTNAME, TEST_USERNAME};

    use super::*;

    #[test]
    fn test_parse_recipient_enode_pass() {
        let enode = TEST_ENODE.to_string();

        let result = parse_recipient_enode(enode);
        assert!(result.is_ok());

        let (username, hostname) = result.unwrap();
        assert_eq!(TEST_USERNAME, username);
        assert_eq!(TEST_HOSTNAME, hostname);
    }

    #[test]
    fn test_parse_recipient_enode_fail_bad_username() {
        let enode = "".to_string();

        let result = parse_recipient_enode(enode);

        assert!(result.is_err());
        assert_eq!(
            Err(CliError::InvalidRecipientUserName("".to_string())),
            result
        );
    }

    #[test]
    fn test_parse_recipient_enode_fail_bad_enode_missing_at() {
        let mut enode = TEST_ENODE.to_string();
        enode = enode.replace('@', "A");

        let result = parse_recipient_enode(enode);

        assert!(result.is_err());
        assert_eq!(Err(CliError::InvalidRecipientEnode), result);
    }
}
