//! The ENA lead: when `is_enabled` may turn true.

use crate::step::lead_passed;

/// A 5 µs lead in tick-timer counts (PCLKD/4 = 12 MHz): 60.
const LEAD_5_US: u64 = 60;

#[test]
fn given_a_lead_when_exactly_its_counts_elapsed_then_it_has_not_passed() {
    // given: the enable stamp may sit just before a count step, so 60 counts
    // read prove only 59 counts of real time
    // when
    let passed = lead_passed(LEAD_5_US, LEAD_5_US);
    // then
    assert!(!passed);
}

#[test]
fn given_a_lead_when_one_count_more_elapsed_then_it_has_passed() {
    // given / when
    let passed = lead_passed(LEAD_5_US + 1, LEAD_5_US);
    // then
    assert!(passed);
}

#[test]
fn given_any_lead_when_counts_are_read_then_the_real_lead_is_never_short() {
    // given: real time covered by `elapsed` counts read is at least
    // `elapsed - 1` counts
    for lead in [1u64, 2, 60, 12_000, 2_400_000] {
        for elapsed in 0..lead + 3 {
            // when
            let passed = lead_passed(elapsed, lead);
            // then
            if passed {
                assert!(elapsed - 1 >= lead, "lead {lead}, elapsed {elapsed}");
            }
        }
    }
}

#[test]
fn given_no_lead_when_checked_then_it_has_passed_at_once() {
    // given / when
    let passed = lead_passed(0, 0);
    // then
    assert!(passed);
}
