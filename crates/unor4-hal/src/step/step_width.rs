//! Which STEP timer widths the generator drives.

use crate::{gpio::Sealed, pwm::Bits32};

/// A STEP timer width the generator's internals drive. 32-bit today; the
/// 16-bit internals (dividers, rate 0 without an 89 s period, re-measured
/// latencies) are design §9 step 5, in progress, and this trait goes away
/// when they land.
#[diagnostic::on_unimplemented(
    message = "16-bit STEP timers (`{Self}`) are not built yet: StepGeneratorDesign.md §9 step 5",
    label = "the STEP pin's timer is 16-bit",
    note = "32-bit STEP pins: WiFi D2–D5, Minima D2, D3, D6, D7, D11, D12"
)]
pub trait StepWidth: Sealed {}

impl StepWidth for Bits32 {}
