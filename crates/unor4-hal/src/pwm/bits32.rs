//! The 32-bit GPT width.

use crate::{gpio::Sealed, pwm::Width};

/// A 32-bit GPT counter: GPT320 and GPT321 (§22.1).
#[derive(Clone, Copy, Debug)]
pub struct Bits32;

impl Sealed for Bits32 {}

impl Width for Bits32 {
    const BITS: u8 = 32;
    const MAX_GTPR: u32 = u32::MAX;
}
