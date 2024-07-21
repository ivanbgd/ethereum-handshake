//! Recipient's enode

use crate::errors::EnodeParseError;

/// Recipient's enode
///
/// - username (part of recipient's enode - public key)
/// - hostname (part of recipient's enode - address:port
#[derive(Debug, PartialEq)]
pub struct Enode {
    pub username: String,
    pub hostname: String,
}

impl Enode {
    /// Creates a new [`Enode`]
    ///
    /// # Errors
    /// - [`EnodeParseError::InvalidRecipientUserName`], if it can't parse it
    /// - [`EnodeParseError::InvalidRecipientEnode`], if `enode` doesn't contain `@`
    pub fn new(enode: &str) -> Result<Self, EnodeParseError> {
        let (username, hostname) = Self::parse(enode)?;

        Ok(Self { username, hostname })
    }

    /// Parses recipient's `enode` into `username` and `hostname`
    ///
    /// Provides some basic and simple validation as example.
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
    /// A tuple of:
    /// - `username`, the recipient's public key.
    /// - `hostname`, the recipient's IP address and port, separated by `:`.
    ///
    /// # Errors
    /// - [`EnodeParseError::InvalidRecipientUserName`], if it can't parse it
    /// - [`EnodeParseError::InvalidRecipientEnode`], if `enode` doesn't contain `@`
    fn parse(recipient_enode: &str) -> Result<(String, String), EnodeParseError> {
        let mut split_enode = recipient_enode.split('@');

        let user = split_enode.next().unwrap_or_default();
        let username = match user.get(8..) {
            Some(key) => key.to_string(),
            None => return Err(EnodeParseError::InvalidRecipientUserName(user.to_string())),
        };

        let hostname = match split_enode.next() {
            Some(addr) => addr.to_string(),
            None => return Err(EnodeParseError::InvalidRecipientEnode),
        };

        Self::validate_hostname(&hostname)?;

        Ok((username, hostname))
    }

    /// Validates hostname
    ///
    /// A simple example
    fn validate_hostname(hostname: &String) -> Result<(), EnodeParseError> {
        if !hostname.contains(':') {
            Err(EnodeParseError::InvalidRecipientHostName(
                hostname.to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{TEST_ENODE, TEST_HOSTNAME, TEST_USERNAME};

    use super::*;

    #[test]
    fn test_parse_pass() {
        let enode = TEST_ENODE;

        let result = Enode::new(enode);
        assert!(result.is_ok());

        let result = result.unwrap();
        let (username, hostname) = (result.username, result.hostname);
        assert_eq!(TEST_USERNAME, username);
        assert_eq!(TEST_HOSTNAME, hostname);
    }

    #[test]
    fn test_parse_fail_bad_enode_missing_at() {
        let mut enode = TEST_ENODE.to_string();
        enode = enode.replace('@', "A");

        let result = Enode::new(enode.as_ref());

        assert!(result.is_err());
        assert_eq!(Err(EnodeParseError::InvalidRecipientEnode), result);
    }

    #[test]
    fn test_parse_fail_bad_username() {
        let enode = "";

        let result = Enode::new(enode);

        assert!(result.is_err());

        assert_eq!(
            Err(EnodeParseError::InvalidRecipientUserName("".to_string())),
            result
        );
    }

    #[test]
    fn test_parse_fail_bad_hostname_missing_colon() {
        let mut enode = TEST_ENODE.to_string();
        enode = enode.replace(":30303", "*30303");

        let result = Enode::new(enode.as_ref());

        assert!(result.is_err());

        assert_eq!(
            Err(EnodeParseError::InvalidRecipientHostName(
                TEST_HOSTNAME.to_string().replace(':', "*")
            )),
            result
        );
    }
}
