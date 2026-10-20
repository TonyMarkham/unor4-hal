//! SCI2 IRQ diagnostic driven by an external Uno stimulus generator —
//! the RX data path completely bypasses the ESP32-S3 bridge.
//!
//! Engines:
//! - SCI2 in IRQ mode (`UartIrq`): RX = P301 (header D0), TX = P302
//!   (header D1). The Uno's TX feeds D0; received bytes land in the
//!   RXI ring.
//! - SCI9 polling: the telemetry line through the bridge (COM3),
//!   untouched by the interrupt machinery under test.
//!
//! Telemetry per round (SCI9, 115200):
//! - T = SCI2 TXI handler invocations
//! - N = bytes drained from the SCI2 RX ring this round
//! - E = SCI2 receive-error events, O = RX-ring overflows
//! - I0/I1/I2 = IELSR slots 0-2 (the slots `take_any` handed SCI2)

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    uart::{SciChannel, Uart, UartIrq},
};

use embedded_hal::delay::DelayNs;
use panic_halt as _;

static UART2: UartIrq<ra4m1::SCI2, 64, 64> = UartIrq::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // Telemetry line: SCI9 polling, the proven slice-4 path.
    #[cfg(feature = "wifi")]
    let mut term = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut term = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    term.write_bytes(b"unor4-hal SCI2-IRQ/UNO diag\r\n");

    // The experiment: SCI2 in IRQ mode, RX on header D0.
    UART2.sci2(p.sci2, p.pins.d1, p.pins.d0, 9600, &p.clocks);

    unsafe { unor4_hal::intc::enable_interrupts() };

    // Hi-Z the SCI2 TX pin (P302 = header D1) after attach: this diag
    // only receives, and TXD2 idling HIGH as a driven output would fight
    // the Uno's RX-line driver wired to D1 (it flattened the 16U2's
    // start bits and blocked both stimulus and Uno uploads). PmnPFS for
    // P302 = PFS_BASE + 0x40*3 + 4*2; clearing PMR (bit 16) returns the
    // pin to GPIO input. PFS writes need the PMISC.PWPR unlock dance.
    let pfs302 = (0x4004_0800usize + 0x40 * 3 + 4 * 2) as *mut u32;
    let pwpr = 0x4004_0d03usize as *mut u8;
    unsafe {
        core::ptr::write_volatile(pwpr, 0x00); // B0WI = 0
        core::ptr::write_volatile(pwpr, 0x40); // PFSWE = 1
        let v = core::ptr::read_volatile(pfs302);
        core::ptr::write_volatile(pfs302, v & !(1 << 16));
        core::ptr::write_volatile(pwpr, 0x00); // PFSWE = 0
        core::ptr::write_volatile(pwpr, 0x80); // B0WI = 1
    }

    let icu = unsafe { &*ra4m1::ICU::PTR };
    loop {
        // Exercise SCI2's TXI chain: write_all queues 4 bytes; the prime
        // sends the first, the TXI handler should drain the rest. The
        // Uno observes SCI2's TX line and echoes it to COM4.
        UART2.write_all(b"ABCD");

        // Drain the SCI2 RX ring; count what arrived this round.
        let mut rx_count: u32 = 0;
        while UART2.read_byte().is_some() {
            rx_count += 1;
        }
        term.write_bytes(b"T ");
        print_u32(&mut term, UART2.txi_events());
        term.write_bytes(b" N ");
        print_u32(&mut term, rx_count);
        term.write_bytes(b" E ");
        print_u32(&mut term, UART2.errored());
        term.write_bytes(b" O ");
        print_u32(&mut term, UART2.overflowed());
        term.write_bytes(b" I0 ");
        print_u32(&mut term, icu.ielsr[0].read().bits());
        term.write_bytes(b" I1 ");
        print_u32(&mut term, icu.ielsr[1].read().bits());
        term.write_bytes(b" I2 ");
        print_u32(&mut term, icu.ielsr[2].read().bits());
        term.write_bytes(b"\r\n");
        delay.delay_ms(2000);
    }
}

/// Appends `value` in decimal — the same digit loop slice 4 uses.
fn print_u32<S: SciChannel>(term: &mut Uart<S>, value: u32) {
    let mut buf = [0u8; 10];
    let mut i = buf.len();
    let mut v = value;
    if v == 0 {
        buf[i - 1] = b'0';
        i -= 1;
    }
    while v > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    term.write_bytes(&buf[i..]);
}
