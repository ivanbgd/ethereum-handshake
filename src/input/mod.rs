//! # Module [`input`]
//!
//! Get user input which is a recipient's enode or a list of recipient enodes
//! that the user would like to connect to.
//!
//! Supported sources are:
//! - CLI
//! - Text file - unimplemented

pub use cli::*;
pub use enode::*;
pub use file::*;

mod cli;
mod enode;
mod file;
