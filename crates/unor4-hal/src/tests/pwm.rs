//! Host tests for the GPT period calculator — arithmetic facts,
//! re-derivable from manual §22.2.12 (TPCS ladder) and §22.2.20 (saw
//! period = GTPR + 1), the same way the SCI baud pairs are.

use crate::{
    gpio::{
        GptOutput, Gtioc,
        pin_map::{D3, D5, D6, D9, D10, D11},
    },
    pwm::{PeriodError, calc_period},
};

use core::any::TypeId;

const PCLKD_48MHZ: u32 = 48_000_000;

#[test]
fn given_pclkd_48mhz_when_1khz_requested_then_divisor_1_and_exact_48000_counts() {
    // given: the Slice-1 clock tree (PCLKD = 48 MHz)
    // when: the slice 7 breathing frequency is requested
    let params = calc_period(PCLKD_48MHZ, 1_000).expect("1 kHz is reachable");
    // then: the smallest divisor (PCLKD/1, TPCS = 0) expresses the
    // period exactly: GTPR = 47999, 48000 ticks — an exact 1 kHz with
    // 48 000 duty steps, and the staircase duties (max/4, max/2,
    // 3·max/4) all land on whole ticks.
    assert_eq!(params.tpcs, 0);
    assert_eq!(params.divisor, 1);
    assert_eq!(params.gpr, 47_999);
    assert_eq!(params.period_counts(), 48_000);
    assert_eq!(params.actual_freq_hz_x100(PCLKD_48MHZ), 100_000);
}

#[test]
fn given_pclkd_48mhz_when_490hz_requested_then_divisor_4_with_24490_counts() {
    // given: PCLKD = 48 MHz
    // when: Arduino's `STANDARD_PWM_FREQ_HZ` (490.0) is requested
    let params = calc_period(PCLKD_48MHZ, 490).expect("490 Hz is reachable");
    // then: /1 would need 97 959 ticks — over the 16-bit cap — so the
    // search escalates to PCLKD/4 and rounds to 24 490 ticks
    // (489.9996 Hz delivered, 4 ppm slow, well inside the 5% tolerance).
    assert_eq!(params.tpcs, 1);
    assert_eq!(params.divisor, 4);
    assert_eq!(params.gpr, 24_489);
    assert_eq!(params.actual_freq_hz_x100(PCLKD_48MHZ), 48_999);
}

#[test]
fn given_pclkd_48mhz_when_10khz_requested_then_divisor_1_and_exact_4800_counts() {
    // given: PCLKD = 48 MHz
    // when: 10 kHz is requested (the "1 kHz+" headroom check)
    let params = calc_period(PCLKD_48MHZ, 10_000).expect("10 kHz is reachable");
    // then: exact at the finest prescaler: 4 800 duty steps.
    assert_eq!(params.tpcs, 0);
    assert_eq!(params.gpr, 4_799);
    assert_eq!(params.actual_freq_hz_x100(PCLKD_48MHZ), 1_000_000);
}

#[test]
fn given_pclkd_48mhz_when_100hz_requested_then_divisor_16_with_30000_counts() {
    // given: PCLKD = 48 MHz
    // when: 100 Hz is requested (divisors /1 and /4 both overflow the
    // 65 535-tick cap)
    let params = calc_period(PCLKD_48MHZ, 100).expect("100 Hz is reachable");
    // then: the search escalates past /4 (120 000 ticks) to PCLKD/16:
    // 30 000 ticks, exact.
    assert_eq!(params.tpcs, 2);
    assert_eq!(params.divisor, 16);
    assert_eq!(params.gpr, 29_999);
}

#[test]
fn given_pclkd_48mhz_when_0hz_requested_then_err_too_low() {
    // given: PCLKD = 48 MHz
    // when: 0 Hz is requested
    // then: refused — a period cannot be infinite.
    assert_eq!(calc_period(PCLKD_48MHZ, 0), Err(PeriodError::TooLow));
}

#[test]
fn given_pclkd_48mhz_when_above_pclkd_requested_then_err_too_high() {
    // given: PCLKD = 48 MHz
    // when: 49 MHz is requested (above the count clock)
    // then: refused — the period would be under one tick.
    assert_eq!(
        calc_period(PCLKD_48MHZ, 49_000_000),
        Err(PeriodError::TooHigh)
    );
}

#[test]
fn given_pclkd_48mhz_when_32mhz_requested_then_err_too_high() {
    // given: PCLKD = 48 MHz
    // when: 32 MHz is requested — the only rounding (2 ticks = 24 MHz
    // delivered) lands 25% off, past the 5% tolerance
    // then: refused as too high rather than silently misdelivered.
    assert_eq!(
        calc_period(PCLKD_48MHZ, 32_000_000),
        Err(PeriodError::TooHigh)
    );
}

#[test]
fn given_pclkd_48mhz_when_1hz_requested_then_divisor_1024_with_46875_counts() {
    // given: PCLKD = 48 MHz
    // when: 1 Hz is requested — every divisor below /1024 overflows the
    // 65 535-tick cap
    // then: PCLKD/1024 delivers it (46 875 ticks = 1.0002 Hz, 0.02%
    // error, inside the tolerance). The truly unreachable region
    // (below ~0.72 Hz) is not expressible in whole hertz.
    let params = calc_period(PCLKD_48MHZ, 1).expect("1 Hz is reachable");
    assert_eq!(params.tpcs, 5);
    assert_eq!(params.divisor, 1024);
    assert_eq!(params.gpr, 46_874);
}

#[test]
fn given_pclkd_48mhz_when_25mhz_requested_then_divisor_1_with_2_counts() {
    // given: PCLKD = 48 MHz
    // when: 25 MHz is requested — 2 ticks deliver 24 MHz, a 4% error,
    // inside the 5% tolerance
    // then: accepted at the finest prescaler.
    let params = calc_period(PCLKD_48MHZ, 25_000_000).expect("24 MHz delivery is in tolerance");
    assert_eq!(params.tpcs, 0);
    assert_eq!(params.gpr, 1);
}

/// Asserts header `P`'s GPT output: channel `T`, output `out`, `PSEL`
/// 00011b. Type-level facts read through `TypeId` (host only).
fn assert_gtioc<P: Gtioc, T: 'static>(out: GptOutput)
where
    P::Timer: 'static,
{
    assert_eq!(
        TypeId::of::<P::Timer>(),
        TypeId::of::<T>(),
        "{}: channel",
        P::NAME
    );
    assert_eq!(P::OUTPUT, out, "{}: output", P::NAME);
    assert_eq!(P::PSEL, 0b00011, "{}: PSEL", P::NAME);
}

/// The six classic Arduino PWM headers: (channel, output) from manual §19
/// Tables 19.6/19.7/19.10/19.12, cross-checked entry-for-entry against the
/// Arduino core's `pinmux.inc`. A wrong pin-table row fails here.
#[cfg(feature = "wifi")]
#[test]
fn given_wifi_map_when_pwm_headers_checked_then_all_six_match_the_manual() {
    // given: the WiFi pin map
    // when / then: each classic PWM header's GPT output matches the manual
    assert_gtioc::<D3, ra4m1::GPT321>(GptOutput::A); // P105 → GTIOC1A (Table 19.6)
    assert_gtioc::<D5, ra4m1::GPT320>(GptOutput::A); // P107 → GTIOC0A (Table 19.6)
    assert_gtioc::<D6, ra4m1::GPT163>(GptOutput::A); // P111 → GTIOC3A (Table 19.7)
    assert_gtioc::<D9, ra4m1::GPT167>(GptOutput::B); // P303 → GTIOC7B (Table 19.10)
    assert_gtioc::<D10, ra4m1::GPT162>(GptOutput::A); // P103 → GTIOC2A (Table 19.6)
    assert_gtioc::<D11, ra4m1::GPT166>(GptOutput::A); // P411 → GTIOC6A (Table 19.12)
}

#[cfg(feature = "minima")]
#[test]
fn given_minima_map_when_pwm_headers_checked_then_all_six_match_the_manual() {
    // given: the Minima pin map
    // when / then: each classic PWM header's GPT output matches the manual
    assert_gtioc::<D3, ra4m1::GPT321>(GptOutput::B); // P104 → GTIOC1B (Table 19.6)
    assert_gtioc::<D5, ra4m1::GPT162>(GptOutput::B); // P102 → GTIOC2B (Table 19.6)
    assert_gtioc::<D6, ra4m1::GPT320>(GptOutput::B); // P106 → GTIOC0B (Table 19.6)
    assert_gtioc::<D9, ra4m1::GPT167>(GptOutput::B); // P303 → GTIOC7B (Table 19.10)
    assert_gtioc::<D10, ra4m1::GPT163>(GptOutput::B); // P112 → GTIOC3B (Table 19.7)
    assert_gtioc::<D11, ra4m1::GPT321>(GptOutput::A); // P109 → GTIOC1A (Table 19.7)
}
