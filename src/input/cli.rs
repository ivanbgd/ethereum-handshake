//! The CLI arguments parser

use std::path::PathBuf;

use clap::Parser;

use crate::constants::TIMEOUT;
use crate::input::Enode;

/// An implementation of the Ethereum handshake procedure
#[derive(Parser)]
#[command(name = "Ethereum Handshake")]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Handshake timeout in milliseconds, from 100 to 10000
    #[arg(short, long, default_value_t = TIMEOUT,
    value_parser = clap::value_parser!(u64).range(100..=10*TIMEOUT))]
    pub timeout: u64,

    /// A list of recipient enodes in the following form:
    /// enode://<node_id>@<ipv4_address>:<port>
    #[arg(short, long)]
    pub recipient_enodes: Option<Vec<String>>,

    /// Path to a text file with a list of recipient enodes in the following form:
    /// enode://<node_id>@<ipv4_address>:<port>
    #[arg(short, long)]
    pub file_path: Option<PathBuf>,
}

/// Parsed CLI arguments
/// - timeout
/// - list of enodes obtained from command line
/// - optional path to a text file with a list of enodes
#[derive(Debug)]
pub struct ParsedArgs {
    pub timeout: u64,
    pub cli_enodes: Vec<Enode>,
    pub file_path: Option<PathBuf>,
}

/// Parse CLI arguments
///
/// # Returns
/// [`ParsedArgs`]
pub fn parse_cli_args() -> ParsedArgs {
    let args = CliArgs::parse();

    let timeout = args.timeout;
    let recipient_enodes = args.recipient_enodes;
    let file_path = args.file_path;

    let cli_enodes = parse_cli_enodes(recipient_enodes);

    ParsedArgs {
        timeout,
        cli_enodes,
        file_path,
    }
}

/// Parse a list of enodes from command line
///
/// Invalid enodes will be omitted, and valid ones will be kept.
///
/// # Returns
/// [`Vec<Enode>`], a list of recipient enodes
fn parse_cli_enodes(recipient_enodes: Option<Vec<String>>) -> Vec<Enode> {
    let mut result = Vec::new();

    if let Some(enodes) = recipient_enodes {
        for enode in enodes {
            match Enode::new(&enode) {
                Ok(enode) => result.push(enode),
                Err(err) => eprintln!("Skipping {} due to {}", enode, err),
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::constants::{TEST_ENODE, TEST_HOSTNAME, TEST_USERNAME};

    use super::*;

    #[test]
    fn test_parse_cli_enodes_none() {
        let recipient_enodes = None;

        let result = parse_cli_enodes(recipient_enodes);

        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_cli_enodes_some_pass() {
        let mut recipient_enodes = vec![TEST_ENODE, TEST_ENODE, TEST_ENODE]
            .iter()
            .map(|enode| enode.to_string())
            .collect::<Vec<String>>();
        let enodes_len = recipient_enodes.len();

        recipient_enodes[1] = recipient_enodes[1].replace(":30303", "30303");

        let result = parse_cli_enodes(Some(recipient_enodes));

        assert_eq!(enodes_len - 1, result.len());

        assert_eq!(TEST_USERNAME, result[0].username);
        assert_eq!(TEST_HOSTNAME, result[0].hostname);
        assert_eq!(TEST_USERNAME, result[1].username);
        assert_eq!(TEST_HOSTNAME, result[1].hostname);
    }

    #[test]
    fn test_parse_cli_enodes_empty() {
        let mut enode = TEST_ENODE.to_string();
        enode = enode.replace(":30303", "30303");

        let recipient_enodes = Some(vec![enode]);

        let result = parse_cli_enodes(recipient_enodes);

        assert!(result.is_empty());
    }
}
