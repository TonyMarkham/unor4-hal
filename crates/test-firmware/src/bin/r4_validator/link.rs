//! The SCI2 request link: TX-driver control and replies.

use crate::{
    LINK,
    body::Body,
    frame::{ETX, STX},
};

use unor4_hal::uart::Uart;

/// The request link baud rate: 9600 8N1 (AGENTS.md bench wiring).
pub(crate) const LINK_BAUD: u32 = 9600;

/// Drives or releases SCI2 TX (P302 = D1 on both maps). Released, the pin
/// is a plain GPIO input and the SCI keeps running internally. Manual §19
/// P302PFS: PMR b16 (peripheral), PDR b2; PWPR unlock (§19.5.1) around
/// the write. Proven on this bench (README, Slice 9: shared-RX limitation).
pub(crate) fn link_tx(enabled: bool) {
    let pfs = 0x4004_08C8 as *mut u32;
    let wanted = if enabled { 1 << 16 } else { 0 };
    if unsafe { pfs.read_volatile() } & 0x0001_0004 == wanted {
        return;
    }
    cortex_m::interrupt::free(|_| unsafe {
        let pwpr = 0x4004_0D03 as *mut u8;
        pwpr.write_volatile(0);
        pwpr.write_volatile(0x40);
        pfs.write_volatile((pfs.read_volatile() & !0x0001_0004) | wanted);
        pwpr.write_volatile(0);
        pwpr.write_volatile(0x80);
    });
}

/// Releases SCI2 TX once the TX ring has drained (SCR.TIE clear) and the
/// last stop bit has left (SSR.TEND), §28.2.
pub(crate) fn release_link_when_idle() {
    let sci = unsafe { &*ra4m1::SCI2::PTR };
    if sci.scr().read().tie().bit_is_clear() && sci.ssr().read().tend().bit_is_set() {
        link_tx(false);
    }
}

/// Sends one framed reply on the link and mirrors it on the bridge
/// monitor, so a PC watching it sees the run.
pub(crate) fn reply(body: &Body, monitor: &mut Uart<ra4m1::SCI9>) {
    link_tx(true);
    LINK.write_all(&[STX]);
    LINK.write_all(body.as_bytes());
    LINK.write_all(&[ETX]);
    monitor.write_bytes(b"R4> ");
    monitor.write_bytes(body.as_bytes());
    monitor.write_bytes(b"\r\n");
}
