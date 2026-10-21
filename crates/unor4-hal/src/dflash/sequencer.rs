//! The flash sequencer's registers and the SRAM-resident mode
//! transitions.
//!
//! The RA4M1's flash is the low-power ("LP") type, version 3 (FSP
//! `bsp_feature.h`: `BSP_FEATURE_FLASH_LP_VERSION = 3`). The manual
//! documents only `DFLCTL` (Rev 1.10 §44.4.3, `FLCN.DFLCTL 407E C090h`,
//! which also fixes the block base); the SVD and the `ra4m1` PAC have no
//! flash peripheral at all. Every other offset below is from the FSP
//! CMSIS device header `R7FA4M1AB.h` (`R_FACI_LP_Type`, BSD-3-Clause),
//! and every sequence from FSP `r_flash_lp.c` (behavioral reference) —
//! raw `volatile` access under HalSpec §4 rule 5, the `PWPR` precedent.
//! The full evidence trail is `Research/slice-10-research.md` §2–§5.

/// `R_FACI_LP` block base (manual Rev 1.10 §44.4.3 via `DFLCTL` at
/// `407E C090h`; FSP `R_FACI_LP_BASE = 0x407EC000`).
const BASE: usize = 0x407E_C000;

/// `DFLCTL`, 8-bit: b0 `DFLEN`, data flash access enable, reset 0
/// (manual §44.4.3).
pub(crate) const DFLCTL: usize = BASE + 0x090;
/// `FPMCR`, 8-bit: flash P/E mode control, written only through the
/// `FPR` unlock (see [`enter_pe`]).
pub(crate) const FPMCR: usize = BASE + 0x100;
/// `FASR`, 8-bit: b0 `EXS`, `0` = user area (the data flash).
pub(crate) const FASR: usize = BASE + 0x104;
/// `FSARL`, 16-bit: command start address, P/E space, bits 15:0.
pub(crate) const FSARL: usize = BASE + 0x108;
/// `FSARH`, 16-bit: command start address, P/E space, bits 31:16.
pub(crate) const FSARH: usize = BASE + 0x110;
/// `FCR`, 8-bit: command register (`OPST` b7 + `CMD` b3:0).
pub(crate) const FCR: usize = BASE + 0x114;
/// `FEARL`, 16-bit: command end address (inclusive), bits 15:0.
pub(crate) const FEARL: usize = BASE + 0x118;
/// `FEARH`, 32-bit register: command end address, bits 31:16.
pub(crate) const FEARH: usize = BASE + 0x120;
/// `FRESETR`, 32-bit: b0 `FRESET`, sequencer software reset.
pub(crate) const FRESETR: usize = BASE + 0x124;
/// `FSTATR00`, 32-bit, read-only: b3 `BCERR0`, blank check result.
pub(crate) const FSTATR00: usize = BASE + 0x128;
/// `FSTATR1`, 32-bit, read-only: b6 `FRDY`, sequencer command done.
pub(crate) const FSTATR1: usize = BASE + 0x12C;
/// `FWBL0`, 32-bit: program data; data flash uses the low 8 bits.
pub(crate) const FWBL0: usize = BASE + 0x130;
/// `FPSR`, 32-bit, read-only: b0 `PERR`, protect error flag.
pub(crate) const FPSR: usize = BASE + 0x184;
/// `FISR`, 32-bit: b5:0 `PCKA` flash clock notification, b7:6 `SAS`
/// temporary boot swap — always read-modify-write.
pub(crate) const FISR: usize = BASE + 0x1D8;
/// `FSTATR2`, 32-bit, read-only: b0 `ERERR`, b1 `PRGERR1`, b4
/// `ILGLERR`.
pub(crate) const FSTATR2: usize = BASE + 0x1F0;
/// `FENTRYR`, 16-bit: P/E mode entry (key `AAh` in b15:8). **LP v3
/// register**: `FENTRYR_MF4` at `+0x3FB0` is the v4 one and is never
/// touched here.
pub(crate) const FENTRYR: usize = BASE + 0x3FB2;

/// `FENTRYR` write: key + `FENTRYD`, data flash P/E mode.
#[cfg_attr(not(target_arch = "arm"), allow(dead_code))] // asm-only
const FENTRYR_DF_PE: u16 = 0xAA80;
/// `FENTRYR` write: key, no P/E bits — read mode.
#[cfg_attr(not(target_arch = "arm"), allow(dead_code))] // asm-only
const FENTRYR_READ: u16 = 0xAA00;
/// `FENTRYR` read-back in data flash P/E mode (`FENTRYD`, b7).
pub(crate) const FENTRYR_DF_PE_MODE: u16 = 0x0080;
/// `FPR` unlock key, written before every `FPMCR` write.
#[cfg_attr(not(target_arch = "arm"), allow(dead_code))] // asm-only
const FPR_KEY: u8 = 0xA5;
/// `FPMCR` data flash P/E mode, high-speed operating mode (`FMS1`; FSP
/// `FLASH_LP_DATAFLASH_PE_MODE`, written without `LVPE` when
/// `OPCCR.OPCM = 0`).
pub(crate) const FPMCR_DF_PE: u8 = 0x10;
/// `FPMCR` read mode (`RPDIS`; FSP `FLASH_LP_READ_MODE`).
#[cfg_attr(not(target_arch = "arm"), allow(dead_code))] // asm-only
const FPMCR_READ: u8 = 0x08;

/// `FCR` program command (FSP `FLASH_LP_FCR_WRITE`).
pub(crate) const FCR_PROGRAM: u8 = 0x81;
/// `FCR` blank check command (FSP `FLASH_LP_FCR_BLANKCHECK`).
pub(crate) const FCR_BLANK_CHECK: u8 = 0x83;
/// `FCR` block erase command (FSP `FLASH_LP_FCR_ERASE`).
pub(crate) const FCR_ERASE: u8 = 0x84;

/// `FSTATR1.FRDY`.
pub(crate) const FRDY: u32 = 1 << 6;
/// `FSTATR00.BCERR0`: set when the checked range is not blank.
pub(crate) const BCERR0: u32 = 1 << 3;
/// `FPSR.PERR`.
pub(crate) const PERR: u32 = 1 << 0;
/// `FISR.PCKA` field mask.
pub(crate) const PCKA_MASK: u32 = 0x3F;
/// `FSTATR2` bits that fail a program: `ILGLERR | PRGERR1` (FSP
/// `FLASH_LP_FSTATR2_WRITE_ERROR_BITS`).
pub(crate) const PROGRAM_ERRORS: u32 = 0x12;
/// `FSTATR2` bits that fail an erase: `ILGLERR | ERERR` (FSP
/// `FLASH_LP_FSTATR2_ERASE_ERROR_BITS`).
pub(crate) const ERASE_ERRORS: u32 = 0x11;
/// `FSTATR2` bits that fail a blank check: `ILGLERR` (FSP
/// `FLASH_LP_FSTATR2_ILLEGAL_ERROR_BITS`).
pub(crate) const BLANK_CHECK_ERRORS: u32 = 0x10;

/// The on-chip SRAM: 32 KB at `0x2000_0000` (the firmware's `memory.x`
/// `RAM` region).
const SRAM: core::ops::Range<usize> = 0x2000_0000..0x2000_8000;

/// Reads an 8-bit sequencer register.
#[inline(always)]
pub(crate) fn read8(address: usize) -> u8 {
    unsafe { (address as *const u8).read_volatile() }
}

/// Writes an 8-bit sequencer register.
#[inline(always)]
pub(crate) fn write8(address: usize, value: u8) {
    unsafe { (address as *mut u8).write_volatile(value) }
}

/// Reads a 16-bit sequencer register.
#[inline(always)]
pub(crate) fn read16(address: usize) -> u16 {
    unsafe { (address as *const u16).read_volatile() }
}

/// Writes a 16-bit sequencer register.
#[inline(always)]
pub(crate) fn write16(address: usize, value: u16) {
    unsafe { (address as *mut u16).write_volatile(value) }
}

/// Reads a 32-bit sequencer register.
#[inline(always)]
pub(crate) fn read32(address: usize) -> u32 {
    unsafe { (address as *const u32).read_volatile() }
}

/// Writes a 32-bit sequencer register.
#[inline(always)]
pub(crate) fn write32(address: usize, value: u32) {
    unsafe { (address as *mut u32).write_volatile(value) }
}

/// True when both mode-transition helpers were linked into SRAM: the
/// firmware's linker script placed the `.data.*` input sections in RAM,
/// as cortex-m-rt's `link.x` does.
pub(crate) fn helpers_in_sram() -> bool {
    // Thumb function pointers carry bit 0; the code starts one byte below.
    let enter = enter_pe as *const () as usize & !1;
    let exit = exit_pe as *const () as usize & !1;
    SRAM.contains(&enter) && SRAM.contains(&exit)
}

/// Read mode → data flash P/E mode, executed from SRAM.
///
/// `FENTRYR = AA80h`, wait tDSTOP (`spin` loop passes), then the `FPMCR`
/// unlock: `FPR = A5h`, `FPMCR` = value, ~value, value (FSP
/// `r_flash_lp_df_enter_pe_mode` + `r_flash_lp_write_fpmcr`, high-speed
/// branch). The caller masks interrupts around the call and checks
/// `FENTRYR`, `FPMCR` and `FPSR.PERR` afterwards.
///
/// One `asm!` block holds the whole sequence, so no instruction of it is
/// fetched from code flash at any optimization level; the constants are
/// immediates of the instructions themselves.
///
/// # Safety
/// The data flash must be in read mode with `DFLCTL.DFLEN = 1`, and the
/// caller must own the sequencer.
#[inline(never)]
#[cfg_attr(
    target_os = "none",
    unsafe(link_section = ".data.unor4_hal_dflash_enter")
)]
pub(crate) unsafe fn enter_pe(spin: u32) {
    #[cfg(target_arch = "arm")]
    unsafe {
        core::arch::asm!(
            "movw {t}, #{entry}",
            "strh {t}, [{fentryr}]",
            "2:",
            "subs {spin}, {spin}, #1",
            "bne 2b",
            "movw {t}, #{key}",
            "str {t}, [{base}, #0x180]",
            "movw {t}, #{mode}",
            "strb {t}, [{base}, #0x100]",
            "movw {t}, #{inverse}",
            "strb {t}, [{base}, #0x100]",
            "movw {t}, #{mode}",
            "strb {t}, [{base}, #0x100]",
            entry = const FENTRYR_DF_PE,
            key = const FPR_KEY,
            mode = const FPMCR_DF_PE,
            inverse = const !FPMCR_DF_PE,
            t = out(reg) _,
            fentryr = in(reg) FENTRYR,
            base = in(reg) BASE,
            spin = inout(reg) spin => _,
            options(nostack),
        );
    }
    #[cfg(not(target_arch = "arm"))]
    let _ = spin;
}

/// Data flash P/E mode → read mode, executed from SRAM. Returns the
/// `FENTRYR` polls left: `0` means it never read `0000h` (timeout).
///
/// `FPR = A5h`, `FPMCR` = read mode, ~, read mode; wait tMS (`spin`
/// passes); `FENTRYR = AA00h`; poll `FENTRYR` until `0000h`, at most
/// `polls` times (FSP `r_flash_lp_pe_mode_exit`; its discharge steps are
/// code-flash-only and never run here). Same single-`asm!` rule as
/// [`enter_pe`].
///
/// # Safety
/// The data flash must be in P/E mode with no command running, and the
/// caller must own the sequencer.
#[inline(never)]
#[cfg_attr(
    target_os = "none",
    unsafe(link_section = ".data.unor4_hal_dflash_exit")
)]
pub(crate) unsafe fn exit_pe(spin: u32, polls: u32) -> u32 {
    #[cfg(target_arch = "arm")]
    {
        let mut left = polls;
        unsafe {
            core::arch::asm!(
                "movw {t}, #{key}",
                "str {t}, [{base}, #0x180]",
                "movw {t}, #{mode}",
                "strb {t}, [{base}, #0x100]",
                "movw {t}, #{inverse}",
                "strb {t}, [{base}, #0x100]",
                "movw {t}, #{mode}",
                "strb {t}, [{base}, #0x100]",
                "2:",
                "subs {spin}, {spin}, #1",
                "bne 2b",
                "movw {t}, #{read}",
                "strh {t}, [{fentryr}]",
                "3:",
                "ldrh {t}, [{fentryr}]",
                "cmp {t}, #0",
                "beq 4f",
                "subs {left}, {left}, #1",
                "bne 3b",
                "4:",
                key = const FPR_KEY,
                mode = const FPMCR_READ,
                inverse = const !FPMCR_READ,
                read = const FENTRYR_READ,
                t = out(reg) _,
                fentryr = in(reg) FENTRYR,
                base = in(reg) BASE,
                spin = inout(reg) spin => _,
                left = inout(reg) left,
                options(nostack),
            );
        }
        left
    }
    #[cfg(not(target_arch = "arm"))]
    {
        let _ = (spin, polls);
        0
    }
}
