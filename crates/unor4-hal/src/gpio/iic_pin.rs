//! The IIC signal capability.

use crate::gpio::PinId;

/// The pin carries signal `Role` ([`role`](crate::gpio::role)) of IIC
/// channel `Unit` (the PAC's `IIC0`, `IIC1`). No driver uses it yet.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be `{Unit}` `{Role}`",
    label = "no such IIC function on this pin",
    note = "each pin's IIC functions are listed in the `pin_map` table"
)]
pub trait IicPin<Unit, Role>: PinId {
    /// `PmnPFS.PSEL` value selecting it.
    const PSEL: u8;
}
