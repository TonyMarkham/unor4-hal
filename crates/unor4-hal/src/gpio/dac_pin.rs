//! The DAC output capability.

use crate::gpio::PinId;

/// The pin is a DAC12 output (`DAn`, selected by `PmnPFS.ASEL`). No driver
/// uses it yet.
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no DAC output",
    label = "no DAn function on this pin"
)]
pub trait DacPin: PinId {
    /// Channel number n of `DAn`.
    const CHANNEL: u8;
}
