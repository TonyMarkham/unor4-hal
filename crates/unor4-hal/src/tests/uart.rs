//! Host tests for the baud calculator (`uart::calc_baud`) — pure
//! arithmetic, no hardware. The SCI bit-rate generator runs from PCLKA
//! (48 MHz after Slice 1's clock init); the tests exercise the real
//! clock plus generic clocks for the CKS divider logic.

use crate::uart::{BaudError, BaudParams, calc_baud};

#[test]
fn given_pclka_48mhz_when_calc_9600_then_base_row_brr_155() {
    // given: PCLKA = 48 MHz (Slice 1's clock tree — the SCI's real
    // base clock)
    let clock = 48_000_000;

    // when: 9600 baud is calculated
    let params = calc_baud(clock, 9_600).unwrap();

    // then: the base row (coefficient 32) with BRR = 155 gives
    // 48 MHz / (32 x 156) = 9615.4 baud, +0.16% — the BGDM/ABCS rows
    // need BRR > 255 at this rate and are rejected
    assert_eq!(
        params,
        BaudParams {
            cks: 0,
            bgdm: false,
            abcs: false,
            brr: 155,
        }
    );
    assert_eq!(params.achieved_baud(clock), 9615);
}

#[test]
fn given_pclka_48mhz_when_calc_115200_then_bgdm_abcs_row_brr_51() {
    // given: PCLKA = 48 MHz
    let clock = 48_000_000;

    // when: 115200 baud is calculated
    let params = calc_baud(clock, 115_200).unwrap();

    // then: the BGDM + ABCS row (coefficient 8) with BRR = 51 gives
    // 48 MHz / (8 x 52) = 115384.6 baud, +0.16%
    assert_eq!(
        params,
        BaudParams {
            cks: 0,
            bgdm: true,
            abcs: true,
            brr: 51,
        }
    );
    assert_eq!(params.achieved_baud(clock), 115_384);
}

#[test]
fn given_pclka_48mhz_when_calc_57600_then_brr_103() {
    // given: PCLKA = 48 MHz
    // when: 57600 baud is calculated
    let params = calc_baud(48_000_000, 57_600).unwrap();

    // then: coefficient 8 with BRR = 103 gives 48 MHz / (8 x 104) =
    // 57692 baud, +0.16%
    assert_eq!(
        params,
        BaudParams {
            cks: 0,
            bgdm: true,
            abcs: true,
            brr: 103,
        }
    );
    assert_eq!(params.achieved_baud(48_000_000), 57_692);
}

#[test]
fn given_pclka_48mhz_when_calc_38400_then_brr_155() {
    // given: PCLKA = 48 MHz
    // when: 38400 baud is calculated
    let params = calc_baud(48_000_000, 38_400).unwrap();

    // then: coefficient 8 with BRR = 155 gives 48 MHz / (8 x 156) =
    // 38461.5 baud, +0.16%
    assert_eq!(
        params,
        BaudParams {
            cks: 0,
            bgdm: true,
            abcs: true,
            brr: 155,
        }
    );
}

#[test]
fn given_slow_300_baud_when_calc_then_cks_divides_the_clock() {
    // given: a 24 MHz base clock and a rate so slow every CKS = 0 row would
    // need BRR > 255
    // when: 300 baud is calculated
    let params = calc_baud(24_000_000, 300).unwrap();

    // then: the CKS = 2 row (coefficient 512) with BRR = 155 gives
    // 24 MHz / (512 x 156) = 300.5 baud, +0.16% — every coarser row
    // would need BRR > 255 and is rejected
    assert_eq!(
        params,
        BaudParams {
            cks: 2,
            bgdm: false,
            abcs: false,
            brr: 155,
        }
    );
}

#[test]
fn given_pclka_48mhz_when_calc_300_then_the_largest_cks_row_wins() {
    // given: PCLKA = 48 MHz (the SCI's real base clock) — still
    // in-register for the CKS rows at 300 baud
    // when: 300 baud is calculated
    let params = calc_baud(48_000_000, 300).unwrap();

    // then: the CKS = 3 row (coefficient 1024) with BRR = 155 gives
    // 48 MHz / (1024 x 156) = 300.5 baud, +0.16%
    assert_eq!(
        params,
        BaudParams {
            cks: 3,
            bgdm: true,
            abcs: false,
            brr: 155,
        }
    );
}

#[test]
fn given_hopeless_clock_when_calc_115200_then_out_of_tolerance() {
    // given: a 1 MHz base clock — the closest 8N1 setting is 125000
    // baud, +8.5% off
    // when: 115200 baud is calculated
    let result = calc_baud(1_000_000, 115_200);

    // then: no setting lands within 3%, and that is an error, not a
    // silently-wrong baud
    assert_eq!(result, Err(BaudError::OutOfTolerance));
}

#[test]
fn given_zero_baud_when_calc_then_invalid_argument() {
    // given: a nonsensical request
    // when: 0 baud is calculated
    let result = calc_baud(24_000_000, 0);

    // then: rejected before any math runs
    assert_eq!(result, Err(BaudError::InvalidBaud));
}

#[test]
fn given_params_when_coefficient_then_it_matches_the_manuals_equation() {
    // given: the three settings the calculator can produce
    let base = BaudParams {
        cks: 0,
        bgdm: false,
        abcs: false,
        brr: 0,
    };
    let bgdm = BaudParams { bgdm: true, ..base };
    let bgdm_abcs = BaudParams { abcs: true, ..bgdm };

    // when: their coefficients are taken
    // then: 32 base, 16 with BGDM, 8 with BGDM + ABCS — and CKS
    // multiplies by 4 per step
    assert_eq!(base.coefficient(), 32);
    assert_eq!(bgdm.coefficient(), 16);
    assert_eq!(bgdm_abcs.coefficient(), 8);
    assert_eq!(BaudParams { cks: 3, ..base }.coefficient(), 32 * 64);
}
