//! The constants that are used in the library and that can be used in a binary crate.
//!
//! Includes constants that are used in tests.

/* CLI Constants */

pub const TIMEOUT: u64 = 1000;

/* Connection Constants */

// A placeholder for connection constants

/* Handshake Constants */

pub const AUTH_VERSION: usize = 5;
pub const PUBLIC_KEY_UNCOMPRESSED_LEN: usize = 65;

/* Test Constants */

pub const TEST_ENODE: &str = "enode://a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc\
    232016e1c51b544cb5b4510ef633ea3278c0e970fa8ad8141e2d4\
    d0f9f95456c537ff05fdf9b31c15072@178.128.136.233:30303";
pub const TEST_USERNAME: &str = "a3435a0155a3e837c02f5e7f5662a2f1fbc25b48e4dc232016e1c51b\
    544cb5b4510ef633ea3278c0e970fa8ad8141e2d4d0f9f95456c537ff05fdf9b31c15072";
pub const TEST_HOSTNAME: &str = "178.128.136.233:30303";

pub const TEST_FILE: &str = "tests/test_enodes.txt";
pub const TEST_FILE_BAD_AND_GOOD: &str = "tests/test_bng.txt";
pub const TEST_FILE_NON_EXISTENT: &str = "tests/test_nonexistent.txt";
