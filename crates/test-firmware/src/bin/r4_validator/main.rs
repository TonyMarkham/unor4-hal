//! All-in-one R4 responder: one image that exercises Slices 1–9 of this
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
//! | RESET | D4 | R3 reset pulses (`!` / `@`), independent of this image |
//!
//! R4 D2 is free/open; this image does not claim it.
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
//!   included (README, Slice 9).
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
//! | `S` | — | `S <usb state> <taken> <up s> <ov> <er>` |
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
mod clock;
mod frame;
mod link;
mod requests;

// ---------------------------------------------------------------------------------------------- //

use crate::{
    body::Body,
    clock::Clock,
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
    intc,
    pwm::Pwm,
    uart::{Uart, UartIrq},
    usb::{CdcAcm, UsbPort, Usbfs},
};

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

    // Slice 4: the polled driver on the bridge monitor (SCI9, 115200).
    #[cfg(feature = "wifi")]
    let mut monitor = Uart::sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    let mut monitor = Uart::sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);
    monitor.write_bytes(b"unor4-hal validator 1 - slices 1-9, R3 link on SCI2 9600\r\n");

    // Slices 3/5: the link. Slice 6: the AT channel. Then CPU interrupts.
    LINK.sci2(p.sci2, p.pins.d1, p.pins.d0, LINK_BAUD, &p.clocks);
    link_tx(false);
    #[cfg(feature = "wifi")]
    AT.sci1(p.sci1, p.sci1_tx, p.sci1_rx, 115_200, &p.clocks);
    unsafe { intc::enable_interrupts() };

    // Slices 1/2: heartbeat LED and the shared D3 line (input, button,
    // and the Slice 1 square wave).
    let mut led = p.pins.d13.into_output();
    let mut d3 = Some(p.pins.d3.into_input_pullup());

    // Slice 7: D6 PWM. Slice 8: A0 at 12 bits.
    #[cfg(feature = "wifi")]
    let mut pwm = Pwm::d6(p.gpt163, p.pins.d6, PWM_HZ, &p.clocks);
    #[cfg(feature = "minima")]
    let mut pwm = Pwm::d6(p.gpt320, p.pins.d6, PWM_HZ, &p.clocks);
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
