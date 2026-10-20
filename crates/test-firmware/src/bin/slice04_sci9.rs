//! Slice 4: SCI9 polling UART — boot banner + `tick N` once per second
//! on the board's internal monitor line (P109 = TX, P110 = RX), 115200
//! 8N1. The ESP32-S3 bridge forwards that line to the board's own USB
//! port, so the ticks appear in `arduino-cli monitor` on `$PORT` with
//! **no adapter and no second board**.
//!
//! Wiring: none — P109/P110 are board-internal (level translator →
//! ESP32-S3). The line is shared with the bootloader's upload path:
//! after this run, flash any sketch again to prove uploading still
//! works (README slice 4).
//!
//! Size budget: release `text` = 3,960 bytes (identical on both board
//! maps - P109/P110 are the same chip pins either way). Was 4,088 at
//! slice 4; Slice 5's real 32-entry IELSR vector table replaces
//! cortex-m-rt's generic 240-entry default table and shrinks every
//! binary.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    uart::{SciChannel, Uart},
};

use embedded_hal::delay::DelayNs;
use panic_halt as _;

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // P109 = TXD9, P110 = RXD9. WiFi map: the board-internal line to the
    // ESP32-S3 bridge (Peripherals hands the pins out directly). Minima
    // map: the same chip pins are header D11/D12, already claimed by
    // `pins` — same types, same constructor.
    #[cfg(feature = "wifi")]
    let mut uart = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut uart = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    uart.write_bytes(b"unor4-hal slice 04 - SCI9 polling, 115200 8N1\r\n");

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
fn print_u32<S: SciChannel>(uart: &mut Uart<S>, value: u32) {
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
