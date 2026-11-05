//! The command-line interface.

mod command;
mod parser;

// ---------------------------------------------------------------------------------------------- //

pub use crate::cli::{command::Command, parser::Cli};
