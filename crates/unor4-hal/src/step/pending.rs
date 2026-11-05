//! A rate change queued for the step generator's next tick.

/// A change queued for the next tick.
#[derive(Clone, Copy)]
pub(crate) struct Pending {
    /// GPT165's value at the target tick.
    pub(crate) tick_raw: u16,
    /// The transition period written to `GTPR` (+1).
    pub(crate) crossing: u32,
    /// The new period written to `GTPBR` (+1).
    pub(crate) period: u32,
    /// A start from stopped (hardware start at the tick) rather than a
    /// change of a running train.
    pub(crate) start: bool,
    /// A reversal: the chain also writes DIR, after its delay record.
    pub(crate) reverse: bool,
}
