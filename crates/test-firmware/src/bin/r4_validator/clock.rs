//! A free-running SysTick millisecond clock for the validator.

use cortex_m::peripheral::SYST;
use unor4_hal::{clock::Clocks, delay::Delay};

/// Free-running SysTick as a millisecond clock (24-bit down-counter, no
/// interrupt; ARMv7-M §B3.3). Call [`Clock::now_ms`] at least once per
/// wrap (about 349 ms at 48 MHz); every wait below does.
pub(crate) struct Clock {
    syst: Option<SYST>,
    previous: u32,
    cycles: u64,
    per_ms: u32,
}

impl Clock {
    pub(crate) fn new(syst: SYST, sysclk: u32) -> Self {
        let mut clock = Self {
            syst: None,
            previous: 0,
            cycles: 0,
            per_ms: sysclk / 1000,
        };
        clock.start(syst);
        clock
    }

    fn start(&mut self, mut syst: SYST) {
        syst.disable_interrupt();
        syst.set_reload(0x00ff_ffff);
        syst.clear_current();
        syst.enable_counter();
        self.previous = SYST::get_current();
        self.syst = Some(syst);
    }

    pub(crate) fn now_ms(&mut self) -> u64 {
        let now = SYST::get_current();
        self.cycles += u64::from(self.previous.wrapping_sub(now) & 0x00ff_ffff);
        self.previous = now;
        self.cycles / u64::from(self.per_ms)
    }

    pub(crate) fn wait_ms(&mut self, ms: u32) {
        let until = self.now_ms() + u64::from(ms);
        while self.now_ms() < until {}
    }

    /// Lends SysTick to a real HAL [`Delay`] (Slice 1), then resumes,
    /// crediting the time the delay calls asked for.
    pub(crate) fn with_delay(
        &mut self,
        clocks: &Clocks,
        credit_ms: u32,
        run: impl FnOnce(&mut Delay),
    ) {
        self.now_ms();
        let mut delay = Delay::new(self.syst.take().expect("clock owns SysTick"), clocks);
        run(&mut delay);
        self.start(delay.free());
        self.cycles += u64::from(credit_ms) * u64::from(self.per_ms);
    }
}
