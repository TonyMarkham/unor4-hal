//! The ownership gate: everything claimed exactly once, at startup.

use crate::{clock, delay, gpio, intc};

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
    /// SCI1, unconfigured (the UART driver claims it exactly once) —
    /// the WiFi AT channel: P501/P502, wired to the ESP32-S3's AT
    /// server (bridge firmware, SudoMaker/chAT, 115200 8N1).
    pub sci1: ra4m1::SCI1,
    /// GPT320 — GPT32 channel 0, unconfigured (the PWM driver claims it
    /// exactly once). Header mappings: WiFi **D5** (P107 → GTIOC0A);
    /// Minima **D6** (P106 → GTIOC0B).
    pub gpt320: ra4m1::GPT320,
    /// GPT321 — GPT32 channel 1, unconfigured (the PWM driver claims it
    /// exactly once). Header mappings: WiFi **D3** (P105 → GTIOC1A);
    /// Minima **D3** (P104 → GTIOC1B) and **D11** (P109 → GTIOC1A).
    pub gpt321: ra4m1::GPT321,
    /// GPT163 — GPT16 channel 3, unconfigured (the PWM driver claims it
    /// exactly once). Header mappings: WiFi **D6** (P111 → GTIOC3A);
    /// Minima **D10** (P112 → GTIOC3B).
    pub gpt163: ra4m1::GPT163,
    /// GPT166 — GPT16 channel 6, unconfigured (the PWM driver claims it
    /// exactly once). Header mapping: WiFi **D11** (P411 → GTIOC6A);
    /// no Minima header pin muxes here.
    pub gpt166: ra4m1::GPT166,
    /// GPT167 — GPT16 channel 7, unconfigured (the PWM driver claims it
    /// exactly once). Header mapping: **D9** on both boards (P303 →
    /// GTIOC7B).
    pub gpt167: ra4m1::GPT167,
    /// P501 — SCI1 TX, wired to the ESP32-S3. Not a header pin on the
    /// WiFi board (board-internal to the ESP32-S3); on the Minima the
    /// same chip pin sits on the SWD connector (Arduino core
    /// `variants/MINIMA/variant.cpp`: "(23) TX on SWD connector"), so
    /// unlike the SCI9 pins this field needs no board feature gate.
    pub sci1_tx: gpio::Pin<'5', 1, gpio::Input>,
    /// P502 — SCI1 RX, wired back from the ESP32-S3. Not a header pin
    /// on the WiFi board; on the Minima the same chip pin sits on the
    /// SWD connector ("(24) RX on SWD connector").
    pub sci1_rx: gpio::Pin<'5', 2, gpio::Input>,
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
        // A bootloader jump is not a hardware reset: discard inherited
        // ICU routes before the HAL allocates its own interrupt slots.
        intc::init();
        // Keep the HAL's 32-entry IELSR vector table in the link even
        // for firmware that enables no interrupts (see
        // `intc::vector_table`) — with cortex-m-rt's `device` feature,
        // its absence is a link error, not a silent fallback.
        let _linked = intc::vector_table();
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
            sci1: device.SCI1,
            gpt320: device.GPT320,
            gpt321: device.GPT321,
            gpt163: device.GPT163,
            gpt166: device.GPT166,
            gpt167: device.GPT167,
            sci1_tx: gpio::Pin::new(),
            sci1_rx: gpio::Pin::new(),
            #[cfg(feature = "wifi")]
            sci9_tx: gpio::Pin::new(),
            #[cfg(feature = "wifi")]
            sci9_rx: gpio::Pin::new(),
        })
    }
}
