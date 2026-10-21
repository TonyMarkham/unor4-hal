//! `unor4-hal` — independent HAL for the Arduino UNO R4 (Renesas RA4M1).
//!
//! Hardware facts in this crate trace to the Renesas RA4M1 hardware manual.
//! Scope and requirements: HalSpec.md.
//!
//! # Board selection
//!
//! Exactly one crate feature selects the pin map: `wifi` (the default —
//! this project's board) or `minima`. Compiling with both, or with
//! neither, is a compile error: a board is a physical fact. Every pin's
//! documentation shows the mapping for *both* boards, so the docs never
//! lie about the board you are not building (HalSpec §5.2).

// `not(test)`: unit tests run on the host, where the test harness needs
// std; the firmware itself does not.
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![allow(clippy::module_inception)]

// HalSpec §3: the two pin maps are mutually exclusive. "Both at once"
// is always a mistake — there is no board with both mappings.
#[cfg(all(feature = "minima", feature = "wifi"))]
compile_error!("`minima` and `wifi` are mutually exclusive — a board is a physical fact, pick one");

// Neither is a mistake too: a silent pinless build would be worse than
// a loud one.
#[cfg(not(any(feature = "minima", feature = "wifi")))]
compile_error!("no board pin map selected — enable `wifi` (default) or `minima`");

pub mod adc;
pub mod clock;
pub mod delay;
pub mod gpio;
pub mod intc;
pub mod peripherals;
pub mod pwm;
pub mod uart;
#[cfg(feature = "usb")]
pub mod usb;

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------------------------- //

pub use peripherals::Peripherals;
