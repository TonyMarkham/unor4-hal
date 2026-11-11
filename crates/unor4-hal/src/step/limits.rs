//! The step generator's timing limits, shared by the compile-time setup
//! checks and the generator.

/// PCLKD in Hz: the HAL's fixed HOCO clock ([`crate::clock::HOCO_FREQ_HZ`]).
pub const PCLKD_HZ: u32 = crate::clock::HOCO_FREQ_HZ;

/// Slack, in PCLKD counts (0.5 µs at 48 MHz), kept around a pulse edge: an
/// idle reversal sets DIR only this far past a pulse's end, and the minimum
/// high time is at least twice it, so a tick's DTC write (14–17 counts after
/// the tick, measured) always lands well before the period it shortens can
/// end.
pub const EDGE_MARGIN: u32 = 24;

/// The longest LOW time, in PCLKD counts, a reversal supports: the delay
/// record that holds the DIR write back must outlast a whole pulse.
pub const MAX_REVERSAL_LOW: u32 = 400;

/// The tick length, µs.
pub const TICK_US: u32 = 2_000;

/// The tick timer's prescaler: `TPCS = 001b` is PCLKD/4 (§22.2.12), so a
/// 2 ms tick at 48 MHz is 24 000 counts — inside a 16-bit `GTPR`.
pub const TICK_TPCS: u8 = 0b001;
/// The divisor [`TICK_TPCS`] selects.
pub const TICK_DIVISOR: u32 = 4;

/// Whole PCLKD/`divider` counts in `ns`, rounded down.
pub(crate) const fn counts_floor(ns: u32, divider: u32) -> u64 {
    ns as u64 * PCLKD_HZ as u64 / (1_000_000_000 * divider as u64)
}

/// PCLKD/`divider` counts covering `ns`, rounded up, so a minimum time is
/// never undershot.
pub(crate) const fn counts_ceil(ns: u32, divider: u32) -> u64 {
    (ns as u64 * PCLKD_HZ as u64).div_ceil(1_000_000_000 * divider as u64)
}
