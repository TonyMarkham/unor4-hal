//! Recovery after ticks that were not polled: the schedule rebuilt from the
//! registers, the idle stop, and a reversal applied at a missed tick.

use crate::step::{
    RESYNC_SLACK, Schedule, idle_expired, missed_reversal_new_pulses, resync_schedule,
};

/// The 2 ms tick at 48 MHz, PCLKD counts.
const TICK: u32 = 96_000;
/// The idle "period" (`IDLE_LEN`).
const IDLE: u32 = u32::MAX;
/// 1 s of PCLKD counts.
const ONE_SECOND: u64 = 48_000_000;

#[test]
fn given_no_overflow_since_the_tick_when_resynced_then_the_registers_give_the_schedule() {
    // given: phase 1 000 at the tick; 9 600 counts later the counter moved 9 605
    let (phase, since) = (1_000, 9_600);

    // when
    let schedule = resync_schedule(phase, phase + since + 5, since, 3_999, 3_999);

    // then
    assert_eq!(schedule, Some(Schedule::steady(1_000, 4_000)));
}

#[test]
fn given_a_transition_period_in_progress_when_resynced_then_current_and_steady_differ() {
    // given / when: GTPR holds a 6 000-count transition, GTPBR the 3 000 steady period
    let schedule = resync_schedule(500, 500 + 2_000, 2_000, 5_999, 2_999);

    // then
    assert_eq!(
        schedule,
        Some(Schedule {
            phase: 500,
            current: 6_000,
            next: 3_000,
            period: 3_000
        })
    );
}

#[test]
fn given_a_steady_train_that_wrapped_below_the_phase_when_resynced_then_the_schedule_is_rebuilt() {
    // given: a 2 000-count period; 9 600 counts after phase 1 000 a steady train
    // stands at (1 000 + 9 600) mod 2 000 = 600
    // when
    let schedule = resync_schedule(1_000, 600, 9_600, 1_999, 1_999);

    // then
    assert_eq!(schedule, Some(Schedule::steady(1_000, 2_000)));
}

#[test]
fn given_a_steady_train_that_wrapped_above_the_phase_when_resynced_then_the_schedule_is_rebuilt() {
    // given: a 5 000-count period; 9 600 counts after phase 100 a steady train
    // stands at (100 + 9 600) mod 5 000 = 4 700
    // when
    let schedule = resync_schedule(100, 4_700, 9_600, 4_999, 4_999);

    // then
    assert_eq!(schedule, Some(Schedule::steady(100, 5_000)));
}

#[test]
fn given_the_tick_just_before_a_period_end_when_resynced_after_the_wrap_then_it_is_rebuilt() {
    // given: the board's failing case, 50 000/s (960 counts) with the tick 122
    // counts before a period's end; the reads come 260 counts after the tick,
    // the counter 3 counts further on by the read order
    let (phase, since) = (838, 260);
    let count = (phase + since) % 960 + 3;

    // when
    let schedule = resync_schedule(phase, count, since, 959, 959);

    // then
    assert_eq!(schedule, Some(Schedule::steady(838, 960)));
}

#[test]
fn given_a_wrap_with_the_counter_off_the_steady_position_when_resynced_then_it_is_refused() {
    // given: a 960-count period, the steady position 138, the counter at 300
    // when
    let schedule = resync_schedule(838, 300, 260, 959, 959);

    // then
    assert_eq!(schedule, None);
}

#[test]
fn given_a_wrap_while_the_period_registers_differ_when_resynced_then_it_is_refused() {
    // given: GTPR 1 199, GTPBR 959 (a period change in the registers), and a
    // wrap since the tick: the period at the tick is not provable
    // when
    let schedule = resync_schedule(1_100, 140, 200, 1_199, 959);

    // then
    assert_eq!(schedule, None);
}

#[test]
fn given_a_wrap_with_the_counter_at_the_slack_edges_when_resynced_then_only_inside_is_accepted() {
    // given: a 10 000-count period, phase 9 900, 1 000 counts later: steady
    // position 900; accepted from slack behind to twice the slack ahead
    let (phase, since, steady) = (9_900, 1_000, 900);

    // when
    let behind = resync_schedule(phase, steady - RESYNC_SLACK, since, 9_999, 9_999);
    let too_far_behind = resync_schedule(phase, steady - RESYNC_SLACK - 1, since, 9_999, 9_999);
    let ahead = resync_schedule(phase, steady + 2 * RESYNC_SLACK, since, 9_999, 9_999);
    let too_far_ahead = resync_schedule(phase, steady + 2 * RESYNC_SLACK + 1, since, 9_999, 9_999);

    // then
    assert_eq!(behind, Some(Schedule::steady(phase, 10_000)));
    assert_eq!(too_far_behind, None);
    assert_eq!(ahead, Some(Schedule::steady(phase, 10_000)));
    assert_eq!(too_far_ahead, None);
}

#[test]
fn given_the_shortest_period_wrapping_once_when_resynced_then_it_is_refused() {
    // given: 50 counts (LOW 1 + HIGH 48 + 1), a wrap takes 50 counts off what the
    // tick timer says, plus up to 10 counts of read latency: moved = 400 + 10 - 50
    // when
    let schedule = resync_schedule(10, 10 + 360, 400, 49_000, 49_000);

    // then
    assert_eq!(schedule, None);
}

#[test]
fn given_the_counter_exactly_at_the_slack_when_resynced_then_it_is_accepted_and_one_below_refused()
{
    // given
    let (phase, since) = (0, 1_000);

    // when
    let at = resync_schedule(phase, since - RESYNC_SLACK, since, 9_999, 9_999);
    let below = resync_schedule(phase, since - RESYNC_SLACK - 1, since, 9_999, 9_999);

    // then
    assert_eq!(at, Some(Schedule::steady(0, 10_000)));
    assert_eq!(below, None);
}

#[test]
fn given_a_phase_past_gtpr_when_resynced_then_it_is_refused() {
    // given / when: 0 ≤ GTCNT ≤ GTPR always holds (§22.9.3), so this is a bad read
    let schedule = resync_schedule(5_000, 6_000, 1_000, 4_999, 4_999);

    // then
    assert_eq!(schedule, None);
}

#[test]
fn given_idle_registers_and_a_phase_over_a_second_when_checked_then_the_idle_stop_fires() {
    // given / when / then
    assert!(idle_expired(
        48_000_001,
        IDLE - 1,
        IDLE - 1,
        IDLE,
        ONE_SECOND
    ));
    assert!(!idle_expired(
        48_000_000,
        IDLE - 1,
        IDLE - 1,
        IDLE,
        ONE_SECOND
    ));
}

#[test]
fn given_a_running_period_in_either_register_when_checked_then_the_idle_stop_does_not_fire() {
    // given / when / then: a pulse is still due, whatever the phase
    assert!(!idle_expired(60_000_000, 3_999, IDLE - 1, IDLE, ONE_SECOND));
    assert!(!idle_expired(60_000_000, IDLE - 1, 3_999, IDLE, ONE_SECOND));
}

#[test]
fn given_a_missed_reversal_with_a_pulse_low_at_the_tick_when_split_then_only_later_pulses_are_new()
{
    // given: 2 000-count periods, phase 0 at the polled tick; pulse ends latch at 242
    let before = Schedule::steady(0, 2_000);
    // 48 pulse ends before the reversal tick (242, 2 242, …, 94 242), the pulse
    // starting at the reversal tick is LOW at it (old), then 10 new-direction ends
    let delta = 48 + 1 + 10;

    // when: the read-back shows the chain ran in order (no old pulse first)
    let new = missed_reversal_new_pulses(before, TICK, 242, 1_000_000, 300, delta);

    // then
    assert_eq!(new, 10);
}

#[test]
fn given_a_missed_reversal_where_the_old_pulse_fired_first_when_split_then_it_counts_old() {
    // given: phase 1 500 → at the reversal tick the phase is 1 500, not LOW
    let before = Schedule::steady(1_500, 2_000);
    let ended_before = before.ends_in(242, TICK);
    let delta = ended_before + 1 + 7;

    // when: the read-back is below phase + chain minimum → the old period's pulse fired first
    let new = missed_reversal_new_pulses(before, TICK, 242, 1_500 + 100, 300, delta);

    // then
    assert_eq!(new, 7);
}

#[test]
fn given_fewer_pulses_than_the_old_direction_owns_when_split_then_none_are_new() {
    // given / when
    let new = missed_reversal_new_pulses(Schedule::steady(0, 2_000), TICK, 242, 1_000_000, 300, 3);

    // then
    assert_eq!(new, 0);
}
