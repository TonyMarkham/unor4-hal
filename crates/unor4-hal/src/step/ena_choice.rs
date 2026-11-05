//! What a step setup's ENA slot holds: a pin, or [`NoEna`](crate::step::NoEna).

use crate::gpio::{Input, Pin, PinId, Sealed};

/// The ENA slot's type: any pin id, or [`NoEna`](crate::step::NoEna).
/// Sealed.
pub trait EnaChoice: Sealed {
    /// What [`StepGenerator::free`](crate::step::StepGenerator::free) gives
    /// back for the slot: the pin in input mode, or `()`.
    type Part;

    /// The pin's port digit and index, `None` without ENA.
    const PIN: Option<(char, u8)>;

    /// The slot's part, out of thin air.
    ///
    /// # Safety
    /// Only for the generator's `free`, which claimed the pin in
    /// `build`: at most one token per pin may exist.
    #[doc(hidden)]
    unsafe fn release() -> Self::Part;
}

impl<Id: PinId> EnaChoice for Id {
    type Part = Pin<Id, Input>;
    const PIN: Option<(char, u8)> = Some((Id::PORT, Id::PIN));

    unsafe fn release() -> Pin<Id, Input> {
        Pin::<Id, Input>::new().into_input()
    }
}
