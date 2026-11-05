//! A checked tick setup.

use core::marker::PhantomData;

/// A checked tick setup: GPT channel `T` makes the 2 ms tick, `N` numbers
/// the ticks. Made by [`TickBuilder::build`](crate::step::TickBuilder::build)
/// in a named `const`.
#[derive(Clone, Copy, Debug)]
pub struct TickSetup<T, N> {
    pub(crate) _timers: PhantomData<(T, N)>,
}
