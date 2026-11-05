//! All-in-one R4 responder: one image that exercises Slices 1–10 of this
//! HAL on request from the R3 (`arduino/validation/r3-validator`), so the
//! whole library can be re-proven on the bench without swapping firmware.
//! The host side is the `validate` CLI (`crates/validation`).
//!
//! Build (from `crates/test-firmware/`; like every binary here, the image
//! uses this crate's `memory.x` — FLASH ORIGIN 0x4000 — and target
//! configuration):
//!
//! ```text
//! cargo objcopy --release --features usb,uart-irq --bin r4_validator -- -O binary r4_validator.bin
//! ```
//!
//! # Bench lines (AGENTS.md, unchanged)
//!
//! | R4 pin | R3 pin | Used for |
//! |---|---|---|
//! | D0 / D1 (SCI2) | D1 / D0 | the request link, 9600 8N1 |
//! | D3 | D3 | Slice 2 input and button (R3 drives) and Slice 1 timing (R4 drives) |
//! | D6 (GPT PWM) | D2 (INT0) | Slice 7 period/duty measurement |
//! | A0 (ADC) | D5 (PWM) | Slice 8 stimulus |
//! | D4 (GPT320 STEP) | D8 (ICP1) | Slice 11 step generator (`T`), timestamped by the R3's Timer1 input capture |
//! | RESET | D4 | R3 reset pulses (`!` / `@`), independent of this image |
//!
//! R4 D2 is free/open; this image does not claim it. On WiFi, R4 D4 is the
//! step generator's STEP output: HIGH whenever it is stopped.
//!
//! D3 is input with pull-up on **both** boards except while one side
//! drives it for a test that side requested, so the boards never fight.
//!
//! # Link contract (shared with the R3 sketch)
//!
//! - Requests and replies are frames: `STX (0x02)`, body, `ETX (0x03)`.
//!   Bytes outside a frame are ignored. The R3's human-readable report
//!   also reaches this board's RX, and it never contains STX.
//! - Reply bodies use only `A–Z`, `0–9` and spaces. The R3 treats
//!   lowercase letters, `?`, `!` and `@` outside a frame as PC commands,
//!   so even a reply that loses its STX cannot start a test or a reset.
//! - One request is outstanding at a time. The only unframed traffic is
//!   the Slice 5 burst that follows an `E READY` reply.
//! - SCI2 TX (P302) is released to GPIO input whenever nothing is being
//!   sent. The PC's bytes and this board's TX share the R3's only RX
//!   input; a driven idle line would block PC commands, `!` and `@`
//!   included (Validation.md, Slice 9).
//!
//! | Request | Slice | Reply |
//! |---|---|---|
//! | `I` | 3 | `ID VALIDATOR 1 WIFI\|MINIMA UP <s>` |
//! | `D` | 1 | `D DONE 10`: 20 ms low, ten 20 ms high/low cycles on D3 via `Delay` |
//! | `G` | 2 | `G <0\|1>`: D3 level |
//! | `B` | 2 | `B OK <press ms> <hold ms>` or `B TIMEOUT`: D3 as an active-low button, pressed by the R3 (10 s limit) |
//! | `M` | 4 | `M <token>`; prints `VALIDATE S4 TOKEN <token>` on SCI9 |
//! | `E<n>` | 5 | `E READY`, echoes `n` raw bytes, then `E <got> OV <ov> ER <er>` |
//! | `R<n>` | 5 | as `E`, but only through the `embedded-io` traits on `&UartIrq`: `R READY`, echo, `R <got> OV <ov> ER <er>` |
//! | `A` | 6 | `AT <1\|0> N <bytes>` (`AT NA` on Minima) |
//! | `P<pm>` | 7 | `P <duty> <max>`: D6 at `pm` per-mille of 1 kHz |
//! | `C<ms>` | 8 | `C <mean> <n> <min> <max> <err>`: A0, 12-bit, `ms` window |
//! | `U` | 9 | `U 1` (taken, knock armed) or `U 0` (already taken) |
//! | `K` | 10 | `K <boots> <ok>`: the boot log, read back now (`K ERR <name>` if recording failed) |
//! | `F` | 10 | `F OK <blank erase ms> <program ms> <erase ms> <mismatches>` or `F FAIL <step> <error>`: block 6 self-test |
//! | `S` | — | `S <usb state> <taken> <up s> <ov> <er>` |
//! | `L<n>` | 11 (step 1) | `L <n> <count> <after>`: `n` ELC software events routed to GPT166's count-up input (`count` = `n`), then 10 more with the route removed (`after` = `count`) |
//! | `N` | 11 (step 3) | `N <pulses>`: the step generator's hardware pulse count (GPT321), wrapping |
//! | `Y<rate>` | 11 (step 4) | `Y <checked> <mismatches> <skipped> <dmin> <dmax> <span> [<phase> <diff+10>]`: 60 ticks at `rate`, each latched count against the previous tick's prediction; the first mismatch's details if any |
//! | `V<rate>` | 11 (step 4) | as `Y`, over 2 500 ticks (5 s) |
//! | `X` | 11 (step 5a) | `X <min> <max> <samples>`: the tick DTC's write latency in PCLKD counts over 500 ticks at 1 000/s |
//! | `W<mode>` | 11 (step 5b) | `W <ticks> <applied> <count misses> <phase misses> <queue errors> <reversals> <in flight> <old first> <violations> <dir min> <dir max> [M]<position change>`: reversals at every tick (mode 1), a ramp through zero (2), from stopped (3); every reversal checked against the DIR rule |
//! | `H<cycles>` | 11 (stop) | `H <cycles> <cut> <end> <between> <cut max> <between max> <period> <pulses>`: start/stop cycles at 55 733/s stopped at random phases; where each stop froze the counter |
//! | `J` | 11 (wake) | `J <ticks> <applied> <count misses> <phase misses> <queue errors> <skipped> <wakes> <wake max> <done max>`: 2 500 ticks of random fast rates with the CPU asleep in `WFI`, woken by the tick |
//! | `O` | 11 (step 5c) | `O <ticks> <applied> <count misses> <phase misses> <queue errors> <idle stops> <starts>`: ramp, rate 0, held ticks, restart from idle, 1 s idle stop, start from stopped, trip stop |
//! | `Q<mode>` | 11 (step 5a) | `Q <ticks> <applied> <count misses> <phase misses> <queue errors> <span>`: a rate queued after every tick (mode 1: 44 600/960 alternating, the R3 times the markers; mode 2: ±2 421/s ramp), every prediction checked |
//! | `T<rate>` | 11 (step 2) | `T <period counts>`: D4 stopped, then started at `rate` pulses/s (5 µs low, ≥ 2.5 µs high); `T0` stops (`T 0`); `T ERR <code>`; `T NA` on Minima |
//! | `w` | 2 (wake) | `w <activated> <wakes> <active at wake>`: D3 as a switch with `with_edge` and `with_wake`, asleep in `WFI` until the R3 presses it |
//! | `e` | 2 (external pull-up) | `e <internal pull-up> <active at idle> <activated> <wakes> <active at wake> <released>`: the same switch with `Pull::External`, the R3's pin pull-up as the pull-up |
//! | `c` | 11 (count stop) | `c <attempts> <stops won> <cancels won> <violations> <first violation> <errors> <last error> <pulses>`: `cancel_stop_at` at swept moments around the count match |
//! | `s<mode>` | 11 (long sleep) | `s <skipped> <polls> <errors> <phase misses> <count misses> <checked> <pulses> <position>`: a sleep of 100 ms / 500 ms / 3 s at a steady rate, then one poll and a `queue` |
//! | `b` | 11 (boot) | `b <ran> <starts> <errors> <phase misses> <count misses> <checked> <freed> <pins input> <timer stopped> <count> <ena counts> <lead counts>`: the boot-time setup (STEP D3 on GPT321 A, DIR D5 with a 200 µs set-up, ENA D2 with a 5 µs lead), four direction changes through `queue`, `free` while running |
//! | `t<mode>` | 12 (wake) | the wake timer on GPT167 (`wake_check`): `t1` wait lengths, `t2` behaviour, `t3` two waits marked on D3 for the R3 |
//! | `m` | — | `m <ms 2 400 000 x4> <ms 960 000 x10> <ms 240 000 x40> <ms 1 200 x8 000>`: `cortex_m::asm::delay` timed on the SysTick clock |
//! | `Z<zone>` | 11 (step 2) | `Z <zone> <phase>`: the running generator stopped inside a chosen phase — 1 mid-pulse, 2 just after a pulse, 3 just before the next; `Z ERR` |
//!
//! # Slice 10
//!
//! Every boot appends one byte to the boot log in data flash block 7
//! (`flash.rs`) before the request loop starts. `K` reads the log back
//! with plain reads, never entering P/E mode, so it is cheap enough to
//! poll. `F` runs the erase/program/blank-check round trip on block 6.
//!
//! # Slice 9
//!
//! The USB stack is built at boot while the bridge still owns the
//! connector, so any unwrap fails while the monitor still works. `U`
//! takes the connector and arms the bootloader knock. The native port then echoes and prints `tick N`
//! about once per second. Opening it at 1200 baud with DTR low returns the
//! bridge and holds the bootloader; an R3 `!` then restarts this image.
//!
//! Long requests (`B`, `E`, `A`) do not poll USB while they run; run them
//! before `U`, as the host script does.
//!
//! Slice 3's polled `Uart` driver is exercised on SCI9 (the monitor);
//! SCI2 runs in interrupt mode here, because the link and the Slice 5
//! burst share it.

#![deny(missing_docs)]
#![no_std]
#![no_main]

mod body;
#[cfg(feature = "wifi")]
mod boot_free;
#[cfg(feature = "wifi")]
mod cancel_race;
mod clock;
mod delay_probe;
mod elc_count;
mod flash;
mod frame;
mod link;
#[cfg(feature = "wifi")]
mod long_sleep;
mod requests;
#[cfg(feature = "wifi")]
mod step_run;
#[cfg(feature = "wifi")]
mod switch_wake;
#[cfg(feature = "wifi")]
mod wake_check;

// ---------------------------------------------------------------------------------------------- //

use crate::{
    body::Body,
    clock::Clock,
    elc_count::elc_count,
    flash::{BootLog, self_test},
    frame::FrameBuf,
    link::{LINK_BAUD, link_tx, release_link_when_idle, reply},
    requests::{
        argument, at_check, burst_echo, d3_button, d3_level, d3_square_wave, identify, s4_token,
        tick_line, trait_echo_request, usb_state,
    },
};

use unor4_hal::{
    Peripherals,
    adc::{Adc, Resolution},
    dflash::DataFlash,
    intc,
    pwm::Pwm,
    uart::{Uart, UartIrq},
    usb::{CdcAcm, UsbPort, Usbfs},
};

#[cfg(feature = "wifi")]
use unor4_hal::{step::StepGenerator, wake::WakeTimer};

use embedded_hal::pwm::SetDutyCycle as _;
use panic_halt as _;
use usb_device::{
    LangID,
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDeviceBuilder, UsbRev, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

/// The request link to the R3 (SCI2, interrupt mode).
static LINK: UartIrq<ra4m1::SCI2, 512, 512> = UartIrq::new();
/// The ESP32-S3 AT channel (SCI1, interrupt mode; WiFi only).
#[cfg(feature = "wifi")]
static AT: UartIrq<ra4m1::SCI1> = UartIrq::new();

/// D6 PWM frequency: 1 kHz, the Slice 7 setting (48 000-tick period).
const PWM_HZ: u32 = 1_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Slice 11: the step generator from its checked setup (`step_run::SETUP`:
    // STEP D4 on GPT320, DIR D5, 16-bit GPT162 counting, GPT164/GPT165
    // ticking), stopped, the pin HIGH. It claims exactly those parts from
    // `p` and gives the rest back, so it comes before the drivers that take
    // fields out of `p` directly. At startup all 32 ICU slots are free, so
    // its one run-time error (no free slot) cannot happen here.
    //
    // Before it, the boot-time step checks (`boot_free`): a second setup
    // built, run, freed and replaced while `p` is still whole (`free` and
    // `build` take all of it). The R3 resets this board to watch them (`b`).
    #[cfg(feature = "wifi")]
    let (boot_free, p) = boot_free::BootFree::run(p);
    #[cfg(feature = "wifi")]
    let (mut stepper, p) = match StepGenerator::new(step_run::SETUP)
        .with_peripherals(p)
        .build()
    {
        Ok((stepper, rest)) => (Some(stepper), rest),
        Err(_) => panic!("no free ICU slot for the step generator"),
    };
    // The wake timer (`wake_check::WAKE`: GPT167, ELC input E), also from
    // the whole `p`, before any field is taken.
    #[cfg(feature = "wifi")]
    let (mut wake, p) = match WakeTimer::new(wake_check::WAKE).with_peripherals(p).build() {
        Ok(built) => built,
        Err(_) => panic!("no free ICU slot for the wake timer"),
    };

    // Slice 4: the polled driver on the bridge monitor (SCI9, 115200).
    #[cfg(feature = "wifi")]
    let mut monitor = Uart::new(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::new(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);
    monitor.write_bytes(b"unor4-hal validator 1 - slices 1-10, R3 link on SCI2 9600\r\n");

    // Slice 10: claim the data flash and log this boot before any
    // request can read the log.
    let mut flash = DataFlash::take(&p.clocks);
    let boot_log = BootLog::record(&mut flash);
    monitor.write_bytes(b"boot log: ");
    monitor.write_bytes(boot_log.reply(flash.as_ref().ok()).as_bytes());
    monitor.write_bytes(b"\r\n");

    // Slices 3/5: the link. Slice 6: the AT channel. Then CPU interrupts.
    LINK.attach(p.sci2, p.pins.d1, p.pins.d0, LINK_BAUD, &p.clocks);
    link_tx(false);
    #[cfg(feature = "wifi")]
    AT.attach(p.sci1, p.sci1_tx, p.sci1_rx, 115_200, &p.clocks);
    unsafe { intc::enable_interrupts() };

    // Slices 1/2: heartbeat LED and the shared D3 line (input, button,
    // and the Slice 1 square wave).
    let mut led = p.pins.d13.into_output();
    let mut d3 = Some(p.pins.d3.into_input_pullup());
    // Slice 11 step 1: the ELC (started by `Peripherals::take`), and GPT166
    // as its software-event counter.
    let mut elc = p.elc;
    let elc_a = p.elc_a;
    let event_gpt = p.gpt166;
    // The wake timer's reference: GPT321 free-running at PCLKD (the main
    // step setup leaves it free; the boot-time setup gave it back).
    #[cfg(feature = "wifi")]
    let reference = p.gpt321;
    #[cfg(feature = "wifi")]
    wake_check::start_reference(&reference);

    // Slice 7: D6 PWM. Slice 8: A0 at 12 bits.
    #[cfg(feature = "wifi")]
    let mut pwm = Pwm::new(p.gpt163, p.pins.d6, PWM_HZ, &p.clocks);
    #[cfg(feature = "minima")]
    let mut pwm = Pwm::new(p.gpt320, p.pins.d6, PWM_HZ, &p.clocks);
    let pwm_max = pwm.max_duty_cycle();
    let mut adc = Adc::new(p.adc140, Resolution::Bits12);
    let mut a0 = p.pins.a0.into_analog();

    // Slice 9: the whole stack now, while the bridge owns the connector,
    // so every unwrap can only fail where the monitor still works.
    let alloc = UsbBusAllocator::new(Usbfs::new(p.usbfs));
    let serial = SerialPort::new(&alloc);
    let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D))
        .usb_rev(UsbRev::Usb200)
        .device_class(USB_CLASS_CDC)
        .strings(&[StringDescriptors::new(LangID::EN_US).product("UNO R4 validator")])
        .unwrap()
        .max_packet_size_0(64)
        .unwrap()
        .build();
    let mut cdc = CdcAcm::new(serial, device);
    let mut mux = Some(p.usb_mux);

    let sysclk = p.clocks.sysclk();
    let mut clock = Clock::new(p.delay.free(), sysclk);

    let mut pending = [0u8; 64];
    let mut pending_len = 0;
    let mut pending_pos = 0;
    let mut next_blink = 500;
    let mut next_tick = 1000;
    let mut tick = 0u32;
    let mut frames = FrameBuf::new();
    loop {
        release_link_when_idle();
        let now = clock.now_ms();
        if now >= next_blink {
            next_blink += 500;
            led.toggle(); // Slice 1 observable: 1 Hz blink
        }

        // Slice 9: echo and ticks on the native port once it is taken,
        // never blocking on the host (the knock lives in poll()).
        if pending_pos < pending_len {
            pending_pos += cdc
                .write_available(&pending[pending_pos..pending_len])
                .unwrap_or(0);
        } else {
            pending_len = cdc.read_available(&mut pending).unwrap_or(0);
            pending_pos = 0;
        }
        if now >= next_tick {
            next_tick += 1000;
            tick += 1;
            if mux.is_none() && pending_pos >= pending_len {
                pending_len = tick_line(&mut pending, tick);
                pending_pos = 0;
            }
        }

        let Some(byte) = LINK.read_byte() else {
            continue;
        };
        let Some(req) = frames.feed(byte) else {
            continue;
        };

        let body = match req[0] {
            b'I' => identify(&mut clock),
            b'D' => d3_square_wave(&mut clock, &p.clocks, &mut d3),
            b'G' => d3_level(&d3),
            b'B' => d3_button(&mut clock, &d3),
            b'M' => s4_token(&mut clock, &mut monitor),
            b'E' => burst_echo(req, &mut clock, &mut monitor),
            b'R' => trait_echo_request(req, &mut monitor),
            b'A' => at_check(&mut clock),
            b'P' => {
                let per_mille = argument(req, 500).min(1000);
                let duty = (u32::from(pwm_max) * per_mille / 1000) as u16;
                pwm.set_duty_cycle(duty).unwrap(); // Infallible
                let mut body = Body::new(b"P");
                body.num(u32::from(duty)).num(u32::from(pwm_max));
                body
            }
            b'C' => {
                let window = argument(req, 200).clamp(1, 2000);
                let (mut sum, mut n, mut low, mut high, mut errors) =
                    (0u64, 0u32, u16::MAX, 0u16, 0u32);
                let start = clock.now_ms();
                while clock.now_ms() - start < u64::from(window) {
                    match adc.read(&mut a0) {
                        Ok(raw) => {
                            sum += u64::from(raw);
                            n += 1;
                            low = low.min(raw);
                            high = high.max(raw);
                        }
                        Err(_) => errors += 1,
                    }
                }
                let mut body = Body::new(b"C");
                body.num((sum / u64::from(n.max(1))) as u32)
                    .num(n)
                    .num(u32::from(low.min(high)))
                    .num(u32::from(high))
                    .num(errors);
                body
            }
            b'U' => match mux.take() {
                Some(pin) => {
                    monitor.write_bytes(b"taking USB - bridge drops now\r\n");
                    clock.wait_ms(10); // let the last monitor byte leave SCI9
                    cdc.enable_bootloader_knock(UsbPort::take(pin));
                    Body::new(b"U 1")
                }
                None => Body::new(b"U 0"),
            },
            b'L' => elc_count(req, &mut elc, &elc_a, &event_gpt),
            #[cfg(feature = "wifi")]
            b'T' => step_run::step_request(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'Z' => step_run::stop_in_zone(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'N' => step_run::pulse_count(&stepper),
            #[cfg(feature = "wifi")]
            b'Y' => step_run::tick_run(req, &mut stepper, step_run::TICK_RUN_TICKS),
            #[cfg(feature = "wifi")]
            b'V' => step_run::tick_run(req, &mut stepper, step_run::TICK_STRESS_TICKS),
            #[cfg(feature = "wifi")]
            b'X' => step_run::dtc_probe(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'Q' => step_run::queue_run(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'O' => step_run::scenario_run(&mut stepper),
            #[cfg(feature = "wifi")]
            b'W' => step_run::reverse_run(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'H' => step_run::stop_burst(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'J' => step_run::wake_run(&mut stepper),
            #[cfg(feature = "wifi")]
            b'k' => step_run::skip_run(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'd' => step_run::dir_lead_run(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'n' => step_run::count_stop_run(req, &mut stepper),
            #[cfg(feature = "wifi")]
            b'w' => switch_wake::switch_wake(&mut d3),
            #[cfg(feature = "wifi")]
            b'e' => switch_wake::switch_external(&mut d3, &mut clock),
            #[cfg(feature = "wifi")]
            b'c' => cancel_race::cancel_race(&mut stepper),
            #[cfg(feature = "wifi")]
            b's' => long_sleep::long_sleep(req, &mut stepper, &mut clock),
            #[cfg(feature = "wifi")]
            b'b' => boot_free.reply(),
            #[cfg(feature = "wifi")]
            b't' => wake_check::wake_request(req, &mut wake, &reference, &mut d3),
            #[cfg(feature = "minima")]
            b'T' | b'Z' | b'N' | b'Y' | b'V' | b'X' | b'Q' | b'O' | b'W' | b'H' | b'J' | b'k'
            | b'd' | b'n' | b'w' | b'e' | b'c' | b's' | b'b' | b't' => Body::new(b"T NA"),
            b'm' => delay_probe::delay_probe(&mut clock),
            b'K' => boot_log.reply(flash.as_ref().ok()),
            b'F' => self_test(flash.as_mut().ok(), &mut clock),
            b'S' => {
                let mut body = Body::new(b"S");
                body.num(usb_state(cdc.state()))
                    .num(u32::from(mux.is_none()))
                    .num((clock.now_ms() / 1000) as u32)
                    .num(LINK.overflowed())
                    .num(LINK.errored());
                body
            }
            _ => Body::new(b"NAK"),
        };
        reply(&body, &mut monitor);
    }
}
