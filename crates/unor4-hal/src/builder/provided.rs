//! The "slot is set" check.

use crate::{builder::Set, gpio::Sealed};

/// The builder slot named `Slot` ([`slot`](crate::builder::slot)) is set.
/// Only [`Set`] implements it. Sealed.
#[diagnostic::on_unimplemented(
    message = "setup incomplete: `{Slot}` not set",
    label = "`build()` needs `{Slot}`"
)]
pub trait Provided<Slot>: Sealed {
    /// What the slot holds.
    type Value;

    /// Takes the slot's value.
    fn into_value(self) -> Self::Value;
}

impl<Slot, T> Provided<Slot> for Set<T> {
    type Value = T;

    fn into_value(self) -> T {
        self.0
    }
}
