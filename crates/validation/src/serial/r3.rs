//! The R3 validator link (SCI2 between the boards, 9600 8N1).

use crate::serial::{frames::strip_frames, monitor::Monitor, read_available::read_available};

use regex::Regex;
use serialport::SerialPort;
use std::{
    io::Write as _,
    time::{Duration, Instant},
};

/// An open R3 validator port.
pub struct R3 {
    name: String,
    port: Box<dyn SerialPort>,
}

impl R3 {
    /// Opens the R3 port at 9600 8N1, DTR deasserted on open.
    ///
    /// # Errors
    /// When the port cannot be opened.
    pub fn open(name: &str) -> Result<Self, serialport::Error> {
        let port = serialport::new(name, 9_600)
            .timeout(Duration::from_millis(10))
            .dtr_on_open(false)
            .open()?;
        Ok(Self {
            name: name.to_owned(),
            port,
        })
    }

    /// The port's COM name, for messages.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Drains and returns everything currently buffered from the R3.
    #[must_use]
    pub fn drain(&mut self) -> String {
        let mut bytes = Vec::new();
        read_available(self.port.as_mut(), &mut bytes);
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// Sends one R3 command and returns its text, frames removed, once
    /// `until` (a regex over the stripped text) matches or `seconds` pass.
    /// Monitor text, when a monitor is supplied, accumulates in the monitor
    /// for the same period.
    #[must_use]
    pub fn send(
        &mut self,
        command: &str,
        until: &str,
        seconds: u64,
        mut monitor: Option<&mut Monitor>,
    ) -> String {
        let until = Regex::new(until).ok();
        let _ = self.drain();
        let _ = self.port.write_all(command.as_bytes());
        let _ = self.port.flush();
        let deadline = Instant::now() + Duration::from_secs(seconds);
        let mut text = String::new();
        while Instant::now() < deadline {
            text.push_str(&self.drain());
            if let Some(m) = monitor.as_mut() {
                m.drain();
            }
            if let Some(re) = &until
                && re.is_match(&strip_frames(&text))
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        strip_frames(&text)
    }

    /// Sends 'i' and reports whether the validator answered its identify
    /// line.
    #[must_use]
    pub fn identify_answered(&mut self) -> bool {
        self.send("i", r"(?m)^ID .*\r\n", 3, None)
            .contains("ID ID VALIDATOR 1")
    }
}
