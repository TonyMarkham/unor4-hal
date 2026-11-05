//! The wake timer's shot: the prescaler and period for each wait.

use crate::wake::{WakeClock, WakeError, WakeShot, wake_shot};

/// PCLKD at 48 MHz (the HAL's HOCO clock).
const PCLKD: u32 = 48_000_000;
/// A 16-bit channel's largest `GTPR`.
const MAX_16: u32 = 0xFFFF;
/// The dividers by prescaler code.
const DIVIDERS: [u128; 6] = [1, 4, 16, 64, 256, 1024];

#[test]
fn given_a_zero_wait_when_shot_then_it_has_ended_at_once() {
    // given / when
    let shot = wake_shot(0, &WakeClock::new(PCLKD, MAX_16));
    // then
    assert_eq!(shot, Ok(None));
}

#[test]
fn given_the_ena_lead_and_the_trip_check_when_shot_then_they_count_at_pclkd() {
    // given: 5 µs (cart-pole's ENA lead) and 1 ms (its trip re-check)
    // when
    let lead = wake_shot(5_000, &WakeClock::new(PCLKD, MAX_16));
    let trip = wake_shot(1_000_000, &WakeClock::new(PCLKD, MAX_16));
    // then: 240 and 48 000 steps of 20.8 ns
    assert_eq!(lead, Ok(Some(WakeShot { tpcs: 0, gtpr: 239 })));
    assert_eq!(
        trip,
        Ok(Some(WakeShot {
            tpcs: 0,
            gtpr: 47_999
        }))
    );
}

#[test]
fn given_the_longest_wait_at_pclkd_when_one_ns_more_then_the_next_prescaler_takes_it() {
    // given: 65 536 steps of 20.8 ns are 1 365 333.3 ns
    // when
    let last = wake_shot(1_365_333, &WakeClock::new(PCLKD, MAX_16));
    let next = wake_shot(1_365_334, &WakeClock::new(PCLKD, MAX_16));
    // then
    assert_eq!(
        last,
        Ok(Some(WakeShot {
            tpcs: 0,
            gtpr: 65_535
        }))
    );
    assert_eq!(
        next,
        Ok(Some(WakeShot {
            tpcs: 1,
            gtpr: 16_384
        }))
    );
}

#[test]
fn given_the_longest_wait_when_one_ns_more_then_it_is_refused() {
    // given: 65 536 steps of 21.3 µs (PCLKD/1024)
    let longest = WakeClock::new(PCLKD, MAX_16).longest_nanos;
    // when
    let last = wake_shot(u128::from(longest), &WakeClock::new(PCLKD, MAX_16));
    let over = wake_shot(u128::from(longest) + 1, &WakeClock::new(PCLKD, MAX_16));
    // then
    assert_eq!(longest, 1_398_101_333);
    assert_eq!(
        last,
        Ok(Some(WakeShot {
            tpcs: 5,
            gtpr: 65_535
        }))
    );
    assert_eq!(over, Err(WakeError::TooLong));
}

#[test]
fn given_any_wait_when_shot_then_it_is_never_short_and_uses_the_finest_step() {
    // given: waits across every prescaler
    let mut wait: u128 = 1;
    while wait <= u128::from(WakeClock::new(PCLKD, MAX_16).longest_nanos) {
        // when
        let shot = wake_shot(wait, &WakeClock::new(PCLKD, MAX_16))
            .unwrap()
            .unwrap();
        // then: the wait counted is at least the one asked
        let divider = DIVIDERS[usize::from(shot.tpcs)];
        let steps = u128::from(shot.gtpr) + 1;
        assert!(
            steps * divider * 1_000_000_000 >= wait * u128::from(PCLKD),
            "wait {wait}"
        );
        // and a finer prescaler could not reach it
        if shot.tpcs > 0 {
            let finer = DIVIDERS[usize::from(shot.tpcs) - 1];
            let finer_steps = (wait * u128::from(PCLKD)).div_ceil(finer * 1_000_000_000);
            assert!(finer_steps > u128::from(MAX_16) + 1, "wait {wait}");
        }
        wait = wait * 3 / 2 + 7;
    }
}

#[test]
fn given_a_32_bit_channel_when_shot_then_it_reaches_89_s_at_pclkd() {
    // given: 89 s, under 2³² steps of 20.8 ns (89.48 s)
    // when
    let shot = wake_shot(89_000_000_000, &WakeClock::new(PCLKD, u32::MAX));
    // then
    assert_eq!(
        shot,
        Ok(Some(WakeShot {
            tpcs: 0,
            gtpr: 4_271_999_999
        }))
    );
}

#[test]
fn given_a_32_bit_channel_when_the_longest_is_asked_then_it_does_not_overflow() {
    // given: 2³² steps of 21.3 µs
    // when
    let longest = WakeClock::new(PCLKD, u32::MAX).longest_nanos;
    let last = wake_shot(u128::from(longest), &WakeClock::new(PCLKD, u32::MAX));
    let over = wake_shot(u128::from(longest) + 1, &WakeClock::new(PCLKD, u32::MAX));
    // then
    assert_eq!(longest, 91_625_968_981_333);
    assert_eq!(
        last,
        Ok(Some(WakeShot {
            tpcs: 5,
            gtpr: u32::MAX
        }))
    );
    assert_eq!(over, Err(WakeError::TooLong));
}

#[test]
fn given_a_32_bit_wait_past_384_s_when_shot_then_the_wide_product_path_matches() {
    // given: 500 s, whose ns × PCLKD overflows u64
    // when
    let shot = wake_shot(500_000_000_000, &WakeClock::new(PCLKD, u32::MAX));
    // then: 24 000 000 000 counts at /1 and 6 000 000 000 at /4 pass 2³²;
    // /16 gives 1 500 000 000 steps
    assert_eq!(
        shot,
        Ok(Some(WakeShot {
            tpcs: 2,
            gtpr: 1_499_999_999
        }))
    );
}
