//! RX diagnostic: pure polled SCI9 receive. No interrupts, no IELSR —
//! the exact TX-proven slice-4 path plus a read loop. Anything typed in
//! the monitor must echo back character-for-character. Receiver error
//! flags return as single marker characters (O = overrun, F = framing,
//! P = parity). One dot per second proves the loop is alive.
//!
//! Verdicts:
//! - echo works → the host→board RX path and the HAL's RX config are
//!   fine, and the IRQ diag's empty RX ring is the puzzle.
//! - silence (no echo, no markers) → the host→board path itself is dead
//!   (monitor, bridge, or RX pin mux).
//! - O/F markers → signal problem (baud, noise).

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    uart::{ReadError, Uart},
};

use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // P109 = TXD9, P110 = RXD9 (WiFi: board-internal line to the ESP32-S3
    // bridge; Minima: header D11/D12).
    #[cfg(feature = "wifi")]
    let mut uart = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut uart = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    uart.write_bytes(b"unor4-hal RX POLL - type slowly, it echoes\r\n");

    loop {
        match uart.read_byte() {
            Ok(b) => uart.write_byte(b),
            Err(ReadError::Overrun) => uart.write_byte(b'O'),
            Err(ReadError::Framing) => uart.write_byte(b'F'),
            Err(ReadError::Parity) => uart.write_byte(b'P'),
            Err(ReadError::Empty) => {
                uart.write_bytes(b".");
                delay.delay_ms(1000);
            }
        }
    }
}
