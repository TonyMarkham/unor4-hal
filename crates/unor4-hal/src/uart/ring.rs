//! The IRQ UART's ring buffers: one producer, one consumer, per ring.
//!
//! The receive ring's producer is the RXI interrupt, its consumer the
//! main thread; the transmit ring is the other way around. All callers
//! take a critical section around ring operations (the `critical-
//! section` provider masks interrupts), so the interior mutability here
//! is single-threaded by construction — the `&self` signatures exist so
//! the driver's shared handle can enqueue/dequeue without aliasing
//! problems, not because the type is lock-free.
//!
//! Indices are free-running `u16`s: the occupied length is the wrapping
//! difference `tail - head`, which is exact as long as it never exceeds
//! `N`, and both counters may wrap `u16` independently. That holds for
//! any `N <= 32_768` (asserted in [`Ring::new`]); buffer slots are
//! addressed with `counter & (N - 1)`, hence the power-of-two size.

use core::cell::{Cell, UnsafeCell};

pub(crate) struct Ring<const N: usize> {
    buf: UnsafeCell<[u8; N]>,
    /// Consumer index — only ever advanced by [`Ring::pop`].
    head: Cell<u16>,
    /// Producer index — only ever advanced by [`Ring::push`].
    tail: Cell<u16>,
}

impl<const N: usize> Ring<N> {
    pub(crate) const fn new() -> Self {
        assert!(N.is_power_of_two(), "ring size must be a power of two");
        assert!(N <= 32_768, "ring size must fit the u16 index math");
        Self {
            buf: UnsafeCell::new([0; N]),
            head: Cell::new(0),
            tail: Cell::new(0),
        }
    }

    /// Appends one byte. Returns `false` (and drops nothing) when the
    /// ring is full — the caller decides what to do with the byte.
    pub(crate) fn push(&self, byte: u8) -> bool {
        let tail = self.tail.get();
        if self.len() == N {
            return false;
        }
        // Safety: interior mutability, single-threaded by the critical
        // section discipline documented at the module level.
        unsafe { (*self.buf.get())[(tail as usize) & (N - 1)] = byte };
        self.tail.set(tail.wrapping_add(1));
        true
    }

    /// Removes and returns the oldest byte, or `None` when empty.
    pub(crate) fn pop(&self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let head = self.head.get();
        // Safety: as in `push`.
        let byte = unsafe { (*self.buf.get())[(head as usize) & (N - 1)] };
        self.head.set(head.wrapping_add(1));
        Some(byte)
    }

    /// Occupied length in bytes.
    pub(crate) fn len(&self) -> usize {
        self.tail.get().wrapping_sub(self.head.get()) as usize
    }

    /// Whether no byte is queued.
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
