//! Host tests for a start's first pulse: how long it waits (`start_lead`)
//! and the first cycle that makes it wait that long (`first_cycle`). The
//! register writes themselves are target code, proven on the board
//! (Validation.md, `S11 dirlead`).

use crate::step::{first_cycle, start_lead};

/// The validator's pulse shape at 48 MHz: LOW 5 µs, minimum HIGH 2.5 µs.
const LOW: u32 = 240;
const MIN_HIGH: u32 = 120;
/// A 5 µs DIR set-up.
const DIR_SETUP: u32 = 240;
/// The queue guard, 40 µs.
const GUARD: u32 = 1_920;

#[test]
fn given_no_dir_change_when_the_lead_is_taken_then_it_is_the_minimum_high_time() {
    // given / when
    let lead = start_lead(false, MIN_HIGH, DIR_SETUP, 0);

    // then
    assert_eq!(lead, MIN_HIGH);
}

#[test]
fn given_a_dir_change_from_the_cpu_when_the_lead_is_taken_then_it_is_the_dir_setup() {
    // given / when: `run` writes DIR right before it starts the counter
    let lead = start_lead(true, MIN_HIGH, DIR_SETUP, 0);

    // then
    assert_eq!(lead, DIR_SETUP);
}

#[test]
fn given_a_dir_setup_shorter_than_the_minimum_high_when_the_lead_is_taken_then_min_high_wins() {
    // given / when
    let lead = start_lead(true, MIN_HIGH, 100, 0);

    // then
    assert_eq!(lead, MIN_HIGH);
}

#[test]
fn given_a_dir_change_a_guard_before_the_tick_when_the_setup_fits_then_min_high_is_left() {
    // given / when: DIR written at least 40 µs before the tick; 5 µs fits
    let lead = start_lead(true, MIN_HIGH, DIR_SETUP, GUARD);

    // then
    assert_eq!(lead, MIN_HIGH);
}

#[test]
fn given_a_dir_change_a_guard_before_the_tick_when_the_setup_is_longer_then_the_rest_is_waited() {
    // given / when: a 100 µs set-up (4 800 counts) less the 1 920-count head start
    let lead = start_lead(true, MIN_HIGH, 4_800, GUARD);

    // then
    assert_eq!(lead, 2_880);
}

#[test]
fn given_the_minimum_high_lead_when_the_first_cycle_is_set_then_it_is_one_steady_period() {
    // given: 1 000 pulses/s, 48 000 counts
    let period = 48_000;

    // when
    let (top, preload) = first_cycle(period, LOW, MIN_HIGH);

    // then: the old preload, period − min_high, and no longer cycle
    assert_eq!((top, preload), (period - 1, period - MIN_HIGH));
}

#[test]
fn given_a_lead_that_fits_when_the_first_cycle_is_set_then_the_overflow_comes_after_the_lead() {
    // given: 1 000 pulses/s, a 5 µs DIR set-up lead
    let period = 48_000;

    // when
    let (top, preload) = first_cycle(period, LOW, DIR_SETUP);

    // then: counts from the preload to the overflow
    assert_eq!((top, top + 1 - preload), (period - 1, DIR_SETUP));
}

#[test]
fn given_a_lead_longer_than_a_period_allows_when_the_first_cycle_is_set_then_it_is_longer() {
    // given: the fastest period for the shape (low + min_high + 1 = 361) and a
    // 1 ms DIR set-up (48 000 counts)
    let (period, lead) = (361, 48_000);

    // when
    let (top, preload) = first_cycle(period, LOW, lead);

    // then: the first cycle holds the lead after a preload just past LOW
    assert_eq!((top, preload), (lead + LOW, LOW + 1));
    assert_eq!(top + 1 - preload, lead);
}

#[test]
fn given_any_lead_when_the_first_cycle_is_set_then_no_compare_match_comes_before_the_first_pulse() {
    // given / when / then: the preload is past the LOW compare value and
    // inside the first period, for leads around every boundary
    for period in [361, 362, 1_000, 48_000] {
        for lead in [1, MIN_HIGH, period - LOW - 1, period - LOW, period, 48_000] {
            let (top, preload) = first_cycle(period, LOW, lead);
            assert!(preload > LOW, "period {period}, lead {lead}");
            assert!(preload <= top, "period {period}, lead {lead}");
            assert!(top >= period - 1, "period {period}, lead {lead}");
            assert_eq!(top + 1 - preload, lead, "period {period}, lead {lead}");
        }
    }
}
