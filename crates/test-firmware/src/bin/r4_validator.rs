//! All-in-one R4 responder: one image that exercises Slices 1–9 of this
//! HAL on request from the R3 (`validation/r3-validator`), so the whole
//! library can be re-proven on the bench without swapping firmware. The
//! host side is `validation/validate.ps1`.
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

#![no_std]
#![no_main]

use cortex_m::peripheral::SYST;
use embedded_hal::{delay::DelayNs, pwm::SetDutyCycle};
use panic_halt as _;
use unor4_hal::{
    Peripherals,
    adc::{Adc, Resolution},
    clock::Clocks,
    delay::Delay,
    gpio::{InputPullUp, Pin},
    intc,
    pwm::Pwm,
    uart::{Uart, UartIrq},
    usb::{CdcAcm, UsbPort, Usbfs},
};
use usb_device::{
    LangID,
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDeviceBuilder, UsbDeviceState, UsbRev, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

/// The request link to the R3 (SCI2, interrupt mode).
static LINK: UartIrq<ra4m1::SCI2, 512, 512> = UartIrq::new();
/// The ESP32-S3 AT channel (SCI1, interrupt mode; WiFi only).
#[cfg(feature = "wifi")]
static AT: UartIrq<ra4m1::SCI1> = UartIrq::new();

const STX: u8 = 0x02;
const ETX: u8 = 0x03;
const LINK_BAUD: u32 = 9600;
/// D6 PWM frequency: 1 kHz, the Slice 7 setting (48 000-tick period).
const PWM_HZ: u32 = 1_000;
/// Largest Slice 5 burst the R3 may request.
const MAX_BURST: u32 = 4096;
/// Slice 1 square wave: one 20 ms phase per edge.
const D3_PHASE_MS: u32 = 20;
const D3_CYCLES: u32 = 10;

/// Free-running SysTick as a millisecond clock (24-bit down-counter, no
/// interrupt; ARMv7-M §B3.3). Call [`Clock::now_ms`] at least once per
/// wrap (about 349 ms at 48 MHz); every wait below does.
struct Clock {
    syst: Option<SYST>,
    previous: u32,
    cycles: u64,
    per_ms: u32,
}

impl Clock {
    fn new(syst: SYST, sysclk: u32) -> Self {
        let mut clock = Self { syst: None, previous: 0, cycles: 0, per_ms: sysclk / 1000 };
        clock.start(syst);
        clock
    }

    fn start(&mut self, mut syst: SYST) {
        syst.disable_interrupt();
        syst.set_reload(0x00ff_ffff);
        syst.clear_current();
        syst.enable_counter();
        self.previous = SYST::get_current();
        self.syst = Some(syst);
    }

    fn now_ms(&mut self) -> u64 {
        let now = SYST::get_current();
        self.cycles += u64::from(self.previous.wrapping_sub(now) & 0x00ff_ffff);
        self.previous = now;
        self.cycles / u64::from(self.per_ms)
    }

    fn wait_ms(&mut self, ms: u32) {
        let until = self.now_ms() + u64::from(ms);
        while self.now_ms() < until {}
    }

    /// Lends SysTick to a real HAL [`Delay`] (Slice 1), then resumes,
    /// crediting the time the delay calls asked for.
    fn with_delay(&mut self, clocks: &Clocks, credit_ms: u32, run: impl FnOnce(&mut Delay)) {
        self.now_ms();
        let mut delay = Delay::new(self.syst.take().expect("clock owns SysTick"), clocks);
        run(&mut delay);
        self.start(delay.free());
        self.cycles += u64::from(credit_ms) * u64::from(self.per_ms);
    }
}

/// A reply body: `A–Z`, `0–9` and spaces only (see the link contract).
struct Body {
    bytes: [u8; 64],
    len: usize,
}

impl Body {
    fn new(text: &[u8]) -> Self {
        let mut body = Self { bytes: [0; 64], len: 0 };
        body.text(text);
        body
    }

    fn text(&mut self, text: &[u8]) -> &mut Self {
        for &byte in text {
            if self.len < self.bytes.len() {
                self.bytes[self.len] = byte;
                self.len += 1;
            }
        }
        self
    }

    fn num(&mut self, value: u32) -> &mut Self {
        let mut digits = [0u8; 10];
        let mut count = 0;
        let mut rest = value;
        loop {
            digits[count] = b'0' + (rest % 10) as u8;
            count += 1;
            rest /= 10;
            if rest == 0 { break; }
        }
        self.text(b" ");
        for index in (0..count).rev() { self.text(&[digits[index]]); }
        self
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// Drives or releases SCI2 TX (P302 = D1 on both maps). Released, the pin
/// is a plain GPIO input and the SCI keeps running internally. Manual §19
/// P302PFS: PMR b16 (peripheral), PDR b2; PWPR unlock (§19.5.1) around
/// the write. Proven on this bench (README, Slice 9: shared-RX limitation).
fn link_tx(enabled: bool) {
    let pfs = 0x4004_08C8 as *mut u32;
    let wanted = if enabled { 1 << 16 } else { 0 };
    if unsafe { pfs.read_volatile() } & 0x0001_0004 == wanted { return; }
    cortex_m::interrupt::free(|_| unsafe {
        let pwpr = 0x4004_0D03 as *mut u8;
        pwpr.write_volatile(0);
        pwpr.write_volatile(0x40);
        pfs.write_volatile((pfs.read_volatile() & !0x0001_0004) | wanted);
        pwpr.write_volatile(0);
        pwpr.write_volatile(0x80);
    });
}

/// Releases SCI2 TX once the TX ring has drained (SCR.TIE clear) and the
/// last stop bit has left (SSR.TEND), §28.2.
fn release_link_when_idle() {
    let sci = unsafe { &*ra4m1::SCI2::PTR };
    if sci.scr().read().tie().bit_is_clear() && sci.ssr().read().tend().bit_is_set() {
        link_tx(false);
    }
}

fn reply(body: &Body, monitor: &mut Uart<ra4m1::SCI9>) {
    link_tx(true);
    LINK.write_all(&[STX]);
    LINK.write_all(body.as_bytes());
    LINK.write_all(&[ETX]);
    // Mirror on the bridge monitor, so a PC watching it sees the run.
    monitor.write_bytes(b"R4> ");
    monitor.write_bytes(body.as_bytes());
    monitor.write_bytes(b"\r\n");
}

/// Parses the decimal argument after the command letter; `default` when
/// absent or malformed.
fn argument(request: &[u8], default: u32) -> u32 {
    let digits = &request[1..];
    if digits.is_empty() || digits.iter().any(|byte| !byte.is_ascii_digit()) || digits.len() > 6 {
        return default;
    }
    digits.iter().fold(0, |value, &byte| value * 10 + u32::from(byte - b'0'))
}

/// Slice 1: an R3-timed square wave on D3 paced by the HAL's `Delay`.
fn drive_d3<const PORT: char, const PIN: u8>(
    pin: Pin<PORT, PIN, InputPullUp>,
    delay: &mut Delay,
) -> Pin<PORT, PIN, InputPullUp> {
    let mut out = pin.into_output(); // falling edge: the timing reference
    delay.delay_ms(D3_PHASE_MS);
    for _ in 0..D3_CYCLES {
        out.set_high();
        delay.delay_ms(D3_PHASE_MS);
        out.set_low();
        delay.delay_ms(D3_PHASE_MS);
    }
    out.into_input_pullup() // final rising edge via both pull-ups
}

/// Formats `tick N\r\n` for the native port; returns its length.
fn tick_line(line: &mut [u8; 64], tick: u32) -> usize {
    line[..5].copy_from_slice(b"tick ");
    let mut digits = [0; 10];
    let mut number = tick;
    let mut count = 0;
    loop {
        digits[count] = b'0' + (number % 10) as u8;
        count += 1;
        number /= 10;
        if number == 0 { break; }
    }
    for index in 0..count { line[5 + index] = digits[count - index - 1]; }
    line[5 + count..7 + count].copy_from_slice(b"\r\n");
    7 + count
}

/// Slice 5 through the traits: echoes `wanted` bytes using only
/// `embedded_io::{Read, Write}` on `&UartIrq` (HalSpec §6). `read` blocks
/// until at least one byte is in the RX ring (the 0.7 contract), so the R3
/// must send all `wanted` bytes. TX is released after every flushed chunk:
/// while `read` waits, the PC can still reach the R3 and its `!`/`@`.
fn trait_echo(wanted: u32) -> u32 {
    let mut port = &LINK;
    let mut chunk = [0u8; 64];
    let mut got = 0;
    while got < wanted {
        let room = chunk.len().min((wanted - got) as usize);
        // Fully qualified: `&UartIrq` also has inherent `write_all`/`flush`.
        let Ok(n) = embedded_io::Read::read(&mut port, &mut chunk[..room]) else { break };
        link_tx(true);
        if embedded_io::Write::write_all(&mut port, &chunk[..n]).is_err()
            || embedded_io::Write::flush(&mut port).is_err()
        {
            break;
        }
        link_tx(false);
        got += n as u32;
    }
    got
}

/// Slice 6: one `AT` to the ESP32-S3's AT server (SCI1, 115200 8N1);
/// passes when `OK` arrives within 1.5 s. The first 16 received bytes
/// follow as uppercase hex (`X …`), so a failure shows what came back.
#[cfg(feature = "wifi")]
fn at_check(clock: &mut Clock) -> Body {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    while AT.read_byte().is_some() {}
    AT.write_all(b"AT\r\n");
    AT.flush();
    let start = clock.now_ms();
    let (mut seen, mut previous, mut ok) = (0u32, 0u8, false);
    let mut first = [0u8; 16];
    while clock.now_ms() - start < 1500 {
        if let Some(byte) = AT.read_byte() {
            if (seen as usize) < first.len() { first[seen as usize] = byte; }
            seen += 1;
            ok |= previous == b'O' && byte == b'K';
            previous = byte;
        }
    }
    let mut body = Body::new(b"AT");
    body.num(u32::from(ok)).text(b" N").num(seen).text(b" X ");
    for &byte in &first[..(seen as usize).min(first.len())] {
        body.text(&[HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 15)]]);
    }
    body
}

/// The Minima's SCI1 pins sit on the SWD connector; there is no AT server.
#[cfg(feature = "minima")]
fn at_check(_clock: &mut Clock) -> Body {
    Body::new(b"AT NA")
}

fn usb_state(state: UsbDeviceState) -> u32 {
    match state {
        UsbDeviceState::Default => 0,
        UsbDeviceState::Addressed => 1,
        UsbDeviceState::Configured => 2,
        UsbDeviceState::Suspend => 3,
    }
}

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

    let mut request = [0u8; 16];
    let mut request_len = 0;
    let mut in_frame = false;
    let mut pending = [0u8; 64];
    let mut pending_len = 0;
    let mut pending_pos = 0;
    let mut next_blink = 500;
    let mut next_tick = 1000;
    let mut tick = 0u32;
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
            pending_pos += cdc.write_available(&pending[pending_pos..pending_len]).unwrap_or(0);
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

        let Some(byte) = LINK.read_byte() else { continue; };
        match byte {
            STX => { in_frame = true; request_len = 0; continue; }
            ETX if in_frame => in_frame = false,
            _ => {
                if in_frame {
                    if request_len < request.len() { request[request_len] = byte; request_len += 1; }
                    else { in_frame = false; }
                }
                continue;
            }
        }
        if request_len == 0 { continue; }
        let req = &request[..request_len];

        let body = match req[0] {
            b'I' => {
                let mut body = Body::new(b"ID VALIDATOR 1");
                #[cfg(feature = "wifi")]
                body.text(b" WIFI UP");
                #[cfg(feature = "minima")]
                body.text(b" MINIMA UP");
                body.num((clock.now_ms() / 1000) as u32);
                body
            }
            b'D' => {
                // Give the R3 time to arm its edge capture, then lend
                // SysTick to a real Delay for the square wave.
                clock.wait_ms(100);
                let credit = D3_PHASE_MS * (1 + 2 * D3_CYCLES);
                let pin = d3.take().expect("D3 is owned between requests");
                let mut returned = None;
                clock.with_delay(&p.clocks, credit, |delay| returned = Some(drive_d3(pin, delay)));
                d3 = returned;
                let mut body = Body::new(b"D DONE");
                body.num(D3_CYCLES);
                body
            }
            b'G' => {
                let high = d3.as_ref().is_some_and(|pin| pin.is_high());
                let mut body = Body::new(b"G");
                body.num(u32::from(high));
                body
            }
            b'B' => {
                // D3 read as a button: pull-up input, pressed = low.
                let button = d3.as_ref().expect("D3 is owned between requests");
                let start = clock.now_ms();
                while button.is_high() && clock.now_ms() - start < 10_000 {}
                if button.is_high() {
                    Body::new(b"B TIMEOUT")
                } else {
                    let pressed = clock.now_ms();
                    clock.wait_ms(20); // contact bounce, as for a real switch
                    while button.is_low() && clock.now_ms() - pressed < 10_000 {}
                    let mut body = Body::new(b"B OK");
                    body.num((pressed - start) as u32).num((clock.now_ms() - pressed) as u32);
                    body
                }
            }
            b'M' => {
                let token = (clock.now_ms() % 100_000) as u32;
                let mut line = Body::new(b"VALIDATE S4 TOKEN");
                line.num(token);
                monitor.write_bytes(line.as_bytes());
                monitor.write_bytes(b"\r\n");
                let mut body = Body::new(b"M");
                body.num(token);
                body
            }
            b'E' => {
                let wanted = argument(req, 256).min(MAX_BURST);
                let (ov, er) = (LINK.overflowed(), LINK.errored());
                reply(&Body::new(b"E READY"), &mut monitor);
                let mut got = 0;
                let mut last = clock.now_ms();
                while got < wanted && clock.now_ms() - last < 2000 {
                    if let Some(echo) = LINK.read_byte() {
                        LINK.write_all(&[echo]);
                        got += 1;
                        last = clock.now_ms();
                    }
                }
                LINK.flush();
                let mut body = Body::new(b"E");
                body.num(got).text(b" OV").num(LINK.overflowed() - ov).text(b" ER").num(LINK.errored() - er);
                body
            }
            b'R' => {
                let wanted = argument(req, 256).min(MAX_BURST);
                let (ov, er) = (LINK.overflowed(), LINK.errored());
                reply(&Body::new(b"R READY"), &mut monitor);
                let got = trait_echo(wanted);
                let mut body = Body::new(b"R");
                body.num(got).text(b" OV").num(LINK.overflowed() - ov).text(b" ER").num(LINK.errored() - er);
                body
            }
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
                let (mut sum, mut n, mut low, mut high, mut errors) = (0u64, 0u32, u16::MAX, 0u16, 0u32);
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
                body.num((sum / u64::from(n.max(1))) as u32).num(n).num(u32::from(low.min(high))).num(u32::from(high)).num(errors);
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
