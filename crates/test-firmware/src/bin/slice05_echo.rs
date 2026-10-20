//! Slice 5: interrupts — IRQ-mode UART echo while the heartbeat keeps
//! its rhythm (the proof the main loop is not polling the wire).
//!
//! SCI9 runs in interrupt mode on the board-internal monitor line
//! (P109 = TX, P110 = RX, 115200 8N1), served by RXI/TXI/ERI events
//! linked through `intc`. The main loop never touches the SCI
//! registers: it reads from the receive ring, queues uppercase into
//! the transmit ring, and checks a free-running SysTick. Every second it
//! prints a tick plus the driver's overflow/error counters.
//!
//! Wiring: the slice 2 pushbutton between D2 and GND (start gate);
//! P109/P110 are board-internal (level translator → ESP32-S3 bridge →
//! board's own USB monitor port).
//!
//! Observables:
//! - the boot banner prints on the first press of the D2 button, so the
//!   monitor is provably attached before any output;
//! - typed characters come back uppercased, instantly;
//! - the D13 LED keeps a clean 1 Hz blink while echoing;
//! - a 4 KB burst pasted into the monitor echoes without `ov=` counting
//!   up (the ring absorbed it) — see README slice 5.
//!
//! WiFi release size: `text` = 9064 bytes, `bss` = 668 bytes.
//! The text carries the 32-entry vector table, the SCI9
//! interrupt mode (rings + three ISRs + dispatch), and the D2 start
//! gate; the bss is mostly the 256/128-byte echo rings.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{Peripherals, uart::UartIrq};

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use ra4m1::SCI9;

#[path = "../heartbeat.rs"]
mod heartbeat;

/// The IRQ-mode SCI9 instance. `static` is the point: the interrupt
/// handlers reach it after `main`'s stack frame is long gone.
static UART9: UartIrq<SCI9> = UartIrq::new();

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;
    // Start gate: nothing goes out on the wire until the slice 2
    // pushbutton (D2→GND, internal pull-up, pressed = low) is pressed.
    // The operator attaches the monitor after upload, then presses —
    // the banner can never be missed.
    let button = p.pins.d2.into_input_pullup();
    while !button.is_low() {
        delay.delay_ms(10);
    }

    let mut led = p.pins.d13.into_output();

    // P109 = TXD9, P110 = RXD9, WiFi map (board-internal line to the
    // ESP32-S3 bridge). Minima map: the same chip pins are header
    // D11/D12. Attaching links three IELSR slots (RXI, TXI, ERI) and
    // turns `RIE` on; `TIE` follows the first queued byte.
    #[cfg(feature = "wifi")]
    UART9.sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    UART9.sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    // Startup, outside a critical section; the bootloader masks IRQs.
    unsafe { unor4_hal::intc::enable_interrupts() };

    UART9.write_all(b"unor4-hal slice 05 - SCI9 IRQ echo, 115200 8N1\r\n");

    // Measure elapsed clock cycles, including time spent in ISRs,
    // instead of treating 20,000 delay calls as one elapsed second.
    let mut heartbeat = heartbeat::Heartbeat::new(delay, p.clocks.sysclk());
    let mut pending = None;
    loop {
        if pending.is_none() {
            pending = UART9.read_byte().map(|byte| byte.to_ascii_uppercase());
        }
        if let Some(byte) = pending
            && UART9.write(&[byte]) == 1
        {
            pending = None;
        }
        if let Some((second, tick)) = heartbeat.poll() {
            led.toggle();
            if second {
                UART9.write_all(b"tick ");
                print_u32(&UART9, tick);
                UART9.write_all(b" ov=");
                print_u32(&UART9, UART9.overflowed());
                UART9.write_all(b" er=");
                print_u32(&UART9, UART9.errored());
                UART9.write_all(b"\r\n");
            }
        }
    }
}

/// Appends `value` in decimal to the UART. No `core::fmt` in firmware —
/// this is the same digit loop the course repo's lesson uses.
fn print_u32(uart: &UartIrq<SCI9>, value: u32) {
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
        uart.write(&[d]);
    }
}
