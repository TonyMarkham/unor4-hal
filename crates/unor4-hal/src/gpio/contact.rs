//! Switch contact types: the second parameter of a
//! [`SwitchSetup`](crate::gpio::SwitchSetup) (`SwitchSetup<D2, Nc>`).
//!
//! A switch is wired between its pin and GND with a pull-up (internal or
//! external): the RA4M1's pins have pull-ups only (`PmnPFS.PCR`, manual
//! §19.2.5). So the contact type alone decides which level means
//! "active":
//! - [`No`] (normally open): pressed closes to GND, the pin reads LOW.
//! - [`Nc`] (normally closed): pressed opens, the pull-up wins, the pin
//!   reads HIGH. A broken wire also reads HIGH, so an NC switch fails
//!   safe: the break looks like "active".

use crate::gpio::Sealed;
use embedded_hal::digital::PinState;

/// A switch contact type, [`No`] or [`Nc`]. Sealed.
pub trait Contact: Sealed + Copy {
    /// The pin level while the switch is active (pressed, or tripped).
    const ACTIVE: PinState;
    /// `IRQCRi.IRQMD` for the edge into the active level: falling (00b)
    /// for [`No`], rising (01b) for [`Nc`] (manual §13.2.1).
    const IRQMD: u8;
}

macro_rules! contacts {
    ( $( $ty:ident: $active:ident, $irqmd:literal, $doc:literal; )* ) => {
        $(
            #[doc = $doc]
            #[derive(Clone, Copy, Debug)]
            pub struct $ty;

            impl Sealed for $ty {}

            impl Contact for $ty {
                const ACTIVE: PinState = PinState::$active;
                const IRQMD: u8 = $irqmd;
            }
        )*
    };
}

contacts! {
    No: Low, 0b00, "Normally open: active (pressed) reads LOW.";
    Nc: High, 0b01, "Normally closed: active (pressed, or a broken wire) reads HIGH.";
}
