//! S9: native USB CDC checks and the 1200-baud bootloader knock.

use crate::{
    discover::{PID_BRIDGE, PID_NATIVE, find_port, wait_port},
    report::{Report, Verdict},
    serial::R3,
};

use regex::{Regex as TextRegex, bytes::Regex};
use serialport::{ClearBuffer, SerialPort};
use std::{
    io::Write as _,
    time::{Duration, Instant},
};

/// The echo payload alphabet, as proven on the bench.
const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// The echo payload sizes, as proven on the bench.
const ECHO_LENGTHS: &[usize] = &[1, 7, 64, 65, 128, 255, 512, 1024, 4096];

/// What may trail an echo: an interrupted `tick N` line.
const TRAILING_TICK: &str = r"^(t(i(c(k( \d*\r?)?)?)?)?)?$";

/// Runs S9 and the knock.
pub fn run(r3: &mut R3, report: &mut Report) {
    let text = r3.send("u", r"S9 .*\r\n", 3, None);
    let native = wait_port(PID_NATIVE, true, 15);
    let bridge_held = find_port(PID_BRIDGE);
    let Some(native) = native else {
        report.add(
            "S9",
            Verdict::Fail,
            &format!("native port did not appear (R3: {})", text.trim()),
        );
        return;
    };
    let outcome = test_native(&native);
    let state = r3.send("s", r"STATUS .*\r?\n", 3, None);
    let configured = state.contains("STATUS usb=2 taken=1");
    let verdict = if outcome.starts_with("PASS") && configured && bridge_held.is_none() {
        Verdict::Pass
    } else {
        Verdict::Fail
    };
    let status = state
        .lines()
        .find(|line| line.starts_with("STATUS"))
        .unwrap_or_default();
    report.add(
        "S9",
        verdict,
        &format!(
            "native {native}, bridge while held: {}; {}; {}",
            bridge_held.as_deref().unwrap_or("absent"),
            outcome.strip_prefix("PASS ").unwrap_or(&outcome),
            status,
        ),
    );
    knock(&native, r3, report);
}

/// The knock: briefly opening the native port at 1200 baud must bring the
/// bridge back and leave the R4 silent (bootloader hold).
fn knock(native: &str, r3: &mut R3, report: &mut Report) {
    // Open, hold 300 ms, close — the baud rate itself is the signal.
    if let Ok(touch) = serialport::new(native, 1_200)
        .timeout(Duration::from_millis(100))
        .dtr_on_open(false)
        .open()
    {
        std::thread::sleep(Duration::from_millis(300));
        drop(touch);
    }
    let back = wait_port(PID_BRIDGE, true, 10);
    let gone = find_port(PID_NATIVE).is_none();
    std::thread::sleep(Duration::from_secs(2));
    let silent = !r3.identify_answered();
    report.add(
        "Knock",
        if back.is_some() && gone && silent {
            Verdict::Pass
        } else {
            Verdict::Fail
        },
        &format!(
            "bridge={} native gone={gone} R4 silent (bootloader hold)={silent}",
            back.as_deref().unwrap_or("none"),
        ),
    );
}

/// Native CDC: ticks for 6 s (>= 4 ticks, max gap 1.5 s), then printable
/// echoes of several sizes. Returns a "PASS …" summary text, or the
/// failure reason.
fn test_native(name: &str) -> String {
    let Ok(mut port) = serialport::new(name, 115_200)
        .timeout(Duration::from_millis(10))
        .dtr_on_open(true)
        .open()
    else {
        return format!("could not open native port {name}");
    };
    let _ = port.clear(ClearBuffer::Input);

    let mut tail: Vec<u8> = Vec::new();
    let mut last: i64 = -1;
    let mut count: u32 = 0;
    let mut gap: f64 = 0.0;
    let mut at: f64 = 0.0;
    let mut skip = true; // the first line may be cut by the discard
    let clock = Instant::now();
    let tick = TextRegex::new(r"^tick (\d+)$").unwrap();
    while clock.elapsed() < Duration::from_secs(6) {
        read_into(&mut port, &mut tail);
        while let Some(newline) = tail.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&tail[..newline])
                .trim_end_matches('\r')
                .to_owned();
            tail.drain(..=newline);
            if skip {
                skip = false;
                continue;
            }
            let Some(caps) = tick.captures(&line) else {
                return format!("malformed line '{line}'");
            };
            let number: i64 = caps.get(1).unwrap().as_str().parse().unwrap_or(-1);
            if last >= 0 && number != last + 1 {
                return format!("tick {last} -> {number}");
            }
            let now = clock.elapsed().as_secs_f64();
            if count > 0 {
                gap = gap.max(now - at);
            }
            at = now;
            last = number;
            count += 1;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    if count < 4 || gap > 1.5 {
        return format!("only {count} ticks, maximum gap {gap:.3} s");
    }

    let mut rng = Lcg::new(9009);
    let mut total = 0usize;
    let ticks = Regex::new(r"tick \d+\r\n").unwrap();
    let trailing = Regex::new(TRAILING_TICK).unwrap();
    for &length in ECHO_LENGTHS {
        let sent: Vec<u8> = (0..length)
            .map(|_| ALPHABET[rng.next_bounded(36)])
            .collect();
        // 4096 bytes take ~0.36 s at 115200: give the write a real timeout.
        let _ = port.set_timeout(Duration::from_secs(5));
        if port.write_all(&sent).is_err() {
            return format!("write of {length} bytes failed");
        }
        let _ = port.set_timeout(Duration::from_millis(10));
        let mut received: Vec<u8> = Vec::new();
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            read_into(&mut port, &mut received);
            if ticks.replace_all(&received, &b""[..]).len() >= length {
                break;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
        let echoed = ticks.replace_all(&received, &b""[..]);
        if echoed.len() < length || echoed[..length] != sent[..] {
            return format!("echo of {length} bytes failed");
        }
        if !trailing.is_match(&echoed[length..]) {
            return format!("extra bytes after {length}-byte echo");
        }
        total += length;
    }
    format!(
        "PASS {count} consecutive ticks (max gap {gap:.3} s), 9 exact echoes, {total} bytes each direction"
    )
}

/// Reads whatever is available right now from the port into `out`.
fn read_into(port: &mut Box<dyn SerialPort>, out: &mut Vec<u8>) {
    let mut buf = [0u8; 4096];
    loop {
        match port.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => out.extend_from_slice(&buf[..n]),
        }
    }
}

/// A tiny deterministic LCG; the payload content is arbitrary on this
/// link, only reproducibility is kept.
struct Lcg(u64);

impl Lcg {
    /// A new generator from its seed.
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    /// The next value below `bound`.
    fn next_bounded(&mut self, bound: u64) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) % bound) as usize
    }
}
