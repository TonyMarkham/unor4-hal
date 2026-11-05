//! Whether a count stop's target can be armed (host-tested).

use crate::step::{StepError, StepResult};

/// Checks a [`StepGenerator::stop_at`](crate::step::StepGenerator::stop_at)
/// target against the pulses counted so far, `now`.
///
/// - `quiet`: no pulse can end while it arms (stopped, no start queued).
///   Then the target must be at least 1 ahead; otherwise at least 2, since
///   one pulse can end while it arms.
/// - A target more than 2³¹ ahead is taken as one already passed.
/// - `mask`: the counter's wrap mask. A 16-bit counter (`0xFFFF`) matches
///   its compare value every 65 536 pulses, so a target further ahead
///   would stop too early.
///
/// # Errors
/// [`StepError::CountNotAhead`] for a target too near or passed;
/// [`StepError::CountTooFar`] for one beyond the counter's reach.
pub(crate) fn check_count_target(count: u32, now: u32, quiet: bool, mask: u32) -> StepResult<()> {
    let ahead = count.wrapping_sub(now);
    let nearest = if quiet { 1 } else { 2 };
    if ahead < nearest || ahead > i32::MAX as u32 {
        return Err(StepError::CountNotAhead);
    }
    if ahead > mask {
        return Err(StepError::CountTooFar);
    }
    Ok(())
}

/// The counter's `GTCCRB` for a count stop at `count`: the count before
/// it, wrapped to the counter's width. Measured on the board (Validation.md,
/// `S11 countstop`): with `GTCCRB = count` the stop came one pulse late in
/// every move, at every rate, so the compare match of the event-counting
/// counter fires on the count event that leaves the compared value.
pub(crate) const fn count_compare_value(count: u32, mask: u32) -> u32 {
    count.wrapping_sub(1) & mask
}
