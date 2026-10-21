//! The validation subcommands.

use clap::Subcommand;

/// One validation run's scope.
#[derive(Subcommand)]
pub enum Command {
    /// Run every check: S1-S8 and the S4 token, S9 + knock (unless
    /// --skip-usb), then the reset bits.
    All {
        /// Skip the native-USB checks (S9, knock).
        #[arg(long)]
        skip_usb: bool,
    },
    /// Send R3 'i'; the validator must answer with its identify line.
    Identify,
    /// Send R3 'a' and report only the named slices' lines.
    Slices {
        /// Slice names ("S1", "S2", ...; "all" reports every line). "S2"
        /// selects both "S2 input" and "S2 button".
        #[arg(default_value = "all")]
        names: Vec<String>,
    },
    /// Only the S4 token check (R3 'a' plus the bridge monitor text).
    S4,
    /// S9: R3 'u' takes native USB, the native port must tick and echo, and
    /// R3 's' must report Configured; then the 1200-baud knock. Needs the
    /// bridge port present.
    Usb,
    /// Pulse the R4 reset line from the R3 and check the outcome.
    Reset {
        /// Single '!' pulse; the validator must answer identify afterwards.
        #[arg(long)]
        single: bool,
        /// Double '@' pulse: bootloader hold (R4 silent, bridge present),
        /// then a final '!' must restart the validator.
        #[arg(long)]
        double: bool,
    },
}
