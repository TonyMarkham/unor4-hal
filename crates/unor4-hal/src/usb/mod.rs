//! USB device support: the RA4M1's USB 2.0 Full-Speed module (USBFS) as
//! a [`usb_device::bus::UsbBus`] adapter, plus a blocking CDC-ACM wrapper.
//!
//! Hardware facts: manual §27 (R01UH0887EJ0100); bring-up sequence
//! hardware-proven by the slice 9 probe (`Research/slice-09-research.md`
//! §10 — including the two traps: `MSTPCRB` is PRC1-protected AND needs
//! 32-bit bus access). Cross-check data only: the Arduino core's TinyUSB
//! `rusb2` port and the `ra-hal` embassy port (neither transcribed).
//!
//! # The mux — read before using on a WiFi board
//!
//! The UNO R4 WiFi's USB connector is shared between the ESP32-S3 bridge
//! and the RA4M1 (ABX00087 datasheet §6.2: U2/U6 analog switches, driven
//! by **P408 high** for the RA4M1). Taking the port **disconnects the
//! bridge**: uploads and the SCI9 monitor path die until the connector
//! returns — through [`UsbPort::release`], the opt-in
//! [`CdcAcm::enable_bootloader_knock`], or a double-tap RESET
//! (bootloader hold; observed 2026-10-20).
//! The Minima has no mux — the connector
//! is wired to the RA4M1 — so there [`UsbPort::take`] is a no-op.
//!
//! Per HalSpec §5.5 this is never implicit: taking [`UsbPort`] is the
//! application's own decision, separate from [`Usbfs::new`] (a post-reset window, a button press —
//! owner decision 2026-10-20: the HAL ships mechanisms, never policies).
//!
//! # Scope
//!
//! Device controller mode only (DCFM = 0), full-speed, polled (the
//! application calls `poll` from its own loop; [`UsbWake`] optionally
//! supplies an `intc`-linked interrupt bell). Isochronous endpoints are
//! rejected (no class needs them yet). SOF is not used.

pub mod bus;
pub mod cdc;
pub mod error;
pub mod port;
pub mod wake;

pub use bus::Usbfs;
pub use cdc::CdcAcm;
pub use error::Error;
pub use port::UsbPort;
pub use wake::UsbWake;
/// Mux ownership guard, also available as [`UsbPort`].
pub use port::UsbPort as UsbPortGuard;

/// Explicitly routes the USB connector to the RA4M1.
///
/// WiFi: immediately disconnects the ESP32-S3 bridge, its monitor, and
/// its upload path. Keep the returned guard and detach before releasing
/// it. This does not change SJ1; consult the board documentation for that
/// solder jumper. Minima has no mux. No capture delay is chosen by the HAL.
pub fn take_usb_port(pin: crate::gpio::Pin<'4', 8, crate::gpio::Input>) -> UsbPortGuard {
    UsbPort::take(pin)
}
