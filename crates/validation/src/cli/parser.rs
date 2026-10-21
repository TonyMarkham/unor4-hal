//! The top-level clap parser.

use crate::cli::command::Command;

use clap::Parser;

/// Host side of the validation pair (`r3-validator` + `r4_validator`).
/// Neither builds nor uploads: flash both boards first (README, Validation).
#[derive(Parser)]
#[command(name = "validate", version)]
pub struct Cli {
    /// R3 serial port; discovered by USB VID/PID 2341:0043 when omitted.
    #[arg(long, global = true)]
    pub r3_port: Option<String>,

    /// R4 bridge serial port; discovered by USB VID/PID 2341:1002 when
    /// omitted.
    #[arg(long, global = true)]
    pub bridge_port: Option<String>,

    /// The check (or checks) to run.
    #[command(subcommand)]
    pub command: Command,
}
