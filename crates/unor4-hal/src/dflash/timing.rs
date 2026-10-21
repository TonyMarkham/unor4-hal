//! Clock-derived settings and worst-case wait bounds.
//!
//! Datasheet R01DS0355EJ0110 Rev 1.10 §2.15.2 Table 2.73 (high-speed
//! operating mode) gives the maxima; the FSP `r_flash_lp.c` constants
//! (BSD-3-Clause, behavioral reference) give the margins this driver
//! keeps. Polling bounds follow the FSP scheme: worst-case µs × ICLK MHz
//! ÷ the minimum cycles one poll iteration takes, so a bound is always
//! at least the worst-case time.

/// tDSTOP, data flash STOP recovery time: 5 µs in high-speed mode
/// (manual §44.4.3; datasheet Table 2.73). Waited after setting
/// `DFLCTL.DFLEN` and after writing `FENTRYR` for P/E entry.
pub(crate) const T_DSTOP_US: u32 = 5;

/// tMS, the flash mode transition wait on LP flash version 3: 6 µs (FSP
/// `FLASH_LP_WAIT_TMS_HIGH` for version 3; the RA4M1 is version 3 per
/// FSP `bsp_feature.h`). Waited after `FPMCR` returns to read mode.
pub(crate) const T_MS_US: u32 = 6;

/// Bound for one byte program, µs. Datasheet Table 2.73: 463 µs max
/// (FCLK = 4 MHz column, the slower bound for this crate's 24 MHz FCLK).
/// Kept at FSP's `FLASH_LP_MAX_WRITE_DF_TIME_US` (886 µs, the
/// middle-speed maximum of Table 2.74) for margin.
pub(crate) const PROGRAM_MAX_US: u32 = 886;

/// Bound for one 1-KB block erase, µs. Datasheet Table 2.73: 286 ms max
/// (FCLK = 4 MHz column). Kept at FSP's
/// `FLASH_LP_MAX_ERASE_DF_BLOCK_TIME_US` (504 ms) for margin.
pub(crate) const ERASE_BLOCK_MAX_US: u32 = 504_000;

/// Bound for a blank check, µs per byte. Datasheet Table 2.73: 24.3 µs
/// max for 1 byte, 1872 µs max for 1 KB (FCLK = 4 MHz column). Kept at
/// FSP's per-byte `FLASH_LP_MAX_BLANK_CHECK_TIME_US` (88 µs) for margin.
pub(crate) const BLANK_CHECK_MAX_US_PER_BYTE: u32 = 88;

/// Minimum CPU cycles one register poll iteration takes (FSP
/// `FLASH_LP_CYCLES_MINIMUM_PER_TIMEOUT_LOOP`).
pub(crate) const CYCLES_PER_POLL: u32 = 6;

/// Poll bound for `FENTRYR` to clear after the exit write (FSP
/// `FLASH_LP_FRDY_CMD_TIMEOUT`, sized for ICLK 240 MHz / FCLK 4 MHz —
/// generous at this crate's 48 MHz).
pub(crate) const MODE_EXIT_POLLS: u32 = 19_200;

/// `FISR.PCKA` for a flash clock of `fclk_hz`, or `None` when the clock
/// cannot program or erase.
///
/// FCLK must be exactly 1, 2 or 3 MHz, or 4–32 MHz (datasheet §2.3.1
/// Table 2.17: FCLK 32 MHz max in high-speed mode at 2.7–5.5 V; Table
/// 2.73 notes: 1 MHz minimum for P/E, below 4 MHz only 1, 2 or 3 MHz).
/// The value is
/// the frequency rounded up to whole MHz, minus one, in 5 bits (FSP
/// `r_flash_lp_set_fisr`, LP version 3 branch).
pub(crate) fn pcka(fclk_hz: u32) -> Option<u8> {
    const MHZ: u32 = 1_000_000;
    let exact_low = fclk_hz == MHZ || fclk_hz == 2 * MHZ || fclk_hz == 3 * MHZ;
    if !(exact_low || (4 * MHZ..=32 * MHZ).contains(&fclk_hz)) {
        return None;
    }
    let mhz = fclk_hz.div_ceil(MHZ);
    Some(((mhz - 1) & 0x1F) as u8)
}

/// Poll-loop iterations that take at least `max_us` at `sysclk_hz`,
/// given [`CYCLES_PER_POLL`]. Saturates instead of overflowing.
pub(crate) fn poll_limit(max_us: u64, sysclk_hz: u32) -> u32 {
    let mhz = u64::from(sysclk_hz.div_ceil(1_000_000));
    let polls = max_us.saturating_mul(mhz) / u64::from(CYCLES_PER_POLL);
    polls.clamp(1, u64::from(u32::MAX)) as u32
}

/// Spin-loop iterations that take at least `us` at `sysclk_hz`,
/// counting one cycle per iteration (each `subs`/`bne` pass takes
/// more, so the wait only errs long). At least 1: the spin loop
/// decrements before testing.
pub(crate) fn spin_count(us: u32, sysclk_hz: u32) -> u32 {
    let mhz = sysclk_hz.div_ceil(1_000_000);
    us.saturating_mul(mhz).max(1)
}
