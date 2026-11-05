//! The ELC's two software event generators (manual §18.2.2).

/// Which software event generator [`crate::elc::Elc::fire`] uses:
/// `ELSEGR0` raises [`crate::elc::ElcEvent::SoftwareEvent0`], `ELSEGR1`
/// raises [`crate::elc::ElcEvent::SoftwareEvent1`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareEvent {
    /// `ELSEGR0` → `ELC_SWEVT0`.
    Zero,
    /// `ELSEGR1` → `ELC_SWEVT1`.
    One,
}
