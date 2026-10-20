//! The conversion-time calculator's result.

/// The pieces of one single-channel, software-triggered, single-scan
/// conversion time, in nanoseconds (manual §35.3.6, Table 35.10's
/// software-trigger column — the trigger this driver uses):
///
/// `t_SCAN = t_D + t_SPL + t_SAM + t_ED`, with no disconnection
/// assistance and no self-diagnosis (both off in this driver, so the
/// manual's `t_DIS`/`t_DIAG` terms are 0).
///
/// Each term is rounded to whole nanoseconds independently, so
/// `total_ns` may differ from the sum of the parts by 1–2 ns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScanTime {
    /// Start-of-scanning delay `t_D = 2 PCLKB + 4 ADCLK` (software
    /// trigger, no self-diagnosis).
    pub start_delay_ns: u64,
    /// Input sampling time `t_SPL = (SST + ½) ADCLK` — the
    /// sample-and-hold charging window.
    pub sampling_ns: u64,
    /// Successive-approximation time `t_SAM`: 31.5 ADCLK (12-bit,
    /// high-speed), 40.5 (12-bit, low-current), 37.5 (14-bit,
    /// high-speed), 46.5 (14-bit, low-current).
    pub sar_ns: u64,
    /// Scan-end processing `t_ED = 1 PCLKB + 3 ADCLK`.
    pub end_delay_ns: u64,
    /// The whole conversion — what a blocking `read` costs.
    pub total_ns: u64,
}
