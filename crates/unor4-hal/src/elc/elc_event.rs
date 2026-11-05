//! The ELC event numbers this crate routes (manual Table 18.3).

use crate::gpio::GptOutput;

/// One event an `ELSRn` destination can select (manual §18.2.3, Table
/// 18.3). Only the events this crate routes are listed: the two software
/// events and, for every GPT channel n, compare match A and B and overflow
/// (`GPTn_CCMPA` = `057h + 8n`, `GPTn_CCMPB` = `058h + 8n`, `GPTn_OVF` =
/// `05Dh + 8n`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElcEvent {
    /// `ELC_SWEVT0` — software event 0, raised through `ELSEGR0`.
    SoftwareEvent0 = 0x53,
    /// `ELC_SWEVT1` — software event 1, raised through `ELSEGR1`.
    SoftwareEvent1 = 0x54,
    /// `GPT0_CCMPA` — GPT320 `GTCCRA` input capture / compare match A.
    Gpt0CompareA = 0x57,
    /// `GPT0_CCMPB` — GPT320 `GTCCRB` input capture / compare match B.
    Gpt0CompareB = 0x58,
    /// `GPT0_OVF` — GPT320 counter overflow (`GTPR` compare match).
    Gpt0Overflow = 0x5D,
    /// `GPT1_CCMPA` — GPT321 compare match A.
    Gpt1CompareA = 0x5F,
    /// `GPT1_CCMPB` — GPT321 compare match B.
    Gpt1CompareB = 0x60,
    /// `GPT1_OVF` — GPT321 counter overflow.
    Gpt1Overflow = 0x65,
    /// `GPT2_CCMPA` — GPT162 compare match A.
    Gpt2CompareA = 0x67,
    /// `GPT2_CCMPB` — GPT162 compare match B.
    Gpt2CompareB = 0x68,
    /// `GPT2_OVF` — GPT162 counter overflow.
    Gpt2Overflow = 0x6D,
    /// `GPT3_CCMPA` — GPT163 compare match A.
    Gpt3CompareA = 0x6F,
    /// `GPT3_CCMPB` — GPT163 compare match B.
    Gpt3CompareB = 0x70,
    /// `GPT3_OVF` — GPT163 counter overflow.
    Gpt3Overflow = 0x75,
    /// `GPT4_CCMPA` — GPT164 compare match A.
    Gpt4CompareA = 0x77,
    /// `GPT4_CCMPB` — GPT164 compare match B.
    Gpt4CompareB = 0x78,
    /// `GPT4_OVF` — GPT164 counter overflow (`GTPR` compare match).
    Gpt4Overflow = 0x7D,
    /// `GPT5_CCMPA` — GPT165 compare match A.
    Gpt5CompareA = 0x7F,
    /// `GPT5_CCMPB` — GPT165 compare match B.
    Gpt5CompareB = 0x80,
    /// `GPT5_OVF` — GPT165 counter overflow.
    Gpt5Overflow = 0x85,
    /// `GPT6_CCMPA` — GPT166 compare match A.
    Gpt6CompareA = 0x87,
    /// `GPT6_CCMPB` — GPT166 compare match B.
    Gpt6CompareB = 0x88,
    /// `GPT6_OVF` — GPT166 counter overflow.
    Gpt6Overflow = 0x8D,
    /// `GPT7_CCMPA` — GPT167 compare match A.
    Gpt7CompareA = 0x8F,
    /// `GPT7_CCMPB` — GPT167 compare match B.
    Gpt7CompareB = 0x90,
    /// `GPT7_OVF` — GPT167 counter overflow.
    Gpt7Overflow = 0x95,
}

impl ElcEvent {
    /// The `ELS[7:0]` value for this event.
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// GPT channel `channel`'s compare match on `output` (`GPTn_CCMPA` /
    /// `GPTn_CCMPB`); `None` for a channel above 7.
    pub const fn gpt_compare(channel: u8, output: GptOutput) -> Option<Self> {
        Some(match (channel, output) {
            (0, GptOutput::A) => Self::Gpt0CompareA,
            (0, GptOutput::B) => Self::Gpt0CompareB,
            (1, GptOutput::A) => Self::Gpt1CompareA,
            (1, GptOutput::B) => Self::Gpt1CompareB,
            (2, GptOutput::A) => Self::Gpt2CompareA,
            (2, GptOutput::B) => Self::Gpt2CompareB,
            (3, GptOutput::A) => Self::Gpt3CompareA,
            (3, GptOutput::B) => Self::Gpt3CompareB,
            (4, GptOutput::A) => Self::Gpt4CompareA,
            (4, GptOutput::B) => Self::Gpt4CompareB,
            (5, GptOutput::A) => Self::Gpt5CompareA,
            (5, GptOutput::B) => Self::Gpt5CompareB,
            (6, GptOutput::A) => Self::Gpt6CompareA,
            (6, GptOutput::B) => Self::Gpt6CompareB,
            (7, GptOutput::A) => Self::Gpt7CompareA,
            (7, GptOutput::B) => Self::Gpt7CompareB,
            _ => return None,
        })
    }

    /// GPT channel `channel`'s overflow (`GPTn_OVF`); `None` for a channel
    /// above 7.
    pub const fn gpt_overflow(channel: u8) -> Option<Self> {
        Some(match channel {
            0 => Self::Gpt0Overflow,
            1 => Self::Gpt1Overflow,
            2 => Self::Gpt2Overflow,
            3 => Self::Gpt3Overflow,
            4 => Self::Gpt4Overflow,
            5 => Self::Gpt5Overflow,
            6 => Self::Gpt6Overflow,
            7 => Self::Gpt7Overflow,
            _ => return None,
        })
    }
}

/// Every [`ElcEvent`], for the table-integrity tests (`src/tests/elc.rs`).
/// Kept next to the enum so a new variant is added here in the same edit.
#[cfg(test)]
pub(crate) const ALL_ELC_EVENTS: &[ElcEvent] = &[
    ElcEvent::SoftwareEvent0,
    ElcEvent::SoftwareEvent1,
    ElcEvent::Gpt0CompareA,
    ElcEvent::Gpt0CompareB,
    ElcEvent::Gpt0Overflow,
    ElcEvent::Gpt1CompareA,
    ElcEvent::Gpt1CompareB,
    ElcEvent::Gpt1Overflow,
    ElcEvent::Gpt2CompareA,
    ElcEvent::Gpt2CompareB,
    ElcEvent::Gpt2Overflow,
    ElcEvent::Gpt3CompareA,
    ElcEvent::Gpt3CompareB,
    ElcEvent::Gpt3Overflow,
    ElcEvent::Gpt4CompareA,
    ElcEvent::Gpt4CompareB,
    ElcEvent::Gpt4Overflow,
    ElcEvent::Gpt5CompareA,
    ElcEvent::Gpt5CompareB,
    ElcEvent::Gpt5Overflow,
    ElcEvent::Gpt6CompareA,
    ElcEvent::Gpt6CompareB,
    ElcEvent::Gpt6Overflow,
    ElcEvent::Gpt7CompareA,
    ElcEvent::Gpt7CompareB,
    ElcEvent::Gpt7Overflow,
];
