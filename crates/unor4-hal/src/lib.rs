//! `unor4-hal` — independent HAL for the Arduino UNO R4 (Renesas RA4M1).
//!
//! Hardware facts in this crate trace to the Renesas RA4M1 hardware manual.
//! Scope and requirements: HalSpec.md.

// `not(test)`: unit tests run on the host, where the test harness needs
// std; the firmware itself does not.
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod clock;
pub mod delay;
pub mod gpio;
pub mod peripherals;

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------------------------- //

pub use peripherals::Peripherals;
