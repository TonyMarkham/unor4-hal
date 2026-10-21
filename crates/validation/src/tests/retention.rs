//! Parsing of the R3's S10 watch lines.

use crate::checks::BootEvent;

const WATCH: &str = "S10 WATCH polling K; x ends, ! resets the R4\r\n\
                     S10 BASE 7 1\r\n\
                     RESET 1\r\n\
                     S10 GAP\r\n\
                     S10 BOOT 8 1 1 1\r\n\
                     S10 BOOT 10 1 2 0\r\n\
                     S10 BOOT 11 0 1 1\r\n\
                     S10 BOOT 1 1 -1023 1\r\n";

#[test]
fn given_watch_text_when_parsed_then_baseline_and_every_boot_in_order() {
    // given
    let text = WATCH;
    // when
    let base = BootEvent::baseline(text);
    let events = BootEvent::parse_all(text);
    // then
    assert_eq!(base, Some((7, true)));
    assert_eq!(events.len(), 4);
    assert_eq!(
        events[0],
        BootEvent {
            count: 8,
            intact: true,
            delta: 1,
            gap: true,
        }
    );
    assert_eq!(events[3].delta, -1023);
}

#[test]
fn given_boot_events_when_judged_then_only_intact_plus_one_or_wrap_is_single() {
    // given
    let events = BootEvent::parse_all(WATCH);
    // when
    let singles: Vec<bool> = events.iter().map(BootEvent::is_single_boot).collect();
    // then: +1 intact passes; +2 (a missed boot) fails; a broken log
    // fails; the restart at 1 after a full block passes
    assert_eq!(singles, [true, false, false, true]);
}

#[test]
fn given_no_watch_lines_when_parsed_then_nothing() {
    // given
    let text = "S9 TAKEN\r\nDONE 9 PASS 0 FAIL\r\n";
    // when / then
    assert_eq!(BootEvent::baseline(text), None);
    assert!(BootEvent::parse_all(text).is_empty());
}
