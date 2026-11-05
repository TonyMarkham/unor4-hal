//! `unor4-hal` — independent HAL for the Arduino UNO R4 (Renesas RA4M1).
//!
//! Hardware facts in this crate trace to the Renesas RA4M1 hardware
//! manual; each module's docs state its facts, their manual sections,
//! and what the module deliberately leaves out.
//!
//! # Board selection
//!
//! Exactly one crate feature selects the pin map: `wifi` (the default —
//! this project's board) or `minima`. Compiling with both, or with
//! neither, is a compile error: a board is a physical fact. The pin table
//! ([`gpio::pin_map`]) lists both boards' headers per port pin; a build
//! compiles the selected board's.
//!
//! # Pins
//!
//! Pins are named by their Arduino header (`Pin<D6, Output>`), and what
//! each can do is one table, [`gpio::pin_map`]. Drivers check their pins
//! with trait bounds, and setups with compile-time builders written as
//! named `const` items ([`builder`], [`step`]), so a wrong pin or a bad
//! number fails `cargo check`.
//!
//! # Where to start
//!
//! - A stepper motor (STEP/DIR/ENA): the [`step`] module docs, which cover
//!   what each part supports, what to call per 2 ms tick, every error, and
//!   a complete example.
//! - Limit switches and buttons: the switch section of the [`gpio`] module
//!   docs.
//! - A timed wait that wakes a sleeping CPU (a switch re-check, a timeout):
//!   the [`wake`] module docs.
//! - Everything starts from [`Peripherals::take`], which hands out every
//!   pin and peripheral once.

// `not(test)`: unit tests run on the host, where the test harness needs
// std; the firmware itself does not.
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]
#![allow(clippy::module_inception)]

// The two pin maps are mutually exclusive. "Both at once"
// is always a mistake — there is no board with both mappings.
#[cfg(all(feature = "minima", feature = "wifi"))]
compile_error!("`minima` and `wifi` are mutually exclusive — a board is a physical fact, pick one");

// Neither is a mistake too: a silent pinless build would be worse than
// a loud one.
#[cfg(not(any(feature = "minima", feature = "wifi")))]
compile_error!("no board pin map selected — enable `wifi` (default) or `minima`");

pub mod adc;
pub mod builder;
pub mod clock;
pub mod delay;
pub mod dflash;
pub mod elc;
pub mod gpio;
pub mod intc;
pub mod peripherals;
pub mod pwm;
pub mod step;
pub mod uart;
#[cfg(feature = "usb")]
pub mod usb;
pub mod wake;

#[cfg(test)]
mod tests;

/// The repository README's examples, compiled as doc tests (`cargo test
/// --doc`) with the feature set they need (WiFi pins, `usb`, `uart-irq`),
/// so the user guide cannot drift from the API. Only seen by rustdoc's
/// doctest collection: normal and published builds never read the file.
#[cfg(all(doctest, feature = "wifi", feature = "usb", feature = "uart-irq"))]
#[doc = include_str!("../../../README.md")]
struct ReadmeExamples;

// ---------------------------------------------------------------------------------------------- //

pub use peripherals::Peripherals;
