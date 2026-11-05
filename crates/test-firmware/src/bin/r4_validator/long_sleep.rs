//! A long sleep at a steady rate, then a `queue` (`s<mode>`): the CPU
//! makes no call for hundreds of ticks while the train runs, then polls
//! once and changes the rate. WiFi only, like the rest of the step
//! generator's checks.

use crate::{
    body::Body,
    clock::Clock,
    requests::argument,
    step_run::{next_tick, push_position, watch_ticks},
};

use unor4_hal::step::{StepError, StepGenerator};

/// Each mode's steady rate (pulses/s), sleep (ms) and the rate queued
/// after it. Every sleep keeps the 16-bit counter (GPT162) under 32 768
/// pulses, the most its software extension can bridge without a read
/// (`StepGenerator::pulses`).
/// - Mode 1: 100 ms at 50 000/s (5 000 pulses), then 30 000/s.
/// - Mode 2: 500 ms at 40 000/s (20 000 pulses), then a reversal.
/// - Mode 3: 3 s at 10 000/s (30 000 pulses), then 20 000/s.
const SLEEPS: [(i32, u32, i32); 3] = [
    (50_000, 100, 30_000),
    (40_000, 500, -40_000),
    (10_000, 3_000, 20_000),
];
/// Ticks checked before the sleep, once the run is steady.
const TICKS_BEFORE: u32 = 5;
/// Ticks checked after the queued change.
const TICKS_AFTER: u32 = 20;
/// Most polls allowed until `queue` is accepted after the sleep.
const MAX_POLLS: u32 = 10;

/// The `s<mode>` request. `run` at the mode's rate, [`TICKS_BEFORE`]
/// ticks with every exact prediction checked, then a busy wait of the
/// mode's sleep on the SysTick clock with no generator call at all (the
/// tick wake is off; `cortex_m::asm::delay` waits at least its count,
/// measured at 3 CPU cycles per count at this call site in the first run,
/// so it does not time this), then one poll,
/// `queue` of the next rate (retried at each tick while `NotReady` /
/// `TooLate`), and [`TICKS_AFTER`] ticks checked. Reply `s <skipped>
/// <polls> <errors> <phase misses> <count misses> <checked> <pulses>
/// <position>`: `skipped` the first poll's passed-over ticks, `polls` the
/// polls until `queue` was accepted (1 = the first), `errors` the refused
/// calls, `pulses` and `position` the changes over the request (`M <n>`
/// when the position fell), for the R3. Then, for diagnosis, `<first
/// phase> <since tick at the first poll> <not ready> <too late>`: the
/// phase the first poll's tick captured, PCLKD counts since that tick
/// when the poll returned, and the refusals while retrying.
pub(crate) fn long_sleep(
    req: &[u8],
    stepper: &mut Option<StepGenerator>,
    clock: &mut Clock,
) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"s NA");
    };
    let mode = argument(req, 1) as usize;
    let Some(&(rate, sleep_ms, next)) = mode.checked_sub(1).and_then(|i| SLEEPS.get(i)) else {
        return Body::new(b"s ERR");
    };
    stepper.stop();
    next_tick(stepper);
    let before = stepper.pulses();
    let start = stepper.position();
    let mut checks = [0u32; 3];
    let mut errors = 0u32;
    stepper.set_tick_wake(false);
    if stepper.run(rate).is_err() {
        errors += 1;
    }
    watch_ticks(stepper, TICKS_BEFORE, &mut checks);
    clock.wait_ms(sleep_ms);
    let first = next_tick(stepper);
    let skipped = first.skipped;
    let first_phase = first.phase_counts;
    let since_poll = stepper.since_tick_counts();
    let (mut not_ready, mut too_late) = (0u32, 0u32);
    let mut polls = 1u32;
    loop {
        match stepper.queue(next) {
            Ok(_) => break,
            Err(error @ (StepError::NotReady | StepError::TooLate)) if polls < MAX_POLLS => {
                if error == StepError::NotReady {
                    not_ready += 1;
                } else {
                    too_late += 1;
                }
                next_tick(stepper);
                polls += 1;
            }
            Err(_) => {
                errors += 1;
                break;
            }
        }
    }
    watch_ticks(stepper, TICKS_AFTER, &mut checks);
    stepper.stop();
    // The position is as of the latest tick: one tick after the stop
    // includes every pulse.
    next_tick(stepper);
    next_tick(stepper);
    let pulses = stepper.pulses().wrapping_sub(before);
    let mut body = Body::new(b"s");
    body.num(skipped)
        .num(polls)
        .num(errors)
        .num(checks[0])
        .num(checks[1])
        .num(checks[2])
        .num(pulses);
    push_position(&mut body, stepper.position() - start);
    body.num(first_phase)
        .num(since_poll)
        .num(not_ready)
        .num(too_late);
    body
}
