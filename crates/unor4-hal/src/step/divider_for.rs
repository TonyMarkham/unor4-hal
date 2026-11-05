//! Which dividers fit which STEP timer width.

use crate::{
    gpio::Sealed,
    pwm::{Bits16, Bits32},
    step::{Div4, Div16, Div64, Div256, Div1024, NoDivider},
};

/// Divider choice `Self` can clock PUL pin `P`, whose timer has width `W`
/// (design §5):
/// - `Bits32`: no divider; the longest period is ~89 s already.
/// - `Bits16`: `Div4` … `Div1024`. `Div1` fits neither: a 16-bit period at
///   /1 (1.37 ms) is shorter than the 2 ms tick, so rate 0 could not hold
///   pulses off between polls.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot clock PUL pin `{P}`: its timer is `{W}`",
    label = "divider does not fit the timer",
    note = "a `Bits16` PUL timer takes `Div4` … `Div1024`; `Div1` is refused (a 1.37 ms period is shorter than the 2 ms tick)",
    note = "a `Bits32` PUL timer takes no `.with_divider(…)`"
)]
pub trait DividerFor<P, W>: Sealed {}

impl<P> DividerFor<P, Bits32> for NoDivider {}
impl<P> DividerFor<P, Bits16> for Div4 {}
impl<P> DividerFor<P, Bits16> for Div16 {}
impl<P> DividerFor<P, Bits16> for Div64 {}
impl<P> DividerFor<P, Bits16> for Div256 {}
impl<P> DividerFor<P, Bits16> for Div1024 {}
