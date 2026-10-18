//! The ownership gate: everything claimed exactly once, at startup.

use crate::{clock, delay, gpio};

/// Claimed peripherals — available exactly once, at startup.
pub struct Peripherals {
    /// Resolved clock frequencies (HOCO 48 MHz).
    pub clocks: clock::Clocks,
    /// Arduino header pins (UNO R4 WiFi map).
    pub pins: gpio::Pins,
    /// SysTick-based blocking delay.
    pub delay: delay::Delay,
}

impl Peripherals {
    /// Takes the peripherals. The cortex-m guard runs first and can only
    /// succeed once — that is what makes the PAC's unguarded `steal()`
    /// below safe.
    pub fn take() -> Option<Self> {
        let core = cortex_m::Peripherals::take()?;
        // Safe: `take()` above succeeded exactly once, so no other caller
        // can reach this line — the single precondition `steal()` demands.
        let device = unsafe { ra4m1::Peripherals::steal() };

        let clocks = clock::init(device.SYSTEM);
        let pins = gpio::Pins::new(
            device.PORT0,
            device.PORT1,
            device.PORT3,
            device.PORT4,
            device.PFS,
            device.PMISC,
        );
        let delay = delay::Delay::new(core.SYST, &clocks);

        Some(Self {
            clocks,
            pins,
            delay,
        })
    }
}
