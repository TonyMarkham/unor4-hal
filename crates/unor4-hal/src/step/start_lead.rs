//! How long a start waits for its first pulse (host-tested).

/// PCLKD counts from a reference instant (a start from the CPU, or the
/// tick a change lands on) to the first pulse.
///
/// - Always at least `min_high`, the pulse shape's minimum HIGH time.
/// - When DIR was just written (`dir_written`), also the DIR set-up time,
///   `dir_setup`, less `head_start`: the counts DIR is known to have been
///   written before the reference instant (0 for a start from the CPU,
///   which writes DIR right before starting the counter; the queue guard
///   for a start at a tick, since `queue` writes DIR before it checks that
///   at least the guard is left before the tick).
pub(crate) fn start_lead(dir_written: bool, min_high: u32, dir_setup: u32, head_start: u32) -> u32 {
    if dir_written {
        min_high.max(dir_setup.saturating_sub(head_start))
    } else {
        min_high
    }
}
