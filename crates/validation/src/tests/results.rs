//! Check-table formatting regressions.

use crate::report::{CheckResult, Verdict, summary, table, wrap};

#[test]
fn given_checks_of_the_other_part_when_summarised_then_they_are_not_counted_as_passed() {
    // given
    let (total, failed, not_run) = (20, 1, 6);
    // when
    let line = summary(total, failed, not_run);
    // then
    assert_eq!(
        line,
        "13/14 checks passed (1 failed); 6 not run: they need another R3 part (Validation.md)"
    );
}

#[test]
fn given_every_check_run_when_summarised_then_no_not_run_note() {
    // given
    let (total, failed, not_run) = (5, 0, 0);
    // when
    let line = summary(total, failed, not_run);
    // then
    assert_eq!(line, "5/5 checks passed (0 failed)");
}

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
