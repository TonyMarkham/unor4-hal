//! Slice 8: ADC — the chip senses. Single-shot reads on **A0** through
//! the ADC140, printed as raw + millivolts at 4 Hz through the slice-4
//! SCI9 monitor path (the board's own USB port).
//!
//! The measurement is instrumented, not eyeballed: R3 D5 drives a PWM
//! duty staircase straight into R4 A0 (the bench's slice-8 rig wire).
//! With no filter, single samples of a ~976 Hz PWM wave are rail
//! values — so the rig rounds **average hundreds of thousands of raw
//! samples over 1.5 s windows**, one window per duty step. The mean of
//! a PWM wave over whole periods is duty × VREF, so the five window
//! means trace the staircase and independently solve for the reference
//! rail (research §9, OQ-A1). Per gate round (R3 D3 → R4 D3 falling
//! edge, the slice 6/7 pattern) the firmware runs the five windows,
//! re-aligned to the R3's duty switches at 2 s boundaries; the
//! heartbeat on D13 keeps its 1 Hz rhythm through everything.
//!
//! The 4 Hz live prints between rounds are the rails check (OQ-A1/OQ-A2):
//! jumper A0 → GND → ~0 mV, → 5 V → ~5000 mV, → 3V3 → ~⅔ of the 5 V
//! reading — the 3V3-vs-5V pair is what pins down the actual reference.
//! Un jumpered, A0 floats (the R3's D5 sits at a defined LOW only while
//! the sketch runs) — noise is expected, not a fault.
//!
//! Size budget: release `text` = 15 108 bytes (WiFi map; the Minima map
//! differs only in the SCI9 pin claim and D13, same code). Larger than
//! slice 7's 12 700 — the windowed averaging carries u64 accumulation
//! and division.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{
    Peripherals,
    adc::{Adc, Resolution},
    uart::Uart,
};

use panic_halt as _;

#[path = "../heartbeat.rs"]
mod heartbeat;

/// Window sampling, in 10 ms bursts: one burst is ~480 000 SysTick
/// ticks — far inside the 24-bit counter wrap (~349 ms), so burst
/// timing is one masked subtraction. 150 bursts = 1.5 s per averaging
/// window; the gaps between windows are paced 10 ms at a time to land
/// each window inside its 2 s duty slot.
const BURSTS_PER_WINDOW: u32 = 150; // 150 × 10 ms = 1.5 s

/// Live-sample cadence: 25 × 10 ms = 250 ms → 4 Hz.
const LIVE_SLICES: u32 = 25;

/// Nominal reference in millivolts, for the mV column. The rails check
/// measures how true this is (hardware result: README slice 8).
const VREF_MV: u32 = 5000;

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();
    let ticks_per_ms = p.clocks.sysclk() / 1000;

    // Monitor first: the banner prints at boot, before the gate, so a
    // silent boot is distinguishable from a never-opened gate.
    #[cfg(feature = "wifi")]
    let mut monitor = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    // The converter: 12-bit (HalSpec §5.7's default; the reset value).
    let max = Resolution::Bits12.max();
    let mut adc = Adc::new(p.adc140, Resolution::Bits12);

    // The measured pin, and the bench test-gate line (R3 D3 → R4 D3,
    // pull-up keeps it defined while the R3 is not driving).
    let mut a0 = p.pins.a0.into_analog();
    let gate = p.pins.d3.into_input_pullup();

    monitor.write_bytes(b"unor4-hal slice 08 - ADC140 A0 single-shot, ");
    print_u32(&mut monitor, adc.resolution().bits() as u32);
    monitor.write_bytes(b"-bit, max=");
    print_u32(&mut monitor, max as u32);
    monitor.write_bytes(b"\r\na0 live samples at 4 Hz; gate round = 5 x 1.5 s averaged windows\r\n");

    let mut heartbeat_led = p.pins.d13.into_output();
    let mut heartbeat = heartbeat::Heartbeat::new(p.delay, p.clocks.sysclk());

    loop {
        // ---- Live phase: 4 Hz raw + millivolts on A0 ----------------
        for _ in 0..LIVE_SLICES {
            pace(ticks_per_ms, 10);
            poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
        }
        let raw = match adc.read(&mut a0) {
            Ok(raw) => raw,
            Err(_) => {
                monitor.write_bytes(b"a0 err=timeout\r\n");
                continue;
            }
        };
        monitor.write_bytes(b"a0 raw=");
        print_u32(&mut monitor, raw as u32);
        monitor.write_bytes(b" mv=");
        print_mv(&mut monitor, mv_x10(raw, max), 1);
        monitor.write_bytes(b"\r\n");

        // ---- Rig round: R3 pulls the gate low to open one -----------
        if gate.is_low() {
            monitor.write_bytes(b"gate open\r\n");
            // Wait for the rising edge — the R3 applies duty step 0
            // exactly when it releases the gate.
            while !gate.is_high() {
                pace(ticks_per_ms, 10);
                poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
            }
            // Five 1.5 s averaging windows with 500 ms gaps, aligned to
            // the R3's 2 s duty slots: W0 [0, 1.5], W1 [2.0, 3.5], …
            for window in 0..5u32 {
                let mut sum: u64 = 0;
                let mut count: u32 = 0;
                let mut min = max;
                let mut max_seen: u16 = 0;
                let mut errors: u32 = 0;
                for _ in 0..BURSTS_PER_WINDOW {
                    // One 10 ms burst of back-to-back conversions.
                    let start = cortex_m::peripheral::SYST::get_current();
                    let target = ticks_per_ms * 10;
                    while start.wrapping_sub(cortex_m::peripheral::SYST::get_current())
                        & 0x00ff_ffff
                        < target
                    {
                        match adc.read(&mut a0) {
                            Ok(raw) => {
                                sum += raw as u64;
                                count += 1;
                                if raw < min {
                                    min = raw;
                                }
                                if raw > max_seen {
                                    max_seen = raw;
                                }
                            }
                            Err(_) => errors += 1,
                        }
                    }
                    poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
                }
                monitor.write_bytes(b"W");
                print_u32(&mut monitor, window);
                monitor.write_bytes(b" n=");
                print_u32(&mut monitor, count);
                monitor.write_bytes(b" mean=");
                print_u32(&mut monitor, (sum / count.max(1) as u64) as u32);
                monitor.write_bytes(b" min=");
                print_u32(&mut monitor, min as u32);
                monitor.write_bytes(b" max=");
                print_u32(&mut monitor, max_seen as u32);
                monitor.write_bytes(b" err=");
                print_u32(&mut monitor, errors);
                monitor.write_bytes(b"\r\n");
                // Gap to the next 2 s slot boundary.
                for _ in 0..50 {
                    pace(ticks_per_ms, 10);
                    poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
                }
            }
            monitor.write_bytes(b"gate cycle\r\n");
        }
    }
}

/// Raw reading to tenths of a millivolt, rounded:
/// `raw × VREF×10 / max`. 32-bit math: `4095 × 50 000` ≈ 2 × 10⁸.
fn mv_x10(raw: u16, max: u16) -> u32 {
    (raw as u32 * VREF_MV * 10 + max as u32 / 2) / max as u32
}

/// One heartbeat poll: toggles the heartbeat LED on the half-second
/// edge and prints the completed-second tick.
fn poll_heartbeat<const PORT: char, const PIN: u8>(
    heartbeat: &mut heartbeat::Heartbeat,
    led: &mut unor4_hal::gpio::Pin<PORT, PIN, unor4_hal::gpio::Output>,
    monitor: &mut Uart<ra4m1::SCI9>,
) {
    if let Some((second, tick)) = heartbeat.poll() {
        led.toggle();
        if second {
            monitor.write_bytes(b"tick ");
            print_u32(monitor, tick);
            monitor.write_bytes(b"\r\n");
        }
    }
}

/// Blocks `ms` milliseconds off the free-running SysTick. `ms` must
/// stay under one counter wrap (~349 ms at 48 MHz — every caller stays
/// ≤ 10 ms and chains). Same read-only register access the heartbeat
/// uses; `SYST::get_current()` is a static, so no handle is needed.
fn pace(ticks_per_ms: u32, ms: u32) {
    let target = ticks_per_ms * ms;
    let start = cortex_m::peripheral::SYST::get_current();
    while start.wrapping_sub(cortex_m::peripheral::SYST::get_current()) & 0x00ff_ffff < target {}
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

/// Appends `value_x10` as a fixed-point decimal with one fractional
/// digit: `25002` → `2500.2`.
fn print_mv(monitor: &mut Uart<ra4m1::SCI9>, value_x10: u32, _decimals: u32) {
    print_u32(monitor, value_x10 / 10);
    monitor.write_byte(b'.');
    monitor.write_byte(b'0' + (value_x10 % 10) as u8);
}
