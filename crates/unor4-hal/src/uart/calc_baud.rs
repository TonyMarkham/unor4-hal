//! The baud-rate calculator: pure arithmetic, host-tested.

use crate::uart::{BaudError, BaudParams, BaudResult};

/// Largest accepted bit-rate error, in percent × 1000 (3,000 = 3%).
pub const MAX_ERROR_X1000: u64 = 3_000;

/// The divisor table: `(bgdm, abcs, cks)` per row, in the same order as
/// FSP's `g_async_baud` with the ABCSE rows removed (parent module docs).
const DIVISORS: [(bool, bool, u8); 9] = [
    (true, true, 0),   // coefficient 8
    (true, false, 0),  // 16
    (false, false, 0), // 32
    (true, false, 1),  // 64
    (false, false, 1), // 128
    (true, false, 2),  // 256
    (false, false, 2), // 512
    (true, false, 3),  // 1024
    (false, false, 3), // 2048
];

/// Finds the register setting whose bit rate is within 3% of `baud`
/// when clocked from `clock_hz` (PCLKA).
///
/// Pure arithmetic — exercised on the host by the unit tests in
/// `src/tests/uart.rs`, so every shipped baud is proven before any
/// hardware run.
pub fn calc_baud(clock_hz: u32, baud: u32) -> BaudResult<BaudParams> {
    if baud == 0 {
        return Err(BaudError::InvalidBaud);
    }
    let clock = u64::from(clock_hz);
    let mut best: Option<(u64, BaudParams)> = None;
    for &(bgdm, abcs, cks) in &DIVISORS {
        let base: u64 = match (bgdm, abcs) {
            (true, true) => 8,
            (true, false) | (false, true) => 16,
            (false, false) => 32,
        };
        let divisor = (base << (2 * cks)) * u64::from(baud);
        // BRR + 1 must fit the 8-bit register (start = the largest
        // candidate N + 1 whose rate is not below the request).
        let start = clock / divisor;
        if start == 0 || start > 256 {
            continue;
        }
        // Mirror FSP's search: every candidate from `start - 1` down.
        // (Without bit-rate modulation the first candidate of a row is
        // already that row's best — the error only grows as N shrinks —
        // but mirroring exactly keeps the numbers identical to the core.)
        let mut n = start;
        while n > 0 {
            n -= 1;
            // Error in percent × 1000, exactly FSP's formula. Never
            // negative: every candidate N + 1 is at most clock/divisor,
            // so the achieved rate is never below the request.
            let err = (clock * 100_000) / (divisor * (n + 1)) - 100_000;
            if err <= MAX_ERROR_X1000 && best.is_none_or(|(best_err, _)| err < best_err) {
                best = Some((
                    err,
                    BaudParams {
                        cks,
                        bgdm,
                        abcs,
                        brr: n as u8,
                    },
                ));
            }
        }
    }
    best.map(|(_, params)| params)
        .ok_or(BaudError::OutOfTolerance)
}
