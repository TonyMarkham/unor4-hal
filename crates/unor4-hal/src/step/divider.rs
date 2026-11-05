//! The STEP timer's count-clock divider choice.

use crate::gpio::Sealed;

/// A count-clock divider for the STEP timer: one of the six `GTCR.TPCS`
/// settings (PCLKD/1 … /1024, manual §22.2.12), or
/// [`NoDivider`](crate::step::NoDivider). Sealed.
pub trait Divider: Sealed + Copy {
    /// The PCLKD divisor.
    const VALUE: u32;
    /// The `GTCR.TPCS[2:0]` value selecting it (§22.2.12: 000b = /1, 001b =
    /// /4, 010b = /16, 011b = /64, 100b = /256, 101b = /1024).
    const TPCS: u8;
    /// A value of the type, for setups written generically.
    const INSTANCE: Self;
}
