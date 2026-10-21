//! S1-S8 and the S10 flash self-test: one R3 'a' burst, parsed into
//! per-slice lines, plus the S4 token check against the bridge monitor text.

use crate::{
    report::{Report, Verdict},
    serial::{Monitor, R3},
};

use regex::Regex;
use std::{collections::HashSet, time::Duration};

/// The per-slice result-line patterns, in the bench's report order:
/// group 1 is the verdict word, group 2 the evidence.
const LINE_PATTERNS: &[(&str, &str)] = &[
    ("S1", r"S1 (PASS|FAIL) ?(.*)"),
    ("S2 input", r"S2 (PASS|FAIL) ?(R4 D3 read.*|no reply.*)"),
    ("S2 button", r"S2 (PASS|FAIL) ?(button on D3.*|no reply.*)"),
    ("S3", r"S3 (PASS|FAIL) ?(.*)"),
    ("S5", r"S5 (PASS|FAIL) ?(echoed.*|no reply.*)"),
    ("S5 traits", r"S5 (PASS|FAIL) ?(traits: .*|no reply.*)"),
    ("S6", r"S6 (PASS|FAIL|NA) ?(.*)"),
    ("S7", r"S7 (PASS|FAIL) ?(.*)"),
    ("S8", r"S8 (PASS|FAIL) ?(.*)"),
    ("S10", r"S10 (PASS|FAIL) ?(flash self-test.*|no reply.*)"),
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

/// Runs S1-S8 (R3 'a') and S4: records every requested slice line, logs the
/// R3 and monitor dumps to the artifacts, and checks the S4 token against
/// the monitor text.
pub fn run(
    r3: &mut R3,
    monitor: &mut Monitor,
    bridge: &str,
    names: &[String],
    report: &mut Report,
) {
    let text = r3.send("a", r"DONE \d+ PASS \d+ FAIL\r\n", 60, Some(&mut *monitor));
    std::thread::sleep(Duration::from_millis(300));
    monitor.drain();
    let _ = report.append_log("r3", &text);

    let mut taken: HashSet<usize> = HashSet::new();
    for (check_name, pattern) in LINE_PATTERNS {
        if !requested(names, check_name) {
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
        s4(&text, monitor, bridge, report);
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
