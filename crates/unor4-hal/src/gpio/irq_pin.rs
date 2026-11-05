//! The external interrupt capability.

use crate::gpio::PinId;

/// The pin is an external interrupt input (`IRQn`, selected by
/// `PmnPFS.ISEL`). No driver uses it yet.
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no external interrupt",
    label = "no IRQn function on this pin"
)]
pub trait IrqPin: PinId {
    /// Interrupt number n of `IRQn`.
    const IRQ: u8;
}
