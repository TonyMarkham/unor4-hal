//! Slice-name selection regressions.

use crate::checks::requested;

#[test]
fn given_a_slice_prefix_when_requested_then_its_variants_match() {
    // given
    let names = ["S2".to_owned()];
    // when / then
    assert!(requested(&names, "S2 input"));
    assert!(requested(&names, "S2 button"));
    assert!(!requested(&names, "S3"));
}

#[test]
fn given_all_when_requested_then_every_check_matches() {
    // given
    let names = ["all".to_owned()];
    // when / then
    assert!(requested(&names, "S1"));
    assert!(requested(&names, "S5 traits"));
}
