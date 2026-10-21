//! Why a data flash operation did not complete.

/// Why a [`DataFlash`](crate::dflash::DataFlash) operation did not
/// complete.
///
/// Every sequencer failure (timeout, error flag) has already been
/// followed by a sequencer reset (`FRESETR`) and a return to read mode
/// when the error reaches the caller. Manual §44.15.8: a program or
/// erase that did not complete successfully requires the area to be
/// erased again before its contents are trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashError {
    /// [`DataFlash::take`](crate::dflash::DataFlash::take) was already
    /// called successfully: the sequencer has exactly one owner.
    Taken,
    /// The flash clock cannot program or erase: FCLK must be 1, 2 or
    /// 3 MHz exactly, or 4–32 MHz (datasheet R01DS0355EJ0110 Table 2.17
    /// and the Table 2.73 notes).
    UnsupportedClock,
    /// The MCU is not in high-speed operating mode (`OPCCR.OPCM ≠ 0`).
    /// This driver's timing constants are the high-speed ones; P/E is
    /// prohibited in low-speed mode (manual §44.15.7).
    UnsupportedMode,
    /// The mode-transition helpers are not running from SRAM: the
    /// firmware's linker script did not place `.data.*` input sections
    /// in RAM (cortex-m-rt's `link.x` does; see the
    /// [module docs](crate::dflash)). Nothing was written.
    RamCodeMissing,
    /// `offset`/`len` reach past the 8 KB data area, or the block index
    /// is not 0–7.
    OutOfRange,
    /// A byte in the write range is not blank. Each byte may be
    /// programmed once per erase (manual §44.15.3); erase its block
    /// first. Nothing was written.
    NotBlank,
    /// `FENTRYR`/`FPMCR` did not read back data flash P/E mode after
    /// the entry sequence.
    ModeEntry,
    /// `FPSR.PERR` (protect error) was set after an `FPMCR` write: the
    /// `FPR` unlock sequence was broken.
    ProtectError,
    /// `FENTRYR` did not return to read mode (`0x0000`) after the exit
    /// sequence.
    ModeExit,
    /// `FSTATR1.FRDY` did not rise (or fall after `FCR` was cleared)
    /// within the worst-case bound for the command.
    Timeout,
    /// `FSTATR2` reported a program error (`PRGERR`/`ILGLERR`).
    ProgramFailed,
    /// `FSTATR2` reported an erase error (`ERERR`/`ILGLERR`).
    EraseFailed,
    /// `FSTATR2` reported an illegal command during a blank check.
    BlankCheckFailed,
}
