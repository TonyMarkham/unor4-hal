//! The step generator's period schedule at a tick instant (host-tested).

/// Where the step counter stands at a tick instant, and the periods that
/// follow: the counter is `phase` counts into a period of `current` counts;
/// after it comes one period of `next` counts, then `period`-count periods
/// for good. A steady train has `current == next == period`; a rate change
/// at a tick leaves one or two transition periods in front.
///
/// All lengths are PCLKD counts; every period starts with its pulse at
/// counter 0. `phase < current` always holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Schedule {
    /// Counts into the current period at the tick instant.
    pub phase: u32,
    /// Length of the period in progress.
    pub current: u32,
    /// Length of the one period after it.
    pub next: u32,
    /// Length of every period after that.
    pub period: u32,
}

impl Schedule {
    /// A steady train: `period`-count periods, `phase` counts into one.
    pub const fn steady(phase: u32, period: u32) -> Self {
        Self {
            phase,
            current: period,
            next: period,
            period,
        }
    }

    /// The schedule `elapsed` counts later, with no change in between.
    pub fn advance(self, elapsed: u32) -> Self {
        let mut t = u64::from(self.phase) + u64::from(elapsed);
        if t < u64::from(self.current) {
            return Self {
                phase: t as u32,
                ..self
            };
        }
        t -= u64::from(self.current);
        if t < u64::from(self.next) {
            return Self {
                phase: t as u32,
                current: self.next,
                next: self.period,
                period: self.period,
            };
        }
        t -= u64::from(self.next);
        Self::steady((t % u64::from(self.period)) as u32, self.period)
    }

    /// How many pulse ends land in the `tick` counts that follow, where a
    /// pulse that started at counter 0 ends at counter `end` (for the tick
    /// latch: the low time plus 2 counts, measured on the board). An
    /// end at offset 0 counts here, one at offset `tick` in the next tick
    /// (the same latch rule at both tick instants).
    pub fn ends_in(self, end: u32, tick: u32) -> u32 {
        let (end, tick) = (i64::from(end), i64::from(tick));
        let in_tick = |offset: i64| (0..tick).contains(&offset);
        let mut count = 0;
        // The current period's own pulse end, if still ahead.
        if in_tick(end - i64::from(self.phase)) {
            count += 1;
        }
        // The next period's pulse end.
        let next_start = i64::from(self.current) - i64::from(self.phase);
        if in_tick(next_start + end) {
            count += 1;
        }
        // The steady periods' ends: next_start + next + k·period + end.
        let first = next_start + i64::from(self.next) + end;
        if first < tick {
            count += (tick - 1 - first) / i64::from(self.period) + 1;
        }
        count as u32
    }
}
