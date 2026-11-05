//! The GPT's eight ELC inputs (manual Table 18.2).

/// One of the GPT's ELC inputs `ELC_GPTA`–`ELC_GPTH`, set by `ELSR0`–
/// `ELSR7` (manual Table 18.2). All eight GPT channels see all eight
/// inputs; each channel picks the ones it acts on (§22.2.5–§22.2.11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GptInput {
    /// `ELC_GPTA`, set by `ELSR0`.
    A = 0,
    /// `ELC_GPTB`, set by `ELSR1`.
    B = 1,
    /// `ELC_GPTC`, set by `ELSR2`.
    C = 2,
    /// `ELC_GPTD`, set by `ELSR3`.
    D = 3,
    /// `ELC_GPTE`, set by `ELSR4`.
    E = 4,
    /// `ELC_GPTF`, set by `ELSR5`.
    F = 5,
    /// `ELC_GPTG`, set by `ELSR6`.
    G = 6,
    /// `ELC_GPTH`, set by `ELSR7`.
    H = 7,
}

impl GptInput {
    /// The `ELSRn` index, which is also the bit position of this input in
    /// the GPT select registers' ELC fields (`USELCA` = bit 16 + 0, …).
    pub const fn index(self) -> u8 {
        self as u8
    }
}

/// Every [`GptInput`], for the table-integrity tests (`src/tests/elc.rs`).
#[cfg(test)]
pub(crate) const ALL_GPT_INPUTS: &[GptInput] = &[
    GptInput::A,
    GptInput::B,
    GptInput::C,
    GptInput::D,
    GptInput::E,
    GptInput::F,
    GptInput::G,
    GptInput::H,
];
