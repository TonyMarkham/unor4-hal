//! The compile-time DIR setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::{HeaderPin, PinId},
    step::{DirSetup, counts_ceil},
};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// The longest DIR set-up time [`DirPinBuilder`] accepts, ns: 1 ms, half
/// the 2 ms tick.
const MAX_DIR_SETUP_NS: u32 = 1_000_000;

/// Builds a [`DirSetup`]: the DIR pin, its set-up time and the level for
/// positive rates. Write it as a named `const`, like the
/// [`PulPinBuilder`](crate::step::PulPinBuilder):
///
/// ```no_run
/// use embedded_hal::digital::PinState;
/// use unor4_hal::{gpio::pin_map::{self, D5}, step::{DirPinBuilder, DirSetup}};
///
/// const DIR: DirSetup<D5> = DirPinBuilder::new()
///     .with_pin(pin_map::d5)          // any header pin
///     .with_setup(5_000)              // 5 µs from a DIR change to the next pulse
///     .with_forward(PinState::High)   // the level for positive rates
///     .build();
/// ```
///
/// Checks, all failing `cargo check`: the pin is an Arduino header, not a
/// board-internal pin (`` `UsbMux` is a board-internal pin, not an Arduino
/// header ``); every part set (`` setup incomplete: `with_setup` not set
/// ``); the set-up time is at least one PCLKD count
/// (21 ns) and at most 1 ms: `` with_setup: DIR set-up time over 1 ms ``.
/// The generator waits it after every DIR change: a start through
/// [`StepGenerator::run`](crate::step::StepGenerator::run) waits it before
/// the first pulse, and that first pulse must come before the next 2 ms
/// tick but one, which 1 ms leaves room for. Stepper drivers need
/// microseconds (5 µs for the DM542T, per copies of its manual).
///
/// Where the generator waits it:
/// - a reversal while pulses run: DIR is written in a window after the
///   tick that depends on the pulse LOW time, and the next pulse comes
///   this set-up time after the window's latest edge;
/// - a start or a direction change from stopped or idle through
///   [`StepGenerator::queue`](crate::step::StepGenerator::queue): DIR is
///   written at least 40 µs before the tick, and the first pulse comes at
///   least this set-up time after that write;
/// - a start through [`StepGenerator::run`](crate::step::StepGenerator::run)
///   that changes the direction: the first pulse comes this set-up time
///   after the DIR write, or the minimum HIGH time if that is longer.
pub struct DirPinBuilder<P, S, F> {
    setup_ns: u32,
    forward: PinState,
    _slots: PhantomData<(P, S, F)>,
}

impl DirPinBuilder<Missing, Missing, Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self {
            setup_ns: 0,
            forward: PinState::High,
            _slots: PhantomData,
        }
    }
}

impl Default for DirPinBuilder<Missing, Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, S, F> DirPinBuilder<P, S, F> {
    /// The DIR pin: any header from [`pin_map`](crate::gpio::pin_map),
    /// e.g. `pin_map::d5`.
    pub const fn with_pin<Id: HeaderPin>(self, _pin: Id) -> DirPinBuilder<Set<Id>, S, F> {
        DirPinBuilder {
            setup_ns: self.setup_ns,
            forward: self.forward,
            _slots: PhantomData,
        }
    }

    /// The shortest time from a DIR change to the next STEP pulse, ns
    /// (rounded up to PCLKD counts; 21 ns to 1 ms).
    pub const fn with_setup(self, setup_ns: u32) -> DirPinBuilder<P, Set<()>, F> {
        DirPinBuilder {
            setup_ns,
            forward: self.forward,
            _slots: PhantomData,
        }
    }

    /// The DIR level for positive rates.
    pub const fn with_forward(self, forward: PinState) -> DirPinBuilder<P, S, Set<()>> {
        DirPinBuilder {
            setup_ns: self.setup_ns,
            forward,
            _slots: PhantomData,
        }
    }

    /// Checks the setup (see the type's docs).
    pub const fn build<Id>(self) -> DirSetup<Id>
    where
        P: Provided<slot::with_pin, Value = Id>,
        S: Provided<slot::with_setup>,
        F: Provided<slot::with_forward>,
        Id: PinId + HeaderPin,
    {
        let counts = counts_ceil(self.setup_ns, 1);
        assert!(
            counts >= 1,
            "with_setup: DIR set-up time under 1 PCLKD count (21 ns)"
        );
        assert!(
            self.setup_ns <= MAX_DIR_SETUP_NS,
            "with_setup: DIR set-up time over 1 ms (the first pulse after a DIR \
             change must come within the next 2 ms tick)"
        );
        DirSetup {
            setup_counts: counts as u32,
            forward: self.forward,
            _pin: PhantomData,
        }
    }
}
