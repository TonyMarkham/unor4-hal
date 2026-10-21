//! S10 retention: the R4's data flash boot counter, watched by the R3
//! (`k`), must advance by one per boot and read back intact.

use crate::{
    checks::boot_event::BootEvent,
    report::{Report, Verdict},
    serial::R3,
};

use std::time::{Duration, Instant};

/// The R3's watch summary line (sent after `x`).
const WATCH_DONE: &str = r"S10 (PASS|FAIL) watch: .*\r\n";

/// Retention across resets: `resets` R3 `!` pulses inside the watch;
/// each must show exactly one boot with the counter advanced by one.
pub fn resets(r3: &mut R3, resets: u32, report: &mut Report) {
    let mut text = r3.send("k", r"S10 BASE \d+ \d+\r\n", 5, None);
    let Some((base, base_intact)) = BootEvent::baseline(&text) else {
        report.add(
            "S10 retention",
            Verdict::Fail,
            &format!("no baseline from the watch (R3: {})", text.trim()),
        );
        let _ = r3.send("x", WATCH_DONE, 5, None);
        return;
    };
    for _ in 0..resets {
        text.push_str(&r3.send("!", r"S10 BOOT .*\r\n", 10, None));
    }
    text.push_str(&r3.send("x", WATCH_DONE, 5, None));
    let _ = report.append_log("r3-s10", &text);

    let events = BootEvent::parse_all(&text);
    let singles = events.iter().filter(|e| e.is_single_boot()).count();
    let last = events.last().map_or(base, |e| e.count);
    let pass = base_intact && events.len() == resets as usize && singles == events.len();
    report.add(
        "S10 retention",
        if pass { Verdict::Pass } else { Verdict::Fail },
        &format!(
            "{}/{resets} resets each +1 with the log intact; counter {base} -> {last}",
            singles
        ),
    );
}

/// Retention across power cycles: watch for `seconds` while the owner
/// power-cycles the R4 (barrel jack, USB unplugged); at least
/// `min_boots` boot events, every one +1 and intact.
pub fn watch(r3: &mut R3, seconds: u64, min_boots: usize, report: &mut Report) {
    println!(
        "S10 watch for {seconds} s: power-cycle the R4 (jack, USB unplugged) {min_boots}+ times"
    );
    let mut text = r3.send("k", r"S10 BASE \d+ \d+\r\n", 10, None);
    print!("{text}");
    let deadline = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < deadline {
        let chunk = r3.drain();
        print!("{chunk}");
        text.push_str(&chunk);
        std::thread::sleep(Duration::from_millis(100));
    }
    text.push_str(&r3.drain());
    let done = r3.send("x", WATCH_DONE, 5, None);
    print!("{done}");
    text.push_str(&done);
    let _ = report.append_log("r3-s10", &text);

    let base = BootEvent::baseline(&text);
    let events = BootEvent::parse_all(&text);
    let bad = events.iter().filter(|e| !e.is_single_boot()).count();
    let gaps = events.iter().filter(|e| e.gap).count();
    let pass = base.is_some_and(|(_, intact)| intact) && events.len() >= min_boots && bad == 0;
    report.add(
        "S10 power cycles",
        if pass { Verdict::Pass } else { Verdict::Fail },
        &format!(
            "{} boot events ({gaps} after a gap), {bad} not +1/intact, need {min_boots}; counter {} -> {}",
            events.len(),
            base.map_or(-1, |(count, _)| count),
            events.last().map_or(-1, |e| e.count),
        ),
    );
}
