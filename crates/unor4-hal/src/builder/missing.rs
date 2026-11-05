//! The unset builder slot.

use crate::gpio::Sealed;

/// A builder slot not set yet. Implements no
/// [`Provided`](crate::builder::Provided), so a `build()` that needs it
/// fails `cargo check` naming the slot.
#[derive(Clone, Copy, Debug)]
pub struct Missing;

impl Sealed for Missing {}
