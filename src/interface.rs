//! Functions for interfacing our app to the world
//!
//! Our app can act as an initiator of a call, trying to connect to another
//! node(s) in the peer-to-peer network, and/or it can act as a recipient of
//! such a call (not implemented).
//!
//! In the former case, a user should provide the address(es) of node(s) that
//! they'd like to call using our app as a client, through CLI.
//!
//! In the latter case, which is not (yet) implemented, no CLI arguments
//! are necessary.

use std::time::Duration;

use k256::SecretKey;
use tokio::net::TcpStream;
use tracing::{debug, error, info};

use crate::errors::{ConnError, DialError};
use crate::handshake::initiate_handshake;
use crate::input::Enode;

/// Dials all provided recipient nodes
///
/// Handshaking with a node should preferably be atomic, i.e., uninterrupted,
/// so nodes should be either dialed successively or concurrently by using
/// multiple threads - a thread per node, or a pool of threads, i.e.,
/// in parallel, but it could probably also be implemented safely
/// in asynchronous concurrent manner, because `tokio` supports multithreading.
pub async fn dial_all(
    static_secret_key: &SecretKey,
    timeout: u64,
    enodes: Vec<Enode>,
) -> Result<(), DialError> {
    // TODO: Make concurrent!
    for enode in enodes {
        dial(static_secret_key, timeout, enode).await?;
    }

    Ok(())
}

/// Dial a single recipient node
///
/// Tries to connect to the node and then to handshake with it.
///
/// Provides some basic and simple validation as example.
///
/// Expects an IPv4 address.
///
/// # Errors
/// - [`DialError::ConnectionError`] wrapping [`ConnError::TcpStreamError`]
async fn dial(static_secret_key: &SecretKey, timeout: u64, enode: Enode) -> Result<(), DialError> {
    let username = enode.username;
    let hostname = enode.hostname;

    let ip = hostname.clone();
    let ip = ip
        .split(':')
        .next()
        .expect("Improperly validated: Expected colon in hostname");

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
                Err(err) => {
                    return Err(DialError::from(ConnError::TcpStreamError(err.to_string())))
                }
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
pub async fn answer(_timeout: u64) -> Result<(), DialError> {
    debug!("Entering `answer()`");
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use k256::SecretKey;
    use rand_core::OsRng;

    use crate::constants::{TEST_HOSTNAME, TEST_USERNAME, TIMEOUT};
    use crate::errors::ConnError::TcpStreamError;
    use crate::errors::DialError::ConnectionError;

    use super::*;

    static STATIC_SK: OnceLock<SecretKey> = OnceLock::new();

    #[tokio::test]
    async fn test_dial_pass() {
        STATIC_SK.get_or_init(|| SecretKey::random(&mut OsRng));

        let enode = Enode {
            username: TEST_USERNAME.to_string(),
            hostname: TEST_HOSTNAME.to_string(),
        };

        assert!(dial(&STATIC_SK.get().unwrap(), TIMEOUT, enode)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_dial_fail_missing_colon() {
        STATIC_SK.get_or_init(|| SecretKey::random(&mut OsRng));

        let bad_hostname = TEST_HOSTNAME.replace(':', "");

        let enode = Enode {
            username: TEST_USERNAME.to_string(),
            hostname: bad_hostname.clone(),
        };

        let result = dial(&STATIC_SK.get().unwrap(), TIMEOUT, enode).await;

        assert!(result.is_err());
        assert_eq!(
            Err(ConnectionError(TcpStreamError(
                "invalid socket address".to_string()
            ))),
            result
        );
    }
}
