//! The resolved period: prescaler code plus the `GTPR` register value.

/// A resolved GPT saw-wave period, as [`calc_period`](super::calc_period)
/// computed it from the count clock and the requested frequency.
///
/// Pure arithmetic facts — re-derivable from manual §22.2.12 (TPCS
/// ladder) and §22.2.20 (saw period = `GTPR + 1`) — so the calculator is
/// host-unit-tested (`src/tests/pwm.rs`) the way the SCI baud
/// calculator is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeriodParams {
    /// `GTCR.TPCS[2:0]` field code, 0–5: PCLKD/1, /4, /16, /64, /256,
    /// /1024 (manual §22.2.12).
    pub tpcs: u8,
    /// The divisor `tpcs` selects — carried alongside the code so
    /// callers never re-derive it.
    pub divisor: u32,
    /// `GTPR` register value; the saw period is `gpr + 1` count ticks
    /// (manual §22.2.20). Capped so `gpr + 1 ≤ 65 535` — `u16` duty
    /// space for [`embedded_hal::pwm::SetDutyCycle::max_duty_cycle`],
    /// and automatically inside GPT16's 16-bit register.
    pub gpr: u32,
}

impl PeriodParams {
    /// Counter ticks per PWM period (`GTPR + 1`); equals
    /// [`Pwm::max_duty_cycle`](super::Pwm::max_duty_cycle).
    pub fn period_counts(&self) -> u32 {
        self.gpr + 1
    }

    /// The frequency actually delivered, in hundredths of a hertz —
    /// `pclkd / (divisor × period_counts)` at the resolution the
    /// rounding lands on.
    pub fn actual_freq_hz_x100(&self, pclkd_hz: u32) -> u64 {
        let denominator = self.divisor as u64 * self.period_counts() as u64;
        pclkd_hz as u64 * 100 / denominator
    }
}
