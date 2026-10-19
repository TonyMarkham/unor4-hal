//! The resolved baud-rate setting.

/// One resolved baud-rate setting: everything the driver writes to
/// `SMR.CKS`, `SEMR` and `BRR`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaudParams {
    /// `SMR.CKS`: base-clock divider exponent (PCLKA / 4^cks).
    pub cks: u8,
    /// `SEMR.BGDM`: baud-rate generator double-speed mode.
    pub bgdm: bool,
    /// `SEMR.ABCS`: base clock cycles per bit (false = 16, true = 8).
    pub abcs: bool,
    /// `BRR`: the N of the bit-rate equation.
    pub brr: u8,
}

impl BaudParams {
    /// The equation's coefficient for this setting
    /// (`B = clock / (coefficient × (BRR + 1))`).
    pub fn coefficient(&self) -> u32 {
        let base: u32 = match (self.bgdm, self.abcs) {
            (true, true) => 8,
            (true, false) => 16,
            (false, false) => 32,
            // The calculator never produces BGDM = 0 with ABCS = 1 (the
            // manual's table pairs ABCS = 1 with BGDM = 1); defensive only.
            (false, true) => 16,
        };
        base << (2 * self.cks)
    }

    /// Bit rate this setting achieves from a given base clock
    /// (PCLKA), truncated to whole Hz.
    pub fn achieved_baud(&self, clock_hz: u32) -> u32 {
        clock_hz / (self.coefficient() * (u32::from(self.brr) + 1))
    }
}
