//! The step generator's one-shot DTC transfer on the 2 ms tick.

use crate::{
    intc::{Event, Ielsr},
    step::{MAX_RECORDS, TransferInfo, VectorTable},
};

use core::{
    ptr::{addr_of, addr_of_mut, write_volatile},
    sync::atomic::{Ordering, compiler_fence},
};

/// `PRCR` key + PRC1 write-enable (`MSTP*` registers are PRC1-protected).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock.
const PRCR_LOCK: u16 = 0xA500;

/// `IELSRn.DTCE` (bit 24; manual §13, IELSRn: "b24 DTCE DTC Activation
/// Enable").
const IELSR_DTCE: u32 = 1 << 24;

/// `DTCSTS.ACT` (bit 15): a DTC transfer is in progress (manual §17.2.10).
const DTCSTS_ACT: u16 = 1 << 15;

/// Only the step generator uses the DTC, and it is claimed exactly once
/// (the PAC's `DTC` token), so these statics have one owner.
static mut VECTORS: VectorTable = VectorTable([0; 32]);
static mut TRANSFER: TransferInfo = TransferInfo([[0; 4]; MAX_RECORDS]);

/// The DTC, claimed by the step generator, with one IELSR slot linked to
/// the tick event. Each [`TickDtc::arm`] runs exactly one transfer at the
/// next tick: when the transfer completes, the DTC clears `IELSRn.DTCE`
/// itself (manual §17.3: "On completion of a specified round of data
/// transfer, the ICU.IELSRn.DTCE bit is set to 0"), so an unarmed tick
/// moves nothing.
pub(crate) struct TickDtc {
    _dtc: ra4m1::DTC,
    ielsr: Ielsr,
    event: Event,
}

impl TickDtc {
    /// Releases the DTC from module stop (`MSTPCRA.MSTPA22 = 0`, manual §10),
    /// points `DTCVBR` at the vector table, links `event` to a free IELSR
    /// slot (NVIC line left disabled — the CPU never takes this interrupt)
    /// and starts the DTC (`DTCST = 1`). When no slot is free, the DTC comes
    /// back untouched.
    pub(crate) fn new(dtc: ra4m1::DTC, event: Event) -> Result<Self, ra4m1::DTC> {
        let Some(mut ielsr) = Ielsr::take_any() else {
            return Err(dtc);
        };
        release_module_stop_dtc();
        dtc.dtcst.write(|w| unsafe { w.bits(0) });
        // DTCCR: RRS = 0 (always read the transfer information — it is
        // rewritten before every arm); bit 3 is reserved, written as 1.
        dtc.dtccr.write(|w| unsafe { w.bits(0x08) });
        let vectors = addr_of!(VECTORS) as u32;
        dtc.dtcvbr.write(|w| unsafe { w.bits(vectors) });
        ielsr.link(event);
        unsafe {
            write_volatile(
                addr_of_mut!(VECTORS.0[ielsr.slot() as usize]),
                addr_of!(TRANSFER) as u32,
            );
        }
        compiler_fence(Ordering::SeqCst);
        dtc.dtcst.write(|w| unsafe { w.bits(1) });
        Ok(Self {
            _dtc: dtc,
            ielsr,
            event,
        })
    }

    /// Writes up to [`MAX_RECORDS`] transfer-information records (see
    /// [`record`](crate::step::record); every record but the last must
    /// carry [`MRB_CHAIN`](crate::step::MRB_CHAIN)) and
    /// arms the slot:
    /// the next tick event runs them once, back to back. The write to
    /// `IELSRn` sets `DTCE` and clears `IR` in one access, so a request left
    /// over from earlier ticks cannot start the transfer at once. Call with
    /// interrupts masked if the arming must not straddle a tick.
    pub(crate) fn arm(&mut self, records: &[[u32; 4]]) {
        unsafe {
            let info = addr_of_mut!(TRANSFER.0);
            for (slot, words) in records.iter().take(MAX_RECORDS).enumerate() {
                for (index, &word) in words.iter().enumerate() {
                    write_volatile(&mut (*info)[slot][index], word);
                }
            }
        }
        compiler_fence(Ordering::SeqCst);
        cortex_m::asm::dsb();
        self.route()
            .write(|w| unsafe { w.bits(self.event as u32 | IELSR_DTCE) });
    }

    /// True while a transfer is armed and has not run (`IELSRn.DTCE`).
    pub(crate) fn armed(&self) -> bool {
        self.route().read().bits() & IELSR_DTCE != 0
    }

    /// Cancels an armed transfer (`DTCE = 0`, `IR` cleared). A transfer
    /// already under way runs to its end; [`TickDtc::busy`] tells when it
    /// has.
    pub(crate) fn disarm(&mut self) {
        self.route().write(|w| unsafe { w.bits(self.event as u32) });
    }

    /// Turns the slot's CPU interrupt on or off. On, every tick raises it:
    /// an unarmed tick directly, an armed one when the transfer completes
    /// (the DTC then clears `DTCE` and hands the request to the CPU; manual
    /// Table 13.5, §17.7). No handler is registered: the slot's trampoline
    /// only acknowledges `IR`. Its use is waking the CPU from `WFI`.
    pub(crate) fn set_cpu_interrupt(&mut self, on: bool) {
        if on {
            self.ielsr.enable();
        } else {
            self.ielsr.disable();
        }
    }

    /// True while a DTC transfer is in progress (`DTCSTS.ACT`, bit 15;
    /// manual §17.2.10). Only the step generator uses the DTC, so this is
    /// the tick's chain.
    pub(crate) fn busy(&self) -> bool {
        self._dtc.dtcsts.read().bits() & DTCSTS_ACT != 0
    }

    /// Takes the tick DTC apart: waits out a transfer in progress, cancels
    /// any armed one, gives the ICU slot back ([`Ielsr::release`]) and
    /// returns the DTC token. The DTC itself stays started (`DTCST = 1`);
    /// with no slot pointing at it, nothing triggers it.
    pub(crate) fn release(mut self) -> ra4m1::DTC {
        while self.busy() {}
        self.disarm();
        self.ielsr.release();
        self._dtc
    }

    /// The slot's `IELSRn` register.
    fn route(&self) -> &'static ra4m1::icu::IELSR {
        let icu = unsafe { &*ra4m1::ICU::PTR };
        &icu.ielsr[self.ielsr.slot() as usize]
    }
}

/// Takes the DTC (with the DMAC) out of module stop: `MSTPCRA.MSTPA22 = 0`
/// (manual §10, MSTPCRA: "b22 MSTPA22 DMA Controller/Data Transfer
/// Controller").
fn release_module_stop_dtc() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    system.mstpcra.modify(|_, w| w.mstpa22()._0());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}
