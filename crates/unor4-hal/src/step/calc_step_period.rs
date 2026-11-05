//! Rate → period maths for the step generator (host-tested).

use crate::step::{StepError, StepResult};

/// Nanoseconds → whole PCLKD counts, rounded **up** so a minimum time is
/// never undershot. `u64` throughout: `ns × pclkd` reaches ~2 × 10¹⁷ for
/// the largest `u32` inputs.
pub fn ns_to_counts(ns: u32, pclkd_hz: u32) -> u64 {
    (u64::from(ns) * u64::from(pclkd_hz)).div_ceil(1_000_000_000)
}

/// The period, in PCLKD counts, that delivers `rate` pulses/s: the nearest
/// whole count, `round(pclkd / rate)`. The delivered rate is then
/// `pclkd / counts`, off by at most `rate / (2 × pclkd)` relative — half a
/// count over the period.
///
/// `low_counts` and `min_high_counts` are the pulse shape in counts; the
/// period must hold both plus one count of margin, which also keeps
/// `GTCCRB` (= `low_counts`) strictly inside `GTPR` (manual §22.9.2(5))
/// and lets a start preload `GTCNT` past `GTCCRB`.
///
/// # Errors
/// [`StepError::ZeroRate`] for `rate == 0`; [`StepError::RateTooHigh`]
/// when the rounded period is shorter than the floor.
pub fn calc_step_period(
    pclkd_hz: u32,
    rate: u32,
    low_counts: u32,
    min_high_counts: u32,
) -> StepResult<u32> {
    if rate == 0 {
        return Err(StepError::ZeroRate);
    }
    let rate = u64::from(rate);
    let counts = (u64::from(pclkd_hz) + rate / 2) / rate;
    let floor = u64::from(low_counts) + u64::from(min_high_counts) + 1;
    if counts < floor {
        return Err(StepError::RateTooHigh);
    }
    // rate ≥ 1 ⇒ counts ≤ pclkd, which fits u32.
    Ok(counts as u32)
}
