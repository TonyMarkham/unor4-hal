//! The count stop cancelled while its match fires (`c`): `cancel_stop_at`
//! called at random moments around the pulse that reaches an armed count,
//! so some cancels land before the match, some after, and some across it.
//! The moments are drawn afresh each request, so repeated runs cover
//! different timings.
//! WiFi only, like the rest of the step generator's checks.

use crate::{
    body::Body,
    step_run::{error_code, next_tick},
};

use unor4_hal::step::StepGenerator;

/// The rate the race runs at: the fastest the validator uses, so a period
/// (861 PCLKD counts) is short and the sweep crosses the match often.
const RACE_RATE: i32 = 55_733;
/// Cancels the request makes.
const RACE_ATTEMPTS: u32 = 2_000;
/// The range of the random wait after the pulse before the target ends,
/// in `cortex_m::asm::delay` counts: at least 400 CPU cycles (8.3 µs).
/// One count took 1.25 to 3 cycles depending on the call site (the delay
/// probe `m`; the first `S11 sleep` run), so not a fixed time; the R3
/// requires both outcomes in every run, which shows the waits reach past
/// the match.
const RACE_SWEEP: u32 = 400;
/// The wait after each cancel before the count is read again, delay
/// counts (at least 33 µs, about 2 periods): a running train has moved on
/// by then.
const RACE_SETTLE: u32 = 1_600;
/// Longest spin for the pulse before the target, in `pulses()` reads.
const RACE_SPINS: u32 = 100_000;

/// What one cancel found, when it broke the contract: the reply carries
/// the first.
const REACHED_WRONG_COUNT: u32 = 1;
/// Reported as stopped at the count, but pulses still came.
const REACHED_BUT_RUNNING: u32 = 2;
/// Not reported as stopped, but no pulse came after the cancel.
const CANCELLED_BUT_STOPPED: u32 = 3;
/// The pulse before the target never came.
const NEVER_REACHED: u32 = 4;

/// The `c` request. Each attempt runs at [`RACE_RATE`] (starting the train
/// if a stop ended the previous attempt), arms a stop 4 pulses ahead,
/// waits for the pulse before the target to end, waits a random time, and
/// cancels. Then exactly one of these must hold:
/// - the stop won: `stopped_at_count()` is true, the count is the target,
///   and no pulse comes in the next 100 µs;
/// - the cancel won: `stopped_at_count()` is false and pulses keep coming.
///
/// Reply `c <attempts> <stops won> <cancels won> <violations> <first
/// violation> <errors> <last error> <pulses>`: `first violation` 0 when
/// none (1 stopped at a wrong count, 2 reported stopped but running, 3 not
/// reported but stopped, 4 the target never neared); `errors` the refused
/// `run`/`stop_at` calls and `last error` the last one's code
/// (`error_code`, 0 when none); `pulses` the count's change over the
/// request, for the R3.
pub(crate) fn cancel_race(stepper: &mut Option<StepGenerator>) -> Body {
    let Some(stepper) = stepper.as_mut() else {
        return Body::new(b"c NA");
    };
    stepper.stop();
    next_tick(stepper);
    let before = stepper.pulses();
    // A different sequence of waits each request: seeded from the count.
    let mut seed: u32 = before ^ 0x9E37_79B9;
    let (mut stops, mut cancels, mut violations, mut first) = (0u32, 0u32, 0u32, 0u32);
    let (mut errors, mut last_error) = (0u32, 0u32);
    for _ in 0..RACE_ATTEMPTS {
        if !stepper.is_running()
            && let Err(error) = stepper.run(RACE_RATE)
        {
            errors += 1;
            last_error = error_code(error);
            continue;
        }
        let target = stepper.pulses().wrapping_add(4);
        if let Err(error) = stepper.stop_at(target) {
            errors += 1;
            last_error = error_code(error);
            stepper.stop();
            continue;
        }
        // The pulse before the target has ended: the match is at most a
        // period away.
        let mut spins = 0;
        while (stepper.pulses().wrapping_sub(target) as i32) < -1 && spins < RACE_SPINS {
            spins += 1;
        }
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        cortex_m::asm::delay((seed >> 8) % RACE_SWEEP + 1);
        stepper.cancel_stop_at();
        let reached = stepper.stopped_at_count();
        let at = stepper.pulses();
        cortex_m::asm::delay(RACE_SETTLE);
        let later = stepper.pulses();
        let violation = if spins >= RACE_SPINS {
            NEVER_REACHED
        } else if reached && at != target {
            REACHED_WRONG_COUNT
        } else if reached && later != at {
            REACHED_BUT_RUNNING
        } else if !reached && later == at {
            CANCELLED_BUT_STOPPED
        } else {
            0
        };
        match (violation, reached) {
            (0, true) => stops += 1,
            (0, false) => cancels += 1,
            _ => {
                violations += 1;
                if first == 0 {
                    first = violation;
                }
                stepper.stop();
            }
        }
    }
    stepper.stop();
    next_tick(stepper);
    let pulses = stepper.pulses().wrapping_sub(before);
    let mut body = Body::new(b"c");
    body.num(RACE_ATTEMPTS)
        .num(stops)
        .num(cancels)
        .num(violations)
        .num(first)
        .num(errors)
        .num(last_error)
        .num(pulses);
    body
}
