//! Parser for a text file with a list of enodes

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::input::Enode;

/// Parse a text file that consists of a list of enodes
///
/// Invalid enodes will be omitted, and valid ones will be kept.
///
/// # Returns
/// [`Vec<Enode>`], a list of recipient enodes
///
/// # Errors
/// - [`fs::io::Result<String>`], for I/O errors
pub fn parse_file_enodes(file_path: Option<PathBuf>) -> Result<Vec<Enode>, Box<dyn Error>> {
    let mut result = Vec::new();

    if let Some(file_path) = file_path {
        for enode in fs::read_to_string(file_path)?.lines() {
            match Enode::new(enode) {
                Ok(enode) => result.push(enode),
                Err(err) => eprintln!("Skipping {} due to {}", enode, err),
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::constants::{TEST_FILE, TEST_FILE_BAD_AND_GOOD, TEST_FILE_NON_EXISTENT};

    use super::*;

    #[test]
    fn test_parse_file_enodes_empty() {
        let recipient_enodes = None;

        let result = parse_file_enodes(recipient_enodes);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_parse_file_enodes_all_good() {
        let file_path = Some(PathBuf::from(TEST_FILE));

        let result = parse_file_enodes(file_path);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(2, result.len());
    }

    #[test]
    fn test_parse_file_enodes_bad_and_good() {
        let file_path = Some(PathBuf::from(TEST_FILE_BAD_AND_GOOD));

        let result = parse_file_enodes(file_path);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(1, result.len());
    }

    #[test]
    fn test_parse_file_enodes_non_existent() {
        let file_path = Some(PathBuf::from(TEST_FILE_NON_EXISTENT));

        let result = parse_file_enodes(file_path);
        assert!(result.is_err());
    }
}
