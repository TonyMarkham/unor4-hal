//! What the generator claims for the setup's ENA choice.

use crate::{
    gpio::{Input, Pin, PinId},
    step::NoEna,
};
use embedded_hal::digital::PinState;

/// What the generator claims for the setup's ENA choice `E`: nothing (`()`)
/// for [`NoEna`], the setup's pin otherwise. Sealed.
pub trait EnaPart<E> {
    /// Makes the pin a push-pull output at `level`; nothing without ENA.
    fn into_output(self, level: PinState);
}

impl EnaPart<NoEna> for () {
    fn into_output(self, _level: PinState) {}
}

impl<Id: PinId> EnaPart<Id> for Pin<Id, Input> {
    fn into_output(self, level: PinState) {
        let _ena = self.into_output_in_state(level);
    }
}
