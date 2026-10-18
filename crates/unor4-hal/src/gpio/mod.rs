//! Typestate GPIO for the RA4M1.
//!
//! Hardware facts (Renesas RA4M1 hardware manual, I/O ports chapter) live
//! in [`pin`]; this module wires the types together.

mod alternate;
mod analog;
mod input;
mod input_pull_up;
mod output;
mod output_open_drain;
mod pin;
mod pins;

// `pub(crate)`: the golden-table test (`src/tests/pins.rs`) reads the
// compiled pin map; it is not part of the public API. Host-only —
// firmware never needs it.
#[cfg(test)]
pub(crate) use pins::PORT_PINS;

// ---------------------------------------------------------------------------------------------- //

pub use alternate::Alternate;
pub use analog::Analog;
pub use input::Input;
pub use input_pull_up::InputPullUp;
pub use output::Output;
pub use output_open_drain::OutputOpenDrain;
pub use pin::Pin;
pub use pins::Pins;

// ---------------------------------------------------------------------------------------------- //

use core::convert::Infallible;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

/// Stamps the input-mode read methods plus the `embedded-hal` traits onto
/// `Pin<.., $mode>`. (Rust has no inheritance; this is the stamp.)
macro_rules! impl_input {
    ($mode:ty) => {
        impl<const PORT: char, const PIN: u8> Pin<PORT, PIN, $mode> {
            /// Input level: `true` = high.
            #[inline]
            pub fn read(&self) -> bool {
                self.port().pidr().read().bits() & self.mask() != 0
            }
            /// Whether the pin reads high.
            #[inline]
            pub fn is_high(&self) -> bool {
                self.read()
            }
            /// Whether the pin reads low.
            #[inline]
            pub fn is_low(&self) -> bool {
                !self.read()
            }
        }

        impl<const PORT: char, const PIN: u8> ErrorType for Pin<PORT, PIN, $mode> {
            type Error = Infallible;
        }

        impl<const PORT: char, const PIN: u8> InputPin for Pin<PORT, PIN, $mode> {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<PORT, PIN, $mode>::is_high(self))
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<PORT, PIN, $mode>::is_low(self))
            }
        }
    };
}
impl_input!(Input);
impl_input!(InputPullUp);

/// The same stamp for the output modes.
macro_rules! impl_output {
    ($mode:ty) => {
        impl<const PORT: char, const PIN: u8> Pin<PORT, PIN, $mode> {
            /// Drives high (atomic POSR write).
            #[inline]
            pub fn set_high(&mut self) {
                self.port().posr().write(|w| unsafe { w.bits(self.mask()) });
            }
            /// Drives low (atomic PORR write).
            #[inline]
            pub fn set_low(&mut self) {
                self.port().porr().write(|w| unsafe { w.bits(self.mask()) });
            }
            /// Drives to the requested state.
            #[inline]
            pub fn set_state(&mut self, state: PinState) {
                match state {
                    PinState::High => self.set_high(),
                    PinState::Low => self.set_low(),
                }
            }
            /// Current latch value (PODR): `true` = high.
            #[inline]
            pub fn is_set_high(&self) -> bool {
                self.port().podr().read().bits() & self.mask() != 0
            }
            /// Current latch value (PODR): `true` = low.
            #[inline]
            pub fn is_set_low(&self) -> bool {
                self.port().podr().read().bits() & self.mask() == 0
            }
            /// Toggles: PODR read, then atomic POSR/PORR — no
            /// read-modify-write race on the port.
            #[inline]
            pub fn toggle(&mut self) {
                if self.port().podr().read().bits() & self.mask() != 0 {
                    self.set_low();
                } else {
                    self.set_high();
                }
            }
        }

        impl<const PORT: char, const PIN: u8> ErrorType for Pin<PORT, PIN, $mode> {
            type Error = Infallible;
        }

        impl<const PORT: char, const PIN: u8> OutputPin for Pin<PORT, PIN, $mode> {
            fn set_low(&mut self) -> Result<(), Self::Error> {
                Pin::<PORT, PIN, $mode>::set_low(self);
                Ok(())
            }
            fn set_high(&mut self) -> Result<(), Self::Error> {
                Pin::<PORT, PIN, $mode>::set_high(self);
                Ok(())
            }
        }

        impl<const PORT: char, const PIN: u8> StatefulOutputPin for Pin<PORT, PIN, $mode> {
            fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<PORT, PIN, $mode>::is_set_high(self))
            }
            fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<PORT, PIN, $mode>::is_set_low(self))
            }
            fn toggle(&mut self) -> Result<(), Self::Error> {
                Pin::<PORT, PIN, $mode>::toggle(self);
                Ok(())
            }
        }
    };
}
impl_output!(Output);
impl_output!(OutputOpenDrain);
