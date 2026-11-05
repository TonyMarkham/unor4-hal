//! A checked STEP (PUL) setup.

use core::marker::PhantomData;

/// A checked STEP setup for pin `P` with divider `D`, made by
/// [`PulPinBuilder::build`](crate::step::PulPinBuilder::build) in a named
/// `const`. Its types tie the rest of the setup and the claimed hardware to the pin
/// and timer it was checked for.
#[derive(Clone, Copy, Debug)]
pub struct PulSetup<P, D> {
    pub(crate) low_counts: u32,
    pub(crate) min_high_counts: u32,
    pub(crate) _types: PhantomData<(P, D)>,
}

impl<P, D> PulSetup<P, D> {
    /// Pulse LOW in STEP-timer counts (PCLKD/divider).
    pub const fn low_counts(&self) -> u32 {
        self.low_counts
    }

    /// Minimum pulse HIGH in STEP-timer counts.
    pub const fn min_high_counts(&self) -> u32 {
        self.min_high_counts
    }
}
