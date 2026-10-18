//! Tests for the delay module's pure logic.

use crate::delay::{MAX_RUN, cycles_for};

/// The resolved system clock (HOCO 48 MHz) every test assumes.
const SYSCLK_HZ: u32 = 48_000_000;

#[test]
fn given_exact_durations_at_48mhz_when_converted_then_cycles_are_exact() {
    // given: durations that divide 48 MHz evenly (1 ns, 1 µs, 1 ms)
    let cases = [
        (1, 1_000_000_000, 1),
        (1, 1_000_000, 48),
        (1, 1_000, 48_000),
    ];

    for &(amount, per_second, expected) in &cases {
        // when: converted to cycles
        let cycles = cycles_for(amount, SYSCLK_HZ, per_second);

        // then: the exact cycle count
        assert_eq!(cycles, expected);
    }
}

#[test]
fn given_sub_cycle_duration_at_48mhz_when_converted_then_ceilings_up() {
    // given: 21 ns — 1.008 cycles at 48 MHz, so not a whole number
    let amount = 21;
    let per_second = 1_000_000_000;

    // when: converted to cycles
    let cycles = cycles_for(amount, SYSCLK_HZ, per_second);

    // then: 2 — a delay is at least the requested duration, never less
    assert_eq!(cycles, 2);
}

#[test]
fn given_u32max_microseconds_at_48mhz_when_converted_then_fits_u64() {
    // given: the largest input the API accepts, delay_us(u32::MAX)
    let amount = u32::MAX as u64;

    // when: converted to cycles at 48 MHz
    let cycles = cycles_for(amount, SYSCLK_HZ, 1_000);

    // then: the exact value, computed without u64 overflow
    assert_eq!(cycles, 206_158_430_160_000);
}

#[test]
fn given_systicks_24_bit_counter_when_chunked_then_max_run_is_2_pow_24() {
    // given: SysTick's 24-bit reload register

    // when: the chunker's per-run bound is chosen
    let max_run = MAX_RUN;

    // then: 2^24 — the largest run where RELOAD = MAX_RUN - 1 still fits
    assert_eq!(max_run, 1 << 24);
}
