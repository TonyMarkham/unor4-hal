//! Wall-clock heartbeat shared by the Slice 5 test binaries.
//!
//! Use the owned SysTick as a free-running 24-bit downcounter, without
//! its interrupt. Unlike counting delay calls, this includes UART ISR
//! and main-loop execution time. Poll at least once per counter wrap
//! (about 349 ms at 48 MHz). Cortex-M4 SysTick behavior: ARMv7-M §B3.3.

use cortex_m::peripheral::SYST;
use unor4_hal::delay::Delay;

pub struct Heartbeat {
    _syst: SYST,
    previous: u32,
    elapsed: u32,
    half_period: u32,
    halves: u32,
}

impl Heartbeat {
    pub fn new(delay: Delay, sysclk: u32) -> Self {
        let mut syst = delay.free();
        syst.disable_counter();
        syst.disable_interrupt();
        syst.set_reload(0x00ff_ffff);
        syst.clear_current();
        syst.enable_counter();
        Self {
            _syst: syst,
            previous: SYST::get_current(),
            elapsed: 0,
            half_period: sysclk / 2,
            halves: 0,
        }
    }

    /// Half-second edge and completed-second count, when due.
    pub fn poll(&mut self) -> Option<(bool, u32)> {
        let now = SYST::get_current();
        self.elapsed += self.previous.wrapping_sub(now) & 0x00ff_ffff;
        self.previous = now;
        if self.elapsed < self.half_period {
            return None;
        }
        self.elapsed -= self.half_period;
        self.halves = self.halves.wrapping_add(1);
        Some((self.halves.is_multiple_of(2), self.halves / 2))
    }
}
