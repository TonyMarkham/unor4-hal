//! Host tests for the IRQ UART's ring buffers — pure data-structure
//! behavior, no hardware. The `uart-irq` feature must be on (`cargo
//! test -p unor4-hal --features uart-irq`).

use crate::uart::Ring;

#[test]
fn given_new_ring_when_pushed_and_popped_then_fifo_order_is_kept() {
    // given: an empty ring of 16 bytes
    let ring: Ring<16> = Ring::new();

    // when: bytes 0..6 are pushed, then popped
    for b in 0u8..6 {
        assert!(ring.push(b));
    }
    let out: Vec<u8> = (0..6).filter_map(|_| ring.pop()).collect();

    // then: they come back in the order they went in
    assert_eq!(out, (0u8..6).collect::<Vec<u8>>());
}

#[test]
fn given_empty_ring_when_popped_then_none() {
    // given: a fresh ring
    let ring: Ring<8> = Ring::new();

    // when: a byte is popped
    // then: there is none — the caller sees "empty", not garbage
    assert_eq!(ring.pop(), None);
}

#[test]
fn given_full_ring_when_pushed_then_rejected_without_loss() {
    // given: a ring of 8, filled to capacity
    let ring: Ring<8> = Ring::new();
    for b in 0u8..8 {
        assert!(ring.push(b));
    }

    // when: one more byte is pushed
    // then: the push is refused — the ring is full, and draining one
    // byte makes room again (the ISR's drop-and-count case)
    assert!(!ring.push(0xAA));
    assert_eq!(ring.len(), 8);
    assert_eq!(ring.pop(), Some(0));
    assert!(ring.push(0xAA));
}

#[test]
fn given_ring_of_256_when_filled_then_len_reports_256() {
    // given: the IRQ UART's default RX size (256)
    let ring: Ring<256> = Ring::new();

    // when: 256 bytes are pushed
    for b in 0..256u16 {
        assert!(ring.push(b as u8));
    }

    // then: the full 256 are reported — the u16 wrapping-difference
    // math must represent a completely full ring, not cap out at 255
    assert_eq!(ring.len(), 256);
}

#[test]
fn given_free_running_indices_when_push_and_pop_cross_the_u16_wrap_then_order_is_kept() {
    // given: a ring whose counters have been advanced far past the u16
    // wrap point (3 x capacity + offset full round trips)
    let ring: Ring<16> = Ring::new();
    for round in 0u16..(3 * 16 + 7) {
        assert!(ring.push(round as u8));
        assert_eq!(ring.pop(), Some(round as u8));
    }

    // when: one more round trip happens
    assert!(ring.push(0x5A));

    // then: the byte survives the wrap with its value intact
    assert_eq!(ring.pop(), Some(0x5A));
}
