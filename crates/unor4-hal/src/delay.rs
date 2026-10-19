//! Cycle-accurate blocking delay on the Cortex-M SysTick.

use crate::clock::Clocks;

use cortex_m::peripheral::{SYST, syst::SystClkSource};
use embedded_hal::delay::DelayNs;

/// Maximum cycles in one SysTick run (2^24). `pub(crate)`: the host unit
/// test in `src/tests/delay.rs` pins this bound.
pub(crate) const MAX_RUN: u32 = 1 << 24;

/// Blocking delay provider.
pub struct Delay {
    syst: SYST,
    sysclk_hz: u32,
}

impl Delay {
    /// Builds the provider; SysTick is clocked from the core clock (ICLK).
    pub fn new(mut syst: SYST, clocks: &Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);
        Self {
            syst,
            sysclk_hz: clocks.sysclk(),
        }
    }

    /// Releases SysTick back to the caller.
    pub fn free(self) -> SYST {
        self.syst
    }

    /// Busy-waits `cycles`, splitting into SysTick runs bounded by the
    /// 24-bit reload limit.
    fn wait_cycles(&mut self, cycles: u64) {
        // Under 2 cycles: function and loop overhead already exceed it,
        // and RELOAD = 0 has no useful wrap to wait for.
        let mut remaining = cycles;
        while remaining >= 2 {
            let run = remaining.min(MAX_RUN as u64) as u32;
            self.syst.set_reload(run - 1); // N cycles need RELOAD = N - 1
            self.syst.clear_current();
            self.syst.enable_counter();
            while !self.syst.has_wrapped() {}
            self.syst.disable_counter();
            remaining -= run as u64;
        }
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.wait_cycles(cycles_for(ns as u64, self.sysclk_hz, 1_000_000_000));
    }

    fn delay_us(&mut self, us: u32) {
        self.wait_cycles(cycles_for(us as u64, self.sysclk_hz, 1_000_000));
    }

    fn delay_ms(&mut self, ms: u32) {
        self.wait_cycles(cycles_for(ms as u64, self.sysclk_hz, 1_000));
    }
}

/// Converts a duration to cycles, rounding up — a delay is *at least* the
/// requested duration.
#[inline]
pub(crate) fn cycles_for(amount: u64, clock_hz: u32, per_second: u64) -> u64 {
    (amount * clock_hz as u64).div_ceil(per_second)
}
