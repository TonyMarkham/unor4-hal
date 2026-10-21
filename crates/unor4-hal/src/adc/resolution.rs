//! The ADC resolution: 12-bit (the reset default) or the full 14 bits.

/// A/D conversion accuracy, per `ADCER.ADPRC[1:0]` (manual §35.2.11:
/// `00b` = 12-bit — the reset value, `11b` = 14-bit, `01b`/`10b`
/// prohibited). Only writable while `ADCSR.ADST = 0`, which the driver
/// guarantees — every write happens with the converter stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// 12-bit conversion — right-justified in bits 11:0. The hardware
    /// reset value, and the driver's default.
    Bits12,
    /// 14-bit conversion — right-justified in bits 13:0. The unit's
    /// native resolution (§35.1 Table 35.1).
    Bits14,
}

impl Resolution {
    /// The valid result bits below the 16-bit register — the mask a
    /// raw `ADDRy` read is ANDed with so the returned value never
    /// carries format noise.
    pub fn mask(self) -> u16 {
        match self {
            Resolution::Bits12 => 0x0FFF,
            Resolution::Bits14 => 0x3FFF,
        }
    }

    /// The largest possible reading (`mask()` by construction — the
    /// result is right-justified and fills its bits).
    pub fn max(self) -> u16 {
        self.mask()
    }

    /// The resolution in bits, for logging and scaling.
    pub fn bits(self) -> u8 {
        match self {
            Resolution::Bits12 => 12,
            Resolution::Bits14 => 14,
        }
    }
}
