//! SCI9-TXI reproduction and ICU route diagnostic. SCI9 runs in IRQ
//! mode and transmits a 44-byte burst every round. Telemetry uses SCI2
//! polling at 9600 through a 9600-baud UNO relay (not the autonomous
//! slice05-controller, which runs at 115200).
//!
//! Verdict (read T over COM4):
//! - T climbs by 44 per round: SCI9's TXI chain sustains.
//! - T stuck at 1: check for inherited duplicate SCI9 event links.
//!
//! The before/now table snapshots routes after HAL startup, before
//! attachment, and after attachment. Startup must have cleared all slots.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    uart::{SciChannel, Uart, UartIrq},
};

use embedded_hal::delay::DelayNs;
use panic_halt as _;

static UART9: UartIrq<ra4m1::SCI9, 64, 64> = UartIrq::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // Telemetry line: SCI2 polling, TX = P302 (D1, wired to the Uno's
    // D0), RX = P301 (D0, unwired for this test).
    #[cfg(feature = "wifi")]
    let mut term = Uart::sci2(p.sci2, p.pins.d1, p.pins.d0, 9600, &p.clocks);
    #[cfg(feature = "minima")]
    let mut term = Uart::sci2(p.sci2, p.pins.d1, p.pins.d0, 9600, &p.clocks);

    term.write_bytes(b"unor4-hal SCI9-TXI minimal\r\n");

    let icu = unsafe { &*ra4m1::ICU::PTR };
    let inherited: [u32; 32] = core::array::from_fn(|i| icu.ielsr[i].read().bits());

    // The engine under test: SCI9 IRQ mode, monitor line — AT 9600 to
    // test whether the TXI one-shot is baud-configuration-dependent.
    #[cfg(feature = "wifi")]
    UART9.sci9(p.sci9, p.sci9_tx, p.sci9_rx, 9600, &p.clocks);
    #[cfg(feature = "minima")]
    UART9.sci9(p.sci9, p.pins.d11, p.pins.d12, 9600, &p.clocks);

    unsafe { unor4_hal::intc::enable_interrupts() };

    loop {
        // Queued burst, then wait for the interrupt-driven ring to drain.
        UART9.write_all(b"SCI9 IRQ burst: abcdefghijklmnopqrstuvwxyz\r\n");
        UART9.flush();
        delay.delay_ms(100);
        term.write_bytes(b"T ");
        print_u32(&mut term, UART9.txi_events());
        term.write_bytes(b" N ");
        let mut rx_count: u32 = 0;
        while UART9.read_byte().is_some() {
            rx_count += 1;
        }
        print_u32(&mut term, rx_count);
        term.write_bytes(b" P ");
        let ispr0 = unsafe { core::ptr::read_volatile(0xE000E200u32 as *const u32) };
        print_u32(&mut term, ispr0);
        term.write_bytes(b"\r\n");
        for (i, before) in inherited.iter().enumerate() {
            term.write_bytes(b"I ");
            print_u32(&mut term, i as u32);
            term.write_bytes(b" before ");
            print_u32(&mut term, *before);
            term.write_bytes(b" now ");
            print_u32(&mut term, icu.ielsr[i].read().bits());
            term.write_bytes(b"\r\n");
        }
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
