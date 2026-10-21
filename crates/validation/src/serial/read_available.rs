//! Non-blocking reads from an open serial port.

use serialport::SerialPort;

/// Reads whatever is available right now into `out`; returns when a read
/// times out or fails.
pub fn read_available(port: &mut dyn SerialPort, out: &mut Vec<u8>) {
    let mut buf = [0u8; 4096];
    loop {
        match port.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
        }
    }
}
