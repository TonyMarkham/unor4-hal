//! Host tests for the interrupt infrastructure: the IELSR event-table
//! integrity (HalSpec §7.3) and the dispatch registry's contract. The
//! ICU/NVIC register paths are target code — they are proven by the
//! slice's hardware run, not here.

use crate::intc::{ALL_EVENTS, Event, Ielsr, call_handler as intc_call_handler, register_handler};

#[test]
fn given_full_event_table_when_enumerated_then_codes_unique() {
    // given: the transcribed IELSR event table (OQ-2)
    // when: every event's code is collected
    let mut codes: Vec<u8> = ALL_EVENTS.iter().map(|e| *e as u8).collect();
    codes.sort_unstable();
    let count = codes.len();
    codes.dedup();

    // then: no two events share a code — a duplicate would make one of
    // them unlinkable
    assert_eq!(codes.len(), count);
}

#[test]
fn given_full_event_table_when_enumerated_then_codes_bounded() {
    // given: the transcribed IELSR event table
    // when: every code is checked
    // then: all codes fit the 8-bit IELS field (manual: IELSRn bits
    // [7:0] select the event)
    for event in ALL_EVENTS {
        assert!((*event as u32) <= 0xFF);
    }
}

#[test]
fn given_transcribed_table_when_sci_entries_checked_then_codes_match_the_sources() {
    // given: the cross-checked transcriptions (ra-hal's InterruptEvent
    // and the community ICU table — both derive from manual §13)
    // when: the codes this project actually drives are read off
    // then: they match the transcriptions exactly — SCI2 is the
    // 0xA3–0xA7 block, SCI9 the 0xA8–0xAC block, SCI1 starts at 0x9E
    assert_eq!(Event::Sci2Rxi as u8, 0xA3);
    assert_eq!(Event::Sci2Txi as u8, 0xA4);
    assert_eq!(Event::Sci2Eri as u8, 0xA6);
    assert_eq!(Event::Sci9Rxi as u8, 0xA8);
    assert_eq!(Event::Sci9Txi as u8, 0xA9);
    assert_eq!(Event::Sci9Tei as u8, 0xAA);
    assert_eq!(Event::Sci9Eri as u8, 0xAB);
    assert_eq!(Event::Sci1Rxi as u8, 0x9E);
}

#[test]
fn given_free_slot_when_taken_twice_then_only_one_claim_succeeds() {
    // given: slot 31 unclaimed (no other test claims it; the ICU is
    // never dereferenced on the host)
    // when: it is claimed, then claimed again
    let first = Ielsr::take(31);
    let second = Ielsr::take(31);

    // then: exactly one claim succeeds
    assert!(first.is_some());
    assert!(second.is_none());
}

#[test]
fn given_slot_beyond_the_table_when_taken_then_none() {
    // given: the RA4M1 has 32 IELSR slots
    // when: slot 32 (out of range) is requested
    // then: no claim is made
    assert!(Ielsr::take(32).is_none());
}

#[test]
fn given_consecutive_free_slots_when_take_any_twice_then_slots_are_consecutive() {
    // given: take_any scans lowest-first and nothing else in this test
    // binary claims through take_any
    // when: two slots are claimed back to back
    let a = Ielsr::take_any().expect("a slot is free");
    let b = Ielsr::take_any().expect("a second slot is free");

    // then: they are consecutive — the allocation is deterministic
    assert_eq!(b.slot(), a.slot() + 1);
}

static DISPATCH_CALLS: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

#[test]
fn given_registered_handler_when_called_then_context_is_passed_through() {
    // given: a handler registered for slot 30 (reserved for this test)
    // with a context pointing at the call counter
    let ctx = &DISPATCH_CALLS as *const _ as *const ();
    register_handler(30, bump_counter, ctx);

    // when: the dispatcher invokes the slot's handler
    intc_call_handler(30);

    // then: the handler ran once, with the context it was given
    assert_eq!(
        DISPATCH_CALLS.load(core::sync::atomic::Ordering::Relaxed),
        1
    );
}

#[test]
#[should_panic]
fn given_already_registered_slot_when_registered_again_then_panic() {
    // given: slot 29 registered by this test
    register_handler(29, bump_counter, core::ptr::null());

    // when: the same slot is registered again
    // then: the registry refuses — a silent overwrite would strand the
    // first driver's interrupts
    register_handler(29, bump_counter, core::ptr::null());
}

fn bump_counter(ctx: *const ()) {
    // Safety: the test registered a pointer to `DISPATCH_CALLS`.
    let counter = unsafe { &*(ctx as *const core::sync::atomic::AtomicUsize) };
    counter.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
}
