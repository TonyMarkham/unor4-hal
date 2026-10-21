//! Request handlers: one function per validator request, per the
//! link-contract table in the crate docs.

#[cfg(feature = "wifi")]
use crate::AT;
use crate::{
    LINK,
    body::Body,
    clock::Clock,
    link::{link_tx, reply},
};

use unor4_hal::{
    clock::Clocks,
    delay::Delay,
    gpio::{InputPullUp, Pin},
    uart::Uart,
};

use embedded_hal::delay::DelayNs as _;
use usb_device::device::UsbDeviceState;

/// Largest Slice 5 burst the R3 may request.
pub(crate) const MAX_BURST: u32 = 4096;
/// Slice 1 square wave: one 20 ms phase per edge.
pub(crate) const D3_PHASE_MS: u32 = 20;
/// Slice 1 square wave cycle count.
pub(crate) const D3_CYCLES: u32 = 10;

/// Parses the decimal argument after the command letter; `default` when
/// absent or malformed.
pub(crate) fn argument(request: &[u8], default: u32) -> u32 {
    let digits = &request[1..];
    if digits.is_empty() || digits.iter().any(|byte| !byte.is_ascii_digit()) || digits.len() > 6 {
        return default;
    }
    digits
        .iter()
        .fold(0, |value, &byte| value * 10 + u32::from(byte - b'0'))
}

/// The `I` request: identity, board map, and uptime.
pub(crate) fn identify(clock: &mut Clock) -> Body {
    let mut body = Body::new(b"ID VALIDATOR 1");
    #[cfg(feature = "wifi")]
    body.text(b" WIFI UP");
    #[cfg(feature = "minima")]
    body.text(b" MINIMA UP");
    body.num((clock.now_ms() / 1000) as u32);
    body
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

/// The `D` request: lend SysTick to a real [`Delay`] for the square wave,
/// then hand D3 back to input.
pub(crate) fn d3_square_wave<const PORT: char, const PIN: u8>(
    clock: &mut Clock,
    clocks: &Clocks,
    d3: &mut Option<Pin<PORT, PIN, InputPullUp>>,
) -> Body {
    // Give the R3 time to arm its edge capture.
    clock.wait_ms(100);
    let credit = D3_PHASE_MS * (1 + 2 * D3_CYCLES);
    let pin = d3.take().expect("D3 is owned between requests");
    let mut returned = None;
    clock.with_delay(clocks, credit, |delay| {
        returned = Some(drive_d3(pin, delay))
    });
    *d3 = returned;
    let mut body = Body::new(b"D DONE");
    body.num(D3_CYCLES);
    body
}

/// The `G` request: the D3 level.
pub(crate) fn d3_level<const PORT: char, const PIN: u8>(
    d3: &Option<Pin<PORT, PIN, InputPullUp>>,
) -> Body {
    let high = d3.as_ref().is_some_and(|pin| pin.is_high());
    let mut body = Body::new(b"G");
    body.num(u32::from(high));
    body
}

/// The `B` request: D3 read as a button (pull-up input, pressed = low),
/// pressed by the R3; 10 s limits on the wait and the hold.
pub(crate) fn d3_button<const PORT: char, const PIN: u8>(
    clock: &mut Clock,
    d3: &Option<Pin<PORT, PIN, InputPullUp>>,
) -> Body {
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
        body.num((pressed - start) as u32)
            .num((clock.now_ms() - pressed) as u32);
        body
    }
}

/// The `M` request: a token printed on SCI9 for the host's bridge-monitor
/// check, echoed in the reply.
pub(crate) fn s4_token(clock: &mut Clock, monitor: &mut Uart<ra4m1::SCI9>) -> Body {
    let token = (clock.now_ms() % 100_000) as u32;
    let mut line = Body::new(b"VALIDATE S4 TOKEN");
    line.num(token);
    monitor.write_bytes(line.as_bytes());
    monitor.write_bytes(b"\r\n");
    let mut body = Body::new(b"M");
    body.num(token);
    body
}

/// Slice 5 through the traits: echoes `wanted` bytes using only
/// `embedded_io::{Read, Write}` on `&UartIrq`. `read` blocks
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
        let Ok(n) = embedded_io::Read::read(&mut port, &mut chunk[..room]) else {
            break;
        };
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

/// The `R<n>` request: the trait echo, with overflow/error deltas.
pub(crate) fn trait_echo_request(req: &[u8], monitor: &mut Uart<ra4m1::SCI9>) -> Body {
    let wanted = argument(req, 256).min(MAX_BURST);
    let (ov, er) = (LINK.overflowed(), LINK.errored());
    reply(&Body::new(b"R READY"), monitor);
    let got = trait_echo(wanted);
    let mut body = Body::new(b"R");
    body.num(got)
        .text(b" OV")
        .num(LINK.overflowed() - ov)
        .text(b" ER")
        .num(LINK.errored() - er);
    body
}

/// The `E<n>` request: the polled-driver echo, with overflow/error deltas.
pub(crate) fn burst_echo(req: &[u8], clock: &mut Clock, monitor: &mut Uart<ra4m1::SCI9>) -> Body {
    let wanted = argument(req, 256).min(MAX_BURST);
    let (ov, er) = (LINK.overflowed(), LINK.errored());
    reply(&Body::new(b"E READY"), monitor);
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
    body.num(got)
        .text(b" OV")
        .num(LINK.overflowed() - ov)
        .text(b" ER")
        .num(LINK.errored() - er);
    body
}

/// Slice 6: one `AT` to the ESP32-S3's AT server (SCI1, 115200 8N1);
/// passes when `OK` arrives within 1.5 s. The first 16 received bytes
/// follow as uppercase hex (`X …`), so a failure shows what came back.
#[cfg(feature = "wifi")]
pub(crate) fn at_check(clock: &mut Clock) -> Body {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    while AT.read_byte().is_some() {}
    AT.write_all(b"AT\r\n");
    AT.flush();
    let start = clock.now_ms();
    let (mut seen, mut previous, mut ok) = (0u32, 0u8, false);
    let mut first = [0u8; 16];
    while clock.now_ms() - start < 1500 {
        if let Some(byte) = AT.read_byte() {
            if (seen as usize) < first.len() {
                first[seen as usize] = byte;
            }
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
pub(crate) fn at_check(_clock: &mut Clock) -> Body {
    Body::new(b"AT NA")
}

/// The `usb-device` state as the `S` reply's small integer.
pub(crate) fn usb_state(state: UsbDeviceState) -> u32 {
    match state {
        UsbDeviceState::Default => 0,
        UsbDeviceState::Addressed => 1,
        UsbDeviceState::Configured => 2,
        UsbDeviceState::Suspend => 3,
    }
}

/// Formats `tick N\r\n` for the native port; returns its length.
pub(crate) fn tick_line(line: &mut [u8; 64], tick: u32) -> usize {
    line[..5].copy_from_slice(b"tick ");
    let mut digits = [0; 10];
    let mut number = tick;
    let mut count = 0;
    loop {
        digits[count] = b'0' + (number % 10) as u8;
        count += 1;
        number /= 10;
        if number == 0 {
            break;
        }
    }
    for index in 0..count {
        line[5 + index] = digits[count - index - 1];
    }
    line[5 + count..7 + count].copy_from_slice(b"\r\n");
    7 + count
}
