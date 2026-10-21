//! Serial links to the bench: the R3 validator port and the bridge monitor.

mod frames;
mod monitor;
mod r3;
mod read_available;

// ---------------------------------------------------------------------------------------------- //

pub use crate::serial::{monitor::Monitor, r3::R3};

#[cfg(test)]
pub(crate) use crate::serial::frames::strip_frames;
