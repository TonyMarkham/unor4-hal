//! The set builder slot.

use crate::gpio::Sealed;

/// A builder slot holding `T`.
#[derive(Clone, Copy, Debug)]
pub struct Set<T>(pub(crate) T);

impl<T> Sealed for Set<T> {}
