//! The compile-time pulse-counter setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    pwm::GptChannel,
    step::CounterSetup,
};
use core::marker::PhantomData;

/// Builds a [`CounterSetup`]: the GPT channel that counts every completed
/// pulse in hardware, through the ELC. Any channel can count; a 16-bit one
/// is extended to 32 bits in software at every tick (a tick holds at most a
/// few hundred pulses, far below 2¹⁶). `StepSetup` checks it is not a timer
/// the rest of the setup uses.
///
/// ```no_run
/// use unor4_hal::step::{CounterBuilder, CounterSetup};
///
/// const COUNTER: CounterSetup<ra4m1::GPT321> = CounterBuilder::new()
///     .with_timer::<ra4m1::GPT321>()
///     .build();
/// ```
pub struct CounterBuilder<C> {
    _slot: PhantomData<C>,
}

impl CounterBuilder<Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self { _slot: PhantomData }
    }
}

impl Default for CounterBuilder<Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> CounterBuilder<C> {
    /// The counting channel: any GPT channel, e.g. `ra4m1::GPT321`.
    pub const fn with_timer<T: GptChannel>(self) -> CounterBuilder<Set<PhantomData<T>>> {
        CounterBuilder { _slot: PhantomData }
    }

    /// Checks the setup: the channel is set (`` setup incomplete:
    /// `with_timer` not set ``).
    pub const fn build<T>(self) -> CounterSetup<T>
    where
        C: Provided<slot::with_timer, Value = PhantomData<T>>,
        T: GptChannel,
    {
        CounterSetup {
            _timer: PhantomData,
        }
    }
}
