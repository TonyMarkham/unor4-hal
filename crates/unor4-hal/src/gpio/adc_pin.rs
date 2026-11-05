//! The ADC input capability.

use crate::gpio::PinId;

/// The pin is an ADC14 input (`ANnnn`, selected by `PmnPFS.ASEL`).
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no ADC channel",
    label = "no ANnnn function on this pin",
    note = "pins with an ADC channel have `adc` in the `pin_map` table"
)]
pub trait AdcPin: PinId {
    /// Channel number nnn of `ANnnn` (0–25, no 15; manual §35.1 Table 35.1).
    /// It indexes `ADANSA0` (≤ 14) or `ADANSA1` (≥ 16) and the matching
    /// `ADDRy` result register.
    const CHANNEL: u8;
}
