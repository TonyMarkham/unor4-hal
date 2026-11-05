//! Taking one piece of hardware out of the peripherals.

/// `Self` (the [`Peripherals`](crate::Peripherals)) still holds `R`, a
/// header pin type (`pin_map::D4`) or a peripheral (`ra4m1::GPT320`,
/// `ra4m1::DTC`), and can give it up: [`Claim::claim`] returns the part and
/// the peripherals with `R` marked [`Taken`](crate::peripherals::Taken) in
/// their type. Drivers built with `with_peripherals` claim what their setup
/// names; a part claimed twice is a compile error naming it.
#[diagnostic::on_unimplemented(
    message = "`{R}` is already used by another driver",
    label = "`{R}` was claimed before: a pin or peripheral has one owner"
)]
pub trait Claim<R>: Sized {
    /// The part: `Pin<R, Input>` for a header, the PAC token otherwise.
    type Part;
    /// The peripherals without it.
    type Rest;

    /// Takes the part out.
    fn claim(self) -> (Self::Part, Self::Rest);
}
