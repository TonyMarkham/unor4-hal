//! The GPT output capability.

use crate::{
    gpio::{GptOutput, PinId},
    pwm::GptChannel,
};

/// The pin has a GPT output (`GTIOCnA`/`GTIOCnB`). Bounded on by PWM and
/// the step generator.
#[diagnostic::on_unimplemented(
    message = "`{Self}` has no GPT output",
    label = "no GTIOCnA/GTIOCnB function on this pin",
    note = "pins with a GPT output have `gtioc` in the `pin_map` table"
)]
pub trait Gtioc: PinId {
    /// The GPT channel whose output the pin carries.
    type Timer: GptChannel;
    /// `GTIOCnA` or `GTIOCnB`.
    const OUTPUT: GptOutput;
    /// `PmnPFS.PSEL` value selecting it.
    const PSEL: u8;
}
