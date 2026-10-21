//! Slice 10: the boot log (`K`) and the data flash self-test (`F`).
//!
//! The boot counter is an append log in data flash block 7: each boot
//! programs the next blank byte, so a boot costs one byte and no
//! erase. "Clear one more bit per boot" is not an option — a byte may be
//! programmed only once per erase (manual §44.15.3). The first blank
//! slot is found by blank check, never by reading for `FFh` (manual
//! §44.7 Table 44.8). When all 1024 slots are used the block is erased
//! and the count restarts at 1. The self-test uses block 6; neither
//! touches blocks 0–5.

use crate::{body::Body, clock::Clock};

use unor4_hal::dflash::{BLOCK_SIZE, DataFlash, FlashError, FlashResult};

/// The boot log's block.
const LOG_BLOCK: usize = 7;
/// The self-test's scratch block.
const TEST_BLOCK: usize = 6;
/// Self-test program chunk: keeps every gap between [`Clock::now_ms`]
/// calls far below SysTick's ~349 ms wrap even at the 463 µs/byte
/// worst case.
const TEST_CHUNK: usize = 128;

/// The byte boot `slot` holds: never `FFh`, and position-dependent, so
/// a shifted or partial log does not verify.
fn log_value(slot: usize) -> u8 {
    (slot % 255) as u8
}

/// The self-test's byte at `index`: never `FFh`.
fn test_value(index: usize) -> u8 {
    ((index * 7 + 3) % 251) as u8
}

/// The boot log as recorded at this boot.
pub(crate) struct BootLog {
    /// Boots counted in the log, this one included; 0 if recording
    /// failed.
    pub(crate) count: usize,
    /// The recording error, if any.
    pub(crate) error: Option<FlashError>,
}

impl BootLog {
    /// Finds the log's first blank slot, checks the log, appends this
    /// boot. A full block is erased first. `flash` is the result of
    /// [`DataFlash::take`]; its error, if any, is what `K` reports.
    pub(crate) fn record(flash: &mut FlashResult<DataFlash>) -> Self {
        match flash.as_mut().map_err(|error| *error).and_then(append) {
            Ok(count) => Self { count, error: None },
            Err(error) => Self {
                count: 0,
                error: Some(error),
            },
        }
    }

    /// The `K` request: `K <count> <ok>`, where `ok` = 1 when every
    /// logged slot reads back its value right now (plain reads, no P/E
    /// mode). `K ERR <name>` when recording failed at boot.
    pub(crate) fn reply(&self, flash: Option<&DataFlash>) -> Body {
        match (self.error, flash) {
            (None, Some(flash)) => {
                let mut body = Body::new(b"K");
                body.num(self.count as u32)
                    .num(u32::from(log_intact(flash, self.count)));
                body
            }
            (error, _) => {
                let mut body = Body::new(b"K ERR ");
                body.text(error_name(error.unwrap_or(FlashError::Taken)));
                body
            }
        }
    }
}

/// The log append behind [`BootLog::record`]; returns the new count.
fn append(flash: &mut DataFlash) -> FlashResult<usize> {
    let base = LOG_BLOCK * BLOCK_SIZE;
    // Smallest slot that is blank; the log fills from slot 0 upwards.
    let (mut low, mut high) = (0, BLOCK_SIZE);
    while low < high {
        let mid = (low + high) / 2;
        if flash.blank_check(base + mid, 1)? {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let mut used = low;
    if used == BLOCK_SIZE {
        flash.erase_block(LOG_BLOCK)?;
        used = 0;
    }
    flash.write(base + used, &[log_value(used)])?;
    Ok(used + 1)
}

/// Whether slots `0..count` read back their values.
fn log_intact(flash: &DataFlash, count: usize) -> bool {
    let base = LOG_BLOCK * BLOCK_SIZE;
    let mut byte = [0u8];
    (0..count).all(|slot| flash.read(base + slot, &mut byte).is_ok() && byte[0] == log_value(slot))
}

/// The `F` request: erase block 6, blank-check it, program 1024 known
/// bytes, read them back, prove a second program of a used byte is
/// refused, erase again and blank-check. `F OK <blank erase ms> <program
/// ms> <erase ms> <mismatches>` — the first erase finds the block blank,
/// the second erases 1024 programmed bytes — or `F FAIL <step> <error>`.
pub(crate) fn self_test(flash: Option<&mut DataFlash>, clock: &mut Clock) -> Body {
    let Some(flash) = flash else {
        return Body::new(b"F FAIL TAKE");
    };
    let base = TEST_BLOCK * BLOCK_SIZE;

    let start = clock.now_ms();
    if let Err(error) = flash.erase_block(TEST_BLOCK) {
        return failed(b"ERASE", error);
    }
    let blank_erase_ms = clock.now_ms() - start;
    match flash.blank_check(base, BLOCK_SIZE) {
        Ok(true) => {}
        Ok(false) => return Body::new(b"F FAIL BLANK1 NOTBLANK"),
        Err(error) => return failed(b"BLANK1", error),
    }

    let start = clock.now_ms();
    let mut chunk = [0u8; TEST_CHUNK];
    for first in (0..BLOCK_SIZE).step_by(TEST_CHUNK) {
        for (index, byte) in chunk.iter_mut().enumerate() {
            *byte = test_value(first + index);
        }
        if let Err(error) = flash.write(base + first, &chunk) {
            return failed(b"PROGRAM", error);
        }
        clock.now_ms();
    }
    let program_ms = clock.now_ms() - start;

    let mut mismatches = 0u32;
    let mut byte = [0u8];
    for index in 0..BLOCK_SIZE {
        if flash.read(base + index, &mut byte).is_err() || byte[0] != test_value(index) {
            mismatches += 1;
        }
    }
    match flash.blank_check(base, BLOCK_SIZE) {
        Ok(false) => {}
        Ok(true) => return Body::new(b"F FAIL BLANK2 BLANK"),
        Err(error) => return failed(b"BLANK2", error),
    }
    match flash.write(base, &[0]) {
        Err(FlashError::NotBlank) => {}
        Ok(()) => return Body::new(b"F FAIL REWRITE ACCEPTED"),
        Err(error) => return failed(b"REWRITE", error),
    }

    let start = clock.now_ms();
    if let Err(error) = flash.erase_block(TEST_BLOCK) {
        return failed(b"ERASE2", error);
    }
    let erase_ms = clock.now_ms() - start;
    match flash.blank_check(base, BLOCK_SIZE) {
        Ok(true) => {}
        Ok(false) => return Body::new(b"F FAIL BLANK3 NOTBLANK"),
        Err(error) => return failed(b"BLANK3", error),
    }

    let mut body = Body::new(b"F OK");
    body.num(blank_erase_ms as u32)
        .num(program_ms as u32)
        .num(erase_ms as u32)
        .num(mismatches);
    body
}

/// `F FAIL <step> <error>`.
fn failed(step: &[u8], error: FlashError) -> Body {
    let mut body = Body::new(b"F FAIL ");
    body.text(step).text(b" ").text(error_name(error));
    body
}

/// The error's name in reply-safe letters.
fn error_name(error: FlashError) -> &'static [u8] {
    match error {
        FlashError::Taken => b"TAKEN",
        FlashError::UnsupportedClock => b"CLOCK",
        FlashError::UnsupportedMode => b"MODE",
        FlashError::RamCodeMissing => b"RAMCODE",
        FlashError::OutOfRange => b"RANGE",
        FlashError::NotBlank => b"NOTBLANK",
        FlashError::ModeEntry => b"ENTRY",
        FlashError::ProtectError => b"PROTECT",
        FlashError::ModeExit => b"EXIT",
        FlashError::Timeout => b"TIMEOUT",
        FlashError::ProgramFailed => b"PROGRAM",
        FlashError::EraseFailed => b"ERASE",
        FlashError::BlankCheckFailed => b"BLANKCHECK",
    }
}
