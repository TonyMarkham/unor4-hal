//! The first cycle of a pulse train started from stopped (host-tested).

/// The STEP channel's first `GTPR` and its `GTCNT` preload for a train of
/// `period`-count periods whose first pulse comes `lead` counts after the
/// counter starts. `GTPBR` holds `period − 1`, so the overflow that makes
/// the first pulse reloads the steady period (`GTBER.PR = 01b`,
/// manual §22.3.2.1), and every later period is `period` counts.
///
/// The first cycle is `GTPR + 1` counts long: `period`, or longer when
/// `lead` plus the LOW time does not fit in one period. The preload is
/// always past `low` (the compare value that ends a pulse), so no compare
/// match fires before the first pulse, and never past `GTPR`
/// (§22.9.3). Needs `lead ≥ 1` and `period > low`.
pub(crate) fn first_cycle(period: u32, low: u32, lead: u32) -> (u32, u32) {
    let top = (period - 1).max(lead + low);
    (top, top + 1 - lead)
}
