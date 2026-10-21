//! Data flash driver (Slice 10) — the RA4M1's 8 KB data flash as
//! byte-programmed, block-erased storage.
//!
//! Hardware facts, Renesas RA4M1 manual R01UH0887EJ0110 **Rev 1.10**
//! and datasheet R01DS0355EJ0110 Rev 1.10 unless the source is named
//! otherwise:
//! - 8 KB in eight 1-KB erase blocks, programmed one byte at a time;
//!   read at `4010 0000h–4010 1FFFh`, programmed and erased through the
//!   P/E space `FE00 0000h–FE00 1FFFh` (§44.2 Tables 44.1/44.3). Every
//!   operation is range-checked against this window, so code flash —
//!   the bootloader and the sketch — is unreachable through this API.
//! - **Once per erase:** no byte may be programmed twice without an
//!   erase of its block in between (§44.15.3; datasheet Table 2.72
//!   Note 1, "Overwriting is prohibited"). [`DataFlash::write`]
//!   blank-checks its range first and refuses non-blank bytes.
//! - **Erased state is a blank check, not a read:** an erased, unwritten
//!   byte's read value is not guaranteed (§44.7 Table 44.8, blank check
//!   row), even though §44.2 Table 44.1 lists `FFh` after erasure — the
//!   conservative statement governs. Use [`DataFlash::blank_check`].
//! - Access enable: `DFLCTL.DFLEN` resets to 0; after setting it, wait
//!   tDSTOP (5 µs, high-speed mode) before the first read or P/E entry
//!   (§44.4.3). [`DataFlash::take`] does this.
//! - The sequencer is the LP type, version 3 (FSP `bsp_feature.h`). Its
//!   registers — `FENTRYR`, `FPMCR`, `FCR`, `FSTATR*`, … — are not in
//!   any manual revision, the SVD or the PAC; offsets come from the FSP
//!   device header, sequences from FSP `r_flash_lp.c` (see
//!   `sequencer.rs`).
//! - Endurance: 100,000 erase cycles per block (datasheet Table 2.72).
//!   Worst-case times (Table 2.73, high-speed, FCLK 4 MHz column): 1-byte
//!   program 463 µs, 1-KB erase 286 ms, 1-KB blank check 1872 µs. Every
//!   call blocks for its whole session.
//! - Abnormal termination (power loss, reset, timeout) during program or
//!   erase: erase the area again before trusting it (§44.15.8).
//!
//! # SRAM execution: no firmware changes needed
//!
//! The two mode transitions (read ↔ data flash P/E) run from SRAM, as
//! FSP places them (`PLACE_IN_RAM_SECTION`). They are emitted into
//! `.data.*` input sections, which cortex-m-rt 0.7's `link.x` already
//! collects into the `.data` output section (`*(.data .data.*)`, `> RAM
//! AT>FLASH`): the startup `.data` copy loads them into RAM, and the
//! linker adds long-branch thunks for the calls from flash. A firmware
//! using cortex-m-rt needs no `memory.x` change. With a linker script
//! that puts `.data.*` anywhere but RAM, [`DataFlash::take`] returns
//! [`FlashError::RamCodeMissing`] before touching the sequencer.

mod data_flash;
mod flash_error;
mod flash_result;
mod layout;
mod sequencer;
mod timing;

// `pub(crate)`: the host tests in `src/tests/dflash.rs` check the
// address and timing arithmetic; it is not part of the public API.
#[cfg(test)]
pub(crate) use crate::dflash::{
    layout::{block_offset, check_range, pe_address},
    timing::{pcka, poll_limit, spin_count},
};

// ---------------------------------------------------------------------------------------------- //

pub use crate::dflash::{
    data_flash::DataFlash,
    flash_error::FlashError,
    flash_result::FlashResult,
    layout::{BLOCK_COUNT, BLOCK_SIZE, READ_BASE, SIZE},
};
