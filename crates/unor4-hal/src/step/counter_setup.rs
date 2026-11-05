//! A checked pulse-counter setup.

use core::marker::PhantomData;

/// A checked pulse-counter setup on GPT channel `C`, made by
/// [`CounterBuilder::build`](crate::step::CounterBuilder::build) in a named
/// `const`.
#[derive(Clone, Copy, Debug)]
pub struct CounterSetup<C> {
    pub(crate) _timer: PhantomData<C>,
}
