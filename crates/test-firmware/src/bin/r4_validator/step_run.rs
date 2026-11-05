//! Slice 11 step 2: the step generator on D4, driven by the R3's `T`
//! requests and measured on R3 D8 (Timer1 input capture). WiFi only — the
//! Minima's D4 (P103) is not on GPT320; there `T` replies `T NA` from
//! `main.rs`.

use crate::{body::Body, requests::argument};

use embedded_hal::digital::PinState;
use ra4m1::{GPT162, GPT164, GPT165};
use unor4_hal::{
    gpio::pin_map::{self, D4, D5},
    step::{
        CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, NoDivider, PulPinBuilder, PulSetup,
        StepError, StepGenerator, StepGeneratorBuilder, StepSetup, TickBuilder, TickReport,
        TickSetup, pulses_in_tick,
    },
};

/// STEP on D4 (GPT320, output B) with the cart-pole pulse shape: LOW 5 µs
/// (the DM542's 2.5 µs plus room for its PNP stage), HIGH at least 2.5 µs.
const PUL: PulSetup<D4, NoDivider> = PulPinBuilder::new()
    .with_pin(pin_map::d4)
    .with_timing(5_000, 2_500)
    .build();
/// DIR on D5, 5 µs set-up (DM542 t2), HIGH for positive rates.
const DIR: DirSetup<D5> = DirPinBuilder::new()
    .with_pin(pin_map::d5)
    .with_setup(5_000)
    .with_forward(PinState::High)
    .build();
/// GPT162 counts the pulses: a 16-bit counter, extended to 32 bits in
/// software at every tick (the bench proof of that path).
pub(crate) const COUNTER: CounterSetup<GPT162> =
    CounterBuilder::new().with_timer::<GPT162>().build();
/// GPT164 ticks, GPT165 numbers the ticks.
pub(crate) const TICK: TickSetup<GPT164, GPT165> =
    TickBuilder::new().with_timers::<GPT164, GPT165>().build();
/// The validator's step generator: the Slice 11 bench wiring, every part
/// by name, cross-checked at `cargo check`.
pub(crate) const SETUP: StepSetup<D4, NoDivider, D5, GPT162, GPT164, GPT165> =
    StepGeneratorBuilder::new()
        .with_pul(PUL)
        .with_dir(DIR)
        .with_counter(COUNTER)
        .with_tick(TICK)
        .build();

/// A [`StepError`] as the small number the replies carry: 1 rate too high,
/// 2 no ICU slot, 3 running, 4 zero rate, 5 not ready, 6 too late,
/// 7 driver disabled (ENA), 8 ENA lead not passed, 9 count not ahead,
/// 10 count too far, 11 reversal with a count stop.
pub(crate) fn error_code(error: StepError) -> u32 {
    match error {
        StepError::RateTooHigh => 1,
        StepError::NoIcuSlot => 2,
        StepError::Running => 3,
        StepError::ZeroRate => 4,
        StepError::NotReady => 5,
        StepError::TooLate => 6,
        StepError::Disabled => 7,
        StepError::Enabling => 8,
        StepError::CountNotAhead => 9,
        StepError::CountTooFar => 10,
        StepError::ReversalWithCountStop => 11,
    }
}

/// The `T<rate>` request: stop the generator, then start it at `rate`
/// pulses/s (`T0` only stops). Reply `T <period counts>` (`T 0` when
/// stopped), `T ERR <code>` when refused (1 rate too high, 2 no ICU slot, 3
/// running), or `T NA` when construction failed.
pub(crate) fn step_request(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"T NA");
    };
    let rate = argument(req, 0);
    stepper.stop();
    let mut body = Body::new(b"T");
    match stepper.run(rate as i32) {
        Ok(period) => {
            body.num(period);
        }
        Err(error) => {
            let code = match error {
                StepError::RateTooHigh => 1,
                StepError::NoIcuSlot => 2,
                StepError::Running
                | StepError::ZeroRate
                | StepError::NotReady
                | StepError::TooLate
                | StepError::Disabled
                | StepError::Enabling
                | StepError::CountNotAhead
                | StepError::CountTooFar
                | StepError::ReversalWithCountStop => 3,
            };
            body.text(b" ERR").num(code);
        }
    }
    body
}

/// The `N` request: the hardware pulse count (`N <pulses>`, wrapping), or
/// `N NA` without a generator.
pub(crate) fn pulse_count(stepper: &Option<StepGenerator>) -> Body {
    match stepper {
        Some(stepper) => {
            let mut body = Body::new(b"N");
            body.num(stepper.pulses());
            body
        }
        None => Body::new(b"N NA"),
    }
}

/// Ticks the `Y` request watches.
pub(crate) const TICK_RUN_TICKS: u32 = 60;
/// Ticks the `V` (stress) request watches: 5 s.
pub(crate) const TICK_STRESS_TICKS: u32 = 2_500;

/// The `Y<rate>` request (Slice 11 step 4): run at `rate` (0 = stopped)
/// for [`TICK_RUN_TICKS`] ticks, polling every tick; each tick's latched
/// count must equal the previous tick's prediction. Then stop. Reply
/// `Y <checked> <mismatches> <skipped> <dmin> <dmax> <span> <s1> <s2> <s3>
/// <s4>`: predictions
/// checked, wrong ones, ticks missed by the poll loop after the first
/// report (the first report's skip count is the idle gap before the run),
/// the smallest and largest per-tick count change (from the second tick on,
/// so the partial first tick is left out), and the tick-number span
/// (= ticks − 1 when none was skipped), and the mismatches models with each
/// pulse end 1, 2, 3 and 4 counts later would have had. On a mismatch two
/// more fields
/// follow for the first one: the phase the prediction was made from, and
/// `actual − predicted + 10`. `Y NA` without a generator. The `V<rate>`
/// request is the same over [`TICK_STRESS_TICKS`] ticks (its reply also
/// starts with `Y`).
pub(crate) fn tick_run(req: &[u8], stepper: &mut Option<StepGenerator>, ticks: u32) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"Y NA");
    };
    let rate = argument(req, 1_000);
    stepper.stop();
    if stepper.run(rate as i32).is_err() {
        return Body::new(b"Y ERR");
    }
    let (mut seen, mut checked, mut mismatches, mut skipped) = (0u32, 0u32, 0u32, 0u32);
    let (mut dmin, mut dmax) = (u32::MAX, 0u32);
    let (mut first_tick, mut last_tick, mut last_pulses) = (0u32, 0u32, 0u32);
    let mut prediction: Option<u32> = None;
    let (mut predicted_from, mut first_miss): (u32, Option<(u32, u32)>) = (0, None);
    // Diagnostic: the same prediction with each pulse's end taken to reach
    // the latch 1–4 counts after GTCCRB (`low + d`), counted side by side
    // with the HAL's (which uses d = PULSE_END_LATCH_DELAY = 2, the value
    // these counts measured: s2 must equal the HAL's mismatches, 0).
    let mut shifted: [Option<u32>; 4] = [None; 4];
    let mut shifted_misses = [0u32; 4];
    let (low, tick_counts) = (stepper.low_counts(), stepper.tick_counts());
    while seen < ticks {
        let Some(report) = stepper.poll_tick() else {
            continue;
        };
        if seen > 0 {
            skipped += report.skipped;
        }
        if let Some(expected) = prediction {
            checked += 1;
            if expected != report.pulses {
                mismatches += 1;
                if first_miss.is_none() {
                    let diff = report.pulses.wrapping_sub(expected).wrapping_add(10);
                    first_miss = Some((predicted_from, diff));
                }
            }
        }
        for (expected, misses) in shifted.iter().zip(shifted_misses.iter_mut()) {
            if let Some(expected) = *expected
                && expected != report.pulses
            {
                *misses += 1;
            }
        }
        predicted_from = report.phase_counts;
        if seen == 0 {
            first_tick = report.tick;
        } else if seen >= 2 {
            let delta = report.pulses.wrapping_sub(last_pulses);
            dmin = dmin.min(delta);
            dmax = dmax.max(delta);
        }
        last_tick = report.tick;
        last_pulses = report.pulses;
        prediction = stepper.predicted_pulses();
        for (shift, slot) in (1u32..).zip(shifted.iter_mut()) {
            *slot = prediction.map(|_| {
                report.pulses.wrapping_add(pulses_in_tick(
                    stepper.period_counts(),
                    low + shift,
                    report.phase_counts,
                    tick_counts,
                ))
            });
        }
        seen += 1;
    }
    stepper.stop();
    let mut body = Body::new(b"Y");
    body.num(checked)
        .num(mismatches)
        .num(skipped)
        .num(dmin)
        .num(dmax)
        .num(last_tick.wrapping_sub(first_tick));
    for misses in shifted_misses {
        body.num(misses);
    }
    if let Some((phase, diff)) = first_miss {
        body.num(phase).num(diff);
    }
    body
}

/// The rate the `Q<mode>` request queues for its `step`-th tick, or `None`
/// when the pattern is over. Mode 1: alternate 44 600/s and 960/s every
/// tick (40 ticks) — each switch to the fast rate puts a pulse
/// `min_high` after the tick, a marker the R3 can time. Mode 2: a ramp of
/// ±2 421 pulses/s per tick (the cart-pole jump limit × 2 ms) from 1 000/s
/// to 44 578/s and back.
fn queue_pattern(mode: u32, step: u32) -> Option<u32> {
    match mode {
        1 if step < 40 => Some(if step.is_multiple_of(2) { 44_600 } else { 960 }),
        2 if step <= 18 => Some(1_000 + 2_421 * step),
        2 if step <= 36 => Some(1_000 + 2_421 * (36 - step)),
        // Mode 3: 2 500 ticks of a different fast rate every tick (a
        // pseudo-random walk over 30 000–55 733/s), so ticks where the old
        // period ends inside the DTC's write latency occur many times.
        3 if step < 2_500 => Some(30_000 + (step.wrapping_mul(7_919) % 25_734)),
        _ => None,
    }
}

/// The `Q<mode>` request (Slice 11 step 5a): starts the generator (960/s
/// for mode 1, 1 000/s for mode 2), waits until its schedule is known, then
/// queues [`queue_pattern`]'s rate after every tick, checking each tick's
/// latched count against the previous prediction and each captured phase
/// against the schedule's. Then stops. Reply `Q <ticks> <applied>
/// <count misses> <phase misses> <queue errors> <span> <old first>`: the last
/// field counts applied ticks where the old pulse fired before the DTC write.
pub(crate) fn queue_run(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"Q NA");
    };
    let mode = argument(req, 1);
    stepper.stop();
    if stepper.run(if mode == 1 { 960 } else { 1_000 }).is_err() {
        return Body::new(b"Q ERR");
    }
    let wait = |stepper: &mut StepGenerator| loop {
        if let Some(report) = stepper.poll_tick() {
            break report;
        }
    };
    // The schedule is known one full tick after run(); give up after 10.
    let mut first = wait(stepper);
    for _ in 0..10 {
        if stepper.schedule().is_some() {
            break;
        }
        first = wait(stepper);
    }
    if stepper.schedule().is_none() {
        stepper.stop();
        return Body::new(b"Q NOSCHED");
    }
    let (mut ticks, mut applied, mut count_misses, mut phase_misses, mut queue_errors) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut old_first = 0u32;
    let mut prediction = stepper.predicted_pulses();
    let mut last_tick = first.tick;
    let mut step = 0;
    while let Some(rate) = queue_pattern(mode, step) {
        if stepper.queue(rate as i32).is_err() {
            queue_errors += 1;
        }
        step += 1;
        let report = wait(stepper);
        ticks += 1;
        last_tick = report.tick;
        if report.applied {
            applied += 1;
        }
        if report.old_pulse_first {
            old_first += 1;
        }
        if prediction.is_some_and(|expected| expected != report.pulses) {
            count_misses += 1;
        }
        if report
            .expected_phase
            .is_some_and(|expected| expected != report.phase_counts)
        {
            phase_misses += 1;
        }
        prediction = stepper.predicted_pulses();
    }
    stepper.stop();
    let mut body = Body::new(b"Q");
    body.num(ticks)
        .num(applied)
        .num(count_misses)
        .num(phase_misses)
        .num(queue_errors)
        .num(last_tick.wrapping_sub(first.tick))
        .num(old_first);
    body
}

/// What the `O` request does after its `step`-th tick: queue a rate, or
/// nothing (`None`: the tick holds the running rate). The scenario: ramp up
/// 5 000 → 20 000/s, go to 0, hold 0 (a repeated 0 and a tick without a
/// queue), restart at 12 000/s from idle while the counter still runs, two
/// ticks without a queue (the rate holds), 8 000/s, 0 again, then 600 ticks
/// with no queue — the generator stops itself after 1 s idle — then a start
/// from stopped at 6 000/s (hardware start at the tick), 9 000/s, and a trip
/// `stop()` at a random point of the last tick.
fn scenario(step: u32) -> Option<u32> {
    match step {
        0 => Some(5_000),
        1 => Some(10_000),
        2 => Some(15_000),
        3 => Some(20_000),
        4 | 5 | 12 => Some(0),
        7 | 8 => Some(12_000),
        11 => Some(8_000),
        613 | 614 => Some(6_000),
        615 | 616 => Some(9_000),
        _ => None,
    }
}

/// Ticks the `O` scenario runs before its trip stop.
const SCENARIO_TICKS: u32 = 618;

/// The `O` request (Slice 11 step 5c): the [`scenario`], every tick's
/// latched count checked against the previous prediction and every phase
/// against the schedule; ends with a trip `stop()` part-way into a tick.
/// Reply `O <ticks> <applied> <count misses> <phase misses> <queue errors>
/// <idle stops> <starts> [<step> <code>]… [<step> <expected> <phase>]`: applied changes (13 expected), whether the
/// generator stopped itself while idle, and whether the start from stopped
/// was applied.
pub(crate) fn scenario_run(stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"O NA");
    };
    stepper.stop();
    if stepper.run(1_000).is_err() {
        return Body::new(b"O ERR");
    }
    let wait = |stepper: &mut StepGenerator| loop {
        if let Some(report) = stepper.poll_tick() {
            break report;
        }
    };
    for _ in 0..10 {
        if stepper.schedule().is_some() {
            break;
        }
        wait(stepper);
    }
    let (mut applied, mut count_misses, mut phase_misses, mut queue_errors) = (0u32, 0, 0, 0);
    let (mut idle_stopped, mut started) = (false, false);
    // Diagnostics: the first two refused queues (step, error code) and the
    // first phase miss (step, expected, captured).
    let mut refused: [(u32, u32); 2] = [(0, 0); 2];
    let mut phase_miss_at: (u32, u32, u32) = (0, 0, 0);
    let mut prediction = stepper.predicted_pulses();
    for step in 0..SCENARIO_TICKS {
        if let Some(rate) = scenario(step)
            && let Err(error) = stepper.queue(rate as i32)
        {
            if (queue_errors as usize) < refused.len() {
                refused[queue_errors as usize] = (step, error_code(error));
            }
            queue_errors += 1;
        }
        let report = wait(stepper);
        if report.applied {
            applied += 1;
            if step == 613 && stepper.is_running() {
                started = true;
            }
        }
        if prediction.is_some_and(|expected| expected != report.pulses) {
            count_misses += 1;
        }
        if let Some(expected) = report.expected_phase
            && expected != report.phase_counts
        {
            if phase_misses == 0 {
                phase_miss_at = (step, expected, report.phase_counts);
            }
            phase_misses += 1;
        }
        if (13..613).contains(&step) && !stepper.is_running() {
            idle_stopped = true;
        }
        prediction = stepper.predicted_pulses();
    }
    // The trip: stop part-way into the next tick.
    let pause = stepper.pulses() % 50_000;
    for _ in 0..pause {
        core::hint::spin_loop();
    }
    stepper.stop();
    let mut body = Body::new(b"O");
    body.num(SCENARIO_TICKS)
        .num(applied)
        .num(count_misses)
        .num(phase_misses)
        .num(queue_errors)
        .num(u32::from(idle_stopped))
        .num(u32::from(started));
    for (step, code) in refused.into_iter().take(queue_errors as usize) {
        body.num(step).num(code);
    }
    if phase_misses > 0 {
        body.num(phase_miss_at.0)
            .num(phase_miss_at.1)
            .num(phase_miss_at.2);
    }
    body
}

/// The signed rate the `W<mode>` request queues for its `step`-th tick.
/// Mode 1: the sign alternating every tick for [`REVERSE_TICKS`] ticks,
/// magnitudes hashed over 15 000–30 000/s — a reversal at every tick, at
/// rates where a pulse is LOW at the tick 7–15 % of the time and fires
/// inside the write latency about 0.5 % of the time; the hashed magnitudes
/// spread the phases evenly, so both cases occur in every run (a walking
/// pattern over 500 ticks met the second only by luck of the starting
/// phase). (Faster, the R3's DIR read and timing at every pulse fall
/// behind.)
/// Mode 2: a ramp through zero, 4 842 → −4 842/s in ±2 421/s steps and back,
/// four times (the zero ticks make the reversal happen while idle). Mode 3:
/// from stopped (the generator idles out first), backward, then a reversal.
/// Ticks in a `W1` run.
const REVERSE_TICKS: u32 = 2_000;

fn reverse_pattern(mode: u32, step: u32) -> Option<i32> {
    match mode {
        1 if step < REVERSE_TICKS => {
            let magnitude = 15_000 + ((step.wrapping_mul(2_654_435_761) >> 16) % 15_001) as i32;
            Some(if step.is_multiple_of(2) {
                -magnitude
            } else {
                magnitude
            })
        }
        2 if step < 80 => {
            let k = (step % 8) as i32;
            Some(4_842 - 2_421 * if k <= 4 { k } else { 8 - k })
        }
        3 if step < 600 => Some(0),
        3 if step < 610 => Some(
            [
                -5_000, -8_000, 3_000, 3_000, -6_000, -6_000, 0, 0, 2_000, 2_000,
            ][(step - 600) as usize],
        ),
        _ => None,
    }
}

/// The `W<mode>` request (Slice 11 step 5b): starts at 1 000/s, then queues
/// [`reverse_pattern`] after every tick; checks every prediction, and every
/// reversal against the agreed DIR rule from the HAL's own read-back: DIR
/// written inside [`StepGenerator::dir_write_window`], after any pulse LOW
/// at the tick or fired inside the write latency has ended, and at least
/// `dir_setup` (240 counts) before the next pulse. Then stops. Reply
/// `W <ticks> <applied> <count misses> <phase misses> <queue errors>
/// <reversals> <in flight> <old first> <violations> <dir min> <dir max>
/// <position change>`.
pub(crate) fn reverse_run(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"W NA");
    };
    let mode = argument(req, 1);
    let wait = |stepper: &mut StepGenerator| loop {
        if let Some(report) = stepper.poll_tick() {
            break report;
        }
    };
    // Fold every earlier pulse into the position before starting, so the
    // position change below covers exactly this run.
    stepper.stop();
    let before = wait(stepper).position;
    if stepper.run(1_000).is_err() {
        return Body::new(b"W ERR");
    }
    let mut start = wait(stepper);
    for _ in 0..10 {
        if stepper.schedule().is_some() {
            break;
        }
        start = wait(stepper);
    }
    let (low, setup) = (stepper.low_counts(), 240u32);
    let (_, write_max) = stepper.dir_write_window();
    let (mut ticks, mut applied, mut count_misses, mut phase_misses, mut queue_errors) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut reversals, mut in_flight, mut old_first, mut violations) = (0u32, 0u32, 0u32, 0u32);
    let (mut dir_min, mut dir_max) = (u32::MAX, 0u32);
    let mut prediction = stepper.predicted_pulses();
    let mut step = 0;
    while let Some(rate) = reverse_pattern(mode, step) {
        if stepper.queue(rate).is_err() {
            queue_errors += 1;
        }
        step += 1;
        let report = wait(stepper);
        ticks += 1;
        if report.applied {
            applied += 1;
        }
        if prediction.is_some_and(|expected| expected != report.pulses) {
            count_misses += 1;
        }
        if report
            .expected_phase
            .is_some_and(|expected| expected != report.phase_counts)
        {
            phase_misses += 1;
        }
        if let (Some(at), Some(schedule)) = (report.dir_changed_at, stepper.schedule()) {
            reversals += 1;
            dir_min = dir_min.min(at);
            dir_max = dir_max.max(at);
            let phase = report.phase_counts;
            let mut ok = at <= write_max;
            // A pulse LOW at the tick ends at low − phase.
            if phase < low {
                in_flight += 1;
                ok &= at > low - phase;
            }
            // A pulse that fired inside the write latency ends low later.
            let next_start = schedule.current - schedule.phase;
            let first_new = if report.old_pulse_first {
                old_first += 1;
                ok &= at > next_start + low;
                next_start + schedule.next
            } else {
                next_start
            };
            ok &= first_new >= at + setup;
            if !ok {
                violations += 1;
            }
        }
        prediction = stepper.predicted_pulses();
    }
    stepper.stop();
    let position_change = wait(stepper).position - before;
    let _ = start;
    let mut body = Body::new(b"W");
    body.num(ticks)
        .num(applied)
        .num(count_misses)
        .num(phase_misses)
        .num(queue_errors)
        .num(reversals)
        .num(in_flight)
        .num(old_first)
        .num(violations)
        .num(if reversals == 0 { 0 } else { dir_min })
        .num(dir_max);
    if position_change < 0 {
        body.text(b" M").num(position_change.unsigned_abs() as u32);
    } else {
        body.num(position_change as u32);
    }
    body
}

/// CPU cycles of a stall that passes over two ticks: at least 5 ms at ICLK
/// 48 MHz (`cortex_m::asm::delay` waits at least its count in CPU cycles; on this
/// board one count took 1.25 to 3 cycles depending on the call site (the
/// delay probe `m`; the first `S11 sleep` run)).
const STALL_CYCLES: u32 = 240_000;

/// The rate for step `step` of a `k` run: mode 1 alternates 1 000/s and
/// 1 500/s every tick (every stall passes over a rate change); mode 2
/// reverses between +1 000/s and −1 000/s every 10 ticks (every stall
/// passes over a reversal). 400 ticks.
fn skip_pattern(mode: u32, step: u32) -> Option<i32> {
    match mode {
        1 if step < 400 => Some(if step.is_multiple_of(2) { 1_000 } else { 1_500 }),
        2 if step < 400 => Some(if (step / 10).is_multiple_of(2) {
            1_000
        } else {
            -1_000
        }),
        _ => None,
    }
}

/// The `k<mode>` request (missed-tick recovery): ticks passed over on
/// purpose. Modes 1 and 2 run [`skip_pattern`], queueing a rate every tick
/// and stalling 5 ms right after every 20th queue, so the change lands on a
/// tick nobody polls. After a stall the generator must rebuild its schedule
/// from the registers: the next `queue` succeeds, and every later
/// prediction is exact. Every other stall ends just after a tick, where the
/// schedule must be rebuilt in the same poll; the rest end anywhere, where
/// it may wait for the next tick. Reply `k <stalls> <recovered> <worst polls
/// to recover, stalls ending anywhere> <worst, stalls ending just after a
/// tick> <queue errors> <phase misses> <count misses> <applied while passed
/// over> <just-after-a-tick stalls left without a schedule> <recovery
/// errors> <position change>` (position as `M <n>` when negative; recovery
/// errors as a bit mask of [`error_code`]s, `1 << code`, of the refused
/// queues while recovering).
///
/// Mode 3: run 1 000/s, queue rate 0, stall 5 ms once it is idle, then poll
/// for up to 2 s; an idle generator must stop 1 s after its last pulse
/// whatever was missed. Reply `k <stopped> <ticks from idle to stop>
/// <pulses after idle> <position change>`.
pub(crate) fn skip_run(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"k NA");
    };
    let mode = argument(req, 1);
    let wait = |stepper: &mut StepGenerator| loop {
        if let Some(report) = stepper.poll_tick() {
            break report;
        }
    };
    stepper.stop();
    let before = wait(stepper).position;
    if stepper.run(1_000).is_err() {
        return Body::new(b"k ERR");
    }
    for _ in 0..10 {
        wait(stepper);
        if stepper.schedule().is_some() {
            break;
        }
    }
    let mut body = Body::new(b"k");
    if mode == 3 {
        let idle_at = loop {
            if stepper.queue(0).is_ok() {
                let report = wait(stepper);
                if report.applied {
                    break report;
                }
            } else {
                wait(stepper);
            }
        };
        wait(stepper);
        cortex_m::asm::delay(STALL_CYCLES);
        let mut ticks = 0u32;
        let mut last = idle_at;
        while stepper.is_running() && ticks < 1_000 {
            last = wait(stepper);
            ticks = last.tick.wrapping_sub(idle_at.tick);
        }
        let stopped = !stepper.is_running();
        let after = wait(stepper);
        let _ = last;
        body.num(u32::from(stopped))
            .num(ticks)
            .num(after.pulses.wrapping_sub(idle_at.pulses));
        push_position(&mut body, after.position - before);
        return body;
    }
    let (mut stalls, mut recovered, mut queue_errors) = (0u32, 0u32, 0u32);
    // Worst polls to recover, after stalls ending anywhere in a tick, and
    // after stalls ending just after a tick (the same-poll rebuild).
    let (mut worst_late, mut worst_aligned) = (0u32, 0u32);
    let mut aligned = false;
    let (mut phase_misses, mut count_misses, mut applied_missed) = (0u32, 0u32, 0u32);
    let mut recovering: Option<u32> = None;
    let (mut unsynced, mut recovery_errors) = (0u32, 0u32);
    let mut prediction = stepper.predicted_pulses();
    let mut step = 0;
    while let Some(rate) = skip_pattern(mode, step) {
        match stepper.queue(rate) {
            Ok(_) => {
                if let Some(polls) = recovering.take() {
                    recovered += 1;
                    if aligned {
                        worst_aligned = worst_aligned.max(polls);
                    } else {
                        worst_late = worst_late.max(polls);
                    }
                }
            }
            Err(error) => match recovering.as_mut() {
                Some(polls) => {
                    *polls += 1;
                    recovery_errors |= 1 << error_code(error);
                }
                None => queue_errors += 1,
            },
        }
        if step % 20 == 10 && recovering.is_none() {
            cortex_m::asm::delay(STALL_CYCLES);
            // Every other stall ends just after a tick (under 400 PCLKD
            // counts into it), so the poll can prove no STEP overflow since
            // the tick and rebuild the schedule at once; the others end
            // wherever the 5 ms lands, often after an overflow, so the
            // rebuild may wait for the next tick.
            aligned = stalls % 2 == 1;
            if aligned {
                while stepper.since_tick_counts() >= 400 {}
            }
            stalls += 1;
            recovering = Some(0);
        }
        step += 1;
        let report = wait(stepper);
        if report.skipped == 0 {
            if prediction.is_some_and(|expected| expected != report.pulses) {
                count_misses += 1;
            }
            if report
                .expected_phase
                .is_some_and(|expected| expected != report.phase_counts)
            {
                phase_misses += 1;
            }
        } else {
            if report.applied {
                applied_missed += 1;
            }
            if aligned && stepper.schedule().is_none() {
                unsynced += 1;
            }
        }
        prediction = stepper.predicted_pulses();
    }
    stepper.stop();
    let position_change = wait(stepper).position - before;
    body.num(stalls)
        .num(recovered)
        .num(worst_late)
        .num(worst_aligned)
        .num(queue_errors)
        .num(phase_misses)
        .num(count_misses)
        .num(applied_missed)
        .num(unsynced)
        .num(recovery_errors);
    push_position(&mut body, position_change);
    body
}

/// Appends a signed position change: `<n>`, or `M <n>` when negative.
pub(crate) fn push_position(body: &mut Body, change: i64) {
    if change < 0 {
        body.text(b" M").num(change.unsigned_abs() as u32);
    } else {
        body.num(change as u32);
    }
}

/// The `X` request (Slice 11 step 5a): the tick DTC's write latency —
/// [`StepGenerator::dtc_latency_probe`] over 500 ticks at 1 000/s. Reply
/// `X <min> <max> <samples>` in PCLKD counts after the tick instant.
pub(crate) fn dtc_probe(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"X NA");
    };
    stepper.stop();
    if stepper.run(1_000).is_err() {
        return Body::new(b"X ERR");
    }
    let words = argument(req, 0);
    let (min, max, samples) = if words == 0 {
        stepper.dtc_latency_probe(500)
    } else {
        stepper.dtc_delay_probe(words.min(256) as u16, 500)
    };
    stepper.stop();
    let mut body = Body::new(b"X");
    body.num(min).num(max).num(samples);
    body
}

/// The `Z<zone>` request: wait until the running generator's counter is in
/// `zone`, then [`StepGenerator::stop`] — so each stop path is exercised on
/// purpose. Zone 1: mid-pulse (LOW, the hardware stop on the pulse's end);
/// 2: just after a pulse ends (HIGH, inside the stop margin); 3: just before
/// the next pulse (HIGH, inside the stop margin). Reply `Z <zone> <phase>`
/// (the counter when `stop` was called), or `Z ERR` when not running or the
/// zone was not seen within 100 periods.
pub(crate) fn stop_in_zone(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"Z ERR");
    };
    let zone = argument(req, 1);
    let (low, period) = (stepper.low_counts(), stepper.period_counts());
    if !stepper.is_running() || period == 0 {
        return Body::new(b"Z ERR");
    }
    // Windows at least 24 counts wide: the read loop's stride must not be
    // able to step over them (8-count windows were missed at 55 733/s).
    let (from, to) = match zone {
        1 => (low / 4, low / 2),
        2 => (low + 1, low + 24),
        _ => (period - 26, period - 2),
    };
    // Each pass over the zone takes one period; give up after ~100.
    let mut tries: u32 = 0;
    // Interrupts off between seeing the zone and calling stop().
    let phase = cortex_m::interrupt::free(|_| {
        loop {
            let phase = stepper.phase_counts();
            if (from..=to).contains(&phase) {
                stepper.stop();
                break Some(phase);
            }
            tries += 1;
            if tries > 100 * period {
                break None;
            }
        }
    });
    match phase {
        Some(phase) => {
            let mut body = Body::new(b"Z");
            body.num(zone).num(phase);
            body
        }
        None => Body::new(b"Z ERR"),
    }
}

/// Rate of the `H` stop burst: the rig's top rate, where a period (861
/// counts) leaves the least room around each stop.
const STOP_BURST_RATE: i32 = 55_733;

/// The `H<cycles>` request (Slice 11 follow-up): `cycles` start/stop cycles
/// at [`STOP_BURST_RATE`] (default 2 000), each stopped after a
/// pseudo-random wait of 0–2 047 cycles (about 2.4 periods; then a 25 µs
/// pause before the next start), so the stops land at every phase. After
/// each `stop()` the frozen counter says where it stopped: below the low
/// time, inside a pulse (a cut pulse); from the low time to low + 3, right
/// at a pulse's end; above, after a pulse had ended (the current `stop()`
/// always lands here, past the old period end, since it stops only after
/// writing the longest period). Reply `H <cycles> <cut> <end> <between>
/// <cut max> <between max> <period> <pulses>`: the three counts, the highest
/// frozen value seen as a cut and as a stop between pulses, the period in
/// counts, and the pulses counted over the burst (for the R3 to compare
/// with its edges). `H NA` without a generator, `H ERR` if a start fails.
pub(crate) fn stop_burst(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"H NA");
    };
    let cycles = argument(req, 2_000);
    stepper.stop();
    let low = stepper.low_counts();
    let before = stepper.pulses();
    let (mut cut, mut end, mut between) = (0u32, 0u32, 0u32);
    let (mut cut_max, mut between_max, mut period) = (0u32, 0u32, 0u32);
    // A different stop sequence each burst: seeded from the running count.
    let mut seed: u32 = before ^ 0x2545_F491;
    for _ in 0..cycles {
        match stepper.run(STOP_BURST_RATE) {
            Ok(counts) => period = counts,
            Err(_) => return Body::new(b"H ERR"),
        }
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        cortex_m::asm::delay(seed >> 21);
        stepper.stop();
        let frozen = stepper.phase_counts();
        // 25 µs before the next start: without it the next cycle's first
        // pulse can fall ~13 µs after this cycle's last, closer than the
        // R3's capture interrupt resolves when its own timer interrupt is
        // due (it then lost one edge in ~35 000 stops, R3 = R4 − 1).
        cortex_m::asm::delay(1_200);
        if frozen < low {
            cut += 1;
            cut_max = cut_max.max(frozen);
        } else if frozen <= low + 3 {
            end += 1;
        } else {
            between += 1;
            between_max = between_max.max(frozen);
        }
    }
    let mut body = Body::new(b"H");
    body.num(cycles)
        .num(cut)
        .num(end)
        .num(between)
        .num(cut_max)
        .num(between_max)
        .num(period)
        .num(stepper.pulses().wrapping_sub(before));
    body
}

/// Ticks in a `J` run: 5 s.
const WAKE_TICKS: u32 = 2_500;

/// The `J` request (Slice 11 follow-up, the tick wake-up): the `Q3` pattern
/// — a different rate over 30 000–55 733/s every tick for [`WAKE_TICKS`]
/// ticks — with the CPU asleep in `WFI` between ticks, woken by the tick's
/// own interrupt ([`StepGenerator::set_tick_wake`]). Every tick must be
/// seen (none skipped), every change applied and every prediction exact.
/// Reply `J <ticks> <applied> <count misses> <phase misses> <queue errors>
/// <skipped> <wakes> <wake max> <done max>`: `wakes` counts every return
/// from `WFI` (other interrupts wake it too), `wake max` is the latest a
/// tick was seen after it happened and `done max` the latest its `queue`
/// finished, both in PCLKD counts after the tick.
pub(crate) fn wake_run(stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"J NA");
    };
    stepper.stop();
    if stepper.run(30_000).is_err() {
        return Body::new(b"J ERR");
    }
    stepper.set_tick_wake(true);
    let (mut wakes, mut wake_max, mut done_max) = (0u32, 0u32, 0u32);
    let mut wait = |stepper: &mut StepGenerator| loop {
        cortex_m::asm::wfi();
        wakes += 1;
        let since = stepper.since_tick_counts();
        if let Some(report) = stepper.poll_tick() {
            wake_max = wake_max.max(since);
            break report;
        }
    };
    // The schedule is known one full tick after run(); give up after 10.
    let mut ready = false;
    for _ in 0..11 {
        wait(stepper);
        if stepper.schedule().is_some() {
            ready = true;
            break;
        }
    }
    if !ready {
        stepper.set_tick_wake(false);
        stepper.stop();
        return Body::new(b"J NOSCHED");
    }
    let (mut ticks, mut applied, mut count_misses, mut phase_misses, mut queue_errors) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut skipped = 0u32;
    let mut prediction = stepper.predicted_pulses();
    for step in 0..WAKE_TICKS {
        let rate = 30_000 + (step.wrapping_mul(7_919) % 25_734);
        if stepper.queue(rate as i32).is_err() {
            queue_errors += 1;
        }
        done_max = done_max.max(stepper.since_tick_counts());
        let report = wait(stepper);
        ticks += 1;
        skipped += report.skipped;
        if report.applied {
            applied += 1;
        }
        if prediction.is_some_and(|expected| expected != report.pulses) {
            count_misses += 1;
        }
        if report
            .expected_phase
            .is_some_and(|expected| expected != report.phase_counts)
        {
            phase_misses += 1;
        }
        prediction = stepper.predicted_pulses();
    }
    stepper.set_tick_wake(false);
    stepper.stop();
    let mut body = Body::new(b"J");
    body.num(ticks)
        .num(applied)
        .num(count_misses)
        .num(phase_misses)
        .num(queue_errors)
        .num(skipped)
        .num(wakes)
        .num(wake_max)
        .num(done_max);
    body
}

/// Starts in one `d` run; their signs alternate, beginning forward.
const DIR_LEAD_STARTS: u32 = 10;
/// Ticks every start runs, with its predictions checked.
const DIR_LEAD_TICKS: u32 = 5;
/// The wait before the first start, CPU cycles (at least 5 ms at 48 MHz;
/// see [`STALL_CYCLES`]): the R3 starts watching DIR only after the
/// request's last byte has gone out.
const DIR_LEAD_SETTLE: u32 = 240_000;
/// The wait before the reply, CPU cycles (at least 20 ms at 48 MHz): the R3
/// times DIR with its interrupts off and must have them back on to receive
/// it.
const DIR_LEAD_REPLY_DELAY: u32 = 960_000;

/// The next tick's report.
pub(crate) fn next_tick(stepper: &mut StepGenerator) -> TickReport {
    loop {
        if let Some(report) = stepper.poll_tick() {
            break report;
        }
    }
}

/// Polls `ticks` ticks, checking every exact prediction: `checks` holds
/// phase misses, count misses and the ticks whose phase was predicted.
pub(crate) fn watch_ticks(stepper: &mut StepGenerator, ticks: u32, checks: &mut [u32; 3]) {
    let mut prediction = stepper.predicted_pulses();
    for _ in 0..ticks {
        let report = next_tick(stepper);
        if report.skipped == 0 {
            if let Some(expected) = report.expected_phase {
                checks[2] += 1;
                if expected != report.phase_counts {
                    checks[0] += 1;
                }
            }
            if prediction.is_some_and(|expected| expected != report.pulses) {
                checks[1] += 1;
            }
        }
        prediction = stepper.predicted_pulses();
    }
}

/// Queues `rate`, retrying at the next tick on `NotReady` / `TooLate`.
/// Returns false on any other refusal.
pub(crate) fn queue_retrying(stepper: &mut StepGenerator, rate: i32) -> bool {
    for _ in 0..10 {
        match stepper.queue(rate) {
            Ok(_) => return true,
            Err(StepError::NotReady | StepError::TooLate) => {
                next_tick(stepper);
            }
            Err(_) => return false,
        }
    }
    false
}

/// The `d<mode>` request (cart-pole dogfooding fix: the DIR set-up before
/// the first pulse). [`DIR_LEAD_STARTS`] starts whose signs alternate,
/// beginning forward, so DIR changes at every start but perhaps the first;
/// the R3 times each DIR change on D5 to the next STEP pulse on D4.
///
/// - Mode 1: `run(±1 000)` from stopped. The first pulse must wait the
///   5 µs DIR set-up, not the 2.5 µs minimum HIGH.
/// - Mode 2: the same at ±120 000/s. The set-up plus the LOW time (480
///   counts) does not fit in the 400-count period, so the first period is
///   longer (`first_cycle`), and the predictions after it must still be
///   exact.
/// - Mode 3: `queue(±1 000)` from stopped: DIR at least 40 µs before the
///   tick, the first pulse the minimum HIGH time after it.
/// - Mode 4: one `run(1 000)`, then every direction change through idle:
///   `queue(0)`, three ticks idle, then `queue` of the other sign.
///
/// The first start comes 5 ms after the request, once the R3 watches DIR.
/// Every start runs [`DIR_LEAD_TICKS`] ticks with every exact prediction
/// checked. Reply `d <starts> <errors> <phase misses> <count misses>
/// <checked ticks>`, after a 20 ms wait (errors: refusals other than
/// `NotReady`/`TooLate`, and starts that never ran).
pub(crate) fn dir_lead_run(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"d NA");
    };
    let mode = argument(req, 1);
    let rate: i32 = if mode == 2 { 120_000 } else { 1_000 };
    let signed = |start: u32| if start.is_multiple_of(2) { rate } else { -rate };
    let (mut starts, mut errors) = (0u32, 0u32);
    let mut checks = [0u32; 3];
    stepper.stop();
    cortex_m::asm::delay(DIR_LEAD_SETTLE);
    match mode {
        1 | 2 => {
            for start in 0..DIR_LEAD_STARTS {
                stepper.stop();
                match stepper.run(signed(start)) {
                    Ok(_) => starts += 1,
                    Err(_) => errors += 1,
                }
                watch_ticks(stepper, DIR_LEAD_TICKS, &mut checks);
            }
        }
        3 => {
            for start in 0..DIR_LEAD_STARTS {
                stepper.stop();
                // A tick polled since the stop, so `queue` can target the next.
                next_tick(stepper);
                if queue_retrying(stepper, signed(start)) {
                    starts += 1;
                } else {
                    errors += 1;
                }
                watch_ticks(stepper, DIR_LEAD_TICKS, &mut checks);
                if !stepper.is_running() {
                    errors += 1;
                }
            }
        }
        4 => {
            match stepper.run(rate) {
                Ok(_) => starts += 1,
                Err(_) => errors += 1,
            }
            watch_ticks(stepper, DIR_LEAD_TICKS, &mut checks);
            for start in 1..DIR_LEAD_STARTS {
                if !queue_retrying(stepper, 0) {
                    errors += 1;
                }
                watch_ticks(stepper, 3, &mut checks);
                if queue_retrying(stepper, signed(start)) {
                    starts += 1;
                } else {
                    errors += 1;
                }
                watch_ticks(stepper, DIR_LEAD_TICKS, &mut checks);
            }
        }
        _ => return Body::new(b"d ERR"),
    }
    stepper.stop();
    cortex_m::asm::delay(DIR_LEAD_REPLY_DELAY);
    let mut body = Body::new(b"d");
    body.num(starts)
        .num(errors)
        .num(checks[0])
        .num(checks[1])
        .num(checks[2]);
    body
}

/// Moves of mode 1 (`queue(1 000)` from stopped): their pulse counts.
const COUNT_MOVES_QUEUE: [u32; 5] = [1, 2, 3, 50, 500];
/// Moves of mode 2 (`run(55 733)` from stopped), up to the 16-bit
/// counter's reach.
const COUNT_MOVES_RUN: [u32; 5] = [1, 10, 1_000, 30_000, 65_535];
/// Moves of mode 3: armed this many pulses ahead while running at
/// 55 733/s.
const COUNT_MOVES_ARMED: [u32; 4] = [2, 3, 100, 5_000];
/// Mode 3's arming attempts per move, when a pulse ending between the
/// count read and `stop_at` makes the target too close.
const ARM_TRIES: u32 = 5;
/// The pulse count of mode 4's move, at 30 000/s.
const COUNT_MOVE_WFI: u32 = 2_000;
/// The wait after each stop, CPU cycles (at least 2 ms at 48 MHz; see
/// [`STALL_CYCLES`]): no pulse may come after it.
const COUNT_SETTLE: u32 = 96_000;

/// Waits for an armed count stop with the CPU asleep: every `WFI` return
/// is counted in `wakes`. With `bounded`, the tick wake is on as a
/// timeout (each tick wakes the CPU, at most `ticks` ticks); without it
/// only the count stop's interrupt (or another interrupt) ends a `WFI`.
fn sleep_until_count(stepper: &mut StepGenerator, bounded: Option<u32>, wakes: &mut u32) -> bool {
    stepper.set_count_wake(true);
    if let Some(ticks) = bounded {
        stepper.set_tick_wake(true);
        let mut seen = 0;
        while !stepper.stopped_at_count() && seen < ticks {
            cortex_m::asm::wfi();
            *wakes += 1;
            if stepper.poll_tick().is_some() {
                seen += 1;
            }
        }
        stepper.set_tick_wake(false);
    } else {
        while !stepper.stopped_at_count() {
            cortex_m::asm::wfi();
            *wakes += 1;
        }
    }
    stepper.set_count_wake(false);
    !stepper.is_running()
}

/// Checks one finished move: stopped, the count exactly the target, and
/// still the target after 2 ms (no pulse after the stop).
fn count_exact(stepper: &StepGenerator, target: u32) -> bool {
    let stopped = !stepper.is_running() && stepper.pulses() == target;
    cortex_m::asm::delay(COUNT_SETTLE);
    stopped && stepper.pulses() == target
}

/// The `n<mode>` request (the count stop, `StepGenerator::stop_at`).
///
/// - Mode 1: moves of 1, 2, 3, 50 and 500 pulses from stopped: arm, then
///   `queue(1 000)`.
/// - Mode 2: moves of 1, 10, 1 000, 30 000 and 65 535 pulses from stopped:
///   arm, then `run(55 733)`. 65 535 is the 16-bit counter's reach.
/// - Mode 3: `run(55 733)`, then arm 2, 3, 100 and 5 000 pulses ahead while
///   it runs. Also the refusals: 1 ahead while running and the current
///   count (`CountNotAhead`), 66 000 ahead (`CountTooFar`).
/// - Mode 4: one move of 2 000 pulses at 30 000/s with the CPU asleep in
///   `WFI` and only the count stop's interrupt to wake it (no tick wake).
///
/// Modes 1-3 sleep in `WFI` too, with the tick wake on as a timeout. After
/// every move the count must be exactly the target, with no pulse in the
/// next 2 ms. Reply `n <moves> <exact> <errors> <refusals as expected>
/// <wakes> <pulses> <last error> <its move> <raced>`: `pulses` is the
/// count's change over the request, for the R3 to compare with the edges
/// it saw; `wakes` the `WFI` returns (mode 4: 1 when only the count stop
/// woke the CPU); then, for diagnosis, mode 3's last refused call (its
/// `error_code`, 100 + code for `run`, and the move's distance; 0 0 when
/// none) and its arming attempts refused because a pulse ended between the
/// count read and `stop_at` (retried, as the refusal is documented).
pub(crate) fn count_stop_run(req: &[u8], stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"n NA");
    };
    let mode = argument(req, 1);
    // The last refused call in mode 3: its error code (100 + code for
    // `run`) and the move's distance, for diagnosis.
    let mut failure = (0u32, 0u32);
    // Mode 3's arming attempts refused because a pulse ended in between.
    let mut raced = 0u32;
    let (mut moves, mut exact, mut errors, mut refusals, mut wakes) =
        (0u32, 0u32, 0u32, 0u32, 0u32);
    stepper.stop();
    next_tick(stepper);
    let before = stepper.pulses();
    match mode {
        1 => {
            for n in COUNT_MOVES_QUEUE {
                let target = stepper.pulses().wrapping_add(n);
                moves += 1;
                if stepper.stop_at(target).is_err() || !queue_retrying(stepper, 1_000) {
                    errors += 1;
                    stepper.stop();
                    continue;
                }
                if sleep_until_count(stepper, Some(2_000), &mut wakes)
                    && count_exact(stepper, target)
                {
                    exact += 1;
                }
                next_tick(stepper);
            }
        }
        2 => {
            for n in COUNT_MOVES_RUN {
                let target = stepper.pulses().wrapping_add(n);
                moves += 1;
                if stepper.stop_at(target).is_err() || stepper.run(55_733).is_err() {
                    errors += 1;
                    stepper.stop();
                    continue;
                }
                if sleep_until_count(stepper, Some(2_000), &mut wakes)
                    && count_exact(stepper, target)
                {
                    exact += 1;
                }
                next_tick(stepper);
            }
        }
        3 => {
            for k in COUNT_MOVES_ARMED {
                moves += 1;
                if let Err(error) = stepper.run(55_733) {
                    errors += 1;
                    failure = (100 + error_code(error), k);
                    continue;
                }
                watch_ticks(stepper, 3, &mut [0; 3]);
                let now = stepper.pulses();
                if stepper.stop_at(now.wrapping_add(1)) == Err(StepError::CountNotAhead) {
                    refusals += 1;
                }
                if stepper.stop_at(stepper.pulses()) == Err(StepError::CountNotAhead) {
                    refusals += 1;
                }
                // Beyond the 16-bit reach even after the pulses that end during the call.
                if stepper.stop_at(stepper.pulses().wrapping_add(66_000))
                    == Err(StepError::CountTooFar)
                {
                    refusals += 1;
                }
                // Armed `k` ahead of a count read just before. A pulse can
                // end between that read and `stop_at`'s own check (one
                // period is 18 µs); with `k` = 2 the target is then 1 ahead
                // and refused as documented. Retry only then.
                let mut armed = None;
                for _ in 0..ARM_TRIES {
                    let read = stepper.pulses();
                    let target = read.wrapping_add(k);
                    match stepper.stop_at(target) {
                        Ok(()) => {
                            armed = Some(target);
                            break;
                        }
                        Err(StepError::CountNotAhead) if stepper.pulses() != read => {
                            raced += 1;
                        }
                        Err(error) => {
                            failure = (error_code(error), k);
                            break;
                        }
                    }
                }
                let Some(target) = armed else {
                    errors += 1;
                    stepper.stop();
                    continue;
                };
                if sleep_until_count(stepper, Some(2_000), &mut wakes)
                    && count_exact(stepper, target)
                {
                    exact += 1;
                }
                next_tick(stepper);
            }
        }
        4 => {
            let target = stepper.pulses().wrapping_add(COUNT_MOVE_WFI);
            moves += 1;
            if stepper.stop_at(target).is_err() || stepper.run(30_000).is_err() {
                errors += 1;
                stepper.stop();
            } else if sleep_until_count(stepper, None, &mut wakes) && count_exact(stepper, target) {
                exact += 1;
            }
        }
        _ => return Body::new(b"n ERR"),
    }
    stepper.stop();
    let pulses = stepper.pulses().wrapping_sub(before);
    let mut body = Body::new(b"n");
    body.num(moves)
        .num(exact)
        .num(errors)
        .num(refusals)
        .num(wakes)
        .num(pulses)
        .num(failure.0)
        .num(failure.1)
        .num(raced);
    body
}
