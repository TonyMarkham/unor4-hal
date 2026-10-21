//! The bridge serial monitor (SCI9 forwarded by the ESP32-S3 to USB).

use crate::serial::read_available::read_available;

use serialport::SerialPort;
use std::time::Duration;

/// An open bridge monitor port.
pub struct Monitor {
    port: Box<dyn SerialPort>,
    text: String,
}

impl Monitor {
    /// Opens the bridge port at 115200 8N1 with DTR asserted: the bridge
    /// forwards SCI9 only to an open, DTR-asserted port.
    ///
    /// # Errors
    /// When the port cannot be opened.
    pub fn open(name: &str) -> Result<Self, serialport::Error> {
        let port = serialport::new(name, 115_200)
            .timeout(Duration::from_millis(10))
            .dtr_on_open(true)
            .open()?;
        Ok(Self {
            port,
            text: String::new(),
        })
    }

    /// Drains everything currently available from the bridge into the
    /// buffer.
    pub fn drain(&mut self) {
        let mut bytes = Vec::new();
        read_available(self.port.as_mut(), &mut bytes);
        self.text.push_str(&String::from_utf8_lossy(&bytes));
    }

    /// Everything accumulated so far.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}
