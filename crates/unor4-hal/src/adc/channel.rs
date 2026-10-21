//! The header analog pins: which ADC140 channel each one muxes to.

use crate::gpio::{Analog, Pin};

mod private {
    /// Sealed supertrait: nothing outside this module can implement it,
    /// so the set of [`Channel`] types is closed — the header's analog
    /// wiring is a hardware fact, not an extension point.
    pub trait Sealed {}
}

/// Marks an analog-state pin that reaches the ADC140.
///
/// The six header analog inputs are the same chip pins on both board
/// maps (the A0–A5 rows of `pins.csv` are identical), so this trait
/// carries no feature gate — only the pin docs do.
///
/// Sources: manual §19 Tables 19.5/19.6 (PORT0/PORT1 `ASEL` rows:
/// P000 = AN000, P001 = AN001, P002 = AN002, P014 = AN009/DA0,
/// P100 = AN022/CMPIN0, P101 = AN021/CMPREF0); datasheet
/// (R01DS0355EJ0110) pin-function tables, same rows. Cross-checked
/// entry-for-entry against the Arduino core's
/// `variants/UNOWIFIR4/pinmux.inc` (`CHANNEL_0/1/2/9/21/22`).
pub trait Channel: private::Sealed {
    /// The AN channel number this pin converts (0–25, no 15 — §35.1
    /// Table 35.1). It indexes `ADANSA0` (channels ≤ 14) or `ADANSA1`
    /// (channels ≥ 16) and the matching `ADDRy` result register.
    const AN: u8;
}

impl private::Sealed for Pin<'0', 14, Analog> {}
impl private::Sealed for Pin<'0', 0, Analog> {}
impl private::Sealed for Pin<'0', 1, Analog> {}
impl private::Sealed for Pin<'0', 2, Analog> {}
impl private::Sealed for Pin<'1', 1, Analog> {}
impl private::Sealed for Pin<'1', 0, Analog> {}

impl Channel for Pin<'0', 14, Analog> {
    /// A0 — WiFi = P014, Minima = P014. Also DA0 (the DAC output pin;
    /// its input function is a normal ADC channel).
    const AN: u8 = 9;
}

impl Channel for Pin<'0', 0, Analog> {
    /// A1 — WiFi = P000, Minima = P000.
    const AN: u8 = 0;
}

impl Channel for Pin<'0', 1, Analog> {
    /// A2 — WiFi = P001, Minima = P001.
    const AN: u8 = 1;
}

impl Channel for Pin<'0', 2, Analog> {
    /// A3 — WiFi = P002, Minima = P002.
    const AN: u8 = 2;
}

impl Channel for Pin<'1', 1, Analog> {
    /// A4/SDA — WiFi = P101, Minima = P101.
    const AN: u8 = 21;
}

impl Channel for Pin<'1', 0, Analog> {
    /// A5/SCL — WiFi = P100, Minima = P100.
    const AN: u8 = 22;
}
