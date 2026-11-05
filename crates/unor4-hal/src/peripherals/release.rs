//! Putting one piece of hardware back into the peripherals.

/// The inverse of [`Claim`](crate::peripherals::Claim): `Self` has `R`
/// marked [`Taken`](crate::peripherals::Taken), and [`Release::release`]
/// puts the part back, so another driver can claim it. A driver's `free`
/// releases everything it claimed.
pub trait Release<R> {
    /// The part: `Pin<R, Input>` for a header, the PAC token otherwise.
    type Part;
    /// The peripherals with it back.
    type Restored;

    /// Puts the part back.
    fn release(self, part: Self::Part) -> Self::Restored;
}
