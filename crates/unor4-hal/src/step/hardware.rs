//! The hardware a built step generator drives, resolved from its setup's
//! types.

use crate::{elc::ElcEvent, gpio::GptOutput, intc::Event};
use embedded_hal::digital::PinState;

/// One GPT channel's registers, in the shared `gpt320` layout (see
/// [`GptChannel`](crate::pwm::GptChannel)).
pub(crate) type GptRegs = ra4m1::gpt320::RegisterBlock;

/// Everything [`StepGeneratorBuilder::build`](crate::step::StepGeneratorBuilder::build)
/// resolves from the setup's types before handing over to the generator.
pub(crate) struct Hardware {
    /// The STEP channel.
    pub(crate) step: &'static GptRegs,
    /// The STEP channel's width, bits.
    pub(crate) step_bits: u8,
    /// The STEP channel's `GTCR.TPCS`.
    pub(crate) step_tpcs: u8,
    /// The STEP channel's PCLKD divisor.
    pub(crate) step_divider: u32,
    /// The STEP pin's output: A or B.
    pub(crate) output: GptOutput,
    /// The STEP channel's compare match on that output: each pulse's end.
    pub(crate) pulse_end: ElcEvent,
    /// The pulse-count channel.
    pub(crate) counter: &'static GptRegs,
    /// The counter's wrap mask: `0xFFFF` (16-bit) or `u32::MAX` (32-bit).
    pub(crate) counter_mask: u32,
    /// The counter's compare match B as an ELC event: an armed count stop
    /// ([`StepGenerator::stop_at`](crate::step::StepGenerator::stop_at)).
    pub(crate) count_match: ElcEvent,
    /// The same compare match as an ICU event: the count stop's wake-up.
    pub(crate) count_event: Event,
    /// The tick channel.
    pub(crate) tick: &'static GptRegs,
    /// The tick channel's overflow as an ELC event.
    pub(crate) tick_elc: ElcEvent,
    /// The tick-number channel.
    pub(crate) ticker: &'static GptRegs,
    /// The DIR pin's port digit.
    pub(crate) dir_port: char,
    /// The DIR pin's index within its port.
    pub(crate) dir_pin: u8,
    /// Pulse LOW, STEP-timer counts.
    pub(crate) low_counts: u32,
    /// Minimum pulse HIGH, STEP-timer counts.
    pub(crate) min_high_counts: u32,
    /// DIR set-up, PCLKD counts.
    pub(crate) dir_setup_counts: u32,
    /// The DIR level for positive rates.
    pub(crate) forward: PinState,
    /// The ENA pin's port digit and index, `None` without ENA.
    pub(crate) ena_pin: Option<(char, u8)>,
    /// The level that enables the driver.
    pub(crate) ena_enabled: PinState,
    /// The lead from enable to the first DIR change or pulse, ns.
    pub(crate) ena_lead_ns: u32,
}
