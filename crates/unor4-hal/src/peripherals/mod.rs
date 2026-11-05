//! The peripherals: every pin and peripheral, handed out once at startup
//! by [`Peripherals::take`].
//!
//! Each header pin, GPT channel and the DTC is a field whose type says
//! whether it is still free. A driver built with `with_peripherals`
//! [`Claim`]s exactly the parts its setup names and returns the rest with
//! those parts [`Taken`]; its `free` [`Release`]s them again. So two
//! drivers can never own the same pin or timer, and a mistake is a
//! compile error naming the part.

mod board;
mod claim;
#[allow(clippy::module_inception)]
mod peripherals;
mod release;
mod taken;

// ---------------------------------------------------------------------------------------------- //

pub use crate::peripherals::{
    board::Board, claim::Claim, peripherals::Peripherals, release::Release, taken::Taken,
};
