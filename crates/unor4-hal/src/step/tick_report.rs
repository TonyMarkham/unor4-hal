//! What one 2 ms tick latched.

/// What one 2 ms tick latched in hardware, returned by
/// `StepGenerator::poll_tick`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickReport {
    /// Tick number since construction (hardware-counted, wrapping).
    pub tick: u32,
    /// Completed pulses latched at this tick, wrapping (the
    /// `StepGenerator::pulses` count at the tick's instant).
    pub pulses: u32,
    /// GPT320's counter at the tick's instant: PCLKD counts since the
    /// current period began (below the low time = inside a pulse). Only
    /// meaningful while the generator runs.
    pub phase_counts: u32,
    /// Ticks that passed since the previous report without being polled.
    /// Non-zero means the caller polled less often than once per tick.
    pub skipped: u32,
    /// True when a rate queued with `StepGenerator::queue` was applied at
    /// this tick, or at a tick passed over since the previous report.
    pub applied: bool,
    /// True when, at an applied tick, the old period ended inside the DTC's
    /// write latency (~0.3 µs), so its pulse fired just after the tick and
    /// the transition period followed it. The schedule and predictions
    /// account for it either way.
    pub old_pulse_first: bool,
    /// The phase the generator's schedule predicted for this tick, when it
    /// had one — equal to [`TickReport::phase_counts`] whenever the timing
    /// model is right.
    pub expected_phase: Option<u32>,
    /// Signed position in pulses as of this tick's latch: completed pulses
    /// forward minus completed pulses backward. Pulses that end after a
    /// reversal but started before DIR flipped count in the old direction.
    pub position: i64,
    /// At a tick that applied a reversal: when DIR flipped, in PCLKD counts
    /// after the tick instant (read back by the DTC chain).
    pub dir_changed_at: Option<u32>,
}
