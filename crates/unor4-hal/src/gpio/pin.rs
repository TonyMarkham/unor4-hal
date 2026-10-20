//! The typestate pin: one struct encoding a pin's physical location and
//! current mode.

use crate::gpio::{
    alternate::Alternate, analog::Analog, input::Input, input_pull_up::InputPullUp, output::Output,
    output_open_drain::OutputOpenDrain,
};

use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// Base of the `PmnPFS` per-pin register array.
const PFS_BASE: usize = 0x4004_0800;
/// `PWPR` (inside PMISC) — write-protect gate for the PFS registers.
const PWPR: *mut u8 = 0x4004_0d03 as *mut u8;

const PDR: u32 = 1 << 2; // direction (0 = in, 1 = out)
const PCR: u32 = 1 << 4; // input pull-up
const NCODR: u32 = 1 << 6; // N-channel open-drain output
const ASEL: u32 = 1 << 15; // analog function
const PMR: u32 = 1 << 16; // 0 = GPIO, 1 = peripheral
const PSEL_MASK: u32 = 0x1F << 24; // peripheral select

/// A pin in a specific mode. `PORT` is '0'..'9', `PIN` is 0..15.
pub struct Pin<const PORT: char, const PIN: u8, MODE> {
    _mode: PhantomData<MODE>,
}

impl<const PORT: char, const PIN: u8, MODE> Pin<PORT, PIN, MODE> {
    pub(crate) const fn new() -> Self {
        Self { _mode: PhantomData }
    }

    #[inline]
    pub(crate) fn port(&self) -> &'static ra4m1::port0::RegisterBlock {
        port_block(PORT)
    }

    #[inline]
    pub(crate) fn mask(&self) -> u16 {
        1u16 << PIN
    }

    /// Input, no pull.
    pub fn into_input(self) -> Pin<PORT, PIN, Input> {
        modify_pfs(PORT, PIN, PDR | PCR | NCODR | PMR | ASEL | PSEL_MASK, 0);
        Pin::new()
    }

    /// Input with pull-up enabled.
    pub fn into_input_pullup(self) -> Pin<PORT, PIN, InputPullUp> {
        modify_pfs(PORT, PIN, PDR | NCODR | PMR | ASEL | PSEL_MASK, PCR);
        Pin::new()
    }

    /// Push-pull output, starting low.
    pub fn into_output(self) -> Pin<PORT, PIN, Output> {
        self.into_output_in_state(PinState::Low)
    }

    /// Push-pull output; the latch is written *before* the driver is
    /// enabled, so no glitch escapes during the transition.
    pub fn into_output_in_state(self, state: PinState) -> Pin<PORT, PIN, Output> {
        match state {
            PinState::High => self.port().posr().write(|w| unsafe { w.bits(self.mask()) }),
            PinState::Low => self.port().porr().write(|w| unsafe { w.bits(self.mask()) }),
        }
        modify_pfs(PORT, PIN, PCR | NCODR | PMR | ASEL | PSEL_MASK, PDR);
        Pin::new()
    }

    /// Open-drain output, released (high) by default.
    pub fn into_output_open_drain(self) -> Pin<PORT, PIN, OutputOpenDrain> {
        self.into_output_open_drain_in_state(PinState::High)
    }

    /// Open-drain output in the requested state.
    pub fn into_output_open_drain_in_state(
        self,
        state: PinState,
    ) -> Pin<PORT, PIN, OutputOpenDrain> {
        match state {
            PinState::High => self.port().posr().write(|w| unsafe { w.bits(self.mask()) }),
            PinState::Low => self.port().porr().write(|w| unsafe { w.bits(self.mask()) }),
        }
        modify_pfs(PORT, PIN, PCR | PMR | ASEL | PSEL_MASK, PDR | NCODR);
        Pin::new()
    }

    /// Analog input (GPIO output stage disabled).
    pub fn into_analog(self) -> Pin<PORT, PIN, Analog> {
        modify_pfs(PORT, PIN, PDR | PCR | NCODR | PMR | PSEL_MASK, ASEL);
        Pin::new()
    }

    /// Assigns the pin to a peripheral function; `psel` comes from the
    /// manual's peripheral-select table (the UART driver passes it).
    pub(crate) fn into_alternate(self, psel: u8) -> Pin<PORT, PIN, Alternate> {
        // PSEL first, while PMR is still 0; then enable peripheral mode.
        modify_pfs(
            PORT,
            PIN,
            PCR | NCODR | ASEL | PMR | PSEL_MASK,
            (psel as u32 & 0x1F) << 24,
        );
        modify_pfs(PORT, PIN, 0, PMR);
        Pin::new()
    }
}

#[inline]
fn pfs_addr(port: char, pin: u8) -> *mut u32 {
    let p = (port as u8 - b'0') as usize;
    (PFS_BASE + 0x40 * p + 4 * pin as usize) as *mut u32
}

#[inline]
fn port_block(port: char) -> &'static ra4m1::port0::RegisterBlock {
    let p = (port as u8 - b'0') as usize;
    // All port register blocks share one layout, spaced 0x20 apart.
    unsafe { &*((0x4004_0000 + 0x20 * p) as *const ra4m1::port0::RegisterBlock) }
}

/// Unlocks `PWPR`, read-modify-writes one `PmnPFS`, relocks.
///
/// Keep the shared write-protection window atomic with respect to ISRs.
fn modify_pfs(port: char, pin: u8, clear: u32, set: u32) {
    critical_section::with(|_| unsafe {
        core::ptr::write_volatile(PWPR, 0x00); // B0WI = 0
        core::ptr::write_volatile(PWPR, 0x40); // PFSWE = 1
        let addr = pfs_addr(port, pin);
        let v = core::ptr::read_volatile(addr);
        core::ptr::write_volatile(addr, (v & !clear) | set);
        core::ptr::write_volatile(PWPR, 0x00); // PFSWE = 0
        core::ptr::write_volatile(PWPR, 0x80); // B0WI = 1
    });
}
