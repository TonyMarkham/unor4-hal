//! The SPI signal capability.

use crate::gpio::PinId;

/// The pin carries signal `Role` ([`role`](crate::gpio::role)) of SPI
/// channel `Unit` (the PAC's `SPI0`, whose pins the manual suffixes `A`, or
/// `SPI1`, suffixed `B`). No driver uses it yet.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be `{Unit}` `{Role}`",
    label = "no such SPI function on this pin",
    note = "each pin's SPI functions are listed in the `pin_map` table"
)]
pub trait SpiPin<Unit, Role>: PinId {
    /// `PmnPFS.PSEL` value selecting it.
    const PSEL: u8;
}
