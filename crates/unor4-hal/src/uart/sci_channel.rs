//! Links the PAC's SCI engine markers to the shared register block.

use crate::intc::Event;

/// `PRCR` key + PRC1 write-enable (module-stop registers are PRC1-
/// protected, like the clock registers of Slice 1).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

mod private {
    /// Sealed supertrait: nothing outside this module can implement it,
    /// so the set of [`SciChannel`] types is closed — the chip's engine
    /// list is a hardware fact, not an extension point.
    pub trait Sealed {}
}

/// Marks a PAC peripheral that is one of the RA4M1's SCI engines.
///
/// The RA4M1's engines share one non-FIFO register-block layout — but
/// the PAC does not alias them all to one type: `SCI2` and `SCI9` deref
/// to `sci2::RegisterBlock`, while `SCI1` derefs to `sci0::RegisterBlock`
/// (`ra4m1` 0.2.0: `pub use self::sci0 as sci1`, block at `0x4007_0020`).
/// Every register this driver touches — `SMR`, `BRR`, `SCR`, `TDR`,
/// `SSR`, `RDR`, `SCMR`, `SEMR`, `SIMR1`–`SIMR3`, `CDR`, `DCCR`, `SPTR` —
/// is byte-for-byte identical between the PAC's `sci0` and `sci2`
/// modules at identical offsets (verified register-by-register against
/// the generated PAC sources); the layouts diverge only in FIFO/9-bit
/// registers the driver never accesses (`sci0`: FCR/FDR/LSR at 0x14–0x19,
/// `sci2`: TDRHL/RDRHL at 0x0E–0x13), and every touched offset is ≤ 0x1C
/// while the blocks sit 0x20 apart, so no access can reach a neighboring
/// engine's registers. That identity is what lets [`Uart`](crate::uart::Uart)
/// and `UartIrq` (feature `uart-irq`) be one generic driver over a
/// `sci2`-typed view ([`SciChannel::block`]).
pub trait SciChannel: private::Sealed {
    /// Raw pointer to the engine's register block, typed as the
    /// `sci2::RegisterBlock` layout (see the trait docs for why that
    /// typing is valid for every engine, SCI1 included). Interrupt
    /// service routines run without the engine value at hand (it lives
    /// behind a `&'static` shared handle), so the ISRs deref this
    /// instead — the engine is claimed exactly once at attach time and
    /// lives forever, which is what makes the raw access sound.
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
    /// (0 = SCI2, 1 = SCI9, 2 = SCI1). Arbitrary but stable per engine.
    const CHANNEL_ID: u8;

    /// Takes the engine out of module stop (its `MSTPCRB` bit; manual
    /// §10.2.3). The MSTP registers are PRC1-protected, so the write window
    /// unlocks and relocks `PRCR`.
    fn release_module_stop();

    /// A shared view of the engine's registers.
    ///
    /// Safe to call on a claimed engine: the peripheral token is
    /// consumed exactly once (by a constructor or `Peripherals::take`),
    /// so no other code can reach the same registers, and the register
    /// block lives at its fixed address forever. The same claim
    /// discipline is what makes the IRQ driver's raw `&*S::PTR` access
    /// in the interrupt handlers sound.
    fn block(&self) -> &'static ra4m1::sci2::RegisterBlock {
        unsafe { &*Self::PTR }
    }
}

impl private::Sealed for ra4m1::SCI1 {}
impl private::Sealed for ra4m1::SCI2 {}
impl private::Sealed for ra4m1::SCI9 {}

/// Clears one `MSTPCRB` bit under the PRC1 unlock. Addresses from the SVD:
/// `MSTP` block at `0x4004_7000`, `MSTPCRB` first at offset `0x00`.
fn release_mstpcrb(bit: u32) {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrb
        .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << bit)) });
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}

impl SciChannel for ra4m1::SCI1 {
    /// SCI1's register block at `0x4007_0020`, typed as
    /// `sci2::RegisterBlock`. The PAC hands the address out as
    /// `*const sci0::RegisterBlock`; the layout identity that makes the
    /// retyping valid for every register the driver touches is the
    /// trait-level evidence above (PAC sources, register-by-register).
    const PTR: *const ra4m1::sci2::RegisterBlock =
        ra4m1::SCI1::PTR as *const ra4m1::sci2::RegisterBlock;
    const EVENT_RXI: Event = Event::Sci1Rxi;
    const EVENT_TXI: Event = Event::Sci1Txi;
    const EVENT_ERI: Event = Event::Sci1Eri;
    const EVENT_TEI: Event = Event::Sci1Tei;
    const CHANNEL_ID: u8 = 2;

    /// `MSTPCRB.MSTPB30`: manual §10.2.3 names bit 30 "Serial
    /// Communication Interface 1 Module Stop"; the SVD field `mstpb30`
    /// matches.
    fn release_module_stop() {
        release_mstpcrb(30);
    }
}

impl SciChannel for ra4m1::SCI2 {
    const PTR: *const ra4m1::sci2::RegisterBlock = ra4m1::SCI2::PTR;
    const EVENT_RXI: Event = Event::Sci2Rxi;
    const EVENT_TXI: Event = Event::Sci2Txi;
    const EVENT_ERI: Event = Event::Sci2Eri;
    const EVENT_TEI: Event = Event::Sci2Tei;
    const CHANNEL_ID: u8 = 0;

    /// `MSTPCRB.MSTPB29` (SVD "Serial Communication Interface 2 Module
    /// Stop").
    fn release_module_stop() {
        release_mstpcrb(29);
    }
}

impl SciChannel for ra4m1::SCI9 {
    const PTR: *const ra4m1::sci2::RegisterBlock = ra4m1::SCI9::PTR;
    const EVENT_RXI: Event = Event::Sci9Rxi;
    const EVENT_TXI: Event = Event::Sci9Txi;
    const EVENT_ERI: Event = Event::Sci9Eri;
    const EVENT_TEI: Event = Event::Sci9Tei;
    const CHANNEL_ID: u8 = 1;

    /// `MSTPCRB.MSTPB22` (SVD "Serial Communication Interface 9 Module
    /// Stop").
    fn release_module_stop() {
        release_mstpcrb(22);
    }
}
