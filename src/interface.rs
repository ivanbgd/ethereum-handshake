//! Functions for interfacing our app to the world
//!
//! Our app can act as an initiator of a call, trying to connect to another
//! node in the peer-to-peer network, and/or it can act as a recipient of
//! such a call (not implemented).
//!
//! In the former case, a user should provide the address of a node that
//! they'd like to call using our app as a client, through CLI.
//!
//! In the latter case, which is not (yet) implemented, no CLI arguments
//! are necessary.

use std::time::Duration;

use tokio::net::TcpStream;
use tracing::{error, info, warn};

use crate::cli::ParsedArgs;
use crate::errors::{Error, Result};
use crate::handshake::initiate_handshake;

/// Dial a single recipient node
///
/// Provides some basic, exemplary, validation.
///
/// # Errors
/// - Error::InvalidRecipientHostName
pub async fn dial(parsed_args: ParsedArgs) -> Result<(), Error> {
    let timeout = parsed_args.timeout;
    let username = parsed_args.username;
    let hostname = parsed_args.hostname;

    let ip = hostname.clone();
    if !ip.contains(':') {
        return Err(Error::InvalidRecipientHostName(ip.to_string()));
    }
    let ip = ip.split(':').next().expect("Expected colon in hostname");

    info!("Connecting to recipient {}...", ip);

    if let Ok(stream) = tokio::time::timeout(
        Duration::from_millis(timeout),
        TcpStream::connect(&hostname),
    )
    .await
    {
        info!("Connected to recipient {}.", ip);
        if let Err(err) = initiate_handshake(&mut stream.unwrap(), username, hostname).await {
            warn!("{}", err);
        }
    } else {
        error!("Failed to connect to recipient {}.", ip);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::cli::ParsedArgs;
    use crate::constants::{TEST_HOSTNAME, TEST_USERNAME};

    use super::*;

    #[tokio::test]
    async fn test_dial_pass() {
        let parsed_args = ParsedArgs {
            timeout: 1000,
            username: TEST_USERNAME.to_string(),
            hostname: TEST_HOSTNAME.to_string(),
        };

        assert!(dial(parsed_args).await.is_ok());
    }

    #[tokio::test]
    async fn test_dial_fail_missing_colon() {
        let bad_hostname = TEST_HOSTNAME.replace(':', "");

        let parsed_args = ParsedArgs {
            timeout: 1000,
            username: TEST_USERNAME.to_string(),
            hostname: bad_hostname.clone(),
        };

        let result = dial(parsed_args).await;

        assert!(result.is_err());
        assert_eq!(Err(Error::InvalidRecipientHostName(bad_hostname)), result);
    }
}
