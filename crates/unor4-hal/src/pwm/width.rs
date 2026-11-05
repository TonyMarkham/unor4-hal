//! A GPT counter width, as a type.

use crate::gpio::Sealed;

/// A GPT counter width: [`Bits16`](crate::pwm::Bits16) or
/// [`Bits32`](crate::pwm::Bits32). Sealed.
pub trait Width: Sealed {
    /// Counter width in bits.
    const BITS: u8;
    /// The largest `GTPR` value: a period of `MAX_GTPR + 1` counts.
    const MAX_GTPR: u32;
}
