//! The period calculator: smallest prescaler that fits, then round.

use crate::pwm::{PeriodError, PeriodParams, PeriodResult};

/// The `GTCR.TPCS` prescaler ladder (manual §22.2.12): index = the TPCS
/// field code, value = the divisor it selects.
const DIVISORS: [u32; 6] = [1, 4, 16, 64, 256, 1024];

/// Largest period in ticks. `u16` duty space for `SetDutyCycle`
/// (its `max_duty_cycle` returns `u16`), and automatically inside
/// GPT16's 16-bit `GTPR`.
const MAX_PERIOD_COUNTS: u64 = 65_535;

/// Tolerance for the rounding error, in thousandths: a request whose
/// only deliverable frequency is more than 5% off is rejected
/// ([`PeriodError`](crate::pwm::PeriodError)) rather than silently
/// misdelivered. The same spirit as the SCI baud calculator's 3% check,
/// looser because GPT's discrete prescaler ladder makes exact matches
/// common everywhere below the top of the range.
const MAX_ERROR_X1000: u64 = 50;

/// Resolves a requested PWM frequency into `GTCR.TPCS` + `GTPR`.
///
/// Picks the **smallest divisor** whose rounded tick count fits the
/// period cap — the finest duty resolution available for the request
/// (the same shape as the SCI baud calculator's smallest-CKS search).
/// All arithmetic is `u64`: the widest intermediate,
/// `divisor × freq_hz`, reaches `1024 × 48 MHz ≈ 4.9 × 10¹⁰` and
/// overflows `u32`.
pub fn calc_period(pclkd_hz: u32, freq_hz: u32) -> PeriodResult<PeriodParams> {
    if freq_hz == 0 {
        return Err(PeriodError::TooLow);
    }
    if freq_hz > pclkd_hz {
        return Err(PeriodError::TooHigh);
    }
    for (tpcs, &divisor) in DIVISORS.iter().enumerate() {
        // Rounded ticks per period: round(pclkd / (divisor × freq)),
        // half-up.
        let denom = divisor as u64 * freq_hz as u64;
        let counts = (pclkd_hz as u64 + denom / 2) / denom;
        if counts == 0 || counts > MAX_PERIOD_COUNTS {
            continue; // this divisor cannot express the period; try the next
        }
        // Delivered frequency = pclkd / (divisor × counts). Its error
        // against the request is |counts × denom − pclkd| / pclkd.
        let delivered = counts * denom;
        let error_x1000 = delivered.abs_diff(pclkd_hz as u64) * 1000 / pclkd_hz as u64;
        if error_x1000 > MAX_ERROR_X1000 {
            // counts·denom > pclkd ⇒ delivered slower than requested.
            return Err(if delivered > pclkd_hz as u64 {
                PeriodError::TooHigh
            } else {
                PeriodError::TooLow
            });
        }
        return Ok(PeriodParams {
            tpcs: tpcs as u8,
            divisor,
            gpr: (counts - 1) as u32,
        });
    }
    Err(PeriodError::TooLow)
}
