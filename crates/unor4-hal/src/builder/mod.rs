//! The builder pattern shared by the HAL's compile-time setups.
//!
//! Every builder slot is [`Missing`] or [`Set<T>`], and a `build()` bounds
//! each required slot on [`Provided<Slot>`]. `Slot` is a marker named after
//! the builder method that fills it ([`slot`]), so a missing one fails
//! `cargo check` with `` setup incomplete: `with_setup` not set ``. The
//! bounds sit on the `build` method, not on its `impl`: on the `impl` a
//! missing slot hides the method (E0599) instead of naming it.

mod missing;
mod provided;
mod set;
pub mod slot;

// ---------------------------------------------------------------------------------------------- //

pub use crate::builder::{missing::Missing, provided::Provided, set::Set};
