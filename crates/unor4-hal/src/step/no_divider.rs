//! No divider set.

use crate::{gpio::Sealed, step::Divider};

/// No `.with_divider(…)`: the STEP timer counts PCLKD/1. Fits a 32-bit
/// STEP timer only.
#[derive(Clone, Copy, Debug)]
pub struct NoDivider;

impl Sealed for NoDivider {}

impl Divider for NoDivider {
    const VALUE: u32 = 1;
    const TPCS: u8 = 0;
    const INSTANCE: Self = NoDivider;
}
