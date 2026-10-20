//! The conversion-time calculator's inputs.

use crate::adc::Resolution;

/// Everything [`calc_scan_time`](super::calc_scan_time) needs: the
/// clocks the unit actually runs at, the sampling-state setting, and
/// the conversion mode.
///
/// At this crate's clock tree ([`crate::clock::Clocks`]) the values are
/// PCLKB = 24 MHz and ADCLK = PCLKC = 48 MHz (`SCKDIVCR` PCKB/2,
/// PCKC/1 — a 1:2 PCLKB:PCLKC ratio, which is also what sets the ≥ 6
/// sampling-state minimum, manual §35.2.14). SST = 0Dh (13 states) is
/// the register reset value the driver ships with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScanTimeParams {
    /// Peripheral module clock, Hz — drives the scan-start and scan-end
    /// delays (manual §35.3.6 Table 35.10).
    pub pclkb_hz: u32,
    /// A/D conversion clock (PCLKC/ADCLK), Hz — drives sampling and
    /// successive approximation.
    pub adclk_hz: u32,
    /// The channel's `ADSSTRn.SST[7:0]` value (5–255 states; register
    /// reset 0Dh = 13). Applies per channel for AN000–AN014 and to the
    /// whole AN016–AN025 group via `ADSSTRL`.
    pub sst_states: u8,
    /// Conversion accuracy — changes the successive-approximation time.
    pub resolution: Resolution,
    /// `ADCSR.ADHSC` mode: `false` = high-speed, `true` = low-current.
    pub low_current: bool,
}
