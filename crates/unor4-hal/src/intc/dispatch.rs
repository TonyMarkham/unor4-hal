//! The dispatch table behind the vector table: 32 IELSR slots, each
//! holding one registered handler, plus the 32-entry `__INTERRUPTS`
//! static that cortex-m-rt's `device` feature expects (see the parent
//! module's docs).
//!
//! Registration protocol (what makes the raw `UnsafeCell` sound):
//! a slot's entry is written from the main thread **before** that
//! slot's NVIC line is enabled and never cleared afterwards. An
//! interrupt on slot `n` can therefore only observe entry `n` after the
//! completed write that installed it, and no two writers ever target
//! the same slot. Nothing else is needed on a single-core MCU.

use core::cell::UnsafeCell;

/// Number of IELSR slots on the RA4M1 — also the NVIC IRQ count (SVD
/// `__INTERRUPTS`; the PAC's `rt` vector table has the same 32 entries).
pub(crate) const SLOT_COUNT: usize = 32;

/// A registered interrupt service routine: a monomorphized thunk plus
/// the driver instance it serves. The thunk downcasts the context back
/// to its concrete type — the registration site guarantees the cast.
pub(crate) type RawIsr = fn(*const ());

/// One registry slot: handler + context, or nothing.
type Entry = Option<(RawIsr, *const ())>;

struct Registry {
    slots: UnsafeCell<[Entry; SLOT_COUNT]>,
}

// Sound per the registration protocol above: interleaved access is
// cross-thread only between a not-yet-enabled slot's writer and that
// same slot's interrupt — which cannot run yet.
unsafe impl Sync for Registry {}

static HANDLERS: Registry = Registry {
    slots: UnsafeCell::new([None; SLOT_COUNT]),
};

/// Registers the handler for `slot`. Must be called before
/// [`crate::intc::Ielsr::enable`] for that slot; registering twice for
/// the same slot is a programming error and panics.
#[cfg_attr(not(feature = "uart-irq"), allow(dead_code))]
pub(crate) fn register_handler(slot: u8, isr: RawIsr, ctx: *const ()) {
    assert!(
        (slot as usize) < SLOT_COUNT,
        "IELSR slot number out of range"
    );
    // Safety: protocol above — this is the one write, before enable.
    let entry = unsafe { &mut (*HANDLERS.slots.get())[slot as usize] };
    assert!(entry.is_none(), "IELSR slot {slot} already registered");
    *entry = Some((isr, ctx));
}

/// Runs the handler registered for `slot`, if any. Host-testable —
/// no hardware access.
pub(crate) fn call_handler(slot: usize) {
    assert!(slot < SLOT_COUNT, "IELSR slot number out of range");
    // Safety: protocol above.
    let entry = unsafe { (*HANDLERS.slots.get())[slot] };
    if let Some((isr, ctx)) = entry {
        isr(ctx);
    }
}

/// Clears the event latch, then services the registered handler.
/// The SCI RXI/TXI handlers use this order, cross-checked against
/// FSP's non-FIFO SCI interrupt flow. The handler services the SCI
/// source (RDR/TDR/error flags); this clear only acknowledges the ICU.
fn dispatch(slot: usize) {
    clear_request_bit(slot);
    call_handler(slot);
}

/// Acknowledge IR with the halfword/word access required by RA4M1
/// manual R01UH0887EJ0100 §13.2.6. Preserve the event selection.
fn clear_request_bit(slot: usize) {
    let icu = unsafe { &*ra4m1::ICU::PTR };
    let route = &icu.ielsr[slot];
    let bits = route.read().bits() & !(1 << 16);
    route.write(|w| unsafe { w.bits(bits) });
}

// The 32 trampolines: each vector is one slot's dispatcher. Generated
// by macro — 32 identical one-liners with only the slot number varying.

macro_rules! iel_vectors {
    ($(($name:ident, $slot:literal)),* $(,)?) => {
        $(
            // Vector entry for IELSR slot `$slot`. Private: only
            // referenced by `__INTERRUPTS` below.
            unsafe extern "C" fn $name() {
                dispatch($slot);
            }
        )*

        /// The RA4M1's device-interrupt vector table: 32 entries, entry
        /// `n` at NVIC IRQ `n` = IELSR slot `n`. Replaces cortex-m-rt's
        /// generic 240-entry `DefaultHandler` table (requires building
        /// cortex-m-rt with its `device` feature — see the parent
        /// module's docs).
        #[unsafe(link_section = ".vector_table.interrupts")]
        #[unsafe(no_mangle)]
        pub static __INTERRUPTS: [unsafe extern "C" fn(); SLOT_COUNT] = [
            $($name),*
        ];
    };
}

iel_vectors!(
    (iel0, 0),
    (iel1, 1),
    (iel2, 2),
    (iel3, 3),
    (iel4, 4),
    (iel5, 5),
    (iel6, 6),
    (iel7, 7),
    (iel8, 8),
    (iel9, 9),
    (iel10, 10),
    (iel11, 11),
    (iel12, 12),
    (iel13, 13),
    (iel14, 14),
    (iel15, 15),
    (iel16, 16),
    (iel17, 17),
    (iel18, 18),
    (iel19, 19),
    (iel20, 20),
    (iel21, 21),
    (iel22, 22),
    (iel23, 23),
    (iel24, 24),
    (iel25, 25),
    (iel26, 26),
    (iel27, 27),
    (iel28, 28),
    (iel29, 29),
    (iel30, 30),
    (iel31, 31),
);

/// Returns the linked vector table. Its purpose is keeping the vector
/// table and trampolines in the binary even for firmware that enables
/// no interrupts: `Peripherals::take` calls this, and every slice
/// binary calls `Peripherals::take`. Without it, a firmware that never
/// mentions `intc` would not pull the table's object file into the link
/// and would fail with an undefined `__INTERRUPTS`.
#[inline(never)]
pub fn vector_table() -> &'static [unsafe extern "C" fn(); SLOT_COUNT] {
    &__INTERRUPTS
}
