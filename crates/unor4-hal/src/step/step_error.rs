//! Why a step-generator call was refused.

/// Why a step-generator call was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepError {
    /// The rate's period is shorter than the low time plus the minimum high
    /// time (plus one count of margin), so the pulse shape cannot fit.
    RateTooHigh,
    /// A rate of 0 has no period; stop the generator instead.
    ZeroRate,
    /// Fewer than two free ICU (IELSR) slots, one for the tick's DTC and
    /// one for the count stop's wake-up: other code holds the rest of the
    /// 32.
    /// The setup's timing checks run at compile time
    /// ([`StepSetup`](crate::step::StepSetup)); this is the one run-time
    /// refusal of [`StepGeneratorBuilder::build`](crate::step::StepGeneratorBuilder::build).
    NoIcuSlot,
    /// The call needs a stopped generator, and it is running.
    Running,
    /// `StepGenerator::queue` cannot target the next tick exactly: the
    /// latest tick has not been polled, a running generator's schedule is
    /// not known yet (one full tick after `run` or `stop`), or a change is
    /// already queued.
    NotReady,
    /// Too close to the next tick to arm the change safely (within 40 µs).
    TooLate,
    /// The setup has an ENA pin and the driver is disabled: call
    /// `StepGenerator::enable` before a non-zero rate.
    Disabled,
    /// The driver was enabled less than the ENA setup's lead ago: retry a
    /// little later (`StepGenerator::is_enabled` says when).
    Enabling,
    /// [`StepGenerator::stop_at`](crate::step::StepGenerator::stop_at): the
    /// count is not ahead of the pulses counted so far by enough (at least
    /// 1 while no pulse can come, 2 while pulses may run), or it has
    /// already passed.
    CountNotAhead,
    /// [`StepGenerator::stop_at`](crate::step::StepGenerator::stop_at): the
    /// count is more than 65 535 pulses ahead and the pulse counter is a
    /// 16-bit timer. Stop on the way, or use a 32-bit counter (GPT320,
    /// GPT321).
    CountTooFar,
    /// A count stop and a reversal while running cannot overlap:
    /// [`StepGenerator::queue`](crate::step::StepGenerator::queue) refuses a
    /// direction change while pulses run and a count stop is armed, and
    /// [`StepGenerator::stop_at`](crate::step::StepGenerator::stop_at)
    /// refuses while such a reversal is queued. The count is the same in
    /// either direction; a move to a count runs one way.
    ReversalWithCountStop,
}
