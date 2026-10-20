//! Slice 6: the WiFi AT channel — the RA4M1 drives the ESP32-S3's AT
//! server over SCI1 (P501 = TX, P502 = RX, 115200 8N1, board-internal)
//! while the SCI9 monitor path stays alive for observation.
//!
//! SCI1 runs in interrupt mode (ring buffers, RXI/TXI/ERI through
//! `intc`); the monitor runs in polling mode. The firmware sends
//! `AT\r\n` once every 5 seconds for 30 seconds and relays everything
//! the ESP32-S3 sends back to the monitor, with non-printable bytes
//! shown as `\xHH` escapes. The stock ESP32-S3 bridge firmware
//! (`uno-r4-wifi-usb-bridge`) implements its AT server via
//! SudoMaker/chAT at 115200 8N1 — the expected answer is `OK`
//! (Plan.md, slice 6 observable).
//!
//! Wiring: no operator — the bench test-gate line (R3 D3 → R4 D3,
//! straight through) starts each run on its falling edge, and the
//! firmware re-arms when the line rises, so the R3's `slice06-gate`
//! sketch cycles it and one flash serves many unattended runs. SCI1
//! itself is board-internal on the WiFi board.
//!
//! Size budget: release `text` = 14,888 bytes, `bss` = 668 bytes. The
//! text carries the 32-entry vector table, two UART drivers (SCI1 IRQ
//! rings + ISRs, SCI9 polling) and the re-arming gate loop; the bss is
//! mostly the 256/128-byte AT-channel rings.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    uart::{Uart, UartIrq},
};

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use ra4m1::SCI1;

/// The IRQ-mode SCI1 instance. `static` is the point: the interrupt
/// handlers reach it after `main`'s stack frame is long gone.
static UART1: UartIrq<SCI1> = UartIrq::new();

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;

    // Monitor first: the banner prints at boot, before the gate, so a
    // silent boot is distinguishable from a never-opened gate.
    #[cfg(feature = "wifi")]
    let mut monitor = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);
    monitor.write_bytes(b"unor4-hal slice 06 - SCI1 AT channel, 115200 8N1\r\n");
    monitor.write_bytes(b"waiting for the D3 gate (R3 drives it low) ");

    // The AT channel: IRQ-mode SCI1 on P501 (TX) / P502 (RX), attached
    // once for the life of the board. Attaching links three IELSR slots
    // (RXI, TXI, ERI) and turns `RIE` on.
    UART1.sci1(p.sci1, p.sci1_tx, p.sci1_rx, 115_200, &p.clocks);

    // Startup, outside a critical section; the bootloader masks IRQs.
    unsafe { unor4_hal::intc::enable_interrupts() };

    // Bench test-gate line: R3 D3 → R4 D3. Pull-up keeps the input
    // defined while the R3's pin is not driving.
    let gate = p.pins.d3.into_input_pullup();

    // Re-triggerable run loop: the R3's `slice06-gate` sketch cycles
    // the gate, and every falling edge runs one AT test. A '.' every
    // second proves the firmware is alive while it waits.
    loop {
        let mut wait_seconds = 0;
        while !gate.is_low() {
            delay.delay_ms(10);
            wait_seconds += 1;
            if wait_seconds == 100 {
                wait_seconds = 0;
                monitor.write_byte(b'.');
            }
        }
        monitor.write_bytes(b"\r\ngate open\r\n");

        monitor.write_bytes(b"giving the ESP32-S3 AT server 2 s to settle...\r\n");
        delay.delay_ms(2000);

        let mut total: u32 = 0;
        for round in 0..6u32 {
            monitor.write_bytes(b"round ");
            print_u32(&mut monitor, round);
            monitor.write_bytes(b": sending AT\\r\\n -> ");
            UART1.write_all(b"AT\r\n");
            UART1.flush();

            // Drain the receive ring for 5 s (10 ms slices), relaying
            // every byte to the monitor as it arrives — the ESP32's
            // answer is the evidence, whatever it turns out to be.
            let mut seen: u32 = 0;
            for _ in 0..500 {
                delay.delay_ms(10);
                while let Some(byte) = UART1.read_byte() {
                    seen += 1;
                    match byte {
                        0x20..=0x7e => monitor.write_byte(byte),
                        _ => {
                            monitor.write_bytes(b"\\x");
                            print_hex_byte(&mut monitor, byte);
                        }
                    }
                }
            }
            total += seen;
            monitor.write_bytes(b"\r\nround ");
            print_u32(&mut monitor, round);
            monitor.write_bytes(b" done: ");
            print_u32(&mut monitor, seen);
            monitor.write_bytes(b" bytes\r\n");
        }

        monitor.write_bytes(b"done: total=");
        print_u32(&mut monitor, total);
        monitor.write_bytes(b" ov=");
        print_u32(&mut monitor, UART1.overflowed());
        monitor.write_bytes(b" er=");
        print_u32(&mut monitor, UART1.errored());
        monitor.write_bytes(b"\r\n");

        // Re-arm: hold here while the gate is low, release when the R3
        // raises the line, then wait for the next falling edge.
        monitor.write_bytes(b"waiting for the gate to rise to re-arm\r\n");
        while gate.is_low() {
            delay.delay_ms(10);
        }
        monitor.write_bytes(b"gate re-armed\r\n");
    }
}

/// Appends `value` in decimal to the monitor. No `core::fmt` in
/// firmware — the same digit loop the earlier slices use.
fn print_u32(monitor: &mut Uart<ra4m1::SCI9>, value: u32) {
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
        monitor.write_byte(d);
    }
}

/// Appends `byte` as two hex digits.
fn print_hex_byte(monitor: &mut Uart<ra4m1::SCI9>, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    monitor.write_byte(HEX[(byte >> 4) as usize]);
    monitor.write_byte(HEX[(byte & 0x0f) as usize]);
}
