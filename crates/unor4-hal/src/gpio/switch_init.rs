//! The run-time switch builder: a checked setup plus the peripherals it
//! claims.

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::{
        Input, Pin, PinId, Pull, Switch, SwitchError, SwitchSetup, contact::Contact,
        pin::configure_input,
    },
    intc::{Event, Ielsr},
    peripherals::{Claim, Release},
};
use core::{marker::PhantomData, ptr::write_volatile};

/// `ICU.IRQCRi` (8-bit, manual §13.2.1): `0x4000_6000 + i`.
const IRQCR0: usize = 0x4000_6000;
/// `IRQCRi.FLTEN` (bit 7).
const IRQCR_FLTEN: u8 = 1 << 7;

/// The run-time builder [`Switch::new`] returns: a checked
/// [`SwitchSetup`], then the peripherals to take its pin from.
pub struct SwitchInit<P, C, Pp> {
    setup: SwitchSetup<P, C>,
    peripherals: Pp,
}

impl<P, C> Switch<P, C> {
    /// Starts building a switch from a checked `setup` (a named `const`,
    /// see [`SwitchPinBuilder`](crate::gpio::SwitchPinBuilder)).
    // The entry of the run-time builder (`new(SETUP).with_peripherals(p)
    // .build()`): it returns the builder, and `build` returns the driver.
    #[allow(clippy::new_ret_no_self)]
    pub const fn new(setup: SwitchSetup<P, C>) -> SwitchInit<P, C, Missing> {
        SwitchInit {
            setup,
            peripherals: Missing,
        }
    }
}

impl<P, C> SwitchInit<P, C, Missing> {
    /// The peripherals to claim the setup's pin from.
    pub fn with_peripherals<X>(self, peripherals: X) -> SwitchInit<P, C, Set<X>> {
        SwitchInit {
            setup: self.setup,
            peripherals: Set(peripherals),
        }
    }
}

impl<P, C, Pp> SwitchInit<P, C, Pp> {
    /// Claims the setup's pin and starts the switch: an input with the
    /// pull-up; with `with_edge`, the edge latch too, in the manual's order
    /// (§13.4.4): filter off, the pin as an `IRQn` input (`PmnPFS.ISEL`),
    /// `IRQCRi` (edge, filter clock, filter on), then the ICU slot. Returns
    /// it with the rest of the peripherals; the pin used again elsewhere
    /// fails to compile: `` `D3` is already used by another driver ``.
    ///
    /// # Errors
    /// With `with_edge` only: [`SwitchError::IrqInUse`] when the pin's
    /// `IRQn` is already linked (several headers share one: A3/A5 IRQ2,
    /// A4/D2 IRQ1, A1/D0 IRQ6), or [`SwitchError::NoIcuSlot`] when all 32
    /// slots are taken. Nothing is written, and the peripherals come back
    /// untouched.
    #[allow(clippy::result_large_err)]
    pub fn build<X, R1>(self) -> Result<(Switch<P, C>, R1), (SwitchError, X)>
    where
        P: PinId,
        C: Contact,
        Pp: Provided<slot::with_peripherals, Value = X>,
        X: Claim<P, Part = Pin<P, Input>, Rest = R1>,
        R1: Release<P, Part = Pin<P, Input>, Restored = X>,
    {
        let setup = self.setup;
        let (pin, rest) = self.peripherals.into_value().claim();
        let pull_up = setup.pull == Pull::Internal;
        let Some(irq) = setup.irq else {
            configure_input(pin, pull_up, false);
            return Ok((
                Switch {
                    edge: None,
                    _types: PhantomData,
                },
                rest,
            ));
        };
        // Every `IrqPin` in the pin table has an IRQ the ICU has (no pin has
        // IRQ13; host-tested), so `None` cannot happen.
        let Some(event) = Event::gpio_irq(irq) else {
            return Err((SwitchError::IrqInUse, rest.release(pin)));
        };
        if Ielsr::is_linked(event) {
            return Err((SwitchError::IrqInUse, rest.release(pin)));
        }
        let Some(mut slot) = Ielsr::take_any() else {
            return Err((SwitchError::NoIcuSlot, rest.release(pin)));
        };
        let irqcr = (IRQCR0 + irq as usize) as *mut u8;
        let filter = match setup.filter {
            Some(filter) => (filter.fclksel() << 4) | IRQCR_FLTEN,
            None => 0,
        };
        // Safety: IRQCRi belongs to this pin's IRQn, which no slot links
        // (checked above); 8-bit access per §13.2.1. FCLKSEL before FLTEN
        // (§13.4.3).
        unsafe {
            write_volatile(irqcr, 0);
            configure_input(pin, pull_up, true);
            write_volatile(irqcr, C::IRQMD | (filter & !IRQCR_FLTEN));
            write_volatile(irqcr, C::IRQMD | filter);
        }
        slot.link(event);
        if setup.wake {
            // The edge's request now also interrupts the CPU (and so ends a
            // `WFI`); the HAL's dispatcher acknowledges it and records that
            // it did, for `activated`.
            slot.enable();
        }
        Ok((
            Switch {
                edge: Some(slot),
                _types: PhantomData,
            },
            rest,
        ))
    }
}
