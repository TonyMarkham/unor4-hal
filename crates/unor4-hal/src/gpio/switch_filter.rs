//! The IRQ digital filter's sampling clock.

/// The sampling clock of the IRQ pin's digital filter (`IRQCRi.FCLKSEL`,
/// manual §13.2.1). The filter removes pulses shorter than 3 sampling
/// cycles (§13.4.3); with the HAL's PCLKB of 24 MHz that is 125 ns, 1 µs,
/// 4 µs or 8 µs. It is a glitch filter, not a debouncer: mechanical bounce
/// lasts milliseconds, which firmware handles. It filters only the
/// activation edge ([`Switch::activated`](crate::gpio::Switch::activated)),
/// not the level that [`Switch::is_active`](crate::gpio::Switch::is_active)
/// reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwitchFilter {
    /// Every PCLKB cycle (`FCLKSEL = 00b`): pulses under 3 cycles removed.
    Pclkb,
    /// PCLKB/8 (`01b`).
    PclkbDiv8,
    /// PCLKB/32 (`10b`).
    PclkbDiv32,
    /// PCLKB/64 (`11b`).
    PclkbDiv64,
}

impl SwitchFilter {
    /// The `FCLKSEL[1:0]` code.
    pub(crate) const fn fclksel(self) -> u8 {
        match self {
            Self::Pclkb => 0b00,
            Self::PclkbDiv8 => 0b01,
            Self::PclkbDiv32 => 0b10,
            Self::PclkbDiv64 => 0b11,
        }
    }
}
