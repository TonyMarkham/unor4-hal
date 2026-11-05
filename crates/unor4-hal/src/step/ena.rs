//! The step generator's driver-enable state.

use core::cell::Cell;
use embedded_hal::digital::PinState;

/// A built generator's ENA pin and lead timing.
pub(crate) struct Ena {
    /// The ENA pin's port `PCNTR3` address (manual §19.2.3).
    pub(crate) pcntr3: u32,
    /// The ENA pin's bit within its port.
    pub(crate) bit: u32,
    /// The level that enables the driver.
    pub(crate) enabled: PinState,
    /// The lead, tick-timer counts (PCLKD/4), rounded up.
    pub(crate) lead_counts: u64,
    /// When the driver was enabled: the tick number's low 16 bits and the
    /// tick timer's count. `None` while disabled.
    pub(crate) since: Option<(u16, u32)>,
    /// The lead has passed since the last enable. A `Cell`, so the
    /// read-only `StepGenerator::is_enabled` can latch it.
    pub(crate) ready: Cell<bool>,
}

impl Ena {
    /// Drives the pin to the enabling level (`on`) or the other one, in
    /// one `PCNTR3` write (bits 15:0 set, bits 31:16 clear).
    pub(crate) fn write(&self, on: bool) {
        let high = (self.enabled == PinState::High) == on;
        let value = if high { self.bit } else { self.bit << 16 };
        // Safety: the pin was claimed by `build`; PCNTR3's set/clear
        // halves touch only this pin's bit.
        unsafe { core::ptr::write_volatile(self.pcntr3 as *mut u32, value) };
    }

    /// The level that disables the driver.
    pub(crate) fn disabled(&self) -> PinState {
        match self.enabled {
            PinState::High => PinState::Low,
            PinState::Low => PinState::High,
        }
    }
}
