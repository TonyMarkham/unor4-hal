//! The typestate pin: one struct encoding a pin's physical location and
//! current mode.

use crate::gpio::{Alternate, Analog, Input, InputPullUp, Output, OutputOpenDrain, PinId, Sealed};

use core::{convert::Infallible, marker::PhantomData};
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, PinState, StatefulOutputPin};

/// Base of the `PmnPFS` per-pin register array.
const PFS_BASE: usize = 0x4004_0800;
/// `PWPR` (inside PMISC) — write-protect gate for the PFS registers.
const PWPR: *mut u8 = 0x4004_0d03 as *mut u8;

const PDR: u32 = 1 << 2; // direction (0 = in, 1 = out)
const PCR: u32 = 1 << 4; // input pull-up
const NCODR: u32 = 1 << 6; // N-channel open-drain output
const ISEL: u32 = 1 << 14; // IRQn input (manual §19.2.5, PmnPFS b14)
const ASEL: u32 = 1 << 15; // analog function
const PMR: u32 = 1 << 16; // 0 = GPIO, 1 = peripheral
const PSEL_MASK: u32 = 0x1F << 24; // peripheral select

/// Pin `ID` in mode `MODE`. `ID` is an Arduino header (`pin_map::D6`) or a
/// board-internal pin (`pin_map::UsbMux`); its port and pin index come
/// from [`PinId`], its functions from the capability traits on it. Each
/// pin exists once ([`Pins`](crate::gpio::Pins),
/// [`Peripherals`](crate::Peripherals)).
pub struct Pin<ID, MODE> {
    _types: PhantomData<(ID, MODE)>,
}

impl<ID, MODE> Sealed for Pin<ID, MODE> {}

impl<ID: PinId, MODE> Pin<ID, MODE> {
    pub(crate) const fn new() -> Self {
        Self {
            _types: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn port(&self) -> &'static ra4m1::port0::RegisterBlock {
        port_block(ID::PORT)
    }

    #[inline]
    pub(crate) fn mask(&self) -> u16 {
        1u16 << ID::PIN
    }

    /// Input, no pull.
    pub fn into_input(self) -> Pin<ID, Input> {
        modify_pfs(
            ID::PORT,
            ID::PIN,
            PDR | PCR | NCODR | PMR | ASEL | PSEL_MASK,
            0,
        );
        Pin::new()
    }

    /// Input with pull-up enabled.
    pub fn into_input_pullup(self) -> Pin<ID, InputPullUp> {
        modify_pfs(ID::PORT, ID::PIN, PDR | NCODR | PMR | ASEL | PSEL_MASK, PCR);
        Pin::new()
    }

    /// Push-pull output, starting low.
    pub fn into_output(self) -> Pin<ID, Output> {
        self.into_output_in_state(PinState::Low)
    }

    /// Push-pull output; the latch is written *before* the driver is
    /// enabled, so no glitch escapes during the transition.
    pub fn into_output_in_state(self, state: PinState) -> Pin<ID, Output> {
        match state {
            PinState::High => self.port().posr().write(|w| unsafe { w.bits(self.mask()) }),
            PinState::Low => self.port().porr().write(|w| unsafe { w.bits(self.mask()) }),
        }
        modify_pfs(ID::PORT, ID::PIN, PCR | NCODR | PMR | ASEL | PSEL_MASK, PDR);
        Pin::new()
    }

    /// Open-drain output, released (high) by default.
    pub fn into_output_open_drain(self) -> Pin<ID, OutputOpenDrain> {
        self.into_output_open_drain_in_state(PinState::High)
    }

    /// Open-drain output in the requested state.
    pub fn into_output_open_drain_in_state(self, state: PinState) -> Pin<ID, OutputOpenDrain> {
        match state {
            PinState::High => self.port().posr().write(|w| unsafe { w.bits(self.mask()) }),
            PinState::Low => self.port().porr().write(|w| unsafe { w.bits(self.mask()) }),
        }
        modify_pfs(ID::PORT, ID::PIN, PCR | PMR | ASEL | PSEL_MASK, PDR | NCODR);
        Pin::new()
    }

    /// Analog input (GPIO output stage disabled).
    pub fn into_analog(self) -> Pin<ID, Analog> {
        modify_pfs(ID::PORT, ID::PIN, PDR | PCR | NCODR | PMR | PSEL_MASK, ASEL);
        Pin::new()
    }

    /// Assigns the pin to a peripheral function; `psel` comes from the pin's
    /// capability trait (`Gtioc::PSEL`, `SciPin::PSEL`, …).
    pub(crate) fn into_alternate(self, psel: u8) -> Pin<ID, Alternate> {
        // PSEL first, while PMR is still 0; then enable peripheral mode.
        modify_pfs(
            ID::PORT,
            ID::PIN,
            PCR | NCODR | ASEL | PMR | PSEL_MASK,
            (psel as u32 & 0x1F) << 24,
        );
        modify_pfs(ID::PORT, ID::PIN, 0, PMR);
        Pin::new()
    }
}

/// Makes `pin` a GPIO input with the internal pull-up on or off and the
/// pin's `IRQn` input (`ISEL`) on or off: the switch input's setup.
pub(crate) fn configure_input<ID: PinId>(pin: Pin<ID, Input>, pull_up: bool, irq: bool) {
    let _ = pin;
    let set = if pull_up { PCR } else { 0 } | if irq { ISEL } else { 0 };
    modify_pfs(
        ID::PORT,
        ID::PIN,
        PDR | PCR | NCODR | PMR | ASEL | PSEL_MASK | ISEL,
        set,
    );
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

/// Stamps the input-mode read methods plus the `embedded-hal` traits onto
/// `Pin<.., $mode>`. (Rust has no inheritance; this is the stamp.)
macro_rules! impl_input {
    ($mode:ty) => {
        impl<ID: PinId> Pin<ID, $mode> {
            /// Input level: `true` = high.
            #[inline]
            pub fn read(&self) -> bool {
                self.port().pidr().read().bits() & self.mask() != 0
            }
            /// Whether the pin reads high.
            #[inline]
            pub fn is_high(&self) -> bool {
                self.read()
            }
            /// Whether the pin reads low.
            #[inline]
            pub fn is_low(&self) -> bool {
                !self.read()
            }
        }

        impl<ID: PinId> ErrorType for Pin<ID, $mode> {
            type Error = Infallible;
        }

        impl<ID: PinId> InputPin for Pin<ID, $mode> {
            fn is_high(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<ID, $mode>::is_high(self))
            }
            fn is_low(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<ID, $mode>::is_low(self))
            }
        }
    };
}
impl_input!(Input);
impl_input!(InputPullUp);

/// The same stamp for the output modes.
macro_rules! impl_output {
    ($mode:ty) => {
        impl<ID: PinId> Pin<ID, $mode> {
            /// Drives high (atomic POSR write).
            #[inline]
            pub fn set_high(&mut self) {
                self.port().posr().write(|w| unsafe { w.bits(self.mask()) });
            }
            /// Drives low (atomic PORR write).
            #[inline]
            pub fn set_low(&mut self) {
                self.port().porr().write(|w| unsafe { w.bits(self.mask()) });
            }
            /// Drives to the requested state.
            #[inline]
            pub fn set_state(&mut self, state: PinState) {
                match state {
                    PinState::High => self.set_high(),
                    PinState::Low => self.set_low(),
                }
            }
            /// Current latch value (PODR): `true` = high.
            #[inline]
            pub fn is_set_high(&self) -> bool {
                self.port().podr().read().bits() & self.mask() != 0
            }
            /// Current latch value (PODR): `true` = low.
            #[inline]
            pub fn is_set_low(&self) -> bool {
                self.port().podr().read().bits() & self.mask() == 0
            }
            /// Toggles: PODR read, then atomic POSR/PORR — no
            /// read-modify-write race on the port.
            #[inline]
            pub fn toggle(&mut self) {
                if self.port().podr().read().bits() & self.mask() != 0 {
                    self.set_low();
                } else {
                    self.set_high();
                }
            }
        }

        impl<ID: PinId> ErrorType for Pin<ID, $mode> {
            type Error = Infallible;
        }

        impl<ID: PinId> OutputPin for Pin<ID, $mode> {
            fn set_low(&mut self) -> Result<(), Self::Error> {
                Pin::<ID, $mode>::set_low(self);
                Ok(())
            }
            fn set_high(&mut self) -> Result<(), Self::Error> {
                Pin::<ID, $mode>::set_high(self);
                Ok(())
            }
        }

        impl<ID: PinId> StatefulOutputPin for Pin<ID, $mode> {
            fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<ID, $mode>::is_set_high(self))
            }
            fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                Ok(Pin::<ID, $mode>::is_set_low(self))
            }
            fn toggle(&mut self) -> Result<(), Self::Error> {
                Pin::<ID, $mode>::toggle(self);
                Ok(())
            }
        }
    };
}
impl_output!(Output);
impl_output!(OutputOpenDrain);
