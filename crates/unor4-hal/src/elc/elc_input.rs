//! The claimable ELC inputs' common trait.

use crate::elc::GptInput;

mod private {
    /// Sealed supertrait: the eight inputs are a hardware fact, not an
    /// extension point.
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

/// One of the GPT's eight ELC inputs (`ELC_GPTA`–`ELC_GPTH`) as a claimable
/// part: [`ElcA`](crate::elc::ElcA) … [`ElcH`](crate::elc::ElcH).
///
/// Each exists once, as a field of [`Peripherals`](crate::Peripherals).
/// A driver that links an event into an input claims it (the step
/// generator claims B, C and D; a wake timer the one its setup names), and
/// [`Elc::link`](crate::elc::Elc::link) takes a claimed input. So two users
/// of the same input fail to compile, like two drivers on one pin.
pub trait ElcInput: private::Sealed {
    /// The input this type stands for.
    const INPUT: GptInput;

    /// A second token for this input, out of thin air.
    ///
    /// # Safety
    /// Only for a driver being taken apart that claimed this input when it
    /// was built: at most one token per input may exist.
    #[doc(hidden)]
    unsafe fn steal() -> Self
    where
        Self: Sized;
}
