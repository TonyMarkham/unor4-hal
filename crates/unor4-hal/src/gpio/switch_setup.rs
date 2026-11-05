//! A checked switch input setup.

use crate::gpio::{Pull, SwitchFilter};
use core::marker::PhantomData;

/// A checked switch input on pin `P` with contact type `C`
/// ([`No`](crate::gpio::contact::No) or [`Nc`](crate::gpio::contact::Nc)),
/// made by [`SwitchPinBuilder::build`](crate::gpio::SwitchPinBuilder::build)
/// in a named `const`.
#[derive(Clone, Copy, Debug)]
pub struct SwitchSetup<P, C> {
    pub(crate) pull: Pull,
    pub(crate) filter: Option<SwitchFilter>,
    pub(crate) irq: Option<u8>,
    pub(crate) wake: bool,
    pub(crate) _types: PhantomData<(P, C)>,
}

impl<P, C> SwitchSetup<P, C> {
    /// Where the pull-up comes from.
    pub const fn pull(&self) -> Pull {
        self.pull
    }

    /// The IRQ digital filter, if set.
    pub const fn filter(&self) -> Option<SwitchFilter> {
        self.filter
    }

    /// The pin's `IRQn` when the edge is latched (`with_edge`), `None`
    /// otherwise.
    pub const fn edge_irq(&self) -> Option<u8> {
        self.irq
    }

    /// True when the activation edge wakes the CPU (`with_wake`).
    pub const fn wakes(&self) -> bool {
        self.wake
    }
}
