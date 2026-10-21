//! Optional USBFS interrupt bell. The consumer still owns USB polling.

use crate::{
    intc::{Event, Ielsr, register_handler},
    usb::{Error, Usbfs},
};

use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering};
use usb_device::UsbError;

// Manual §27.2.7 / §27.3.2: combined USBFS_USBI sources, no SOF/NRDY.
const SOURCES: u16 = (1 << 15) | (1 << 14) | (1 << 12) | (1 << 11) | (1 << 10) | (1 << 8);

/// An optional `intc`-linked bell for a consumer that polls USB itself.
///
/// Declare one static instance. The ISR masks the USB source until
/// [`Self::arm`] is called again; it never accesses endpoint state or
/// runs class callbacks. This avoids an interrupt storm while the main
/// loop services the latched hardware flags.
pub struct UsbWake {
    slot: AtomicU8,
    pending: AtomicBool,
    events: AtomicU32,
}

impl UsbWake {
    /// Creates an unattached bell for a static declaration.
    pub const fn new() -> Self {
        Self {
            slot: AtomicU8::new(u8::MAX),
            pending: AtomicBool::new(false),
            events: AtomicU32::new(0),
        }
    }

    /// Permanently claims an IELSR slot for `Event::UsbfsInt` (0x33).
    ///
    /// Call after bus allocation, before enabling CPU interrupts. Returns
    /// InvalidState if already attached or if no IELSR slot is available.
    /// The application must still poll at least every 10 ms while connected
    /// (usb-device's contract); a bell is not a replacement for polling.
    pub fn attach(&'static self, bus: &Usbfs) -> Result<(), Error> {
        if self.slot.load(Ordering::Acquire) != u8::MAX {
            return Err(UsbError::InvalidState.into());
        }
        let mut route = Ielsr::take_any().ok_or(Error(UsbError::InvalidState))?;
        self.slot.store(route.slot(), Ordering::Release);
        register_handler(route.slot(), wake_isr, self as *const Self as *const ());
        route.link(Event::UsbfsInt);
        bus.prepare_wakeup();
        route.enable();
        self.arm();
        Ok(())
    }

    /// Takes the software bell flag. This does not acknowledge USB packets.
    pub fn take_pending(&self) -> bool {
        self.pending.swap(false, Ordering::AcqRel)
    }

    /// Number of USB bell interrupts observed, wrapping at u32::MAX.
    pub fn events(&self) -> u32 {
        self.events.load(Ordering::Relaxed)
    }

    /// Re-enables USB interrupt sources after the consumer has polled.
    /// Has no effect before attachment. Does not enable CPU interrupts.
    pub fn arm(&self) {
        if self.slot.load(Ordering::Acquire) != u8::MAX {
            critical_section::with(|_| unsafe {
                (&*ra4m1::USBFS::PTR).intenb0.write(|w| w.bits(SOURCES));
            });
        }
    }
}

impl Default for UsbWake {
    fn default() -> Self {
        Self::new()
    }
}

fn wake_isr(context: *const ()) {
    // attach requires a static instance, registered before the route is enabled.
    let wake = unsafe { &*context.cast::<UsbWake>() };
    unsafe {
        (&*ra4m1::USBFS::PTR).intenb0.write(|w| w.bits(0));
    }
    wake.events.fetch_add(1, Ordering::Relaxed);
    wake.pending.store(true, Ordering::Release);
}
