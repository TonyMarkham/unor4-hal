//! The validation subcommands.

use clap::Subcommand;

/// One validation run's scope.
#[derive(Subcommand)]
pub enum Command {
    /// Run the loaded R3 validator part's session. Part 1: S1-S8, S10
    /// flash, the S4 token, S10 retention, S9 + knock (unless --skip-usb),
    /// then the reset bits. Parts 2 and 3: their S11 checks, then the reset
    /// bits. The other parts' checks show as NOT RUN.
    All {
        /// Skip the native-USB checks (S9, knock).
        #[arg(long)]
        skip_usb: bool,
    },
    /// Send R3 'i'; the validator must answer with its identify line.
    Identify,
    /// Send R3 'a' and report only the named slices' lines. A slice of the
    /// other R3 validator part shows as NOT RUN.
    Slices {
        /// Slice names ("S1", "S2", ...; "all" reports every line). "S2"
        /// selects "S2 input", "S2 button" and "S2 wake".
        #[arg(default_value = "all")]
        names: Vec<String>,
    },
    /// Only the S4 token check (R3 'a' plus the bridge monitor text). R3
    /// validator part 1.
    S4,
    /// S9: R3 'u' takes native USB, the native port must tick and echo, and
    /// R3 's' must report Configured; then the 1200-baud knock. Needs the
    /// bridge port present and R3 validator part 1.
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
    /// S10 retention: the R4's data flash boot counter, watched by the R3
    /// ('k'). By default the R3 resets the R4 `--resets` times; with
    /// `--watch` the owner power-cycles it instead. Needs only the R3
    /// port, so the R4 may run from its barrel jack with USB unplugged.
    /// R3 validator part 1.
    S10 {
        /// R3 '!' resets inside the watch; each must advance the counter
        /// by exactly one with the log intact.
        #[arg(long, default_value_t = 3)]
        resets: u32,
        /// Watch this many seconds for manual power cycles instead of
        /// resetting.
        #[arg(long)]
        watch: Option<u64>,
        /// With --watch: boot events needed to pass (the done-when: ~20).
        #[arg(long, default_value_t = 20)]
        min_boots: usize,
    },
}
