//! The compile-time tick setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    pwm::{GptChannel, Width},
    step::{PCLKD_HZ, TICK_DIVISOR, TICK_US, TickSetup},
};
use core::marker::PhantomData;

/// Builds a [`TickSetup`]: the channel that makes the 2 ms tick (it counts
/// PCLKD/4 and overflows every 2 ms; its overflow latches the pulse count
/// and the STEP phase and starts the DTC) and the channel that numbers the
/// ticks. Any two different channels, 16- or 32-bit.
///
/// ```no_run
/// use unor4_hal::step::{TickBuilder, TickSetup};
///
/// const TICK: TickSetup<ra4m1::GPT164, ra4m1::GPT165> = TickBuilder::new()
///     .with_timers::<ra4m1::GPT164, ra4m1::GPT165>()
///     .build();
/// ```
///
/// Checks, all failing `cargo check`: the timers are set; they differ; the
/// tick is a whole number of PCLKD/4 counts that fits the tick timer.
pub struct TickBuilder<S> {
    _slot: PhantomData<S>,
}

impl TickBuilder<Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self { _slot: PhantomData }
    }
}

impl Default for TickBuilder<Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> TickBuilder<S> {
    /// The tick channel `T` and the tick-number channel `N`, e.g.
    /// `with_timers::<ra4m1::GPT164, ra4m1::GPT165>()`.
    pub const fn with_timers<T: GptChannel, N: GptChannel>(
        self,
    ) -> TickBuilder<Set<PhantomData<(T, N)>>> {
        TickBuilder { _slot: PhantomData }
    }

    /// Checks the setup (see the type's docs).
    pub const fn build<T, N>(self) -> TickSetup<T, N>
    where
        S: Provided<slot::with_timers, Value = PhantomData<(T, N)>>,
        T: GptChannel,
        N: GptChannel,
    {
        assert!(
            T::CHANNEL != N::CHANNEL,
            "TickBuilder: the tick and tick-number timers must differ"
        );
        let tick_counts = PCLKD_HZ as u64 * TICK_US as u64 / 1_000_000;
        assert!(
            tick_counts.is_multiple_of(TICK_DIVISOR as u64),
            "TickBuilder: PCLKD gives no whole 2 ms tick at PCLKD/4"
        );
        assert!(
            tick_counts / TICK_DIVISOR as u64 - 1 <= <T::Width as Width>::MAX_GTPR as u64,
            "TickBuilder: the 2 ms tick does not fit the tick timer"
        );
        TickSetup {
            _timers: PhantomData,
        }
    }
}
