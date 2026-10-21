//! VID/PID based serial-port discovery.

use crate::discover::usb_ids::VID_ARDUINO;

use serialport::SerialPortType;
use std::time::{Duration, Instant};

/// Finds the single present COM port matching an Arduino VID/PID pair.
/// Returns None when zero or several match.
#[must_use]
pub fn find_port(pid: u16) -> Option<String> {
    let found: Vec<String> = serialport::available_ports()
        .ok()?
        .into_iter()
        .filter_map(|info| match info.port_type {
            SerialPortType::UsbPort(usb) if usb.vid == VID_ARDUINO && usb.pid == pid => {
                Some(info.port_name)
            }
            _ => None,
        })
        .collect();
    if found.len() == 1 {
        Some(found[0].clone())
    } else {
        None
    }
}

/// Polls [`find_port`] until a port is `present` (or absent) or `seconds`
/// pass.
#[must_use]
pub fn wait_port(pid: u16, present: bool, seconds: u64) -> Option<String> {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < deadline {
        let port = find_port(pid);
        if port.is_some() == present {
            return port;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    None
}
