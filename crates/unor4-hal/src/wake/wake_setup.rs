//! A checked wake-timer setup.

use core::marker::PhantomData;

/// A checked wake-timer setup: GPT channel `T`, ELC input `I`. Made by
/// [`WakeTimerBuilder::build`](crate::wake::WakeTimerBuilder::build) in a
/// named `const`.
#[derive(Clone, Copy, Debug)]
pub struct WakeSetup<T, I> {
    pub(crate) _timer: PhantomData<T>,
    pub(crate) _input: PhantomData<I>,
}
