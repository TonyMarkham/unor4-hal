//! Step generator (Slice 11) — a stepper-driver STEP/DIR pulse train on the
//! GPT, first built for the cart-pole rig's DM542 (plan: `Slice11Plan.md`,
//! design: `StepGeneratorDesign.md`).
//!
//! The generator drives a stepper driver's STEP (PUL) and DIR inputs, and
//! optionally its enable (ENA). You give it a signed rate in pulses per
//! second; it makes the pulses in hardware, sets DIR from the sign, counts
//! every pulse in hardware, and applies a new rate at the next 2 ms tick.
//! While the rate changes, your loop runs once per tick: read where the
//! motor is, decide the next rate, queue it. At a steady rate the CPU can
//! sleep through ticks (section 2, after the refusals).
//!
//! # 1. The parts and what each supports
//!
//! Every part has a builder. **Write each setup as a named `const`**, with
//! its type spelled out (`const PUL: PulSetup<D4, NoDivider> = …`):
//! - a wrong pin or a missing setting is a type error, reported by
//!   `cargo check` however the builder is written;
//! - a wrong *number* (timing, a pin used twice, a board load) is an
//!   `assert!` in a `const fn`, which `cargo check` evaluates only in a
//!   named `const`. In a `let` or an inline `const { }` block the same
//!   mistake compiles, and panics on the board when it runs.
//!
//! | Part | Builder | What it takes | Refused at `cargo check` |
//! | --- | --- | --- | --- |
//! | STEP (PUL) | [`PulPinBuilder`] | a header with a GPT output, A or B ([`pin_map`](crate::gpio::pin_map)); the LOW time and the minimum HIGH time, ns; optionally the timer width you expect | a pin without a GPT output; LOW or HIGH under one timer count; LOW over 8.3 µs; HIGH under 1 µs; a width that differs. 32-bit timers only for now (WiFi D2–D5, Minima D2/D3/D6/D7/D11/D12): 16-bit STEP is refused until built |
//! | DIR | [`DirPinBuilder`] | any header; the set-up time from a DIR change to the next pulse, ns; the level for positive rates | a board-internal pin; a set-up time of 0 or over 1 ms |
//! | ENA, optional | [`EnaPinBuilder`] | any header; the level that enables your driver (no default); the lead from enable to the first DIR change or pulse, ns | a board-internal pin; a missing level or lead; an enabling level the board itself holds while the pin is undriven (WiFi D13: LOW) |
//! | Pulse counter | [`CounterBuilder`] | any GPT channel; a 16-bit one is extended to 32 bits in software | a type that is not a GPT channel |
//! | Tick | [`TickBuilder`] | two GPT channels: the 2 ms tick and the tick numbers | the same channel twice |
//! | All of it | [`StepGeneratorBuilder`]: `with_pul`, `with_dir`, `with_ena` (optional), `with_counter`, `with_tick` | the parts above | a missing part; a GPT channel used twice; DIR or ENA on the STEP pin; ENA on DIR; a 16-bit STEP timer (not built yet) |
//!
//! Every builder slot without a default must be set, or `build()` names the
//! missing one: `` setup incomplete: `with_lead` not set ``.
//!
//! # 2. Control: what to call, and when
//!
//! | Call | When | What it does |
//! | --- | --- | --- |
//! | `StepGenerator::new(STEPPER).with_peripherals(p).build()` | once | claims exactly the pins and timers `STEPPER` names from the [`Peripherals`](crate::Peripherals) and gives back the rest; the generator starts stopped, STEP HIGH, DIR at the forward level, ENA disabling. The same pin or timer used again by another driver fails to compile, naming it |
//! | [`StepGenerator::enable`], then [`StepGenerator::is_enabled`] | before the first motion (with ENA) | drives ENA to the enabling level; rates are refused with [`StepError::Enabling`] until the lead has passed |
//! | [`StepGenerator::poll_tick`] | once per tick while you change the rate; at a steady rate, whenever the CPU wakes | `Some(TickReport)` for the latest tick: [`TickReport::position`] (signed pulses), [`TickReport::pulses`], [`TickReport::skipped`] (ticks passed over since the previous report) |
//! | [`StepGenerator::queue`] | once per tick, after `poll_tick` | the signed rate for the *next* tick, applied there by the DTC. The sign is the direction: DIR changes only while STEP is HIGH, and the next pulse comes at least the DIR set-up time later; you never write DIR. `0` idles: no pulses, and 1 s later the generator stops itself. From stopped it starts at the next tick |
//! | [`StepGenerator::run`] | to start at once from stopped | starts now instead of at a tick (refused while running). The first pulse comes the minimum HIGH time later, or the DIR set-up time when the direction changed, whichever is longer |
//! | [`StepGenerator::stop`] | any time | stops without cutting a pulse; cancels a queued rate |
//! | [`StepGenerator::position`], [`StepGenerator::pulses`] | any time | the position as of the latest tick; the live hardware count |
//! | [`StepGenerator::set_tick_wake`] | optional | each tick wakes the CPU from `WFI`, so the loop can sleep between ticks |
//! | [`StepGenerator::stop_at`] | before or during a move | stops the train in hardware when [`StepGenerator::pulses`] reaches the count, that pulse complete and none after it |
//! | [`StepGenerator::set_count_wake`] | optional | the count stop wakes the CPU from `WFI` |
//! | [`StepGenerator::stopped_at_count`] | after waking | `true` once after the count stop fired |
//! | [`StepGenerator::cancel_stop_at`] | any time | cancels an armed count stop |
//! | [`StepGenerator::disable`] | while stopped (with ENA) | drives ENA to the disabling level; the motor loses holding torque |
//! | [`StepGenerator::free`] | to take it apart | stops, disables ENA, and puts its pins and timers back into the peripherals |
//!
//! What [`StepGenerator::queue`] can refuse, and what to do:
//! - [`StepError::NotReady`]: the latest tick was not polled yet, a change
//!   is already queued, it is within one tick of `run`/`stop`, or the
//!   schedule is being rebuilt after ticks were missed (it rebuilds at the
//!   first poll early enough in a tick). Try again at the next tick; the
//!   running rate holds meanwhile.
//! - [`StepError::TooLate`]: within 40 µs of the next tick. That tick keeps
//!   the running rate; queue at the next one. [`StepGenerator::since_tick_counts`]
//!   tells how much of the tick is left.
//! - [`StepError::RateTooHigh`]: the period cannot fit the LOW time plus the
//!   minimum HIGH time. Clamp the rate.
//! - [`StepError::Disabled`] / [`StepError::Enabling`]: ENA is not enabled,
//!   or its lead has not passed. Call `enable()` and wait.
//! - [`StepError::ReversalWithCountStop`]: a direction change while pulses
//!   run and a count stop is armed.
//!
//! **Sleeping through ticks.** The pulse train needs no CPU: between rate
//! changes the timer keeps the last rate, so at a steady rate the CPU can
//! sleep through any number of ticks (the tick wake off), or a stall such
//! as a data-flash erase can pass them by:
//! - the next [`StepGenerator::poll_tick`] reports the latest tick, with
//!   [`TickReport::skipped`] counting the ticks passed over, and rebuilds
//!   the schedule from the timer registers at the first poll early enough
//!   in a tick (normally that same poll); [`StepGenerator::queue`] returns
//!   [`StepError::NotReady`] until then;
//! - the position and the pulse count stay right; a change queued for a
//!   tick that was passed over still takes effect, a reversal included,
//!   with DIR kept right; an idle generator still stops 1 s after its last
//!   pulse;
//! - rate changes wait for a poll: each is queued after a polled tick;
//! - a 16-bit counter needs a read ([`StepGenerator::pulses`] or
//!   `poll_tick`) at least every 32 768 pulses, 0.25 s at the highest rate,
//!   or the count it extends to 32 bits in software goes wrong; a 32-bit
//!   counter needs no reads.
//!
//! # 3. A complete example: DM542 with ENA, and two limit switches (WiFi)
//!
//! The parts and the setup are named `const`s, checked by `cargo check`.
//! At run time each builder claims exactly the hardware its setup names
//! from the [`Peripherals`](crate::Peripherals) and gives back the rest.
//!
//! ```no_run
//! # #[cfg(feature = "wifi")] {
//! use embedded_hal::digital::PinState;
//! use ra4m1::{GPT164, GPT165, GPT321};
//! use unor4_hal::{
//!     Peripherals,
//!     gpio::{
//!         Pull, Switch, SwitchFilter, SwitchPinBuilder, SwitchSetup,
//!         contact::Nc,
//!         pin_map::{self, D2, D3, D4, D5, D8},
//!     },
//!     intc,
//!     step::{
//!         CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup,
//!         NoDivider, PulPinBuilder, PulSetup, StepError, StepGenerator, StepGeneratorBuilder,
//!         StepSetup, TickBuilder, TickSetup,
//!     },
//! };
//!
//! // STEP: WiFi D4 is GPT320 output B (32-bit). LOW 5 µs, HIGH at least 2.5 µs.
//! const PUL: PulSetup<D4, NoDivider> =
//!     PulPinBuilder::new().with_pin(pin_map::d4).with_timing(5_000, 2_500).build();
//! // DIR: 5 µs from a DIR change to the next pulse; HIGH for positive rates.
//! const DIR: DirSetup<D5> = DirPinBuilder::new()
//!     .with_pin(pin_map::d5)
//!     .with_setup(5_000)
//!     .with_forward(PinState::High)
//!     .build();
//! // ENA: your driver's values. A DM542T enables on LOW, and its manual asks
//! // for ENA 5 µs ahead of DIR and 200 ms for enabling: the lead is 200 ms.
//! const ENA: EnaSetup<D8> = EnaPinBuilder::new()
//!     .with_pin(pin_map::d8)
//!     .with_enabled(PinState::Low)
//!     .with_lead(200_000_000)
//!     .build();
//! // The pulse counter (32-bit) and the tick timers.
//! const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
//! const TICK: TickSetup<GPT164, GPT165> =
//!     TickBuilder::new().with_timers::<GPT164, GPT165>().build();
//! // The generator: every part by name, cross-checked.
//! const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
//!     StepGeneratorBuilder::new()
//!         .with_pul(PUL)
//!         .with_dir(DIR)
//!         .with_ena(ENA)
//!         .with_counter(COUNTER)
//!         .with_tick(TICK)
//!         .build();
//! // Normally-closed limit switches on IRQ pins with different IRQ numbers,
//! // each trip latched in hardware and waking the CPU.
//! const LEFT: SwitchSetup<D3, Nc> = SwitchPinBuilder::new()
//!     .with_pin(pin_map::d3)
//!     .with_contact(Nc)
//!     .with_pull(Pull::Internal)
//!     .with_edge()
//!     .with_filter(SwitchFilter::PclkbDiv64)
//!     .with_wake()
//!     .build();
//! const RIGHT: SwitchSetup<D2, Nc> = SwitchPinBuilder::new()
//!     .with_pin(pin_map::d2)
//!     .with_contact(Nc)
//!     .with_pull(Pull::Internal)
//!     .with_edge()
//!     .with_filter(SwitchFilter::PclkbDiv64)
//!     .with_wake()
//!     .build();
//!
//! /// Your controller: the next rate, pulses/s, signed.
//! fn next_rate(position: i64) -> i32 {
//!     (-position as i32).clamp(-20_000, 20_000)
//! }
//!
//! let p = Peripherals::take().unwrap();
//! let Ok((mut stepper, p)) = StepGenerator::new(STEPPER).with_peripherals(p).build() else {
//!     panic!("no free interrupt slot");
//! };
//! let Ok((mut left, p)) = Switch::new(LEFT).with_peripherals(p).build() else { panic!() };
//! let Ok((mut right, _p)) = Switch::new(RIGHT).with_peripherals(p).build() else { panic!() };
//!
//! // The CPU sleeps in WFI: each 2 ms tick and each limit trip wakes it.
//! unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups
//! stepper.set_tick_wake(true);
//!
//! stepper.enable();
//! while !stepper.is_enabled() {
//!     cortex_m::asm::wfi(); // the 200 ms lead, asleep between ticks
//! }
//!
//! loop {
//!     cortex_m::asm::wfi(); // asleep until the next tick or a limit trip
//!     let left_tripped = left.activated();
//!     let right_tripped = right.activated();
//!     let Some(tick) = stepper.poll_tick() else {
//!         // A limit tripped between ticks: stop now (your machine's call).
//!         if left_tripped || right_tripped {
//!             stepper.stop();
//!         }
//!         continue;
//!     };
//!     let left_hit = left_tripped || left.is_active();
//!     let right_hit = right_tripped || right.is_active();
//!     let mut rate = next_rate(tick.position);
//!     // Never drive further into a pressed limit; backing off is allowed.
//!     if (left_hit && rate < 0) || (right_hit && rate > 0) {
//!         rate = 0;
//!     }
//!     match stepper.queue(rate) {
//!         Ok(_period_counts) => {}
//!         // Rebuilding after missed ticks, or polled too late in this tick:
//!         // the running rate holds; queue again at the next tick.
//!         Err(StepError::NotReady | StepError::TooLate) => {}
//!         Err(_) => stepper.stop(),
//!     }
//! }
//! # }
//! ```
//!
//! Without ENA, leave out `ENA`, `.with_ena(ENA)` and `enable()`; the setup's
//! type then has six parameters. Never drive a STEP, DIR or ENA pin
//! yourself: once claimed, the plain pin API can no longer reach it.
//!
//! # Hardware facts
//!
//! Renesas RA4M1 User's Manual R01UH0887 Rev 1.10:
//! - **Pin.** The STEP pin's `PSEL` muxes it to its channel's `GTIOCnA` or
//!   `GTIOCnB` (§19 tables; the pin table's `Gtioc` facts).
//! - **Waveform.** Saw-wave PWM mode (`GTCR.MD = 000b`), counting up at
//!   PCLKD/1 for a 32-bit channel; the period is `GTPR + 1` counts
//!   (§22.2.20). `GTIOR.GTIOx[4:0] = 10110b` (Table 22.5) on the STEP
//!   output: the pin starts HIGH when counting starts, goes LOW at the cycle
//!   end (overflow, GTCNT GTPR → 0) and HIGH again at the output's compare
//!   match (`GTCCRA` for A, `GTCCRB` for B). So every period begins with a
//!   LOW pulse of the compare value's counts and the rest is HIGH — a fixed
//!   low time, not a fixed duty. The compare value must stay inside
//!   0 < GTCCRx < GTPR (§22.9.2(5)); the period floor keeps it there.
//! - **Stopped = HIGH.** `GTIOR.xDFLT = 1`, `xHLD = 0`: while the counter is
//!   stopped the pin outputs HIGH (§22.2.14). The cart-pole rig's PNP stage
//!   inverts, so HIGH is "PUL unlit".
//! - **Counter writes.** `GTCNT` may only be written while stopped
//!   (§22.3.1.1(6)) and must satisfy 0 ≤ GTCNT ≤ GTPR (§22.9.3).
//! - **Stop.** Clearing `GTCR.CST` stops at once (synchronised to the count
//!   clock, §22.9.4) — mid-pulse, that would cut the pulse. So
//!   `StepGenerator::stop` first writes the longest period to
//!   `GTPR`/`GTPBR`, after which no pulse can start for ~89 s, lets any
//!   pulse in flight end, and only then clears `CST`. Racing the counter
//!   with a margin instead (the first design) cut 0.65 % of stops at
//!   55 733/s in a release build and 27 % in an unoptimised one (measured
//!   2026-11-04): the read-to-write time depends on how the code was
//!   compiled.
//! - **Count.** The counter channel counts up on ELC input B
//!   (`GTUPSR.USELCB`, §22.2.8; event count, §22.3.1.1(4)) — the STEP
//!   channel's compare match on its output (`GPTn_CCMPA`/`GPTn_CCMPB`), the
//!   end of each pulse, routed to ELC input B — so it counts each pulse when
//!   it *ends*. GPT to GPT through the ELC takes 0 cycles (same PCLKD
//!   domain, Table 18.5); pulses are at least the period floor apart, so
//!   none can merge. The counter wraps at its width; a 16-bit counter is
//!   extended to 32 bits in software at every tick.
//! - **Tick.** The tick channel counts PCLKD/4 (`TPCS = 001b`) with
//!   `GTPR = 23 999`: it overflows every 96 000 PCLKD counts, 2 ms exactly
//!   in clock counts (the HOCO's ±1 % applies in real time). Its overflow
//!   (`GPTn_OVF`, Table 18.3) goes to ELC input C, where the counter
//!   captures its count into `GTCCRA`, the STEP channel captures its own
//!   counter (the phase) into the compare register its output does not use
//!   (`GTICASR.ASELCC` / `GTICBSR.BSELCC`, §22.2.10–§22.2.11), and the
//!   tick-number channel counts the ticks. A pulse end coinciding with the
//!   capture is latched as the count before it (§22.9.5(3)), so it belongs
//!   to the next tick; the counter's `GTST.TCFA` flags each capture
//!   (§22.4.1).
//! - **Latch delay (measured, not from the manual).** A pulse end reaches
//!   the tick latch [`PULSE_END_LATCH_DELAY`] = 2 PCLKD counts after the
//!   STEP channel reaches its compare value, as seen through its own
//!   captured phase. Found on the board 2026-11-03 with STEP on GPT320,
//!   counter GPT321 (Validation.md, Slice 11 step 4): over 27 runs, six of them
//!   sweeping every phase, only the 2-count model predicted every tick.
//!   The manual gives no figure; other channel pairs are re-measured on the
//!   board (design §6).
//!
//! ELC inputs B (the pulse end) and C (the tick) are linked from
//! construction on; nothing else may relink them while a `StepGenerator`
//! exists.

mod calc_step_period;
mod check_count_target;
mod counter_builder;
mod counter_setup;
mod dir_pin_builder;
mod dir_setup;
mod divider;
mod divider_for;
mod dividers;
mod ena;
mod ena_choice;
mod ena_part;
mod ena_pin_builder;
mod ena_setup;
mod first_cycle;
mod hardware;
mod lead_passed;
mod limits;
mod no_divider;
mod no_ena;
mod pending;
mod pul_pin_builder;
mod pul_setup;
mod pulses_in_tick;
mod recovery;
mod schedule;
mod start_lead;
mod step_error;
mod step_generator;
mod step_generator_builder;
mod step_generator_init;
mod step_result;
mod step_setup;
mod step_width;
mod tick_builder;
mod tick_dtc;
mod tick_report;
mod tick_setup;
mod transfer_info;
mod vector_table;

// ---------------------------------------------------------------------------------------------- //

pub use crate::step::{
    calc_step_period::{calc_step_period, ns_to_counts},
    counter_builder::CounterBuilder,
    counter_setup::CounterSetup,
    dir_pin_builder::DirPinBuilder,
    dir_setup::DirSetup,
    divider::Divider,
    divider_for::DividerFor,
    dividers::{Div1, Div4, Div16, Div64, Div256, Div1024},
    ena_choice::EnaChoice,
    ena_part::EnaPart,
    ena_pin_builder::EnaPinBuilder,
    ena_setup::EnaSetup,
    limits::{
        EDGE_MARGIN, MAX_REVERSAL_LOW, PCLKD_HZ, TICK_DIVISOR, TICK_TPCS, TICK_US, counts_ceil,
        counts_floor,
    },
    no_divider::NoDivider,
    no_ena::NoEna,
    pul_pin_builder::PulPinBuilder,
    pul_setup::PulSetup,
    pulses_in_tick::{PULSE_END_LATCH_DELAY, pulses_in_tick},
    schedule::Schedule,
    step_error::StepError,
    step_generator::StepGenerator,
    step_generator_builder::StepGeneratorBuilder,
    step_generator_init::StepGeneratorInit,
    step_result::StepResult,
    step_setup::StepSetup,
    step_width::StepWidth,
    tick_builder::TickBuilder,
    tick_report::TickReport,
    tick_setup::TickSetup,
};

// ---------------------------------------------------------------------------------------------- //

pub(crate) use crate::step::{
    check_count_target::{check_count_target, count_compare_value},
    ena::Ena,
    first_cycle::first_cycle,
    hardware::{GptRegs, Hardware},
    lead_passed::lead_passed,
    pending::Pending,
    recovery::{idle_expired, missed_reversal_new_pulses, resync_schedule},
    start_lead::start_lead,
    tick_dtc::TickDtc,
    transfer_info::{MAX_RECORDS, MRB_CHAIN, TransferInfo, record},
    vector_table::VectorTable,
};

#[cfg(test)]
pub(crate) use crate::step::recovery::RESYNC_SLACK;
