//! Slice-name selection regressions.

use crate::checks::requested;

#[test]
fn given_a_slice_prefix_when_requested_then_its_variants_match() {
    // given
    let names = ["S2".to_owned()];
    // when / then
    assert!(requested(&names, "S2 input"));
    assert!(requested(&names, "S2 button"));
    assert!(requested(&names, "S2 wake"));
    assert!(requested(&names, "S2 pullup"));
    assert!(!requested(&names, "S3"));
}

#[test]
fn given_s11_when_requested_then_its_checks_match_and_s1_does_not() {
    // given
    let names = ["S11".to_owned()];
    // when / then
    assert!(requested(&names, "S11 step"));
    assert!(requested(&names, "S11 count"));
    assert!(requested(&names, "S11 tick"));
    assert!(requested(&names, "S11 queue"));
    assert!(requested(&names, "S11 scenario"));
    assert!(requested(&names, "S11 reverse"));
    assert!(requested(&names, "S11 wake"));
    assert!(requested(&names, "S11 dirlead"));
    assert!(requested(&names, "S11 countstop"));
    assert!(requested(&names, "S11 cancelrace"));
    assert!(requested(&names, "S11 sleep"));
    assert!(requested(&names, "S11 boot"));
    assert!(requested(&names, "S11 elc"));
    assert!(!requested(&names, "S1"));
}

#[test]
fn given_all_when_requested_then_every_check_matches() {
    // given
    let names = ["all".to_owned()];
    // when / then
    assert!(requested(&names, "S1"));
    assert!(requested(&names, "S5 traits"));
}

#[test]
fn given_s12_when_requested_then_the_wake_timer_matches_and_s1_does_not() {
    // given
    let names = ["S12".to_owned()];
    // when / then
    assert!(requested(&names, "S12 wake"));
    assert!(!requested(&names, "S1"));
}
