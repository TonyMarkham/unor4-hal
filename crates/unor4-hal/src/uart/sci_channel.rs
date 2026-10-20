//! Links the PAC's SCI engine markers to the shared register block.

use crate::intc::Event;

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
pub trait SciChannel: Deref<Target = ra4m1::sci2::RegisterBlock> + private::Sealed {
    /// Raw pointer to the engine's register block. Interrupt service
    /// routines run without the engine value at hand (it lives behind a
    /// `&'static` shared handle), so the ISRs deref this instead — the
    /// engine is claimed exactly once at attach time and lives forever,
    /// which is what makes the raw access sound.
    const PTR: *const ra4m1::sci2::RegisterBlock;

    /// IELSR event raised when RDRF sets (while `SCR.RIE` is on).
    const EVENT_RXI: Event;

    /// IELSR event raised when TDRE sets (while `SCR.TIE` is on).
    const EVENT_TXI: Event;

    /// IELSR event raised when a receive error flag (ORER/FER/PER)
    /// sets (while `SCR.RIE` is on).
    const EVENT_ERI: Event;

    /// IELSR event raised at transmit end (TEND, while `SCR.TEIE` is
    /// on). Reserved for future transmit-end interrupt support; the
    /// current IRQ driver polls TEND for flush and does not link TEI.
    const EVENT_TEI: Event;

    /// Small engine identity for the IRQ driver's attach-once claim
    /// (0 = SCI2, 1 = SCI9). Arbitrary but stable per engine.
    const CHANNEL_ID: u8;
}

impl private::Sealed for ra4m1::SCI2 {}
impl private::Sealed for ra4m1::SCI9 {}

impl SciChannel for ra4m1::SCI2 {
    const PTR: *const ra4m1::sci2::RegisterBlock = ra4m1::SCI2::PTR;
    const EVENT_RXI: Event = Event::Sci2Rxi;
    const EVENT_TXI: Event = Event::Sci2Txi;
    const EVENT_ERI: Event = Event::Sci2Eri;
    const EVENT_TEI: Event = Event::Sci2Tei;
    const CHANNEL_ID: u8 = 0;
}

impl SciChannel for ra4m1::SCI9 {
    const PTR: *const ra4m1::sci2::RegisterBlock = ra4m1::SCI9::PTR;
    const EVENT_RXI: Event = Event::Sci9Rxi;
    const EVENT_TXI: Event = Event::Sci9Txi;
    const EVENT_ERI: Event = Event::Sci9Eri;
    const EVENT_TEI: Event = Event::Sci9Tei;
    const CHANNEL_ID: u8 = 1;
}
