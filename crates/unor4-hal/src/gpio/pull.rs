//! Where a switch input's pull-up comes from.

/// Where a switch input's pull-up comes from. The RA4M1 has pull-ups only
/// (`PmnPFS.PCR`, manual §19.2.5), so a switch goes between its pin and
/// GND either way.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pull {
    /// The pin's internal pull-up (`PCR = 1`): 10–50 kΩ (datasheet
    /// R01DS0355 Rev 1.10, Table 2.10, "Input pull-up resistor").
    Internal,
    /// A resistor on the board, in ohms; the internal pull-up stays off.
    /// The value is checked against what the board itself puts on the pin
    /// ([`SwitchPinBuilder`](crate::gpio::SwitchPinBuilder)).
    External(u32),
}
