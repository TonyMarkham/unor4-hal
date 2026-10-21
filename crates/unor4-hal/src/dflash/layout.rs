//! The data flash address map: sizes, the read and P/E address spaces,
//! and the range guard every operation passes through.
//!
//! Manual R01UH0887EJ0110 Rev 1.10 §44.2: Table 44.1 (8 KB, 8-bit
//! programming units, 1-KB erasure units) and Table 44.3 (read
//! `4010 0000h–4010 1FFFh`, P/E `FE00 0000h–FE00 1FFFh`, blocks 0–7).

use crate::dflash::{FlashError, FlashResult};

/// Data flash size in bytes (manual §44.2 Table 44.1).
pub const SIZE: usize = 0x2000;

/// Erasure unit in bytes: one block (manual §44.2 Table 44.1).
pub const BLOCK_SIZE: usize = 0x400;

/// Number of erase blocks, 0–7 (manual §44.2 Table 44.3).
pub const BLOCK_COUNT: usize = SIZE / BLOCK_SIZE;

/// Start of the data flash in the read address space (manual §44.2
/// Table 44.3). Reads are plain loads from here.
pub const READ_BASE: u32 = 0x4010_0000;

/// Start of the data flash in the P/E address space (manual §44.2
/// Table 44.3) — what `FSAR`/`FEAR` take for sequencer commands.
pub(crate) const PE_BASE: u32 = 0xFE00_0000;

/// Checks that `len` bytes at `offset` lie inside the data area.
/// An empty range at any `offset ≤ SIZE` is valid.
pub(crate) fn check_range(offset: usize, len: usize) -> FlashResult<()> {
    match offset.checked_add(len) {
        Some(end) if end <= SIZE => Ok(()),
        _ => Err(FlashError::OutOfRange),
    }
}

/// The P/E-space address of data flash byte `offset`. The caller has
/// already range-checked `offset`, so the sum stays inside
/// `FE00 0000h–FE00 1FFFh` and code flash is unreachable by
/// construction.
pub(crate) fn pe_address(offset: usize) -> u32 {
    PE_BASE + offset as u32
}

/// The offset of erase block `block`.
pub(crate) fn block_offset(block: usize) -> FlashResult<usize> {
    if block < BLOCK_COUNT {
        Ok(block * BLOCK_SIZE)
    } else {
        Err(FlashError::OutOfRange)
    }
}
