//! Slice 7: PWM — the LED "breathes": GPT saw-wave PWM on **D6** at
//! 1 kHz while D13 keeps the 1 Hz heartbeat.
//!
//! D6's engine depends on the board map (manual §19 pin-function
//! tables): WiFi D6 = P111 → `GTIOC3A` on **GPT163**; Minima D6 =
//! P106 → `GTIOC0B` on **GPT320**. Both constructors come from the same
//! generic driver; the feature gate below only picks the hardware the
//! board actually has.
//!
//! The measurement is instrumented, not eyeballed: D6 is wired to the
//! R3's D2 (INT0), and `arduino/sketches/slice07-measure` measures the
//! duty and period edge-by-edge. Per gate round (R3 D3 → R4 D3 falling
//! edge, the slice 6 pattern) the firmware runs:
//!
//! 1. a 1 s settle, then a **duty staircase** — 0, 25, 50, 75, 100%,
//!    2 s each. 48 000 duty steps at 1 kHz make every step an exact
//!    fraction (12 000 ticks per quarter), and the two extreme steps
//!    exercise the manual's dedicated mechanisms: 0% via
//!    `GTUDDTYC.OADTY` (a compare match at 0 coincides with the cycle
//!    end and the cycle-end setting wins — Table 22.5 Note 2) and 100%
//!    via a never-matching compare.
//! 2. **sine breathing** — 256 steps of 8 ms → a 2.048 s cycle from a
//!    quarter-sine table, until the next gate edge.
//!
//! then re-arms on the gate's rising edge, so one flash serves many
//! unattended measurement rounds.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use unor4_hal::{Peripherals, pwm::Pwm, uart::Uart};

use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

#[path = "../heartbeat.rs"]
mod heartbeat;

/// The breathing PWM frequency: exactly 1 kHz from PCLKD/1 with a
/// 48 000-tick period (48 000 divides evenly by 4 — every staircase
/// step lands on a whole tick).
const PWM_HZ: u32 = 1_000;

/// Quarter-sine table: `QUARTER_SINE[i] = round(24000 · sin(π/2 · i/64))`
/// for i = 0..=64 — half the period's 48 000 ticks, scaled to the
/// driver's actual half-period at runtime. The full cycle folds this
/// quarter across four quadrants.
const QUARTER_SINE: [u16; 65] = [
    0, 589, 1178, 1766, 2352, 2938, 3522, 4103, 4682, 5258, 5832, 6401, 6967, 7528, 8085, 8637,
    9184, 9726, 10261, 10791, 11314, 11830, 12338, 12840, 13334, 13819, 14297, 14766, 15225, 15676,
    16117, 16549, 16971, 17382, 17783, 18173, 18552, 18920, 19277, 19622, 19955, 20276, 20585,
    20882, 21166, 21437, 21696, 21941, 22173, 22392, 22597, 22789, 22967, 23131, 23281, 23417,
    23539, 23647, 23740, 23820, 23884, 23935, 23971, 23993, 24000,
];

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();

    // Monitor first: the banner prints at boot, before the gate, so a
    // silent boot is distinguishable from a never-opened gate.
    #[cfg(feature = "wifi")]
    let mut monitor = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);
    monitor.write_bytes(b"unor4-hal slice 07 - GPT PWM breathing on D6, 1 kHz\r\n");
    monitor.write_bytes(b"waiting for the D3 gate (R3 drives it low) ");

    // PWM on D6. The duty stays at the reset value (1 tick — visually
    // off) until the first gate round.
    #[cfg(feature = "wifi")]
    let mut led_pwm = Pwm::d6(p.gpt163, p.pins.d6, PWM_HZ, &p.clocks);
    #[cfg(feature = "minima")]
    let mut led_pwm = Pwm::d6(p.gpt320, p.pins.d6, PWM_HZ, &p.clocks);
    let max = led_pwm.max_duty_cycle();

    // Bench test-gate line: R3 D3 → R4 D3. Pull-up keeps the input
    // defined while the R3's pin is not driving.
    let gate = p.pins.d3.into_input_pullup();

    // The heartbeat LED (D13) and its SysTick poller. `Heartbeat` owns
    // the Delay, so wall-clock pacing below reads the same free-running
    // SysTick directly — register reads only, no ownership conflict.
    let mut heartbeat_led = p.pins.d13.into_output();
    let mut heartbeat = heartbeat::Heartbeat::new(p.delay, p.clocks.sysclk());
    let ticks_per_ms = p.clocks.sysclk() / 1000;

    // Re-triggerable run loop: the R3's `slice07-measure` sketch drives
    // the gate low for 1 s per round; every falling edge runs one
    // staircase + breathing cycle.
    loop {
        let mut dots = 0;
        while !gate.is_low() {
            pace(ticks_per_ms, 10);
            poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
            dots += 1;
            if dots == 100 {
                dots = 0;
                monitor.write_byte(b'.');
            }
        }
        monitor.write_bytes(b"\r\ngate open\r\n");

        // Give the R3 one second to open its measurement window after
        // releasing the gate, so its 10 s window covers the staircase.
        for _ in 0..100 {
            pace(ticks_per_ms, 10);
            poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
        }

        // Duty staircase: 0, 25, 50, 75, 100% — 2 s per step.
        monitor.write_bytes(b"duty staircase, 2 s per step:\r\n");
        let steps: [u16; 5] = [0, max / 4, max / 2, 3 * (max / 4), max];
        for (k, &duty) in steps.iter().enumerate() {
            monitor.write_bytes(b"step ");
            print_u32(&mut monitor, k as u32);
            monitor.write_bytes(b" duty=");
            print_u32(&mut monitor, duty as u32);
            monitor.write_bytes(b" of ");
            print_u32(&mut monitor, max as u32);
            monitor.write_bytes(b"\r\n");
            led_pwm.set_duty_cycle(duty).unwrap(); // Infallible error
            for _ in 0..200 {
                pace(ticks_per_ms, 10);
                poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
            }
        }

        // Sine breathing: 256 quarter-sine-folded steps of 8 ms → a
        // 2.048 s cycle, until the R3 asks for the next round.
        monitor.write_bytes(b"breathing (2.048 s sine cycle)\r\n");
        let scale = max as u32 / 2;
        'breathe: loop {
            for i in 0..256u32 {
                if gate.is_low() {
                    break 'breathe;
                }
                let r = (i % 64) as usize;
                let quarter = QUARTER_SINE[r] as u32 * scale / 24_000;
                let mirror = QUARTER_SINE[64 - r] as u32 * scale / 24_000;
                let duty = match i / 64 {
                    0 => scale + quarter,
                    1 => scale + mirror,
                    2 => scale - quarter,
                    _ => scale - mirror,
                };
                led_pwm.set_duty_cycle(duty as u16).unwrap(); // Infallible
                pace(ticks_per_ms, 8);
                poll_heartbeat(&mut heartbeat, &mut heartbeat_led, &mut monitor);
            }
        }

        // Re-arm: the falling edge that broke the breathing loop *is*
        // the next round's trigger, so fall straight through — the loop
        // head's wait exits immediately and the staircase restarts.
        monitor.write_bytes(b"gate cycle - restarting the staircase\r\n");
    }
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
