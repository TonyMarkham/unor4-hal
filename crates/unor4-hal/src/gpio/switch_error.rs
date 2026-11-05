//! Why a switch's activation edge was refused.

/// Why a switch with `with_edge` in its setup could not be built
/// ([`SwitchInit::build`](crate::gpio::SwitchInit::build)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchError {
    /// No free ICU (IELSR) slot to latch the edge: other code holds all 32.
    NoIcuSlot,
    /// The pin's `IRQn` is already linked to an ICU slot — by another
    /// switch on a pin that shares the number (several headers share one:
    /// A3 and A5 are both IRQ2, for one), or by other code. The hardware
    /// forbids one event in two slots (manual §13.4.2).
    IrqInUse,
}
