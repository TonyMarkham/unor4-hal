//! Host tests for the GPT period calculator — arithmetic facts,
//! re-derivable from manual §22.2.12 (TPCS ladder) and §22.2.20 (saw
//! period = GTPR + 1), the same way the SCI baud pairs are.

use crate::pwm::{PeriodError, calc_period};

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

/// The golden constructor surface: every header PWM pin's
/// (engine, chip pin, output) triple from manual §19 Tables
/// 19.6/19.7/19.10/19.12, asserted as function-pointer types so a
/// missing or mistyped constructor fails to compile. Purely type-level —
/// nothing here touches hardware.
//
// Cross-check data: manual §19 Tables 19.6/19.7/19.10/19.12 (the table
// cross-checked entry-for-entry against the Arduino core's `pinmux.inc`).
#[cfg(feature = "wifi")]
#[test]
fn given_wifi_map_when_constructor_surface_checked_then_all_six_pins_match_the_manual() {
    use crate::clock::Clocks;
    use crate::gpio::{Input, Pin};
    use crate::pwm::Pwm;

    // given: the WiFi pin map
    // when: each header PWM pin's constructor is viewed as a signature
    // then: engine, chip pin, and output match the manual's tables
    // D3: P105 → GTIOC1A (Table 19.7)
    let _: fn(
        ra4m1::GPT321,
        Pin<'1', 5, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT321, '1', 5, false> = Pwm::d3;
    // D5: P107 → GTIOC0A (Table 19.6)
    let _: fn(
        ra4m1::GPT320,
        Pin<'1', 7, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT320, '1', 7, false> = Pwm::d5;
    // D6: P111 → GTIOC3A (Table 19.7)
    let _: fn(
        ra4m1::GPT163,
        Pin<'1', 11, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT163, '1', 11, false> = Pwm::d6;
    // D9: P303 → GTIOC7B (Table 19.10)
    let _: fn(ra4m1::GPT167, Pin<'3', 3, Input>, u32, &Clocks) -> Pwm<ra4m1::GPT167, '3', 3, true> =
        Pwm::d9;
    // D10: P103 → GTIOC2A (Table 19.6)
    let _: fn(
        ra4m1::GPT162,
        Pin<'1', 3, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT162, '1', 3, false> = Pwm::d10;
    // D11: P411 → GTIOC6A (Table 19.12)
    let _: fn(
        ra4m1::GPT166,
        Pin<'4', 11, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT166, '4', 11, false> = Pwm::d11;
}

#[cfg(feature = "minima")]
#[test]
fn given_minima_map_when_constructor_surface_checked_then_all_six_pins_match_the_manual() {
    use crate::clock::Clocks;
    use crate::gpio::{Input, Pin};
    use crate::pwm::Pwm;

    // given: the Minima pin map
    // when: each header PWM pin's constructor is viewed as a signature
    // then: engine, chip pin, and output match the manual's tables
    // D3: P104 → GTIOC1B (Table 19.6)
    let _: fn(ra4m1::GPT321, Pin<'1', 4, Input>, u32, &Clocks) -> Pwm<ra4m1::GPT321, '1', 4, true> =
        Pwm::d3;
    // D5: P102 → GTIOC2B (Table 19.6)
    let _: fn(ra4m1::GPT162, Pin<'1', 2, Input>, u32, &Clocks) -> Pwm<ra4m1::GPT162, '1', 2, true> =
        Pwm::d5;
    // D6: P106 → GTIOC0B (Table 19.6)
    let _: fn(ra4m1::GPT320, Pin<'1', 6, Input>, u32, &Clocks) -> Pwm<ra4m1::GPT320, '1', 6, true> =
        Pwm::d6;
    // D9: P303 → GTIOC7B (Table 19.10)
    let _: fn(ra4m1::GPT167, Pin<'3', 3, Input>, u32, &Clocks) -> Pwm<ra4m1::GPT167, '3', 3, true> =
        Pwm::d9;
    // D10: P112 → GTIOC3B (Table 19.7)
    let _: fn(
        ra4m1::GPT163,
        Pin<'1', 12, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT163, '1', 12, true> = Pwm::d10;
    // D11: P109 → GTIOC1A (Table 19.7)
    let _: fn(
        ra4m1::GPT321,
        Pin<'1', 9, Input>,
        u32,
        &Clocks,
    ) -> Pwm<ra4m1::GPT321, '1', 9, false> = Pwm::d11;
}
