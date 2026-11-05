//! The 16-bit GPT width.

use crate::{gpio::Sealed, pwm::Width};

/// A 16-bit GPT counter: GPT162–GPT167 (§22.2: "For GPT16m … GTCNT is a
/// 16-bit register"; upper bits read 0, writes ignored, §22.2.20).
#[derive(Clone, Copy, Debug)]
pub struct Bits16;

impl Sealed for Bits16 {}

impl Width for Bits16 {
    const BITS: u8 = 16;
    const MAX_GTPR: u32 = 0xFFFF;
}
