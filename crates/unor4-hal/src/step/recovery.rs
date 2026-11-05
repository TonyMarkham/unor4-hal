//! Recovery after ticks that were not polled (host-tested): the schedule
//! rebuilt from the STEP channel's registers, the idle stop, and a
//! reversal the DTC applied at a tick nobody polled.

use crate::step::Schedule;

/// Read-order slack, PCLKD counts, when comparing how far the STEP counter
/// moved since the tick with the time the tick timer says has passed. The
/// tick timer counts PCLKD/4 and is read *before* the STEP counter, so
/// without an overflow the counter moved at least the tick timer's time;
/// the slack covers the capture's own latency (a GPT-to-GPT ELC event is
/// 0 cycles, manual Table 18.5; the 2-count pulse-end latch delay is the
/// largest measured on the board). An overflow in between would have taken
/// a whole period off — at least the low time plus the minimum HIGH time
/// (≥ 48 counts, `PulPinBuilder`) — far beyond this slack.
pub(crate) const RESYNC_SLACK: u32 = 8;

/// The schedule at the latest tick, rebuilt from the STEP channel's
/// registers read after it, or `None` when the reads cannot prove it.
///
/// - `phase`: the STEP counter captured at the tick;
/// - `count_now`: the STEP counter read after the tick;
/// - `since_tick`: PCLKD counts since the tick by the tick timer, read
///   before `count_now`;
/// - `gtpr`, `gtpbr`: the period registers, read between the two;
///
/// Valid only with no DTC change armed or running: then `GTPR` changes only
/// at an overflow (when `GTPBR` reloads it, §22.3.2.1).
/// - If the counter moved by at least `since_tick` (minus the slack) there
///   was no overflow since the tick, so the period in progress at the tick
///   was `GTPR + 1`, and every later one `GTPBR + 1`.
/// - Otherwise the counter wrapped since the tick. With `GTPR = GTPBR` (a
///   steady train) every period is `GTPBR + 1`, so the counter must stand
///   at `(phase + since_tick) mod period`, within the slack behind and
///   twice the slack ahead (the tick timer's 4-count steps and the read
///   order). If it does, the schedule is proven; if not (a different
///   period ended in between), `None`. Refusing every wrap made the
///   rebuild fail at every poll when a whole number of periods fits the
///   tick and the tick lands within a poll's latency of a period's end
///   (measured on the board: phases 118–260 counts before the end, at
///   10 000, 40 000 and 50 000/s).
pub(crate) fn resync_schedule(
    phase: u32,
    count_now: u32,
    since_tick: u32,
    gtpr: u32,
    gtpbr: u32,
) -> Option<Schedule> {
    if phase > gtpr {
        return None;
    }
    let current = gtpr.checked_add(1)?;
    let period = gtpbr.checked_add(1)?;
    let unwrapped = count_now
        .checked_sub(phase)
        .is_some_and(|moved| moved.saturating_add(RESYNC_SLACK) >= since_tick);
    if !unwrapped {
        if gtpr != gtpbr {
            return None;
        }
        let span = u64::from(period);
        let expected = (u64::from(phase) + u64::from(since_tick)) % span;
        let ahead = (u64::from(count_now) + span - expected) % span;
        let close = ahead <= u64::from(2 * RESYNC_SLACK) || ahead >= span - u64::from(RESYNC_SLACK);
        if !close {
            return None;
        }
    }
    Some(Schedule {
        phase,
        current,
        next: period,
        period,
    })
}

/// True when an idle generator should stop: both period registers hold the
/// idle length (`GTPR = GTPBR = idle − 1`: no pulse due for ~89 s) and the
/// phase captured at the tick — the time since the last pulse began, since
/// no overflow comes while idle — is over `limit` counts. Needs no
/// schedule, so ticks passed over cannot keep an idle generator running.
pub(crate) fn idle_expired(phase: u32, gtpr: u32, gtpbr: u32, idle: u32, limit: u64) -> bool {
    gtpr == idle - 1 && gtpbr == idle - 1 && u64::from(phase) > limit
}

/// For a reversal the DTC applied at a tick that was passed over: how many
/// of the `delta` pulses that ended since the previous latch ran in the
/// *new* direction.
///
/// - `before`: the schedule at the previous (polled) tick, exact — a
///   reversal is only queued with one;
/// - `tick`: the tick length, PCLKD counts (the reversal's target is always
///   the tick after the polled one);
/// - `latch_end`: where a pulse's end reaches the latch (low time +
///   `PULSE_END_LATCH_DELAY`);
/// - `readback`, `in_order_floor_extra`: the DTC's counter read-back after
///   its chain, and the chain's minimum delay (`dir_write_min`).
///
/// Old-direction pulses are those ending before the reversal tick, the one
/// LOW at it, and one that fired inside the write latency — the same split
/// [`StepGenerator::poll_tick`](crate::step::StepGenerator::poll_tick)
/// makes at a polled reversal tick.
pub(crate) fn missed_reversal_new_pulses(
    before: Schedule,
    tick: u32,
    latch_end: u32,
    readback: u32,
    in_order_floor_extra: u32,
    delta: u32,
) -> u32 {
    let ended_before = before.ends_in(latch_end, tick);
    let at_tick = before.advance(tick);
    let in_flight = u32::from(at_tick.phase < latch_end);
    let old_pulse_first = u32::from(readback < at_tick.phase.saturating_add(in_order_floor_extra));
    delta.saturating_sub(ended_before + in_flight + old_pulse_first)
}
