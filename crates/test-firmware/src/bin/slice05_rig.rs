//! Automated Slice 5 regression using an UNO R3 controller.
//!
//! D0/D1 are crossed, D3 is connected straight through, GND is common.
//! Both SCI2 (header) and SCI9 (USB bridge) use interrupts at 115200 8N1.
//! The UNO drives D3 low during each 4096-byte full-duplex test. D3 high
//! disables header echo so the UNO can report results without feedback.
//! The host independently checks paced SCI9 echo and both UART counters.

#![no_std]
#![no_main]

use core::fmt::Write;
use panic_halt as _;
use unor4_hal::{Peripherals, intc, uart::UartIrq};

#[path = "../heartbeat.rs"]
mod heartbeat;

// Exercise the public defaults (256 RX / 128 TX), not enlarged rings.
static UART2: UartIrq<ra4m1::SCI2> = UartIrq::new();
static UART9: UartIrq<ra4m1::SCI9> = UartIrq::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gate = p.pins.d3.into_input_pullup();
    let mut led = p.pins.d13.into_output();
    let icu = unsafe { &*ra4m1::ICU::PTR };
    // Hardware regression: no bootloader routes may survive ownership
    // handoff, otherwise SCI9 can become one-shot again.
    let stale = icu.ielsr.iter().filter(|r| r.read().bits() != 0).count();
    assert_eq!(stale, 0);

    UART2.sci2(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
    #[cfg(feature = "wifi")]
    UART9.sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    UART9.sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    let routes: [u32; 32] = core::array::from_fn(|i| icu.ielsr[i].read().bits() & 0xff);
    let active = routes.iter().filter(|&&r| r != 0).count();
    let duplicates = routes
        .iter()
        .enumerate()
        .filter(|&(i, &r)| r != 0 && routes[..i].contains(&r))
        .count();
    assert_eq!(active, 6);
    assert_eq!(duplicates, 0);
    // This is main startup, not a critical section.
    unsafe { intc::enable_interrupts() };

    let mut heartbeat = heartbeat::Heartbeat::new(p.delay, p.clocks.sysclk());
    let mut count2 = 0u32;
    let mut count9 = 0u32;
    let mut pending2 = None;
    let mut pending9 = None;
    loop {
        if gate.is_low() {
            if pending2.is_none() {
                pending2 = UART2.read_byte().map(|b| {
                    count2 += 1;
                    b.to_ascii_uppercase()
                });
            }
            if let Some(b) = pending2
                && UART2.write(&[b]) == 1
            {
                pending2 = None;
            }
        } else {
            pending2 = None;
            // The UNO prints its summary on the same wire as stimulus.
            // Consume it without echoing it back into the controller.
            while UART2.read_byte().is_some() {}
        }
        if pending9.is_none() {
            pending9 = UART9.read_byte().map(|b| {
                count9 += 1;
                b.to_ascii_uppercase()
            });
        }
        if let Some(b) = pending9
            && UART9.write(&[b]) == 1
        {
            pending9 = None;
        }
        if let Some((second, tick)) = heartbeat.poll() {
            led.toggle();
            if second {
                let _ = writeln!(
                    &mut &UART9,
                    "\r\ntick={} n2={} tx2={} ov2={} er2={} n9={} tx9={} ov9={} er9={} stale={} routes={} duplicates={}\r",
                    tick,
                    count2,
                    UART2.txi_events(),
                    UART2.overflowed(),
                    UART2.errored(),
                    count9,
                    UART9.txi_events(),
                    UART9.overflowed(),
                    UART9.errored(),
                    stale,
                    active,
                    duplicates
                );
            }
        }
    }
}
