//! Host regressions for USB integration and the measured EP0 status bug.

use crate::usb::{CdcAcm, Error, UsbWake, bus::control_status_in};
use core::sync::atomic::{AtomicBool, Ordering};
use embedded_io::{Read, Write};
use usb_device::{
    UsbDirection, UsbError,
    bus::{PollResult, UsbBus, UsbBusAllocator},
    device::{UsbDeviceBuilder, UsbVidPid},
    endpoint::{EndpointAddress, EndpointType},
};
use usbd_serial::SerialPort;

struct MockBus {
    next: [usize; 2],
    enabled: AtomicBool,
    error: UsbError,
}

impl MockBus {
    fn allocator(error: UsbError) -> UsbBusAllocator<Self> {
        UsbBusAllocator::new(Self {
            next: [1, 1],
            enabled: AtomicBool::new(false),
            error,
        })
    }
}

impl UsbBus for MockBus {
    fn alloc_ep(
        &mut self,
        direction: UsbDirection,
        address: Option<EndpointAddress>,
        _: EndpointType,
        _: u16,
        _: u8,
    ) -> Result<EndpointAddress, UsbError> {
        if let Some(address) = address {
            return Ok(address);
        }
        let index = usize::from(direction == UsbDirection::In);
        let address = EndpointAddress::from_parts(self.next[index], direction);
        self.next[index] += 1;
        Ok(address)
    }
    fn enable(&mut self) {
        self.enabled.store(true, Ordering::Relaxed);
    }
    fn reset(&self) {}
    fn set_device_address(&self, _: u8) {}
    fn write(&self, _: EndpointAddress, _: &[u8]) -> Result<usize, UsbError> {
        Err(self.error)
    }
    fn read(&self, _: EndpointAddress, _: &mut [u8]) -> Result<usize, UsbError> {
        Err(self.error)
    }
    fn set_stalled(&self, _: EndpointAddress, _: bool) {}
    fn is_stalled(&self, _: EndpointAddress) -> bool {
        false
    }
    fn suspend(&self) {}
    fn resume(&self) {}
    fn poll(&self) -> PollResult {
        PollResult::None
    }
}

#[test]
fn given_control_out_data_consumed_when_status_queued_before_in_token_then_ccpl_is_used() {
    // given
    let ctsq = 3;
    // when
    let ccpl = control_status_in(ctsq, true, false);
    // then
    assert!(ccpl);
}

#[test]
fn given_control_in_data_stage_when_data_zlp_queued_then_ccpl_is_not_used() {
    // given
    let ctsq = 1;
    // when
    let ccpl = control_status_in(ctsq, true, false);
    // then
    assert!(!ccpl);
}

#[test]
fn given_allocator_when_device_built_then_usb_bus_enable_is_called() {
    // given
    let alloc = MockBus::allocator(UsbError::WouldBlock);
    // when
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    // then
    assert!(device.bus().enabled.load(Ordering::Relaxed));
}

#[test]
fn given_usb_read_failure_when_blocking_read_called_then_error_is_returned() {
    // given
    let alloc = MockBus::allocator(UsbError::BufferOverflow);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    let result = cdc.read(&mut [0; 8]);
    // then
    assert_eq!(result, Err(Error(UsbError::BufferOverflow)));
}

#[test]
fn given_usb_write_failure_when_blocking_write_called_then_error_is_returned() {
    // given
    let alloc = MockBus::allocator(UsbError::InvalidEndpoint);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    let result = cdc.write(b"data");
    // then
    assert_eq!(result, Err(Error(UsbError::InvalidEndpoint)));
}

#[test]
fn given_bus_not_sending_when_trait_write_called_with_data_then_nonzero_count_is_returned() {
    // given
    // A bus that never takes a packet: SerialPort still buffers the bytes.
    let alloc = MockBus::allocator(UsbError::WouldBlock);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    let result = embedded_io::Write::write(&mut cdc, b"data");
    // then
    // embedded-io 0.7: Ok(0) is forbidden for a non-empty buffer.
    assert!(matches!(result, Ok(n) if n > 0));
}

#[test]
fn given_bus_not_sending_when_trait_write_all_called_with_data_then_it_completes() {
    // given
    let alloc = MockBus::allocator(UsbError::WouldBlock);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    // The trait's default write_all, which panics if write returns Ok(0);
    // the inherent CdcAcm::write_all would shadow it in method syntax.
    let result = embedded_io::Write::write_all(&mut cdc, b"data");
    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_no_usb_data_when_nonblocking_read_called_then_zero_is_returned() {
    // given
    let alloc = MockBus::allocator(UsbError::WouldBlock);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    let result = cdc.read_available(&mut [0; 8]);
    // then
    assert_eq!(result, Ok(0));
}

#[test]
fn given_usb_failure_when_formatted_output_called_then_fmt_error_is_returned() {
    // given
    let alloc = MockBus::allocator(UsbError::InvalidEndpoint);
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D)).build();
    let mut cdc = CdcAcm::new(serial, device);
    // when
    let result = core::fmt::Write::write_str(&mut cdc, "data");
    // then
    assert_eq!(result, Err(core::fmt::Error));
}

#[test]
fn given_usb_cdc_type_when_traits_are_required_then_all_spec_traits_are_implemented() {
    // given
    fn requires_traits<T: Read + Write + core::fmt::Write>() {}
    // when
    requires_traits::<CdcAcm<'static, MockBus>>();
    // then
    // Compilation proves HalSpec §6's complete USB CDC trait surface.
}

#[test]
fn given_unattached_usb_bell_when_pending_taken_then_no_event_is_fabricated() {
    // given
    let wake = UsbWake::new();
    // when
    let pending = wake.take_pending();
    // then
    assert!(!pending);
    assert_eq!(wake.events(), 0);
}
