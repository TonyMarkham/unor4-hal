//! `Verdict` parsing and counting regressions.

use crate::report::Verdict;

#[test]
fn given_verdict_words_when_parsed_then_the_matching_verdicts_result() {
    // given
    let words = ["PASS", "FAIL", "NA"];
    // when
    let parsed: Vec<Verdict> = words.iter().map(|word| Verdict::parse(word)).collect();
    // then
    assert_eq!(
        parsed,
        [Verdict::Pass, Verdict::Fail, Verdict::NotApplicable]
    );
}

#[test]
fn given_the_verdicts_when_counted_then_only_fail_counts() {
    // given
    let verdicts = [Verdict::Pass, Verdict::Fail, Verdict::NotApplicable];
    // when
    let failed: usize = verdicts.iter().filter(|v| v.is_failed()).count();
    // then
    assert_eq!(failed, 1);
}
