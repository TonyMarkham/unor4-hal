//! The compile-time wake-timer setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    elc::ElcInput,
    pwm::GptChannel,
    wake::WakeSetup,
};
use core::marker::PhantomData;

/// Builds a [`WakeSetup`]: the GPT channel that times the waits and the
/// ELC input its overflow stops it through. Write it as a named `const`:
///
/// ```no_run
/// use unor4_hal::{elc::ElcE, wake::{WakeSetup, WakeTimerBuilder}};
///
/// const WAKE: WakeSetup<ra4m1::GPT167, ElcE> = WakeTimerBuilder::new()
///     .with_timer::<ra4m1::GPT167>()
///     .with_input::<ElcE>()
///     .build();
/// ```
///
/// Both are required: a missing one fails `cargo check` naming it
/// (`` setup incomplete: `with_input` not set ``). Which channel and input
/// are free is checked when the timer claims them
/// ([`WakeTimer::new`](crate::wake::WakeTimer::new)): one another driver
/// holds fails to compile, naming it.
pub struct WakeTimerBuilder<T, I> {
    _timer: PhantomData<T>,
    _input: PhantomData<I>,
}

impl WakeTimerBuilder<Missing, Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self {
            _timer: PhantomData,
            _input: PhantomData,
        }
    }
}

impl Default for WakeTimerBuilder<Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, I> WakeTimerBuilder<T, I> {
    /// The timing channel: any GPT channel, e.g. `ra4m1::GPT167`. A 16-bit
    /// channel reaches 1.398 s, a 32-bit one 25 h (at PCLKD/1024).
    pub const fn with_timer<C: GptChannel>(self) -> WakeTimerBuilder<Set<PhantomData<C>>, I> {
        WakeTimerBuilder {
            _timer: PhantomData,
            _input: PhantomData,
        }
    }

    /// The ELC input the channel's overflow stops it through: any of
    /// [`ElcA`](crate::elc::ElcA) … [`ElcH`](crate::elc::ElcH) no other
    /// driver claims (the step generator claims B, C and D).
    pub const fn with_input<E: ElcInput>(self) -> WakeTimerBuilder<T, Set<PhantomData<E>>> {
        WakeTimerBuilder {
            _timer: PhantomData,
            _input: PhantomData,
        }
    }

    /// Checks the setup: the channel and the input are set.
    pub const fn build<C, E>(self) -> WakeSetup<C, E>
    where
        T: Provided<slot::with_timer, Value = PhantomData<C>>,
        I: Provided<slot::with_input, Value = PhantomData<E>>,
        C: GptChannel,
        E: ElcInput,
    {
        WakeSetup {
            _timer: PhantomData,
            _input: PhantomData,
        }
    }
}
