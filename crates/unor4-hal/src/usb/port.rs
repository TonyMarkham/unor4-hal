//! The mux port: explicit ownership of the USB connector.
//!
//! WiFi: taking the port drives **P408 high** (variant.cpp pin 21: "USB
//! switch, drive high for RA4"; ABX00087 datasheet §6.2 — U2/U6 analog
//! switches route the connector's D+/D− to the RA4M1). The ESP32-S3
//! bridge loses the connector **the moment this returns**: its COM port
//! drops, and the SCI9 monitor path with it (probe-proven 2026-10-20:
//! the bridge re-enumerates by itself after explicit release). A
//! double-tap RESET (bootloader hold)
//! also returns the connector to the bridge (observed 2026-10-20); after
//! a single reset the application decides again.
//!
//! Before flipping, the core's marker write is replicated:
//! `SYSTEM.VBTBKR[1] = 40` (the Arduino core's `configure_usb_mux` does
//! this). OQ-USB-1, partly answered by disassembling the bootloader that
//! ArduinoCore-renesas 1.6.0 ships for this board (`dfu_wifi.hex`, read
//! back byte-identical from a board's flash): at boot it reads
//! `VBTBKR[1]`; if it is 40 it makes P408 an output, drives it high,
//! delays (argument 1000, unit unverified), drives it low, returns it to
//! input and clears the byte (offsets 0x2160..0x21BC). Behavioral reference
//! only — never transcribed. §11.2.6: VBTBKRn is 8-bit access; §12.1
//! lists it under PRC1 protection.

#[cfg(feature = "wifi")]
use crate::gpio::Output;
use crate::gpio::{Input, Pin};

#[cfg(feature = "wifi")]
use embedded_hal::digital::PinState;

/// `SYSTEM.VBTBKR[1]` (§11.2.6: VBTBKR0 at 0x4001_E500, 8-bit access).
#[cfg(feature = "wifi")]
const VBTBKR1: *mut u8 = 0x4001_E501 as *mut u8;

/// PRC1 unlock for VBTBKR, explicitly listed in manual §12.1.
#[cfg(feature = "wifi")]
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
#[cfg(feature = "wifi")]
const PRCR_LOCK: u16 = 0xA500;

/// The taken USB port.
///
/// Taking this value drives the mux to the RA4M1 (WiFi). Explicit
/// [`Self::release`] returns it to the bridge after the device detaches.
/// Dropping the value alone does not release the hardware. Mux ownership
/// is separate from [`super::Usbfs::new`]. See also SJ1 in the board docs.
pub struct UsbPort {
    #[cfg(feature = "wifi")]
    _mux_pin: Pin<'4', 8, Output>,
}

impl UsbPort {
    /// Routes the connector back to the ESP32-S3 bridge (WiFi).
    ///
    /// Detach the USB device before calling. The native CDC port
    /// disappears. Minima has no mux, so this has no effect there.
    pub fn release(self) {
        #[cfg(feature = "wifi")]
        let _pin = self._mux_pin.into_output_in_state(PinState::Low);
    }

    /// Takes the USB connector for the RA4M1.
    ///
    /// WiFi: writes the core's `VBTBKR[1] = 40` marker (asks the Arduino
    /// bootloader to restore the switch after a reset; see the module
    /// docs), then drives
    /// P408 high. **The bridge disconnects here** — call this when the
    /// application wants the port (a post-reset upload window or a
    /// button gate are application policies; the HAL ships no default).
    ///
    /// Minima: the connector has no switch — this consumes the pin and
    /// returns immediately.
    pub fn take(pin: Pin<'4', 8, Input>) -> Self {
        #[cfg(feature = "wifi")]
        {
            let system = unsafe { &*ra4m1::SYSTEM::PTR };
            system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
            unsafe { core::ptr::write_volatile(VBTBKR1, 40) };
            system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
            UsbPort {
                _mux_pin: pin.into_output_in_state(PinState::High),
            }
        }
        #[cfg(feature = "minima")]
        {
            let _ = pin; // consumed: the connector has no switch to own
            UsbPort {}
        }
    }
}
