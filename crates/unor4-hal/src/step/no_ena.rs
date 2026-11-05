//! The "no ENA pin" marker.

use crate::{
    gpio::Sealed,
    peripherals::{Claim, Release},
    step::{EnaChoice, EnaSetup},
};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// The ENA slot of a [`StepSetup`](crate::step::StepSetup) without an
/// enable pin: the driver's enable input is not wired (or not driven by
/// this board), so the generator is always enabled. The default of
/// `StepSetup`'s last type parameter.
#[derive(Clone, Copy, Debug)]
pub struct NoEna;

impl Sealed for NoEna {}

impl EnaChoice for NoEna {
    type Part = ();
    const PIN: Option<(char, u8)> = None;

    unsafe fn release() {}
}

impl EnaSetup<NoEna> {
    /// The placeholder setup of a `StepSetup` without ENA.
    pub(crate) const NONE: Self = Self {
        enabled: PinState::Low,
        lead_ns: 0,
        _pin: PhantomData,
    };
}

/// Without ENA there is nothing to claim: any peripherals give up `()`.
impl<X> Claim<NoEna> for X {
    type Part = ();
    type Rest = X;

    fn claim(self) -> ((), X) {
        ((), self)
    }
}

/// Without ENA there is nothing to give back.
impl<X> Release<NoEna> for X {
    type Part = ();
    type Restored = X;

    fn release(self, _part: ()) -> X {
        self
    }
}
