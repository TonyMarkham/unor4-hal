//! The R3 identify check: the validator answers 'i' with its ID line.

use crate::{
    report::{Report, Verdict},
    serial::R3,
};

/// Sends 'i' and records whether the validator answered.
pub fn run(r3: &mut R3, report: &mut Report) {
    let answered = r3.identify_answered();
    report.add(
        "Identify",
        if answered {
            Verdict::Pass
        } else {
            Verdict::Fail
        },
        &format!("validator answers identify: {answered}"),
    );
}
