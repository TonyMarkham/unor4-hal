//! Links the PAC's SCI engine markers to the shared register block.

use core::ops::Deref;

mod private {
    /// Sealed supertrait: nothing outside this module can implement it,
    /// so the set of [`SciChannel`] types is closed — the chip's engine
    /// list is a hardware fact, not an extension point.
    pub trait Sealed {}
}

/// Marks a PAC peripheral that is one of the RA4M1's SCI engines.
///
/// Every engine shares one non-FIFO register-block layout — the PAC
/// aliases `sci9` to `sci2`, so both markers deref to the same
/// `sci2::RegisterBlock`. That alias is what lets [`Uart`](super::Uart)
/// be one generic driver instead of one struct per engine. SCI1 joins
/// with Slice 6 (the WiFi AT channel).
pub trait SciChannel: Deref<Target = ra4m1::sci2::RegisterBlock> + private::Sealed {}

impl private::Sealed for ra4m1::SCI2 {}
impl private::Sealed for ra4m1::SCI9 {}

impl SciChannel for ra4m1::SCI2 {}
impl SciChannel for ra4m1::SCI9 {}
