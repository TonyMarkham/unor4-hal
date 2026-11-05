//! A checked DIR setup.

use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// A checked DIR setup for pin `P`, made by
/// [`DirPinBuilder::build`](crate::step::DirPinBuilder::build) in a named
/// `const`.
#[derive(Clone, Copy, Debug)]
pub struct DirSetup<P> {
    pub(crate) setup_counts: u32,
    pub(crate) forward: PinState,
    pub(crate) _pin: PhantomData<P>,
}

impl<P> DirSetup<P> {
    /// Time from a DIR change to the next pulse, PCLKD counts.
    pub const fn setup_counts(&self) -> u32 {
        self.setup_counts
    }

    /// The DIR level for positive rates.
    pub const fn forward(&self) -> PinState {
        self.forward
    }
}
