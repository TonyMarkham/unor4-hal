//! The explicit connector hand-over to the RA4M1.

use crate::{
    gpio::{Input, Pin, pin_map::UsbMux},
    usb::UsbPort,
};

/// Explicitly routes the USB connector to the RA4M1.
///
/// WiFi: immediately disconnects the ESP32-S3 bridge, its monitor, and
/// its upload path. Keep the returned [`UsbPort`] and release it before
/// handing the connector back. This does not change SJ1; consult the
/// board documentation for that solder jumper. Minima has no mux. No
/// capture delay is chosen by the HAL.
pub fn take_usb_port(pin: Pin<UsbMux, Input>) -> UsbPort {
    UsbPort::take(pin)
}
