//! Slice 8 bench diagnostic: scan all 20 header pins as digital
//! inputs and classify each as always-high / always-low / toggling
//! over a 300 ms window. Purpose: the R3 drives a ~976 Hz PWM
//! staircase on D5 wired (per the bench list) to R4 A0. When the
//! staircase is not visible on any analog channel (the slice 8
//! bring-up incident — the wire was parked in R3 D4, which is not a
//! PWM pin), this probe finds where the signal actually lands, or
//! proves it lands nowhere.
//!
//! Expected toggling pins even with the wire dead:
//! - `d1` — the R3's hardware Serial TX (D0) prints its round log at
//!   115200 into it (SCI2 RX).
//! - `d3` — the bench gate line (R3 D3), low 1 s / high 1 s per round.
//! Anything else toggling is the D5 signal landing on an unexpected
//! pin. All 20 pins are forced to plain inputs first (PFS write with
//! the PWPR unlock — also neutralizes the bootloader's inherited D13
//! GPT-PWM state), so nothing configures itself.

#![no_std]
#![no_main]

use unor4_hal::{Peripherals, uart::Uart};

use panic_halt as _;

const PFS_BASE: usize = 0x4004_0800;
const PWPR: *mut u8 = 0x4004_0d03 as *mut u8;
const CLEAR: u32 = (1 << 2) | (1 << 4) | (1 << 6) | (1 << 15) | (1 << 16) | (0x1F << 24);

/// The 20 header pins: (name, port index, pin index) — the WiFi map's
/// port/pin pairs (identical names on the Minima map).
const HEADER: [(&str, usize, usize); 20] = [
    ("d0", 3, 1),
    ("d1", 3, 2),
    ("d2", 1, 4),
    ("d3", 1, 5),
    ("d4", 1, 6),
    ("d5", 1, 7),
    ("d6", 1, 11),
    ("d7", 1, 12),
    ("d8", 3, 4),
    ("d9", 3, 3),
    ("d10", 1, 3),
    ("d11", 4, 11),
    ("d12", 4, 10),
    ("d13", 1, 2),
    ("a0", 0, 14),
    ("a1", 0, 0),
    ("a2", 0, 1),
    ("a3", 0, 2),
    ("a4", 1, 1),
    ("a5", 1, 0),
];

fn pfs(port: usize, pin: usize) -> *mut u32 {
    (PFS_BASE + 0x40 * port + 4 * pin) as *mut u32
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    #[cfg(feature = "wifi")]
    let mut monitor = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    // Force all 20 header pins to plain inputs: PDR/PCR/NCODR/PMR/ASEL/
    // PSEL all cleared, everything else left as found. PWPR unlock
    // sequence identical to the gpio driver's `modify_pfs`.
    unsafe {
        core::ptr::write_volatile(PWPR, 0x00); // B0WI = 0
        core::ptr::write_volatile(PWPR, 0x40); // PFSWE = 1
        for &(_, port, pin) in HEADER.iter() {
            let addr = pfs(port, pin);
            let v = core::ptr::read_volatile(addr);
            core::ptr::write_volatile(addr, v & !CLEAR);
        }
        core::ptr::write_volatile(PWPR, 0x00); // PFSWE = 0
        core::ptr::write_volatile(PWPR, 0x80); // B0WI = 1
    }

    monitor.write_bytes(b"slice08 probe - 20-pin scan, 300 ms windows\r\n");
    monitor.write_bytes(b"H=always high L=always low T=toggles ?=never sampled\r\n");
    monitor.write_bytes(b"(d1 = R3 serial traffic, d3 = gate line - expected toggles)\r\n");

    let seen_high = &mut [false; 20];
    let seen_low = &mut [false; 20];
    let mut round: u32 = 0;

    loop {
        seen_high.fill(false);
        seen_low.fill(false);
        // One 300 ms classification window (SysTick-masked; 300 ms is
        // inside the 24-bit wrap). Sampling is continuous — each sweep
        // of all 20 pins is a handful of cycles, so a 976 Hz PWM (1 ms
        // period) is caught thousands of times.
        let target = p.clocks.sysclk() * 3 / 10;
        let start = cortex_m::peripheral::SYST::get_current();
        while start.wrapping_sub(cortex_m::peripheral::SYST::get_current()) & 0x00ff_ffff
            < target
        {
            for (i, &(_, port, pin)) in HEADER.iter().enumerate() {
                let level = unsafe { core::ptr::read_volatile(pfs(port, pin)) } & 1 == 1;
                if level {
                    seen_high[i] = true;
                } else {
                    seen_low[i] = true;
                }
            }
        }
        round += 1;
        print_u32(&mut monitor, round);
        monitor.write_bytes(b": ");
        for (i, &(name, _, _)) in HEADER.iter().enumerate() {
            monitor.write_bytes(name.as_bytes());
            monitor.write_byte(b'=');
            monitor.write_byte(match (seen_high[i], seen_low[i]) {
                (true, true) => b'T',
                (true, false) => b'H',
                (false, true) => b'L',
                (false, false) => b'?',
            });
            monitor.write_byte(b' ');
        }
        monitor.write_bytes(b"\r\n");
    }
}

/// Appends `value` in decimal to the monitor — the shared digit loop.
fn print_u32(monitor: &mut Uart<ra4m1::SCI9>, value: u32) {
    let mut digits = [0u8; 10];
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
