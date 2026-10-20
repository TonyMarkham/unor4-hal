//! The scan-conversion-time calculator: Table 35.10 as arithmetic.

use crate::adc::{Resolution, ScanTime, ScanTimeParams};

/// Cycles to nanoseconds, floored at 1 ns granularity:
/// `cycles × 10⁹ / freq_hz`. The widest intermediate (`100 × 10⁹`) is
/// far inside `u64`.
fn cycles_to_ns(cycles: u64, freq_hz: u32) -> u64 {
    cycles * 1_000_000_000 / freq_hz as u64
}

/// Computes one software-triggered, single-channel conversion time from
/// the manual's terms (§35.3.6, Table 35.10 software-trigger column;
/// disconnection assistance and self-diagnosis are off in this driver,
/// so their terms are zero):
///
/// - `t_D  = 2 PCLKB + 4 ADCLK`
/// - `t_SPL = (SST + ½) ADCLK`
/// - `t_SAM` = 31.5 / 40.5 / 37.5 / 46.5 ADCLK (resolution × `ADHSC`)
/// - `t_ED = 1 PCLKB + 3 ADCLK`
///
/// Half-ADCLK terms are computed at twice the clock with twice the
/// cycle count — exact, no rounding beyond the per-term ns floor.
///
/// The parameters are caller-supplied facts, not searched: there is no
/// failure mode, and the result is informational (the driver polls
/// `ADST` rather than timing anything). Domain: `sst_states` in
/// 5..=255 (§35.2.14's hardware range — 6 minimum at this crate's 1:2
/// PCLKB:PCLKC ratio), nonzero clocks.
pub fn calc_scan_time(params: ScanTimeParams) -> ScanTime {
    let adclk = params.adclk_hz as u64;

    let start_delay_ns = cycles_to_ns(2, params.pclkb_hz) + cycles_to_ns(4, params.adclk_hz);
    // (SST + ½) ADCLK = (2·SST + 1) half-cycles.
    let sampling_ns = (2 * params.sst_states as u64 + 1) * 1_000_000_000 / (2 * adclk);
    // Successive approximation in half-ADCLK units.
    let half_cycles: u64 = match (params.resolution, params.low_current) {
        (Resolution::Bits12, false) => 63,
        (Resolution::Bits12, true) => 81,
        (Resolution::Bits14, false) => 75,
        (Resolution::Bits14, true) => 93,
    };
    let sar_ns = half_cycles * 1_000_000_000 / (2 * adclk);
    let end_delay_ns = cycles_to_ns(1, params.pclkb_hz) + cycles_to_ns(3, params.adclk_hz);

    ScanTime {
        start_delay_ns,
        sampling_ns,
        sar_ns,
        end_delay_ns,
        total_ns: start_delay_ns + sampling_ns + sar_ns + end_delay_ns,
    }
}
