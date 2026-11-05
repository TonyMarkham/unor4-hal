//! A checked ENA (driver enable) setup.

use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// A checked ENA setup for pin `P`, made by
/// [`EnaPinBuilder::build`](crate::step::EnaPinBuilder::build) in a named
/// `const`. A [`StepSetup`](crate::step::StepSetup) without one holds
/// `EnaSetup<NoEna>` ([`NoEna`](crate::step::NoEna)).
#[derive(Clone, Copy, Debug)]
pub struct EnaSetup<P> {
    pub(crate) enabled: PinState,
    pub(crate) lead_ns: u32,
    pub(crate) _pin: PhantomData<P>,
}

impl<P> EnaSetup<P> {
    /// The pin level that enables the driver.
    pub const fn enabled(&self) -> PinState {
        self.enabled
    }

    /// The shortest time from enabling to the first DIR change or pulse, ns.
    pub const fn lead_ns(&self) -> u32 {
        self.lead_ns
    }
}
