//! One claimed ICU/IELSR slot: the link between a peripheral event and
//! its NVIC interrupt line.

use crate::intc::{Event, dispatch};

use core::sync::atomic::{AtomicU32, Ordering};

/// Which of the 32 slots are claimed. Attach-time only — slots are
/// claimed from the main thread before the slot's NVIC line is ever
/// enabled, so there is no interrupt-side contention to guard against.
static CLAIMED: AtomicU32 = AtomicU32::new(0);

/// `IELSRn.IR` — the sticky request flag, bit 16 of the register
/// (SVD; FSP's `R_ICU_IELSR_IR_Pos` agrees).
pub(crate) const IELSR_IR_BIT: u32 = 1 << 16;

/// One claimed IELSR slot.
///
/// Most of the HAL's drivers attach once at startup and keep their
/// interrupts forever, so there is no `Drop`; a driver that can be taken
/// apart again (the step generator's `free`) gives its slot back with
/// [`Ielsr::release`]. [`Ielsr::take_any`] hands out slots
/// deterministically lowest-first, which keeps the allocation trivial to
/// reason about across drivers.
pub struct Ielsr {
    slot: u8,
}

impl Ielsr {
    /// Claims IELSR slot `slot` (0–31). Returns `None` if the number is
    /// out of range or the slot is already claimed.
    pub fn take(slot: u8) -> Option<Self> {
        if slot >= dispatch::SLOT_COUNT as u8 {
            return None;
        }
        let mask = 1u32 << slot;
        // `fetch_or` returns the previous value: if our bit was already
        // set, someone else holds the slot.
        if CLAIMED.fetch_or(mask, Ordering::AcqRel) & mask != 0 {
            return None;
        }
        Some(Self { slot })
    }

    /// Claims the lowest free slot, or `None` if all 32 are taken.
    pub fn take_any() -> Option<Self> {
        (0..dispatch::SLOT_COUNT as u8).find_map(Ielsr::take)
    }

    /// The slot number — also the NVIC IRQ number (see the parent
    /// module's docs).
    pub fn slot(&self) -> u8 {
        self.slot
    }

    /// Links `event` to this slot: writes the event code into
    /// `IELSRn.IELS` and clears the `IR` request flag in the same write
    /// (reserved bits are written as 0). The slot's NVIC line stays
    /// disabled until [`Ielsr::enable`].
    ///
    /// # Panics
    /// If another IELSR slot already selects this event. The hardware
    /// prohibits duplicate event destinations (manual §13.4.2).
    pub fn link(&mut self, event: Event) {
        critical_section::with(|_| {
            let icu = unsafe { &*ra4m1::ICU::PTR };
            // RA4M1 manual R01UH0887EJ0100 §13.4.2: a peripheral
            // event must never be selected in two IELSR registers.
            for (slot, route) in icu.ielsr.iter().enumerate() {
                assert!(
                    slot == self.slot as usize || route.read().bits() & 0xff != event as u32,
                    "event is already linked to another IELSR slot"
                );
            }
            let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
            unsafe {
                nvic.icer[0].write(1 << self.slot);
                nvic.icpr[0].write(1 << self.slot);
            }
            let route = &icu.ielsr[self.slot as usize];
            route.write(|w| unsafe { w.bits(0) });
            route.write(|w| unsafe { w.bits(event as u32) });
            // No acknowledgement left over from the slot's previous owner.
            dispatch::ACKNOWLEDGED.fetch_and(!(1u32 << self.slot), Ordering::AcqRel);
        });
    }

    /// Enables the slot's NVIC line: clears any stale pending request
    /// in the NVIC, unmasks the interrupt, and drains the pipeline
    /// (`DSB`/`ISB`, so code after this call really runs with the line
    /// live). Does not change the CPU's global interrupt mask: enabling
    /// a slot inside a critical section must not end that section.
    /// Register the handler before calling this — the
    /// registration protocol lives in the crate's dispatch registry
    /// (`intc::dispatch`, private).
    pub fn enable(&mut self) {
        let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
        let (word, bit) = ((self.slot / 32) as usize, self.slot % 32);
        // Safety: plain volatile NVIC register accesses (ARMv7-M B3.4);
        // `word` is 0 because the slot count fits one ISER word.
        unsafe {
            // RA4M1 implements four priority bits in IPR[7:4]
            // (device header __NVIC_PRIO_BITS = 4). Logical priority
            // 12 must be encoded as 0xc0, not the raw value 12.
            nvic.ipr[self.slot as usize].write(12 << 4);
            nvic.icpr[word].write(1 << bit);
            nvic.iser[word].write(1 << bit);
        }
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    /// Disables the slot's NVIC line (`ICER`) and drops any request still
    /// pending in the NVIC (`ICPR`), then drains the pipeline (`DSB`/`ISB`),
    /// so no interrupt from this slot runs after the call. The event link
    /// itself is left as it is.
    pub fn disable(&mut self) {
        let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
        let (word, bit) = ((self.slot / 32) as usize, self.slot % 32);
        // Safety: plain volatile NVIC register accesses (ARMv7-M B3.4);
        // `word` is 0 because the slot count fits one ISER word.
        unsafe {
            nvic.icer[word].write(1 << bit);
            nvic.icpr[word].write(1 << bit);
        }
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    /// Drops a request still pending in the NVIC (`ICPR`), leaving the line
    /// enabled: a driver forgetting an event its interrupt has not run for
    /// yet (the wake timer's `start` and `cancel`), so no stray wake-up
    /// follows.
    pub(crate) fn unpend(&self) {
        let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
        let (word, bit) = ((self.slot / 32) as usize, self.slot % 32);
        // Safety: a plain volatile NVIC register write (ARMv7-M B3.4);
        // `word` is 0 because the slot count fits one ISER word.
        unsafe { nvic.icpr[word].write(1 << bit) };
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    /// Gives the slot back: disables its NVIC line ([`Ielsr::disable`]),
    /// unlinks the event (`IELSRn = 0`: no event, `IR` and `DTCE` cleared)
    /// and frees the slot for [`Ielsr::take`].
    pub fn release(mut self) {
        self.disable();
        let icu = unsafe { &*ra4m1::ICU::PTR };
        icu.ielsr[self.slot as usize].write(|w| unsafe { w.bits(0) });
        dispatch::ACKNOWLEDGED.fetch_and(!(1u32 << self.slot), Ordering::AcqRel);
        CLAIMED.fetch_and(!(1u32 << self.slot), Ordering::AcqRel);
    }

    /// True if some IELSR slot selects `event` (`IELSRn.IELS`): linking it
    /// again would break manual §13.4.2's one-slot rule.
    pub(crate) fn is_linked(event: Event) -> bool {
        let icu = unsafe { &*ra4m1::ICU::PTR };
        icu.ielsr
            .iter()
            .any(|route| route.read().bits() & 0xff == event as u32)
    }

    /// Clears `IELSRn.IR` (writes the register back with bit 16 = 0).
    /// The dispatcher performs the equivalent acknowledgement at ISR
    /// entry. Drivers can also drop a latched request without running
    /// their handler through this method.
    pub fn clear_request(&self) {
        let icu = unsafe { &*ra4m1::ICU::PTR };
        let bits = icu.ielsr[self.slot as usize].read().bits() & !IELSR_IR_BIT;
        icu.ielsr[self.slot as usize].write(|w| unsafe { w.bits(bits) });
    }

    /// True if the slot's event happened since the last call, and forgets
    /// it: the request flag `IR` is set, or the dispatcher acknowledged a
    /// request since (its interrupt ran and cleared `IR`). For a driver
    /// that uses the request flag as a latch whether or not its interrupt
    /// is enabled. One event that is both still flagged and acknowledged
    /// (its interrupt ran between the two reads) counts once.
    pub(crate) fn take_request(&self) -> bool {
        let icu = unsafe { &*ra4m1::ICU::PTR };
        let flagged = icu.ielsr[self.slot as usize].read().bits() & IELSR_IR_BIT != 0;
        if flagged {
            self.clear_request();
        }
        let bit = 1u32 << self.slot;
        let acknowledged = dispatch::ACKNOWLEDGED.fetch_and(!bit, Ordering::AcqRel) & bit != 0;
        flagged || acknowledged
    }
}

const _: () = {
    // The NVIC claims/enable code above assumes 32 slots = one ISER
    // word (slot / 32 == 0). This holds for the RA4M1; if a future
    // target has more lines, the word index math must be revisited.
    assert!(dispatch::SLOT_COUNT <= 32);
};
