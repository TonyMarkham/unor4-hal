//! The CAN signal capability.

use crate::gpio::PinId;

/// The pin carries signal `Role` ([`role`](crate::gpio::role)) of CAN
/// channel `Unit` (the PAC's `CAN0`). No driver uses it yet.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be `{Unit}` `{Role}`",
    label = "no such CAN function on this pin",
    note = "each pin's CAN functions are listed in the `pin_map` table"
)]
pub trait CanPin<Unit, Role>: PinId {
    /// `PmnPFS.PSEL` value selecting it.
    const PSEL: u8;
}
