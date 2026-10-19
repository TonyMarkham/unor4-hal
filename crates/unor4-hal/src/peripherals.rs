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
    /// SCI2, unconfigured (the UART driver claims it exactly once).
    pub sci2: ra4m1::SCI2,
    /// SCI9, unconfigured (the UART driver claims it exactly once) —
    /// the monitor path: P109/P110, wired to the ESP32-S3 bridge.
    pub sci9: ra4m1::SCI9,
    /// P109 — SCI9 TX, wired through the level translator to the
    /// ESP32-S3. Not a header pin on the WiFi board (on the Minima the
    /// same chip pin is header D11, already claimed by [`gpio::Pins`]
    /// as `d11` — hence this field is WiFi-only).
    #[cfg(feature = "wifi")]
    pub sci9_tx: gpio::Pin<'1', 9, gpio::Input>,
    /// P110 — SCI9 RX, wired back from the ESP32-S3. Not a header pin
    /// on the WiFi board (on the Minima the same chip pin is header
    /// D12, already claimed by [`gpio::Pins`] as `d12`).
    #[cfg(feature = "wifi")]
    pub sci9_rx: gpio::Pin<'1', 10, gpio::Input>,
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
            sci2: device.SCI2,
            sci9: device.SCI9,
            #[cfg(feature = "wifi")]
            sci9_tx: gpio::Pin::new(),
            #[cfg(feature = "wifi")]
            sci9_rx: gpio::Pin::new(),
        })
    }
}
