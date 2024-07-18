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

use k256::SecretKey;
use tokio::net::TcpStream;
use tracing::{debug, error, info};

use crate::cli::ParsedArgs;
use crate::errors::{CliError, ConnError};
use crate::handshake::initiate_handshake;

/// Dial a single recipient node
///
/// Tries to connect to the node and then to handshake with it.
///
/// Provides some basic, exemplary, validation.
///
/// Expects an IPv4 address.
///
/// # Errors
/// - [`CliError::InvalidRecipientHostName`]
/// - [`CliError::ConnectionError`] wrapping [`ConnError::TcpStreamError`]
pub async fn dial(static_secret_key: &SecretKey, parsed_args: ParsedArgs) -> Result<(), CliError> {
    let timeout = parsed_args.timeout;
    let username = parsed_args.username;
    let hostname = parsed_args.hostname;

    let ip = hostname.clone();
    if !ip.contains(':') {
        return Err(CliError::InvalidRecipientHostName(ip.to_string()));
    }
    let ip = ip.split(':').next().expect("Expected colon in hostname");

    info!("Connecting to recipient {}...", ip);

    // connection timeout
    match tokio::time::timeout(
        Duration::from_millis(timeout),
        TcpStream::connect(&hostname),
    )
    .await
    {
        Ok(stream) => {
            let mut stream = match stream {
                Ok(stream) => stream,
                Err(err) => return Err(CliError::from(ConnError::TcpStreamError(err.to_string()))),
            };

            info!("Connected to recipient {}.", ip);

            // handshake timeout
            if let Err(err) = tokio::time::timeout(
                Duration::from_millis(timeout),
                initiate_handshake(static_secret_key, &mut stream, username, hostname),
            )
            .await
            {
                error!("Failed to handshake with recipient {} due to {}.", ip, err);
            }
        }
        Err(err) => error!("Failed to connect to recipient {} due to {}.", ip, err),
    }

    Ok(())
}

/// Answer to a single connection and handshake request
pub async fn answer(_timeout: u64) -> Result<(), CliError> {
    debug!("Entering `answer()`");
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use k256::SecretKey;
    use rand_core::OsRng;

    use crate::cli::ParsedArgs;
    use crate::constants::{TEST_HOSTNAME, TEST_USERNAME};

    use super::*;

    static STATIC_SK: OnceLock<SecretKey> = OnceLock::new();

    #[tokio::test]
    async fn test_dial_pass() {
        STATIC_SK.get_or_init(|| SecretKey::random(&mut OsRng));

        let parsed_args = ParsedArgs {
            timeout: 1000,
            username: TEST_USERNAME.to_string(),
            hostname: TEST_HOSTNAME.to_string(),
        };

        assert!(dial(&STATIC_SK.get().unwrap(), parsed_args).await.is_ok());
    }

    #[tokio::test]
    async fn test_dial_fail_missing_colon() {
        STATIC_SK.get_or_init(|| SecretKey::random(&mut OsRng));

        let bad_hostname = TEST_HOSTNAME.replace(':', "");

        let parsed_args = ParsedArgs {
            timeout: 1000,
            username: TEST_USERNAME.to_string(),
            hostname: bad_hostname.clone(),
        };

        let result = dial(&STATIC_SK.get().unwrap(), parsed_args).await;

        assert!(result.is_err());
        assert_eq!(
            Err(CliError::InvalidRecipientHostName(bad_hostname)),
            result
        );
    }
}
