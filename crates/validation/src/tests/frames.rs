//! `strip_frames` regressions.

use crate::serial::strip_frames;

#[test]
fn given_framed_text_when_stripped_then_frames_are_removed() {
    // given
    let text = "before\u{2}hidden\u{3}after";
    // when
    let stripped = strip_frames(text);
    // then
    assert_eq!(stripped, "beforeafter");
}

#[test]
fn given_an_unterminated_frame_when_stripped_then_the_rest_is_hidden() {
    // given
    let text = "before\u{2}hidden";
    // when
    let stripped = strip_frames(text);
    // then
    assert_eq!(stripped, "before");
}
