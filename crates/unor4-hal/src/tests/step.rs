//! Host tests for the step generator's period maths. The register paths
//! (waveform, start, stop) are target code — proven by Slice 11 step 2's
//! hardware run (Validation.md, `S11 step`), not here.

use crate::step::{Schedule, StepError, calc_step_period, ns_to_counts, pulses_in_tick};

/// Brute force for `Schedule`: lists every pulse start (offset from the
/// tick instant, possibly negative for the current period's) up to `span`.
fn starts(schedule: Schedule, span: i64) -> Vec<i64> {
    let mut list = vec![-i64::from(schedule.phase)];
    let mut at = i64::from(schedule.current) - i64::from(schedule.phase);
    list.push(at);
    at += i64::from(schedule.next);
    while at <= span {
        list.push(at);
        at += i64::from(schedule.period);
    }
    list
}

/// The 2 ms tick at 48 MHz, in PCLKD counts.
const TICK: u32 = 96_000;

/// Brute force for `pulses_in_tick`: pulse k ends at `low − phase + k ×
/// period`; count those at offsets 0 ≤ t < tick.
fn ends_by_brute_force(period: u32, low: u32, phase: u32, tick: u32) -> u32 {
    let first = i64::from(low) - i64::from(phase);
    (-2..=i64::from(tick / period) + 2)
        .map(|k| first + k * i64::from(period))
        .filter(|&t| t >= 0 && t < i64::from(tick))
        .count() as u32
}

/// PCLKD on both boards (HOCO 48 MHz).
const PCLKD: u32 = 48_000_000;
/// The cart-pole pulse shape in counts: 5 000 ns low, 2 500 ns high.
const LOW: u32 = 240;
const HIGH: u32 = 120;

#[test]
fn given_the_rig_pulse_shape_when_converted_then_counts_are_240_and_120() {
    // given: 5 000 ns low, 2 500 ns minimum high at 48 MHz
    // when: converted to counts
    // then: exact multiples of 20.83 ns
    assert_eq!(ns_to_counts(5_000, PCLKD), 240);
    assert_eq!(ns_to_counts(2_500, PCLKD), 120);
}

#[test]
fn given_a_time_between_counts_when_converted_then_it_rounds_up() {
    // given: 30 ns, between one count (20.8 ns) and two
    // when: converted
    // then: two counts — a minimum time is never undershot
    assert_eq!(ns_to_counts(30, PCLKD), 2);
    assert_eq!(ns_to_counts(0, PCLKD), 0);
}

#[test]
fn given_the_rig_rates_when_resolved_then_periods_are_the_nearest_counts() {
    // given: the rates the cart-pole rig uses
    // when: resolved to periods
    // then: round(48 MHz / rate)
    assert_eq!(calc_step_period(PCLKD, 1, LOW, HIGH), Ok(48_000_000));
    assert_eq!(calc_step_period(PCLKD, 1_000, LOW, HIGH), Ok(48_000));
    assert_eq!(calc_step_period(PCLKD, 44_600, LOW, HIGH), Ok(1_076));
    assert_eq!(calc_step_period(PCLKD, 55_733, LOW, HIGH), Ok(861));
}

#[test]
fn given_rates_around_the_ceiling_when_resolved_then_the_floor_holds() {
    // given: the floor low + high + 1 = 361 counts (132 963 pulses/s)
    // when: rates just under and over it are resolved
    // then: 361 counts is accepted, 360 is refused
    assert_eq!(calc_step_period(PCLKD, 132_964, LOW, HIGH), Ok(361));
    assert_eq!(
        calc_step_period(PCLKD, 133_334, LOW, HIGH),
        Err(StepError::RateTooHigh)
    );
}

#[test]
fn given_edge_phases_when_pulses_in_a_tick_are_counted_then_brute_force_agrees() {
    // given: the rig's periods and every phase near a pulse edge
    for period in [361u32, 861, 1_076, 48_000, 96_000, 100_000, 48_000_000] {
        let phases = [
            0,
            1,
            LOW - 1,
            LOW,
            LOW + 1,
            period / 2,
            period - 2,
            period - 1,
        ];
        for phase in phases.into_iter().filter(|&phase| phase < period) {
            // when: the closed form counts the ends in one tick
            let counted = pulses_in_tick(period, LOW, phase, TICK);
            // then: it matches counting them one by one
            assert_eq!(
                counted,
                ends_by_brute_force(period, LOW, phase, TICK),
                "period {period} phase {phase}"
            );
        }
    }
}

#[test]
fn given_consecutive_ticks_when_counted_then_they_partition_the_pulse_ends() {
    // given: 55 733/s (861 counts), starting mid-pulse
    let (period, mut phase) = (861u32, 100u32);
    let mut total = 0;
    // when: 1 000 consecutive ticks are counted, the phase advancing by a
    // tick each time
    for _ in 0..1_000 {
        total += pulses_in_tick(period, LOW, phase, TICK);
        phase = (phase + TICK) % period;
    }
    // then: the sum equals the ends in the whole 2 s, counted directly
    let first = i64::from(LOW) - 100;
    let span = 1_000 * i64::from(TICK);
    let direct = (0..=span / i64::from(period) + 1)
        .map(|k| first + k * i64::from(period))
        .filter(|&t| t >= 0 && t < span)
        .count() as u32;
    assert_eq!(total, direct);
}

#[test]
fn given_the_measured_boundary_when_counted_then_ends_two_counts_before_a_tick_go_next() {
    // given: 44 486/s (1 079 counts), whose phase advances 1 048 counts per
    // tick; a tick at phase 272 is followed by one at 241, then one at 242
    // would follow 273 — the board showed that the pulse ending at counter
    // 240 is NOT in the latch at phase 241 or 242 (Slice 11 step 4)
    let (period, end) = (1_079u32, LOW + crate::step::PULSE_END_LATCH_DELAY);
    // when: the ticks starting at phase 272 and 273 are counted
    let from_272 = pulses_in_tick(period, end, 272, TICK);
    let from_273 = pulses_in_tick(period, end, 273, TICK);
    // then: the end 1 or 2 counts before the next tick is left to that tick
    // (88 ends), while 3 counts before is counted (phase 274 → 243: 89)
    assert_eq!(from_272, 88);
    assert_eq!(from_273, 88);
    assert_eq!(pulses_in_tick(period, end, 274, TICK), 89);
}

#[test]
fn given_rig_rates_when_a_tick_is_counted_then_it_holds_rate_times_2_ms() {
    // given: the rig's periods (44 600, 55 733, 1 000/s) and a stopped train
    // when / then: about rate × 2 ms ends per tick, ±1
    assert!((89..=90).contains(&pulses_in_tick(1_076, LOW, 500, TICK))); // 44 600/s
    assert!((111..=112).contains(&pulses_in_tick(861, LOW, 500, TICK))); // 55 733/s
    assert_eq!(pulses_in_tick(48_000, LOW, 1_000, TICK), 2); // 1 000/s
    assert_eq!(pulses_in_tick(0, LOW, 0, TICK), 0); // stopped
}

#[test]
fn given_a_steady_schedule_when_ends_are_counted_then_pulses_in_tick_agrees() {
    // given: steady trains at the rig's periods and edge phases
    let end = LOW + crate::step::PULSE_END_LATCH_DELAY;
    for period in [361u32, 861, 1_076, 1_079, 48_000] {
        for phase in [0, 1, end - 1, end, end + 1, period / 2, period - 1] {
            // when / then: the schedule and the closed form count alike
            assert_eq!(
                Schedule::steady(phase, period).ends_in(end, TICK),
                pulses_in_tick(period, end, phase, TICK),
                "period {period} phase {phase}"
            );
        }
    }
}

#[test]
fn given_transition_schedules_when_counted_and_advanced_then_brute_force_agrees() {
    // given: schedules with transition periods, as a tick-applied change
    // leaves them (current, then next, then period)
    let end = LOW + crate::step::PULSE_END_LATCH_DELAY;
    let cases = [
        (100, 1_076, 50_120, 861),
        (46_000, 50_000, 46_120, 1_076),
        (5, 861, 1_200, 48_000),
        (859, 861, 2_000, 1_079),
        (1_000, 96_500, 1_076, 1_076),
        (0, 50_000, 361, 361),
    ];
    for (phase, current, next, period) in cases {
        let schedule = Schedule {
            phase,
            current,
            next,
            period,
        };
        // when: ends in the tick are counted, and the schedule advanced
        let counted = schedule.ends_in(end, TICK);
        let advanced = schedule.advance(TICK);
        // then: they match a pulse-by-pulse walk
        let list = starts(schedule, 3 * i64::from(TICK));
        let ends = list
            .iter()
            .filter(|&&start| (0..i64::from(TICK)).contains(&(start + i64::from(end))))
            .count() as u32;
        assert_eq!(counted, ends, "{schedule:?}");
        let last_start = *list
            .iter()
            .filter(|&&start| start <= i64::from(TICK))
            .max()
            .unwrap();
        assert_eq!(
            i64::from(advanced.phase),
            i64::from(TICK) - last_start,
            "{schedule:?}"
        );
    }
}

#[test]
fn given_rate_zero_when_resolved_then_it_is_refused() {
    // given / when: a rate of 0
    // then: no period exists — the caller stops instead
    assert_eq!(
        calc_step_period(PCLKD, 0, LOW, HIGH),
        Err(StepError::ZeroRate)
    );
}

#[test]
fn given_any_rate_when_resolved_then_the_delivered_error_is_at_most_half_a_count() {
    // given: every rate from 1 to 55 733 in coarse steps
    for rate in (1..=55_733u32).step_by(997) {
        // when: resolved
        let counts = calc_step_period(PCLKD, rate, LOW, HIGH).unwrap();
        // then: |counts × rate − pclkd| ≤ rate / 2 (half a count)
        let delivered = u64::from(counts) * u64::from(rate);
        assert!(delivered.abs_diff(u64::from(PCLKD)) <= u64::from(rate) / 2 + 1);
    }
}
