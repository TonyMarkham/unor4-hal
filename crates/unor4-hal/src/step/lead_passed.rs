//! When the ENA lead has passed (host-tested).

/// True once `elapsed` tick-timer counts (PCLKD/4) since `enable` cover a
/// lead of `lead` counts in real time.
///
/// `enable` stamps the tick timer's count, which can be taken just before
/// the counter steps: the first count may then cover almost no time, so
/// `elapsed` counts prove only `elapsed − 1` counts of real time. The lead
/// has passed once `elapsed` is at least `lead + 1`. Counting from
/// `elapsed ≥ lead` let the real lead end up to one count (83 ns at 48 MHz)
/// short. So `StepGenerator::is_enabled` turns true at most 2 counts (167 ns)
/// after the lead: one for this margin, one for the read's own step.
pub(crate) const fn lead_passed(elapsed: u64, lead: u64) -> bool {
    lead == 0 || elapsed > lead
}
