//! Artifact-stamp shape regression.

use crate::report::stamp;

#[test]
fn given_a_stamp_when_formatted_then_it_has_the_canonical_shape() {
    // given / when
    let stamp = stamp();
    // then
    assert_eq!(stamp.len(), 15); // yyyyMMdd-HHmmss
    assert_eq!(stamp.as_bytes()[8], b'-');
}
