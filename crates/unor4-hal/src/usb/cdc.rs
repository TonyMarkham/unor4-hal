//! The blocking CDC-ACM wrapper: `usbd-serial` behind a console API.
//!
//! HalSpec §6 gives USB CDC the trait surface the UART module has:
//! [`embedded_io::Read`], [`embedded_io::Write`] and [`core::fmt::Write`].
//! The class logic (line coding, notification endpoint, packet
//! chunking) is `usbd_serial::SerialPort`'s; this wrapper adds the
//! blocking semantics — every call drives [`usb_device::device::UsbDevice::poll`]
//! until it can proceed, so the device never needs a dedicated poll
//! task. For tick/echo loops that must not block on read,
//! [`CdcAcm::read_available`] returns whatever one poll produced.
//!
//! The kernel (`UsbDevice`) lives inside the wrapper; the application
//! only constructs it (see [`CdcAcm::new`]) and calls in. The Arduino
//! bootloader handoff is explicitly opt-in; see
//! [`CdcAcm::enable_bootloader_knock`] for what it does and does not do.

use core::fmt;
use super::{Error, UsbPort, Usbfs};
use embedded_io::{ErrorType, Read, Write};
use usb_device::{UsbError, bus::UsbBus, device::{UsbDevice, UsbDeviceState}};
use usbd_serial::SerialPort;

/// A blocking USB CDC-ACM serial port.
///
/// `B` is the bus adapter ([`super::Usbfs`]); `'a` ties the class and
/// kernel to the caller-owned `UsbBusAllocator`; no allocation or leak
/// is required.
pub struct CdcAcm<'a, B: UsbBus> {
    device: UsbDevice<'a, B>,
    serial: SerialPort<'a, B>,
    bootloader_knock: Option<fn(&B, &mut Option<UsbPort>)>,
    port: Option<UsbPort>,
}

impl<'a, B: UsbBus> CdcAcm<'a, B> {
    /// Puts the kernel inside the wrapper.
    ///
    /// ```no_run
    /// use unor4_hal::usb::{CdcAcm, Usbfs};
    /// use usb_device::{bus::UsbBusAllocator, device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid}};
    /// use usbd_serial::SerialPort;
    /// fn console(alloc: &UsbBusAllocator<Usbfs>) -> CdcAcm<'_, Usbfs> {
    ///     let serial = SerialPort::new(alloc);
    ///     let device = UsbDeviceBuilder::new(alloc, UsbVidPid(0x2341, 0x006D))
    ///         .strings(&[StringDescriptors::default().product("UNO R4")]).unwrap()
    ///         .build();
    ///     CdcAcm::new(serial, device)
    /// }
    /// ```
    pub fn new(serial: SerialPort<'a, B>, device: UsbDevice<'a, B>) -> Self {
        Self { device, serial, bootloader_knock: None, port: None }
    }

    /// Drives the USB stack one poll. Call this from the main loop (or
    /// let the blocking read/write calls drive it for you).
    pub fn poll(&mut self) -> bool {
        let active = self.device.poll(&mut [&mut self.serial]);
        if self.serial.line_coding().data_rate() == 1200 && !self.serial.dtr() {
            if let Some(reboot) = self.bootloader_knock {
                reboot(self.device.bus(), &mut self.port);
            }
        }
        active
    }

    /// The kernel's view of the device state (Default / Addressed /
    /// Configured / Suspend) — the enumeration progress indicator.
    pub fn state(&self) -> UsbDeviceState {
        self.device.state()
    }

    /// Borrows the bus for platform-specific facilities such as [`super::UsbWake`].
    pub fn bus(&self) -> &B {
        self.device.bus()
    }

    /// Writes all of `data`, blocking until the last packet is
    /// accepted by the class's software queue. This is not proof of host
    /// receipt. USB-full-speed bulk packets are ≤ 64 bytes; the loop
    /// feeds the kernel while it waits and returns permanent USB errors.
    pub fn write_all(&mut self, mut data: &[u8]) -> Result<(), Error> {
        while !data.is_empty() {
            self.poll();
            match self.serial.write(data) {
                Ok(0) => {} // poll again — the host is not polling IN yet
                Ok(n) => data = &data[n..],
                Err(UsbError::WouldBlock) => {}
                Err(error) => return Err(error.into()),
            }
        }
        Ok(())
    }

    /// Non-blocking read: drives one poll and returns what arrived
    /// (0 = nothing). The echo/tick loop pattern.
    pub fn read_available(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        if buf.is_empty() { return Ok(0); }
        self.poll();
        match self.serial.read(buf) {
            Ok(n) => Ok(n),
            Err(UsbError::WouldBlock) => Ok(0),
            Err(error) => Err(error.into()),
        }
    }

    /// Polls once and queues as many bytes as the class can accept.
    /// Returns zero when full; unlike blocking Write, never waits for a host.
    pub fn write_available(&mut self, data: &[u8]) -> Result<usize, Error> {
        if data.is_empty() { return Ok(0); }
        self.poll();
        match self.serial.write(data) {
            Ok(n) => Ok(n),
            Err(UsbError::WouldBlock) => Ok(0),
            Err(error) => Err(error.into()),
        }
    }

    /// Blocking read: drives polls until at least one byte arrives.
    fn read_blocking(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        loop {
            self.poll();
            match self.serial.read(buf) {
                Ok(n) => return Ok(n),
                Err(UsbError::WouldBlock) => {}
                Err(error) => return Err(error.into()),
            }
        }
    }
}

impl CdcAcm<'_, Usbfs> {
    /// Opts in to the Arduino upload knock: 1200 baud with DTR low.
    ///
    /// Consumes the explicitly taken mux guard. The next matching line
    /// coding/control state detaches native USB, sets the Arduino
    /// bootloader marker, releases the WiFi mux, and resets the MCU. Never
    /// armed by construction. This is bootloader-specific, not USB CDC
    /// behavior.
    ///
    /// Verified on an UNO R4 WiFi with the bootloader shipped in
    /// ArduinoCore-renesas 1.6.0 (`dfu_wifi.hex`): the bridge port returns
    /// and the bootloader holds. Uploading then takes a **second step** —
    /// a standard `arduino-cli upload` on the bridge port. A single
    /// command aimed at the native port (`upload.wait_for_upload_port`)
    /// fails, because the bridge forwards `bossac` only after its own
    /// 1200-baud touch. UNO R4 Minima: unverified.
    pub fn enable_bootloader_knock(&mut self, port: UsbPort) {
        self.port = Some(port);
        self.bootloader_knock = Some(|bus, port| {
            // Let USBFS finish the host's control status handshake before
            // detaching; line_coding changes during the preceding poll.
            if bus.control_transfer_idle() {
                if let Some(port) = port.take() { bus.reboot_to_bootloader(port); }
            }
        });
    }
}

impl<'a, B: UsbBus> fmt::Write for CdcAcm<'a, B> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

impl<'a, B: UsbBus> ErrorType for CdcAcm<'a, B> {
    type Error = Error;
}

impl<'a, B: UsbBus> Read for CdcAcm<'a, B> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        self.read_blocking(buf)
    }
}

impl<'a, B: UsbBus> Write for CdcAcm<'a, B> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // Drains one packet's worth (or everything, whichever is less),
        // blocking until the adapter accepts it — the trait's "write
        // some bytes" semantics on a blocking port. embedded-io 0.7 forbids
        // Ok(0) for a non-empty buffer (write_all panics on it), so a zero
        // acceptance keeps polling like WouldBlock.
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            self.poll();
            match self.serial.write(buf) {
                Ok(0) | Err(UsbError::WouldBlock) => {}
                Ok(n) => return Ok(n),
                Err(error) => return Err(error.into()),
            }
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        loop {
            self.poll();
            match self.serial.flush() {
                Ok(()) => return Ok(()),
                Err(UsbError::WouldBlock) => {}
                Err(error) => return Err(error.into()),
            }
        }
    }
}
