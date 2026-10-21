//! Check-table formatting regressions.

use crate::report::{CheckResult, Verdict, table, wrap};

#[test]
fn given_long_evidence_when_tabulated_then_lines_stay_within_200_columns() {
    // given
    let results = [CheckResult {
        check: "S1".to_owned(),
        verdict: Verdict::Pass,
        evidence: "evidence ".repeat(40).trim_end().to_owned(),
    }];
    // when
    let rendered = table(&results);
    // then
    assert!(rendered.lines().all(|line| line.chars().count() <= 200));
}

#[test]
fn given_text_wider_than_the_wrap_when_wrapped_then_the_lines_fit() {
    // given
    let text = "aa bb cc dd ee";
    // when
    let lines = wrap(text, 5);
    // then
    assert_eq!(lines, ["aa bb", "cc dd", "ee"]);
}
