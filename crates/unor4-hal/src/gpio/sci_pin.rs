//! The SCI signal capability.

use crate::gpio::PinId;

/// The pin carries signal `Role` ([`role`](crate::gpio::role)) of SCI
/// channel `Unit` (the PAC's `SCI2`, `SCI9`, …). Bounded on by
/// [`Uart`](crate::uart::Uart).
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be `{Unit}` `{Role}`",
    label = "no such SCI function on this pin",
    note = "each pin's SCI functions are listed in the `pin_map` table"
)]
pub trait SciPin<Unit, Role>: PinId {
    /// `PmnPFS.PSEL` value selecting it. It varies by pin, not only by
    /// channel: SCI2 `TXD2` is `00101b` on P102 but `00100b` on P302
    /// (Tables 19.6, 19.10).
    const PSEL: u8;
}
