//! Every builder's numeric checks and outputs, run on the host.
//!
//! The builders are `const fn`s: in a named `const` a failed `assert!`
//! fails `cargo check`, and called at run time — as here — the same
//! `assert!` panics with the same message. Each refusal test compares the
//! *whole* message, so a garbled or changed message fails. Type-level
//! checks (a pin without a GPT output, a missing slot, a wrong divider) are
//! not run-time code; the `compile_fail` doc tests cover those.
//!
//! Pins are chosen with the same facts on both boards (D2: GPT321, 32-bit;
//! D8: GPT167, 16-bit); WiFi-only board facts are `cfg`-gated.

use crate::{
    gpio::{
        Pull, SwitchFilter, SwitchPinBuilder,
        contact::{Nc, No},
        pin_map::{self, D2, D7},
    },
    step::{
        CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, Div4, Div16, EnaPinBuilder,
        NoDivider, PulPinBuilder, PulSetup, StepGeneratorBuilder, TickBuilder, TickSetup,
    },
};
use embedded_hal::digital::PinState;
use ra4m1::{GPT162, GPT164, GPT165, GPT321};
use std::{
    panic::{UnwindSafe, catch_unwind},
    string::{String, ToString},
};

/// Runs `build`, which must refuse, and returns its whole panic message.
fn refusal(build: impl FnOnce() + UnwindSafe) -> String {
    let payload = catch_unwind(build).expect_err("the builder accepted it");
    if let Some(text) = payload.downcast_ref::<&str>() {
        return text.to_string();
    }
    payload
        .downcast_ref::<String>()
        .cloned()
        .expect("the refusal carries a text message")
}

/// A valid STEP setup on D2 (GPT321), for the cross-checks.
fn pul_d2() -> PulSetup<D2, NoDivider> {
    PulPinBuilder::new()
        .with_pin(pin_map::d2)
        .with_timing(5_000, 2_500)
        .build()
}

/// A valid DIR setup on D7.
fn dir_d7() -> DirSetup<D7> {
    DirPinBuilder::new()
        .with_pin(pin_map::d7)
        .with_setup(5_000)
        .with_forward(PinState::High)
        .build()
}

/// A valid tick setup on GPT164/GPT165.
fn tick() -> TickSetup<GPT164, GPT165> {
    TickBuilder::new().with_timers::<GPT164, GPT165>().build()
}

/// A valid counter setup on GPT162.
fn counter() -> CounterSetup<GPT162> {
    CounterBuilder::new().with_timer::<GPT162>().build()
}

// ---------------------------------------------------------------------------------------------- //
// PulPinBuilder

#[test]
fn given_a_32_bit_pul_pin_when_built_with_5us_low_and_2_5us_high_then_counts_are_240_and_120() {
    // given / when
    let pul = pul_d2();

    // then
    assert_eq!((pul.low_counts(), pul.min_high_counts()), (240, 120));
}

#[test]
fn given_a_16_bit_pul_pin_at_div16_when_built_then_counts_round_up_to_15_and_8() {
    // given / when
    let pul = PulPinBuilder::new()
        .with_pin(pin_map::d8)
        .with_divider(Div16)
        .with_timing(5_000, 2_500)
        .build();

    // then: 5 µs and 2.5 µs at 3 MHz are 15 and 7.5 → 8 counts
    assert_eq!((pul.low_counts(), pul.min_high_counts()), (15, 8));
}

#[test]
fn given_a_stated_width_of_8_when_set_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = PulPinBuilder::new().with_type(8);
    });

    // then
    assert_eq!(message, "with_type: 16 or 32");
}

#[test]
fn given_a_32_bit_pin_stated_as_16_when_built_then_the_width_mismatch_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_type(16)
            .with_timing(5_000, 2_500)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_type: stated width differs from the PUL pin's timer"
    );
}

#[test]
fn given_a_low_time_under_one_count_when_built_then_it_is_refused() {
    // given / when: 10 ns is 0.48 PCLKD counts
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_timing(10, 2_500)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_timing: LOW time under 1 timer count at this divider"
    );
}

#[test]
fn given_a_high_time_under_one_count_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_timing(5_000, 10)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_timing: HIGH time under 1 timer count at this divider"
    );
}

#[test]
fn given_a_low_time_over_the_dir_window_when_built_then_it_is_refused() {
    // given / when: 9 µs is 432 PCLKD counts, over 400
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_timing(9_000, 2_500)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_timing: LOW time over 8.3 us (400 PCLKD counts, the DIR write window)"
    );
}

#[test]
fn given_a_low_time_of_exactly_400_counts_when_built_then_it_is_accepted() {
    // given / when: 400 counts at 48 MHz is 8 333.3 ns; 8 333 ns rounds up to 400
    let pul = PulPinBuilder::new()
        .with_pin(pin_map::d2)
        .with_timing(8_333, 2_500)
        .build();

    // then
    assert_eq!(pul.low_counts(), 400);
}

#[test]
fn given_a_high_time_under_1us_when_built_then_it_is_refused() {
    // given / when: 900 ns is 44 PCLKD counts, under 48
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_timing(5_000, 900)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_timing: minimum HIGH time under 1 us (the tick's DTC write must land first)"
    );
}

#[test]
fn given_a_shape_longer_than_a_16_bit_period_when_built_then_it_is_refused() {
    // given / when: 6 ms HIGH at PCLKD/4 is 72 000 counts, over 65 535
    let message = refusal(|| {
        let _ = PulPinBuilder::new()
            .with_pin(pin_map::d8)
            .with_divider(Div4)
            .with_timing(2_000, 6_000_000)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_timing: the pulse shape is longer than the timer's longest period"
    );
}

// ---------------------------------------------------------------------------------------------- //
// DirPinBuilder

#[test]
fn given_a_5us_dir_setup_when_built_then_it_is_240_counts_at_the_forward_level() {
    // given / when
    let dir = dir_d7();

    // then
    assert_eq!((dir.setup_counts(), dir.forward()), (240, PinState::High));
}

#[test]
fn given_a_zero_dir_setup_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = DirPinBuilder::new()
            .with_pin(pin_map::d7)
            .with_setup(0)
            .with_forward(PinState::High)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_setup: DIR set-up time under 1 PCLKD count (21 ns)"
    );
}

#[test]
fn given_a_1ms_dir_setup_when_built_then_it_is_48_000_counts() {
    // given / when: the longest set-up the builder accepts
    let dir = DirPinBuilder::new()
        .with_pin(pin_map::d7)
        .with_setup(1_000_000)
        .with_forward(PinState::Low)
        .build();

    // then
    assert_eq!(dir.setup_counts(), 48_000);
}

#[test]
fn given_a_dir_setup_over_1ms_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = DirPinBuilder::new()
            .with_pin(pin_map::d7)
            .with_setup(1_000_001)
            .with_forward(PinState::High)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_setup: DIR set-up time over 1 ms (the first pulse after a DIR \
         change must come within the next 2 ms tick)"
    );
}

// ---------------------------------------------------------------------------------------------- //
// TickBuilder

#[test]
fn given_the_same_tick_and_number_timer_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = TickBuilder::new().with_timers::<GPT164, GPT164>().build();
    });

    // then
    assert_eq!(
        message,
        "TickBuilder: the tick and tick-number timers must differ"
    );
}

// ---------------------------------------------------------------------------------------------- //
// StepGeneratorBuilder (the setup composer)

#[test]
fn given_the_counter_on_the_pul_timer_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let counter = CounterBuilder::new().with_timer::<GPT321>().build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_counter(counter)
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(
        message,
        "StepGeneratorBuilder: the counter uses the PUL timer"
    );
}

#[test]
fn given_the_tick_on_the_pul_timer_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let tick = TickBuilder::new().with_timers::<GPT321, GPT165>().build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_counter(counter())
            .with_tick(tick)
            .build();
    });

    // then
    assert_eq!(message, "StepGeneratorBuilder: the tick uses the PUL timer");
}

#[test]
fn given_the_tick_number_on_the_pul_timer_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let tick = TickBuilder::new().with_timers::<GPT164, GPT321>().build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_counter(counter())
            .with_tick(tick)
            .build();
    });

    // then
    assert_eq!(
        message,
        "StepGeneratorBuilder: the tick number uses the PUL timer"
    );
}

#[test]
fn given_the_counter_on_the_tick_timer_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let counter = CounterBuilder::new().with_timer::<GPT164>().build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_counter(counter)
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(
        message,
        "StepGeneratorBuilder: the counter uses the tick timer"
    );
}

#[test]
fn given_the_counter_on_the_tick_number_timer_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let counter = CounterBuilder::new().with_timer::<GPT165>().build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_counter(counter)
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(
        message,
        "StepGeneratorBuilder: the counter uses the tick-number timer"
    );
}

#[test]
fn given_dir_on_the_pul_pin_when_combined_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let dir = DirPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_setup(5_000)
            .with_forward(PinState::High)
            .build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir)
            .with_counter(counter())
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(message, "StepGeneratorBuilder: DIR is on the PUL pin");
}

#[test]
fn given_four_valid_parts_when_combined_then_the_setup_keeps_their_values() {
    // given / when
    let setup = StepGeneratorBuilder::new()
        .with_pul(pul_d2())
        .with_dir(dir_d7())
        .with_counter(counter())
        .with_tick(tick())
        .build();

    // then
    assert_eq!(
        (
            setup.pul.low_counts(),
            setup.pul.min_high_counts(),
            setup.dir.setup_counts(),
            setup.ena.lead_ns()
        ),
        (240, 120, 240, 0)
    );
}

// ---------------------------------------------------------------------------------------------- //
// EnaPinBuilder and StepGeneratorBuilder::with_ena

#[test]
fn given_an_ena_setup_when_built_then_it_keeps_the_level_and_lead() {
    // given / when
    let ena = EnaPinBuilder::new()
        .with_pin(pin_map::d8)
        .with_enabled(PinState::Low)
        .with_lead(200_000_000)
        .build();

    // then
    assert_eq!((ena.enabled(), ena.lead_ns()), (PinState::Low, 200_000_000));
}

#[test]
fn given_ena_on_the_dir_pin_when_added_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let ena = EnaPinBuilder::new()
            .with_pin(pin_map::d7)
            .with_enabled(PinState::Low)
            .with_lead(5_000)
            .build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_ena(ena)
            .with_counter(counter())
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(message, "StepGeneratorBuilder: ENA is on the DIR pin");
}

#[test]
fn given_ena_on_the_pul_pin_when_added_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let ena = EnaPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_enabled(PinState::Low)
            .with_lead(5_000)
            .build();
        let _ = StepGeneratorBuilder::new()
            .with_pul(pul_d2())
            .with_dir(dir_d7())
            .with_ena(ena)
            .with_counter(counter())
            .with_tick(tick())
            .build();
    });

    // then
    assert_eq!(message, "StepGeneratorBuilder: ENA is on the PUL pin");
}

#[test]
fn given_ena_on_its_own_pin_when_added_then_the_setup_carries_it() {
    // given
    let ena = EnaPinBuilder::new()
        .with_pin(pin_map::d8)
        .with_enabled(PinState::High)
        .with_lead(5_000)
        .build();

    // when
    let setup = StepGeneratorBuilder::new()
        .with_pul(pul_d2())
        .with_dir(dir_d7())
        .with_ena(ena)
        .with_counter(counter())
        .with_tick(tick())
        .build();

    // then
    assert_eq!(
        (setup.ena.enabled(), setup.ena.lead_ns()),
        (PinState::High, 5_000)
    );
}

#[cfg(feature = "wifi")]
#[test]
fn given_wifi_d13_held_low_when_ena_enables_on_low_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = EnaPinBuilder::new()
            .with_pin(pin_map::d13)
            .with_enabled(PinState::Low)
            .with_lead(5_000)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_enabled: the board holds this pin LOW while it is not driven (reset, bootloader), \
         so a driver enabled on LOW would run before the firmware does"
    );
}

#[cfg(feature = "wifi")]
#[test]
fn given_wifi_d13_held_low_when_ena_enables_on_high_then_it_is_accepted() {
    // given / when
    let ena = EnaPinBuilder::new()
        .with_pin(pin_map::d13)
        .with_enabled(PinState::High)
        .with_lead(5_000)
        .build();

    // then
    assert_eq!(ena.enabled(), PinState::High);
}

// ---------------------------------------------------------------------------------------------- //
// SwitchPinBuilder

#[test]
fn given_a_switch_with_a_filter_when_built_then_it_keeps_the_pull_and_filter() {
    // given / when
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d2)
        .with_contact(Nc)
        .with_pull(Pull::Internal)
        .with_edge()
        .with_filter(SwitchFilter::PclkbDiv64)
        .build();

    // then: D2 is IRQ1 on WiFi, IRQ0 on Minima
    let irq = if cfg!(feature = "wifi") { 1 } else { 0 };
    assert_eq!(
        (switch.pull(), switch.filter(), switch.edge_irq()),
        (Pull::Internal, Some(SwitchFilter::PclkbDiv64), Some(irq))
    );
}

#[test]
fn given_a_filter_without_the_edge_latch_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = SwitchPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_contact(Nc)
            .with_pull(Pull::Internal)
            .with_filter(SwitchFilter::Pclkb)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_filter: the filter acts on the edge latch; add with_edge()"
    );
}

#[test]
fn given_a_wake_up_without_the_edge_latch_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = SwitchPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_contact(Nc)
            .with_pull(Pull::Internal)
            .with_wake()
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_wake: the wake-up is the edge latch's interrupt; add with_edge()"
    );
}

#[test]
fn given_a_switch_with_the_edge_and_a_wake_up_when_built_then_it_wakes() {
    // given / when
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d3)
        .with_contact(No)
        .with_pull(Pull::Internal)
        .with_edge()
        .with_wake()
        .build();

    // then
    assert!(switch.wakes() && switch.edge_irq().is_some());
}

#[test]
fn given_a_switch_with_only_the_edge_when_built_then_it_does_not_wake() {
    // given / when
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d3)
        .with_contact(No)
        .with_pull(Pull::Internal)
        .with_edge()
        .build();

    // then
    assert!(!switch.wakes());
}

#[test]
fn given_a_switch_without_a_filter_when_built_then_it_has_none() {
    // given / when
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::a0)
        .with_contact(No)
        .with_pull(Pull::External(10_000))
        .build();

    // then
    assert_eq!(
        (switch.pull(), switch.filter()),
        (Pull::External(10_000), None)
    );
}

#[test]
fn given_an_external_pull_up_of_0_ohm_when_built_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = SwitchPinBuilder::new()
            .with_pin(pin_map::d2)
            .with_contact(No)
            .with_pull(Pull::External(0))
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_pull: an external pull-up of 0 Ohm ties the pin to VCC"
    );
}

#[cfg(feature = "wifi")]
#[test]
fn given_wifi_d13_when_the_internal_pull_up_is_chosen_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = SwitchPinBuilder::new()
            .with_pin(pin_map::d13)
            .with_contact(No)
            .with_pull(Pull::Internal)
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_pull: the board pulls this pin down; the internal pull-up (10-50 kOhm) cannot hold \
         it above VIH (0.8 x VCC): use Pull::External"
    );
}

#[cfg(feature = "wifi")]
#[test]
fn given_wifi_d13_when_an_external_pull_up_over_25k_is_chosen_then_it_is_refused() {
    // given / when
    let message = refusal(|| {
        let _ = SwitchPinBuilder::new()
            .with_pin(pin_map::d13)
            .with_contact(No)
            .with_pull(Pull::External(25_001))
            .build();
    });

    // then
    assert_eq!(
        message,
        "with_pull: the external pull-up is too weak for the board's pull-down on this pin: at \
         most a quarter of it (25 kOhm on WiFi D13)"
    );
}

#[cfg(feature = "wifi")]
#[test]
fn given_wifi_d13_when_an_external_pull_up_of_25k_is_chosen_then_it_is_accepted() {
    // given / when: 100 kΩ / (100 kΩ + 25 kΩ) = 0.8 × VCC exactly
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d13)
        .with_contact(No)
        .with_pull(Pull::External(25_000))
        .build();

    // then
    assert_eq!(switch.pull(), Pull::External(25_000));
}

#[cfg(feature = "minima")]
#[test]
fn given_minima_d13_when_the_internal_pull_up_is_chosen_then_it_is_accepted_unverified() {
    // given / when: no Minima schematic in the repo, so no board load is
    // recorded (pin_map::board_pull_down_ohms)
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d13)
        .with_contact(No)
        .with_pull(Pull::Internal)
        .build();

    // then
    assert_eq!(switch.pull(), Pull::Internal);
}

#[test]
fn given_any_header_but_wifi_d13_when_the_internal_pull_up_is_chosen_then_it_is_accepted() {
    // given / when
    let switch = SwitchPinBuilder::new()
        .with_pin(pin_map::d8)
        .with_contact(Nc)
        .with_pull(Pull::Internal)
        .build();

    // then
    assert_eq!(switch.pull(), Pull::Internal);
}
