//! The shared parts every `Peripherals` type carries.

use crate::{clock::Clocks, elc::Elc, gpio::Sealed};

/// The parts of [`Peripherals`](crate::Peripherals) that are shared, not
/// claimed: the clock frequencies and the event link. Implemented for
/// every `Peripherals` type, whatever has been claimed. Sealed.
pub trait Board: Sealed {
    /// The resolved clock frequencies.
    fn clocks(&self) -> Clocks;
    /// The event link controller.
    fn elc(&mut self) -> &mut Elc;
}
