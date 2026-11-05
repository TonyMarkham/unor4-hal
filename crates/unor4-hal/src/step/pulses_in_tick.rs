//! How many pulses end within one tick (host-tested).

/// PCLKD counts between GPT320 reaching `GTCCRB` (a pulse's end) and that
/// end being included in a tick latch, relative to the phase GPT320 itself
/// captures at the tick. **Measured on the board, not from the manual**
/// (Validation.md, Slice 11 step 4, 2026-11-03): only this value predicted every
/// tick in 27 runs, six of which swept every phase; 0, 1, 3 and 4 failed.
pub const PULSE_END_LATCH_DELAY: u32 = 2;

/// The number of pulse ends that land between one tick latch and the next,
/// given the step counter's `phase` at the first tick (PCLKD counts into a
/// period of `period` counts) and the tick length `tick` in PCLKD counts.
///
/// `end` is the counter value at which a pulse end reaches the tick latch:
/// a pulse ending at counter `end` exactly at a tick instant counts in the
/// tick that *starts* there. For GPT320 → ELC → GPT321 that is the low time
/// plus [`crate::step::PULSE_END_LATCH_DELAY`] counts (measured on the
/// board; see that constant). Pulse `k` reaches the latch at offset
/// `t = end − phase + k × period` after the first tick instant, and the
/// ends counted are those with `0 ≤ t < tick`; the same rule at both
/// instants makes consecutive ticks partition the pulse ends exactly.
///
/// Returns 0 for a zero period or tick. Integer maths only (`i64`).
pub fn pulses_in_tick(period: u32, end: u32, phase: u32, tick: u32) -> u32 {
    if period == 0 || tick == 0 {
        return 0;
    }
    let (period, first, tick) = (
        i64::from(period),
        i64::from(end) - i64::from(phase),
        i64::from(tick),
    );
    // Count k with 0 ≤ first + k·period ≤ tick − 1:
    // k from ceil(−first / period) to floor((tick − 1 − first) / period).
    let lowest = -(first.div_euclid(period));
    let highest = (tick - 1 - first).div_euclid(period);
    (highest - lowest + 1).max(0) as u32
}
