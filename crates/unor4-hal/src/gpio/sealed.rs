//! The seal shared by the pin identity and capability traits.

/// Supertrait of [`PinId`](crate::gpio::PinId) and every capability trait.
/// Crate-private, so firmware can bound on those traits but not implement
/// them: which pin can do what is a hardware fact from the manual, not an
/// extension point.
pub trait Sealed {}
