//! The wake timer's clock constants, worked out once at build.

/// What each [`wake_shot`](crate::wake::wake_shot) needs about the clock
/// and the channel, computed once when the timer is built, so `start` does
/// no division (a software routine on this CPU: each one took tens of µs
/// before the count began, measured on the bench through the R3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WakeClock {
    /// PCLKD, Hz.
    pub(crate) pclkd_hz: u32,
    /// `PCLKD × 2⁶⁴ / 10⁹`, rounded down: ns × this, shifted down 64, is
    /// the wait in PCLKD counts, at most one short.
    pub(crate) counts_per_ns: u128,
    /// The channel's largest `GTPR`.
    pub(crate) max_gtpr: u32,
    /// The longest wait, ns: the whole counter at PCLKD/1024, rounded down.
    pub(crate) longest_nanos: u64,
}

impl WakeClock {
    /// The constants for PCLKD `pclkd_hz` and a channel reaching `max_gtpr`.
    pub(crate) const fn new(pclkd_hz: u32, max_gtpr: u32) -> Self {
        Self {
            pclkd_hz,
            counts_per_ns: ((pclkd_hz as u128) << 64) / 1_000_000_000,
            max_gtpr,
            longest_nanos: ((max_gtpr as u128 + 1) * 1024 * 1_000_000_000 / pclkd_hz as u128)
                as u64,
        }
    }
}
