//! The R3-driven reset bits: '!' single reset, '@' bootloader hold.

use crate::{
    discover::{PID_BRIDGE, find_port},
    report::{Report, Verdict},
    serial::R3,
};

use std::time::Duration;

/// The single reset bit: '!' must restart the validator, which then answers
/// identify again.
pub fn single(r3: &mut R3, report: &mut Report) {
    let _ = r3.send("!", r"RESET 1", 2, None);
    std::thread::sleep(Duration::from_secs(3));
    let restarted = r3.identify_answered();
    report.add(
        "R3 !",
        if restarted {
            Verdict::Pass
        } else {
            Verdict::Fail
        },
        &format!("validator answers identify after a single RESET pulse: {restarted}"),
    );
}

/// The bootloader-hold bit: '@' must hold the bootloader (R4 silent, bridge
/// present), and a final '!' must restart the validator again.
pub fn double(r3: &mut R3, report: &mut Report) {
    let _ = r3.send("@", r"RESET 2", 2, None);
    std::thread::sleep(Duration::from_secs(3));
    let held = !r3.identify_answered();
    let bridge_up = find_port(PID_BRIDGE).is_some();
    let _ = r3.send("!", r"RESET 1", 2, None);
    std::thread::sleep(Duration::from_secs(3));
    let again = r3.identify_answered();
    report.add(
        "R3 @",
        if held && bridge_up && again {
            Verdict::Pass
        } else {
            Verdict::Fail
        },
        &format!(
            "bootloader hold (silent)={held} bridge present={bridge_up}; restarted by ! ={again}"
        ),
    );
}

/// Both reset bits in sequence, ending the full `validate all` run.
pub fn run(r3: &mut R3, report: &mut Report) {
    single(r3, report);
    double(r3, report);
}
