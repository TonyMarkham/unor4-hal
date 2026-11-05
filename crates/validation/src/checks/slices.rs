//! S1-S8, the S10 flash self-test and the S11 step generator (R4 D4 ->
//! R3 D8): one R3 'a' burst of the loaded validator part, parsed into
//! per-slice lines, plus the S4 token check against the bridge monitor
//! text. Part 1 runs S1-S10, part 2 runs S11.

use crate::{
    report::{Report, Verdict},
    serial::{Monitor, R3},
};

use regex::Regex;
use std::{collections::HashSet, time::Duration};

/// The per-slice result-line patterns, in the bench's report order: the R3
/// validator part that runs the check, its name, and its pattern (group 1
/// is the verdict word, group 2 the evidence).
///
/// `pub(crate)`: the host tests in `src/tests` exercise it; it is not
/// part of the CLI's API.
pub(crate) const LINE_PATTERNS: &[(u8, &str, &str)] = &[
    (1, "S1", r"S1 (PASS|FAIL) ?(.*)"),
    (1, "S2 input", r"S2 (PASS|FAIL) ?(R4 D3 read.*|no reply.*)"),
    (
        1,
        "S2 button",
        r"S2 (PASS|FAIL) ?(button on D3.*|no reply.*)",
    ),
    (1, "S2 wake", r"S2 (PASS|FAIL) ?(switch wake: .*)"),
    (1, "S2 pullup", r"S2 (PASS|FAIL) ?(switch pullup: .*)"),
    (1, "S3", r"S3 (PASS|FAIL) ?(.*)"),
    (1, "S5", r"S5 (PASS|FAIL) ?(echoed.*|no reply.*)"),
    (1, "S5 traits", r"S5 (PASS|FAIL) ?(traits: .*|no reply.*)"),
    (1, "S6", r"S6 (PASS|FAIL|NA) ?(.*)"),
    (1, "S7", r"S7 (PASS|FAIL) ?(.*)"),
    (1, "S8", r"S8 (PASS|FAIL) ?(.*)"),
    (1, "S10", r"S10 (PASS|FAIL) ?(flash self-test.*|no reply.*)"),
    (2, "S11 step", r"S11 (PASS|FAIL) ?(step: .*)"),
    (2, "S11 count", r"S11 (PASS|FAIL) ?(count: .*)"),
    (2, "S11 tick", r"S11 (PASS|FAIL) ?(tick: .*)"),
    (2, "S11 queue", r"S11 (PASS|FAIL) ?(queue: .*)"),
    (2, "S11 scenario", r"S11 (PASS|FAIL) ?(scenario: .*)"),
    (2, "S11 reverse", r"S11 (PASS|FAIL) ?(reverse: .*)"),
    (2, "S11 wake", r"S11 (PASS|FAIL) ?(wake: .*)"),
    (2, "S11 skip", r"S11 (PASS|FAIL) ?(skip: .*)"),
    (3, "S11 dirlead", r"S11 (PASS|FAIL) ?(dirlead: .*)"),
    (3, "S11 countstop", r"S11 (PASS|FAIL) ?(countstop: .*)"),
    (3, "S11 cancelrace", r"S11 (PASS|FAIL) ?(cancelrace: .*)"),
    (3, "S11 sleep", r"S11 (PASS|FAIL) ?(sleep: .*)"),
    (2, "S11 elc", r"S11 (PASS|FAIL) ?(elc: .*)"),
    (3, "S11 boot", r"S11 (PASS|FAIL) ?(boot: .*)"),
    (3, "S12 wake", r"S12 (PASS|FAIL) ?(wake: .*)"),
];

/// True when `check_name` is selected by the requested slice names: "all",
/// an exact name, or a slice prefix selecting its variants ("S2" selects
/// both "S2 input" and "S2 button").
///
/// `pub(crate)`: the host tests in `src/tests` exercise it; it is not
/// part of the CLI's API.
pub(crate) fn requested(names: &[String], check_name: &str) -> bool {
    names.iter().any(|name| {
        name == "all"
            || name == check_name
            || check_name
                .strip_prefix(name.as_str())
                .is_some_and(|rest| rest.starts_with(' '))
    })
}

/// The R3 validator part that runs S4 (its token goes out with part 1's
/// checks).
pub const S4_PART: u8 = 1;

/// Records a check the loaded R3 part cannot run as not run, naming the
/// part that runs it.
pub fn not_run(check_name: &str, part: u8, report: &mut Report) {
    report.add(
        check_name,
        Verdict::NotRun,
        &format!("runs on R3 part {part}: upload it and run again (Validation.md)"),
    );
}

/// Runs the loaded R3 part's checks (R3 'a') and, on part 1, S4: records
/// every requested slice line of that part, records the other parts'
/// requested checks as not run, logs the R3 and monitor dumps to the
/// artifacts, and checks the S4 token against the monitor text.
pub fn run(
    r3: &mut R3,
    monitor: &mut Monitor,
    bridge: &str,
    names: &[String],
    part: u8,
    report: &mut Report,
) {
    // The R3's 'a' run takes about 60 s on part 2 (Slice 11's first checks);
    // 120 s leaves room.
    let text = r3.send("a", r"DONE \d+ PASS \d+ FAIL\r\n", 120, Some(&mut *monitor));
    std::thread::sleep(Duration::from_millis(300));
    monitor.drain();
    let _ = report.append_log("r3", &text);

    let mut taken: HashSet<usize> = HashSet::new();
    for (check_part, check_name, pattern) in LINE_PATTERNS {
        if !requested(names, check_name) {
            continue;
        }
        if *check_part != part {
            not_run(check_name, *check_part, report);
            continue;
        }
        let re = Regex::new(&format!(r"(?m){pattern}\r?$")).unwrap();
        // S2 and S5 each print two lines that differ only in their
        // evidence; take each line once.
        let found = re
            .captures_iter(&text)
            .find(|caps| !taken.contains(&caps.get(0).unwrap().start()));
        match found {
            Some(caps) => {
                taken.insert(caps.get(0).unwrap().start());
                report.add(
                    check_name,
                    Verdict::parse(caps.get(1).unwrap().as_str()),
                    caps.get(2).map(|m| m.as_str().trim()).unwrap_or_default(),
                );
            }
            None => report.add(check_name, Verdict::Fail, "no result line from the R3"),
        }
    }

    if names.iter().any(|name| name == "all" || name == "S4") {
        if part == S4_PART {
            s4(&text, monitor, bridge, report);
        } else {
            not_run("S4", S4_PART, report);
        }
    }
    let _ = report.append_log("monitor", monitor.text());
}

/// The S4 token check: the R3 sends a token over SCI9; the bridge monitor
/// must show the R4 receiving it.
fn s4(text: &str, monitor: &Monitor, bridge: &str, report: &mut Report) {
    let token = Regex::new(r"S4 SENT TOKEN (\d+)")
        .unwrap()
        .captures(text)
        .and_then(|caps| caps.get(1));
    match token {
        None => report.add("S4", Verdict::Fail, "no token from the R3"),
        Some(token) => {
            let token = token.as_str();
            let seen = Regex::new(&format!(r"VALIDATE S4 TOKEN {token}\r?\n"))
                .unwrap()
                .is_match(monitor.text());
            if seen {
                report.add(
                    "S4",
                    Verdict::Pass,
                    &format!("token {token} seen on the bridge monitor {bridge}"),
                );
            } else {
                report.add(
                    "S4",
                    Verdict::Fail,
                    &format!("token {token} not seen on {bridge}"),
                );
            }
        }
    }
}
