//! One wait's prescaler and period (host-tested).

use crate::wake::{WakeClock, WakeError};

/// The GPT prescalers (`GTCR.TPCS`, manual §22.2.12): code and divider as a
/// power of two (PCLKD/1, /4, /16, /64, /256, /1024).
const PRESCALERS: [(u8, u32); 6] = [(0, 0), (1, 2), (2, 4), (3, 6), (4, 8), (5, 10)];

/// One wait as the channel runs it: the prescaler code and `GTPR`. The
/// counter counts `GTPR + 1` steps of `divider / PCLKD` from 0 and
/// overflows; the overflow stops it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WakeShot {
    /// `GTCR.TPCS`.
    pub(crate) tpcs: u8,
    /// `GTPR`: the wait's step count − 1.
    pub(crate) gtpr: u32,
}

/// The shot for a wait of `nanos`: the finest prescaler whose step count,
/// rounded up, fits the channel. `Ok(None)` for a zero wait (it has ended
/// already).
///
/// No division: the wait in PCLKD counts comes from one multiply by
/// [`WakeClock::counts_per_ns`] (at most one short), then is raised until
/// `counts × 10⁹ ≥ nanos × PCLKD`, the exact round-up. Each prescaler's step
/// count is that, divided by its power of two by shifting and rounded up
/// again (the same as rounding the exact quotient up once).
///
/// # Errors
/// [`WakeError::TooLong`] past [`WakeClock::longest_nanos`].
pub(crate) fn wake_shot(nanos: u128, clock: &WakeClock) -> Result<Option<WakeShot>, WakeError> {
    if nanos == 0 {
        return Ok(None);
    }
    if nanos > u128::from(clock.longest_nanos) {
        return Err(WakeError::TooLong);
    }
    // nanos < 2⁴⁷ (the longest is 25 h), counts_per_ns < 2⁶¹: no overflow.
    let mut counts = ((nanos * clock.counts_per_ns) >> 64) as u64;
    let wanted = nanos * u128::from(clock.pclkd_hz);
    while u128::from(counts) * 1_000_000_000 < wanted {
        counts += 1;
    }
    for (tpcs, shift) in PRESCALERS {
        let steps = (counts + (1 << shift) - 1) >> shift;
        if steps <= u64::from(clock.max_gtpr) + 1 {
            return Ok(Some(WakeShot {
                tpcs,
                gtpr: (steps - 1) as u32,
            }));
        }
    }
    Err(WakeError::TooLong)
}
