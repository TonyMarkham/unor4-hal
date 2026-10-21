//! The data flash driver: one owner, blocking P/E sessions.

use crate::{
    clock::Clocks,
    dflash::{
        FlashError, FlashResult,
        layout::{BLOCK_SIZE, READ_BASE, SIZE, block_offset, check_range, pe_address},
        sequencer::{
            self, BCERR0, BLANK_CHECK_ERRORS, DFLCTL, ERASE_ERRORS, FASR, FCR, FCR_BLANK_CHECK,
            FCR_ERASE, FCR_PROGRAM, FEARH, FEARL, FENTRYR, FENTRYR_DF_PE_MODE, FISR, FPMCR,
            FPMCR_DF_PE, FPSR, FRDY, FRESETR, FSARH, FSARL, FSTATR00, FSTATR1, FSTATR2, FWBL0,
            PCKA_MASK, PERR, PROGRAM_ERRORS,
        },
        timing::{
            BLANK_CHECK_MAX_US_PER_BYTE, ERASE_BLOCK_MAX_US, MODE_EXIT_POLLS, PROGRAM_MAX_US,
            T_DSTOP_US, T_MS_US, pcka, poll_limit, spin_count,
        },
    },
};

use core::sync::atomic::{AtomicBool, Ordering};

/// Set once [`DataFlash::take`] has handed out the driver.
static TAKEN: AtomicBool = AtomicBool::new(false);

/// The RA4M1's 8 KB data flash: plain reads, and blocking program,
/// erase and blank-check sessions.
///
/// Every mutating method is one complete P/E session — enter data flash
/// P/E mode, run the commands, return to read mode — so between calls
/// the data flash is always readable, and `&mut self` keeps a read from
/// overlapping a session.
///
/// Programming rule (manual §44.15.3; datasheet Table 2.72 Note 1): a
/// byte may be programmed **once per erase** of its block. Erased state
/// is established with [`blank_check`](Self::blank_check), never by
/// reading for `FFh`: the manual does not guarantee what an erased,
/// unwritten byte reads as (§44.7 Table 44.8, blank check row).
pub struct DataFlash {
    /// ICLK, for the poll and spin bounds.
    sysclk_hz: u32,
}

impl DataFlash {
    /// Claims the data flash. Succeeds once per boot.
    ///
    /// Checks, before touching the sequencer: the flash clock can
    /// program and erase ([`FlashError::UnsupportedClock`]), the MCU is
    /// in high-speed mode ([`FlashError::UnsupportedMode`]), and the
    /// mode-transition helpers were linked into SRAM
    /// ([`FlashError::RamCodeMissing`]). Then:
    ///
    /// 1. `DFLCTL.DFLEN = 1` if it is not already, then tDSTOP (manual
    ///    §44.4.3: required before the first read or P/E entry; `DFLEN`
    ///    resets to 0);
    /// 2. one P/E session that sets `FISR.PCKA` from FCLK,
    ///    read-modify-write so `FISR.SAS` is kept (FSP
    ///    `r_flash_lp_set_fisr`).
    pub fn take(clocks: &Clocks) -> FlashResult<Self> {
        let pcka = pcka(clocks.fclk()).ok_or(FlashError::UnsupportedClock)?;
        if !high_speed_mode() {
            return Err(FlashError::UnsupportedMode);
        }
        if !sequencer::helpers_in_sram() {
            return Err(FlashError::RamCodeMissing);
        }
        if TAKEN.swap(true, Ordering::AcqRel) {
            return Err(FlashError::Taken);
        }

        let mut flash = Self {
            sysclk_hz: clocks.sysclk(),
        };
        if sequencer::read8(DFLCTL) & 1 == 0 {
            sequencer::write8(DFLCTL, 1);
            cortex_m::asm::delay(spin_count(T_DSTOP_US, flash.sysclk_hz));
        }
        let fisr = flash.session(|_| {
            let fisr = sequencer::read32(FISR);
            sequencer::write32(FISR, (fisr & !PCKA_MASK) | u32::from(pcka));
            Ok(())
        });
        match fisr {
            Ok(()) => Ok(flash),
            Err(error) => {
                TAKEN.store(false, Ordering::Release);
                Err(error)
            }
        }
    }

    /// Copies `buf.len()` bytes starting at `offset` out of the data
    /// flash: plain loads from `4010 0000h + offset` (manual §44.14.2:
    /// no settings beyond `DFLEN`, which [`take`](Self::take) set).
    ///
    /// Only bytes programmed since their block's last erase have a
    /// defined value (see the type docs).
    pub fn read(&self, offset: usize, buf: &mut [u8]) -> FlashResult<()> {
        check_range(offset, buf.len())?;
        let base = (READ_BASE as usize + offset) as *const u8;
        for (index, byte) in buf.iter_mut().enumerate() {
            *byte = unsafe { base.add(index).read_volatile() };
        }
        Ok(())
    }

    /// Whether all `len` bytes at `offset` are blank (erased, not
    /// programmed since): the sequencer's blank check command over the
    /// range, `FSTATR00.BCERR0` clear. An empty range is blank.
    pub fn blank_check(&mut self, offset: usize, len: usize) -> FlashResult<bool> {
        check_range(offset, len)?;
        if len == 0 {
            return Ok(true);
        }
        self.session(|flash| flash.blank_check_in_session(offset, len))
    }

    /// Programs `data` at `offset`, one byte per program command.
    ///
    /// The whole range is blank-checked first, in the same session: if
    /// any byte is not blank the call returns [`FlashError::NotBlank`]
    /// and programs nothing. Programming `FFh` still consumes the byte
    /// for this erase cycle (§44.15.3).
    pub fn write(&mut self, offset: usize, data: &[u8]) -> FlashResult<()> {
        check_range(offset, data.len())?;
        if data.is_empty() {
            return Ok(());
        }
        self.session(|flash| {
            if !flash.blank_check_in_session(offset, data.len())? {
                return Err(FlashError::NotBlank);
            }
            let limit = poll_limit(u64::from(PROGRAM_MAX_US), flash.sysclk_hz);
            for (index, &byte) in data.iter().enumerate() {
                let address = pe_address(offset + index);
                sequencer::write8(FASR, 0);
                sequencer::write16(FSARH, (address >> 16) as u16);
                sequencer::write16(FSARL, address as u16);
                sequencer::write32(FWBL0, u32::from(byte));
                flash.run(
                    FCR_PROGRAM,
                    limit,
                    PROGRAM_ERRORS,
                    FlashError::ProgramFailed,
                )?;
            }
            Ok(())
        })
    }

    /// Erases block `block` (0–7, 1 KB each, at offset `block × 1024`).
    pub fn erase_block(&mut self, block: usize) -> FlashResult<()> {
        let offset = block_offset(block)?;
        self.session(|flash| flash.erase_in_session(offset, BLOCK_SIZE))
    }

    /// Erases all eight blocks, as one erase command over the whole
    /// data area (FSP `r_flash_lp_df_erase` with `num_blocks = 8`).
    pub fn erase_all(&mut self) -> FlashResult<()> {
        self.session(|flash| flash.erase_in_session(0, SIZE))
    }

    /// One P/E session: enter, run `op`, and always return to read mode.
    /// A failed command has already reset the sequencer (see
    /// [`run`](Self::run)); the first error wins.
    fn session<T>(&mut self, op: impl FnOnce(&mut Self) -> FlashResult<T>) -> FlashResult<T> {
        let result = self.enter().and_then(|()| op(self));
        let exit = self.exit();
        match result {
            Ok(value) => exit.map(|()| value),
            Err(error) => Err(error),
        }
    }

    /// Read mode → data flash P/E mode. The SRAM helper runs with
    /// interrupts masked, so the `FPMCR` unlock sequence cannot be
    /// interleaved; `FPSR.PERR` then proves it was not broken.
    fn enter(&mut self) -> FlashResult<()> {
        let spin = spin_count(T_DSTOP_US, self.sysclk_hz);
        cortex_m::interrupt::free(|_| unsafe { sequencer::enter_pe(spin) });
        if sequencer::read32(FPSR) & PERR != 0 {
            return Err(FlashError::ProtectError);
        }
        if sequencer::read16(FENTRYR) != FENTRYR_DF_PE_MODE
            || sequencer::read8(FPMCR) != FPMCR_DF_PE
        {
            return Err(FlashError::ModeEntry);
        }
        Ok(())
    }

    /// Data flash P/E mode → read mode (same interrupt rule as
    /// [`enter`](Self::enter)).
    fn exit(&mut self) -> FlashResult<()> {
        let spin = spin_count(T_MS_US, self.sysclk_hz);
        let left =
            cortex_m::interrupt::free(|_| unsafe { sequencer::exit_pe(spin, MODE_EXIT_POLLS) });
        if left == 0 {
            return Err(FlashError::ModeExit);
        }
        Ok(())
    }

    /// Starts `command` (start/end/data registers already set) and waits
    /// for it: `FRDY` rises → `FCR = 0` → `FRDY` falls, each within
    /// `limit` polls; then `FSTATR2 & errors` must be clear (FSP
    /// `r_flash_lp_command_finish` + `r_flash_lp_wait_for_ready`). A
    /// timeout or error resets the sequencer (`FRESETR` 1 → 0, FSP
    /// `r_flash_lp_reset`) before returning. Polling runs from code
    /// flash with interrupts enabled: code flash stays readable while
    /// the data flash is programmed or erased (manual §44.13, BGO).
    fn run(&mut self, command: u8, limit: u32, errors: u32, failed: FlashError) -> FlashResult<()> {
        sequencer::write8(FCR, command);
        let mut outcome = Err(FlashError::Timeout);
        if wait_frdy(true, limit) {
            sequencer::write8(FCR, 0);
            if wait_frdy(false, limit) {
                outcome = if sequencer::read32(FSTATR2) & errors != 0 {
                    Err(failed)
                } else {
                    Ok(())
                };
            }
        }
        if outcome.is_err() {
            sequencer::write32(FRESETR, 1);
            sequencer::write32(FRESETR, 0);
        }
        outcome
    }

    /// Sets `FASR`/`FSAR`/`FEAR` for `len` bytes at `offset` — `FEAR` is
    /// the inclusive last byte (FSP `r_flash_lp_process_command`).
    fn set_range(offset: usize, len: usize) {
        let start = pe_address(offset);
        let end = pe_address(offset + len - 1);
        sequencer::write8(FASR, 0);
        sequencer::write16(FSARH, (start >> 16) as u16);
        sequencer::write16(FSARL, start as u16);
        sequencer::write32(FEARH, end >> 16);
        sequencer::write16(FEARL, end as u16);
    }

    /// The blank check command, inside an open session. `len > 0`.
    fn blank_check_in_session(&mut self, offset: usize, len: usize) -> FlashResult<bool> {
        Self::set_range(offset, len);
        let max_us = u64::from(BLANK_CHECK_MAX_US_PER_BYTE) * len as u64;
        let limit = poll_limit(max_us, self.sysclk_hz);
        self.run(
            FCR_BLANK_CHECK,
            limit,
            BLANK_CHECK_ERRORS,
            FlashError::BlankCheckFailed,
        )?;
        Ok(sequencer::read32(FSTATR00) & BCERR0 == 0)
    }

    /// The erase command over whole blocks, inside an open session.
    fn erase_in_session(&mut self, offset: usize, len: usize) -> FlashResult<()> {
        Self::set_range(offset, len);
        let blocks = (len / BLOCK_SIZE) as u64;
        let limit = poll_limit(u64::from(ERASE_BLOCK_MAX_US) * blocks, self.sysclk_hz);
        self.run(FCR_ERASE, limit, ERASE_ERRORS, FlashError::EraseFailed)
    }
}

/// Polls `FSTATR1.FRDY` until it equals `ready`, at most `limit` times.
fn wait_frdy(ready: bool, limit: u32) -> bool {
    for _ in 0..limit {
        if (sequencer::read32(FSTATR1) & FRDY != 0) == ready {
            return true;
        }
    }
    false
}

/// Whether `SYSTEM.OPCCR.OPCM` selects high-speed mode (`00b`), the
/// mode `clock::init` sets and the only one this driver's timings and
/// `FPMCR` value cover (manual §10.2.6 `OPCCR`; FSP adds `LVPE` for
/// any other mode).
fn high_speed_mode() -> bool {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    system.opccr.read().opcm().is_00()
}
