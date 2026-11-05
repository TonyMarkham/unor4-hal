//! A switch input, built from a checked setup.

use crate::{
    gpio::{Input, Pin, PinId, contact::Contact, pin::configure_input},
    intc::Ielsr,
    peripherals::Release,
};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// A switch between a header pin and GND, from a checked
/// [`SwitchSetup`](crate::gpio::SwitchSetup) ([`SwitchPinBuilder`](crate::gpio::SwitchPinBuilder)).
/// The pin is an input with the setup's pull-up, and [`Switch::is_active`]
/// reads it through the contact type `C`. With `with_edge` in the setup,
/// the edge into the active level is also latched in hardware, through the
/// setup's filter, so a press shorter than the time between two polls is
/// not missed ([`Switch::activated`]); the latch is the edge's ICU request
/// flag (`IELSRn.IR`). With `with_wake` as well, the edge also interrupts
/// the CPU, so it ends a `WFI`: firmware can sleep until a switch trips,
/// with no polling. The HAL's dispatcher acknowledges that interrupt and
/// records that it did, so `activated` still reports the edge.
///
/// ```no_run
/// use unor4_hal::{
///     Peripherals, intc,
///     gpio::{Pull, Switch, SwitchPinBuilder, SwitchSetup, contact::Nc, pin_map::{self, D3}},
/// };
/// const LIMIT: SwitchSetup<D3, Nc> = SwitchPinBuilder::new()
///     .with_pin(pin_map::d3)
///     .with_contact(Nc)
///     .with_pull(Pull::Internal)
///     .with_edge()
///     .with_wake()
///     .build();
///
/// let p = Peripherals::take().unwrap();
/// let Ok((mut limit, _p)) = Switch::new(LIMIT).with_peripherals(p).build() else {
///     panic!("IRQ in use, or no free interrupt slot");
/// };
/// unsafe { intc::enable_interrupts() }; // so the HAL acknowledges the wake-up
/// // A switch already pressed at start-up makes no edge: read its level once.
/// let mut tripped = limit.is_active();
/// loop {
///     if !tripped {
///         cortex_m::asm::wfi(); // sleeps until an interrupt: here, the switch
///     }
///     tripped = limit.activated();
///     if tripped {
///         // the firmware decides what a tripped switch means
///     }
/// }
/// ```
pub struct Switch<P, C> {
    pub(crate) edge: Option<Ielsr>,
    pub(crate) _types: PhantomData<(P, C)>,
}

impl<P: PinId, C: Contact> Switch<P, C> {
    /// True while the switch is active: the pin reads the contact type's
    /// active level (LOW for [`No`](crate::gpio::contact::No), HIGH for
    /// [`Nc`](crate::gpio::contact::Nc)). The raw level, unfiltered.
    pub fn is_active(&self) -> bool {
        let high = Pin::<P, Input>::new().read();
        high == (C::ACTIVE == PinState::High)
    }

    /// True while the switch is not active.
    pub fn is_inactive(&self) -> bool {
        !self.is_active()
    }

    /// True if the switch became active since the last call (or since it
    /// was built), and clears the latch. Always false without `with_edge`
    /// in the setup. With `with_wake`, an edge whose interrupt already ran
    /// (and woke the CPU) is reported too.
    pub fn activated(&mut self) -> bool {
        match &self.edge {
            Some(edge) => edge.take_request(),
            None => false,
        }
    }

    /// Takes the switch apart: the ICU slot back to the pool, the pin back
    /// to a plain input (pull-up and `IRQn` off) and back into `p`, so
    /// another driver can claim it.
    pub fn free<X>(self, p: X) -> X::Restored
    where
        X: Release<P, Part = Pin<P, Input>>,
    {
        if let Some(edge) = self.edge {
            edge.release();
        }
        let pin = Pin::<P, Input>::new();
        configure_input(Pin::<P, Input>::new(), false, false);
        p.release(pin)
    }
}
