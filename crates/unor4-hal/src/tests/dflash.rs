//! Host tests for the data flash module's pure arithmetic: the address
//! map (manual Rev 1.10 §44.2 Tables 44.1/44.3), the `FISR.PCKA`
//! derivation (FSP `r_flash_lp_set_fisr`, LP version 3), and the
//! poll/spin bounds (datasheet Table 2.73 maxima, FSP's loop scheme).

use crate::dflash::{
    BLOCK_COUNT, BLOCK_SIZE, FlashError, READ_BASE, SIZE, block_offset, check_range, pcka,
    pe_address, poll_limit, spin_count,
};

const SYSCLK_48MHZ: u32 = 48_000_000;

#[test]
fn given_manual_map_when_constants_read_then_8kb_in_eight_1kb_blocks() {
    // given: the data flash geometry constants
    // when: they are read
    // then: 8 KB, 1-KB erase blocks, blocks 0-7, read space at
    // 4010 0000h (Table 44.1 / Table 44.3)
    assert_eq!(SIZE, 8 * 1024);
    assert_eq!(BLOCK_SIZE, 1024);
    assert_eq!(BLOCK_COUNT, 8);
    assert_eq!(READ_BASE, 0x4010_0000);
}

#[test]
fn given_offsets_when_pe_address_taken_then_fe00_window_and_read_offset() {
    // given: the first and last data flash bytes and a block start
    // when: their P/E-space addresses are taken
    // then: FE00 0000h-FE00 1FFFh (Table 44.3), and the read-to-P/E
    // offset is FE00 0000h - 4010 0000h = BDF0 0000h, FSP's
    // FLASH_LP_DATAFLASH_ADDR_OFFSET
    assert_eq!(pe_address(0), 0xFE00_0000);
    assert_eq!(pe_address(SIZE - 1), 0xFE00_1FFF);
    assert_eq!(pe_address(7 * BLOCK_SIZE), 0xFE00_1C00);
    assert_eq!(pe_address(0x123) - (READ_BASE + 0x123), 0xBDF0_0000);
}

#[test]
fn given_ranges_inside_when_checked_then_accepted() {
    // given: ranges that end at or before the last byte
    // when: they are range-checked
    // then: all pass, including the full area and empty ranges at
    // either end
    assert_eq!(check_range(0, SIZE), Ok(()));
    assert_eq!(check_range(SIZE - 1, 1), Ok(()));
    assert_eq!(check_range(0, 0), Ok(()));
    assert_eq!(check_range(SIZE, 0), Ok(()));
}

#[test]
fn given_ranges_outside_when_checked_then_out_of_range() {
    // given: ranges that reach past the data area, including one whose
    // end overflows usize
    // when: they are range-checked
    // then: every one is refused, so no P/E address outside
    // FE00 0000h-FE00 1FFFh is ever formed
    assert_eq!(check_range(SIZE - 1, 2), Err(FlashError::OutOfRange));
    assert_eq!(check_range(SIZE, 1), Err(FlashError::OutOfRange));
    assert_eq!(check_range(SIZE + 1, 0), Err(FlashError::OutOfRange));
    assert_eq!(check_range(usize::MAX, 2), Err(FlashError::OutOfRange));
}

#[test]
fn given_block_indices_when_offset_taken_then_0_to_7_only() {
    // given: the valid block indices and the first invalid one
    // when: their offsets are taken
    // then: block n starts at n x 1024; block 8 does not exist
    assert_eq!(block_offset(0), Ok(0));
    assert_eq!(block_offset(7), Ok(0x1C00));
    assert_eq!(block_offset(8), Err(FlashError::OutOfRange));
}

#[test]
fn given_this_crates_fclk_when_pcka_derived_then_23() {
    // given: FCLK = 24 MHz (HOCO 48 MHz / 2, clock::init)
    // when: FISR.PCKA is derived
    // then: (24 - 1) & 0x1F = 23
    assert_eq!(pcka(24_000_000), Some(23));
}

#[test]
fn given_fclk_limits_when_pcka_derived_then_only_programmable_clocks() {
    // given: the FCLK boundary cases of datasheet Table 2.17 / 2.73
    // when: FISR.PCKA is derived
    // then: 1/2/3 MHz exact and 4-32 MHz are accepted (non-integer MHz
    // rounds up, as FSP does); 1.5 MHz, 0 and above 32 MHz are refused
    assert_eq!(pcka(1_000_000), Some(0));
    assert_eq!(pcka(3_000_000), Some(2));
    assert_eq!(pcka(4_000_000), Some(3));
    assert_eq!(pcka(4_500_000), Some(4));
    assert_eq!(pcka(32_000_000), Some(31));
    assert_eq!(pcka(1_500_000), None);
    assert_eq!(pcka(0), None);
    assert_eq!(pcka(32_000_001), None);
}

#[test]
fn given_48mhz_when_poll_limits_computed_then_fsp_scheme() {
    // given: ICLK = 48 MHz and the driver's worst-case bounds
    // when: poll-loop limits are computed (us x MHz / 6 cycles)
    // then: one program 886 us -> 7088 polls; one block erase
    // 504 ms -> 4 032 000; all eight blocks fit u32 unsaturated
    assert_eq!(poll_limit(886, SYSCLK_48MHZ), 7_088);
    assert_eq!(poll_limit(504_000, SYSCLK_48MHZ), 4_032_000);
    assert_eq!(poll_limit(8 * 504_000, SYSCLK_48MHZ), 32_256_000);
}

#[test]
fn given_degenerate_inputs_when_poll_limit_computed_then_clamped() {
    // given: a zero bound and an absurd one
    // when: poll-loop limits are computed
    // then: never 0 (a 0 limit would never poll) and never wrapped
    assert_eq!(poll_limit(0, SYSCLK_48MHZ), 1);
    assert_eq!(poll_limit(u64::MAX / 64, SYSCLK_48MHZ), u32::MAX);
}

#[test]
fn given_48mhz_when_spin_counts_computed_then_one_cycle_per_pass_bound() {
    // given: ICLK = 48 MHz, tDSTOP = 5 us, tMS = 6 us
    // when: spin-loop pass counts are computed
    // then: 240 and 288 passes (each pass takes more than one cycle,
    // so the wait only errs long); never 0, which would wrap the
    // decrement-then-test loop
    assert_eq!(spin_count(5, SYSCLK_48MHZ), 240);
    assert_eq!(spin_count(6, SYSCLK_48MHZ), 288);
    assert_eq!(spin_count(0, SYSCLK_48MHZ), 1);
}
