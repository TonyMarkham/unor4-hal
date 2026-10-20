//! Why the period calculator refuses a request.

/// Why [`calc_period`](crate::pwm::calc_period) refused a request.
///
/// The reachable band at PCLKD = 48 MHz runs from ~0.72 Hz (PCLKD/1024
/// with the largest capped 16-bit period) to the count clock itself
/// (period = 1 tick).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodError {
    /// The requested frequency is above what the counter can divide
    /// down to: either above the count clock itself, or so close to it
    /// that the only deliverable period lands more than 5% fast.
    TooHigh,
    /// Even PCLKD/1024 with the largest capped 16-bit period cannot get
    /// this slow (below ~0.72 Hz at PCLKD = 48 MHz) — or the only
    /// deliverable period would land more than 5% slow.
    TooLow,
}
