//! Slice 3: SCI2 polling UART — boot banner + `tick N` once per second
//! on D0/D1, 115200 8N1, to a classic-UNO serial bridge (relay sketch
//! `arduino/sketches/slice03-adapter`; a USB-UART adapter works too).
//!
//! Wiring: bridge D0 <-> this board's D1 (P302, TXD2), bridge D1 <->
//! this board's D0 (P301, RXD2), GND common. Monitor on the bridge at
//! 115200 8N1.
//!
//! Size budget: release `text` = 3,960 bytes (identical on both board
//! maps - D0/D1 are P301/P302 either way). Was 4,084 at slice 3; Slice
//! 5's real 32-entry IELSR vector table replaces cortex-m-rt's generic
//! 240-entry default table and shrinks every binary.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{Peripherals, uart::Uart};

use embedded_hal::delay::DelayNs;
use panic_halt as _;

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // D1 = P302 = TXD2, D0 = P301 = RXD2 — identical on both board maps.
    let mut uart = Uart::sci2(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);

    uart.write_bytes(b"unor4-hal slice 03 - SCI2 polling, 115200 8N1\r\n");

    let mut tick: u32 = 0;
    loop {
        uart.write_bytes(b"tick ");
        print_u32(&mut uart, tick);
        uart.write_bytes(b"\r\n");
        delay.delay_ms(1000);
        tick += 1;
    }
}

/// Appends `value` in decimal to the UART. No `core::fmt` in firmware —
/// this is the same digit loop the course repo's lesson uses.
fn print_u32(uart: &mut Uart, value: u32) {
    let mut digits = [0u8; 10]; // u32::MAX = 4294967295 — 10 digits
    let mut len = 0;
    let mut rest = value;
    loop {
        digits[len] = b'0' + (rest % 10) as u8;
        len += 1;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    for &d in digits[..len].iter().rev() {
        uart.write_byte(d);
    }
}
