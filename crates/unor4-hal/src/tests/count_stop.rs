//! Host tests for the count stop: which targets
//! `StepGenerator::stop_at` arms (`check_count_target`), and the counter
//! events it uses. The hardware stop itself is target code, proven on the
//! board (Validation.md, `S11 countstop`).

use crate::{
    elc::ElcEvent,
    intc::Event,
    pwm::GptChannel,
    step::{StepError, check_count_target, count_compare_value},
};
use ra4m1::{GPT162, GPT163, GPT164, GPT165, GPT166, GPT167, GPT320, GPT321};

/// A 16-bit counter's wrap mask.
const MASK16: u32 = 0xFFFF;
/// A 32-bit counter's wrap mask.
const MASK32: u32 = u32::MAX;

#[test]
fn given_a_stopped_generator_when_the_next_pulse_is_the_target_then_it_arms() {
    // given / when: no pulse can end while it arms
    let result = check_count_target(101, 100, true, MASK16);

    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_pulses_may_run_when_the_next_pulse_is_the_target_then_it_is_refused() {
    // given / when: one pulse can end while it arms
    let result = check_count_target(101, 100, false, MASK16);

    // then
    assert_eq!(result, Err(StepError::CountNotAhead));
}

#[test]
fn given_pulses_may_run_when_the_target_is_two_ahead_then_it_arms() {
    // given / when
    let result = check_count_target(102, 100, false, MASK16);

    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_the_target_equals_the_count_when_checked_then_it_is_refused() {
    // given / when
    let result = check_count_target(100, 100, true, MASK32);

    // then
    assert_eq!(result, Err(StepError::CountNotAhead));
}

#[test]
fn given_a_target_already_passed_when_checked_then_it_is_refused_not_armed_a_wrap_later() {
    // given / when: 5 behind, which wraps to 2^32 - 5 ahead
    let result = check_count_target(95, 100, true, MASK32);

    // then
    assert_eq!(result, Err(StepError::CountNotAhead));
}

#[test]
fn given_a_16bit_counter_when_the_target_is_65535_ahead_then_it_arms() {
    // given / when: the farthest a 16-bit compare reaches first time
    let result = check_count_target(100 + 65_535, 100, false, MASK16);

    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_a_16bit_counter_when_the_target_is_65536_ahead_then_it_is_too_far() {
    // given / when: the compare would match 65 536 pulses early
    let result = check_count_target(100 + 65_536, 100, false, MASK16);

    // then
    assert_eq!(result, Err(StepError::CountTooFar));
}

#[test]
fn given_a_32bit_counter_when_the_target_is_2_pow_31_minus_1_ahead_then_it_arms() {
    // given / when
    let result = check_count_target(100u32.wrapping_add(i32::MAX as u32), 100, false, MASK32);

    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_a_target_across_the_counter_wrap_when_checked_then_it_arms() {
    // given / when: the count is about to wrap past 2^32
    let result = check_count_target(10, u32::MAX - 10, false, MASK32);

    // then
    assert_eq!(result, Ok(()));
}

#[test]
fn given_a_target_when_the_compare_value_is_taken_then_it_is_the_count_before_it() {
    // given / when / then: the match fires on the count event leaving it
    assert_eq!(count_compare_value(500, MASK32), 499);
}

#[test]
fn given_a_16bit_counter_when_the_compare_value_is_taken_then_it_wraps_to_16_bits() {
    // given / when / then: 70 000 - 1 = 69 999, which is 4 463 on 16 bits
    assert_eq!(count_compare_value(70_000, MASK16), 4_463);
}

#[test]
fn given_a_target_of_0_when_the_compare_value_is_taken_then_it_wraps_to_the_top() {
    // given / when / then: the count wraps from the top to 0
    assert_eq!(
        (
            count_compare_value(0, MASK16),
            count_compare_value(0, MASK32)
        ),
        (0xFFFF, u32::MAX)
    );
}

#[test]
fn given_every_gpt_channel_when_its_compare_b_events_are_read_then_icu_and_elc_codes_are_058h_plus_8n()
 {
    // given: manual Table 13.4 / 18.3, GPTn_CCMPB = 058h + 8n
    let channels: [(u8, Event, ElcEvent); 8] = [
        (
            GPT320::CHANNEL,
            GPT320::COMPARE_B_EVENT,
            GPT320::ELC_COMPARE_B,
        ),
        (
            GPT321::CHANNEL,
            GPT321::COMPARE_B_EVENT,
            GPT321::ELC_COMPARE_B,
        ),
        (
            GPT162::CHANNEL,
            GPT162::COMPARE_B_EVENT,
            GPT162::ELC_COMPARE_B,
        ),
        (
            GPT163::CHANNEL,
            GPT163::COMPARE_B_EVENT,
            GPT163::ELC_COMPARE_B,
        ),
        (
            GPT164::CHANNEL,
            GPT164::COMPARE_B_EVENT,
            GPT164::ELC_COMPARE_B,
        ),
        (
            GPT165::CHANNEL,
            GPT165::COMPARE_B_EVENT,
            GPT165::ELC_COMPARE_B,
        ),
        (
            GPT166::CHANNEL,
            GPT166::COMPARE_B_EVENT,
            GPT166::ELC_COMPARE_B,
        ),
        (
            GPT167::CHANNEL,
            GPT167::COMPARE_B_EVENT,
            GPT167::ELC_COMPARE_B,
        ),
    ];

    // when / then
    for (channel, icu, elc) in channels {
        let code = 0x58 + 8 * u32::from(channel);
        assert_eq!(icu as u32, code, "GPT channel {channel} ICU event");
        assert_eq!(
            u32::from(elc.code()),
            code,
            "GPT channel {channel} ELC event"
        );
    }
}
