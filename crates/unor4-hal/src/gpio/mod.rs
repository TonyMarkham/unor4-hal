//! Typestate GPIO for the RA4M1.
//!
//! A pin is [`Pin<ID, MODE>`](Pin). `ID` is its Arduino header
//! ([`pin_map::D6`]) or a board-internal pin ([`pin_map::UsbMux`]); `MODE`
//! is its current state ([`Input`], [`Output`], …). What a pin can do is a
//! set of capability traits on `ID` ([`Gtioc`], [`AdcPin`], [`SciPin`], …),
//! hard-coded in one table ([`pin_map`]) from the hardware manual. Drivers
//! validate their pins by bounding on those traits, so a wrong pin fails
//! `cargo check` with a message naming it: `` `A0` has no GPT output ``.
//!
//! # Switch inputs: limit switches, buttons, contacts
//!
//! A switch goes between a header pin and GND, with a pull-up (the RA4M1's
//! pins have pull-ups only). [`SwitchPinBuilder`] checks the setup at
//! `cargo check`; [`Switch`] reads it at run time. **Write the setup as a
//! named `const`** (`const LIMIT: SwitchSetup<D3, Nc> = …`): the pull-up and
//! board-load checks are `assert!`s that `cargo check` evaluates only in a
//! named `const`; in a `let` they run, and panic, on the board.
//!
//! | Builder call | What it takes | Refused at `cargo check` |
//! | --- | --- | --- |
//! | `with_pin` | any header ([`pin_map`]) | a board-internal pin |
//! | `with_contact` | [`contact::No`] (normally open: active reads LOW) or [`contact::Nc`] (normally closed: active reads HIGH, and a broken wire reads active, so NC fails safe). No default | a missing contact type |
//! | `with_pull` | [`Pull::Internal`] (10–50 kΩ) or [`Pull::External`]`(ohms)`. No default | a missing pull; `External(0)`; a pull-up too weak for a pull-down the board puts on the pin (WiFi D13: internal refused, external at most 25 kΩ) |
//! | `with_edge`, optional | latch the edge into the active level in hardware, so a press shorter than your poll interval is not missed | a pin without an external interrupt (`IRQn`) |
//! | `with_filter`, optional | a [`SwitchFilter`]: removes pulses under 3 samples (at most 8 µs) on the edge latch. A glitch filter, not a debouncer | a pin without an `IRQn`; a filter without `with_edge` |
//! | `with_wake`, optional | the edge also interrupts the CPU, so it ends a `WFI`: sleep until the switch trips instead of polling. Unmask interrupts ([`crate::intc::enable_interrupts`]) so the HAL acknowledges it | a pin without an `IRQn`; a wake-up without `with_edge` |
//!
//! At run time, in the same builder shape as the setup:
//! - `Switch::new(LIMIT).with_peripherals(p).build()` claims exactly the
//!   setup's pin from the [`Peripherals`](crate::Peripherals) and returns
//!   the switch with the rest of them. The same pin used again (by the
//!   step generator, another switch, or `p.pins.d3` directly) fails to
//!   compile: `` `D3` is already used by another driver ``. With
//!   `with_edge`, two switches need different IRQ numbers; several headers
//!   share one (A3/A5 are both IRQ2, A4/D2 IRQ1, A1/D0 IRQ6), so a clash is
//!   refused at run time with [`SwitchError::IrqInUse`], and the
//!   peripherals come back with the error.
//! - [`Switch::is_active`] reads the pin through the contact type.
//! - [`Switch::activated`] (with `with_edge`) returns a latched press once
//!   and clears it, also after its wake-up interrupt ran (with
//!   `with_wake`).
//! - [`Switch::free`] puts the pin back into the peripherals.
//!
//! Never configure a switch's pin yourself (`p.pins.d3.into_input_pullup()`):
//! that skips the contact type, the pull-up check and the edge latch.
//!
//! Mechanical bounce lasts milliseconds; debounce in firmware (for example,
//! require the same reading on two wake-ups). For a limit switch, check
//! both: `activated()` catches a trip since the last look, `is_active()`
//! says whether it is still pressed.
//!
//! ```no_run
//! use unor4_hal::{
//!     Peripherals,
//!     gpio::{
//!         Pull, Switch, SwitchFilter, SwitchPinBuilder, SwitchSetup,
//!         contact::Nc,
//!         pin_map::{self, D2, D3},
//!     },
//!     intc,
//! };
//!
//! // Two NC limit switches on IRQ pins with different numbers (D3: IRQ0 on
//! // WiFi, IRQ1 on Minima; D2 the other way round).
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
//! let p = Peripherals::take().unwrap();
//! let Ok((mut left, p)) = Switch::new(LEFT).with_peripherals(p).build() else { panic!() };
//! let Ok((mut right, _p)) = Switch::new(RIGHT).with_peripherals(p).build() else { panic!() };
//! unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups
//! loop {
//!     cortex_m::asm::wfi(); // asleep until a switch trips (or another interrupt)
//!     let left_hit = left.activated() || left.is_active();
//!     let right_hit = right.activated() || right.is_active();
//!     if left_hit || right_hit {
//!         // your policy: stop the motor, back off, home …
//!     }
//! }
//! ```
//!
//! Register-level facts (Renesas RA4M1 hardware manual, I/O ports chapter)
//! live in the `pin` module.

mod adc_pin;
mod alternate;
mod analog;
mod can_pin;
pub mod contact;
mod dac_pin;
mod gpt_output;
mod gtioc;
mod header_pin;
mod iic_pin;
mod input;
mod input_pull_up;
mod irq_pin;
mod output;
mod output_open_drain;
mod pin;
mod pin_id;
pub mod pin_map;
mod pins;
pub mod psel;
mod pull;
pub mod role;
mod sci_pin;
mod sealed;
mod spi_pin;
mod switch;
mod switch_error;
mod switch_filter;
mod switch_init;
mod switch_pin_builder;
mod switch_setup;

// ---------------------------------------------------------------------------------------------- //

pub use crate::gpio::{
    adc_pin::AdcPin, alternate::Alternate, analog::Analog, can_pin::CanPin, dac_pin::DacPin,
    gpt_output::GptOutput, gtioc::Gtioc, header_pin::HeaderPin, iic_pin::IicPin, input::Input,
    input_pull_up::InputPullUp, irq_pin::IrqPin, output::Output,
    output_open_drain::OutputOpenDrain, pin::Pin, pin_id::PinId, pins::Pins, pull::Pull,
    sci_pin::SciPin, spi_pin::SpiPin, switch::Switch, switch_error::SwitchError,
    switch_filter::SwitchFilter, switch_init::SwitchInit, switch_pin_builder::SwitchPinBuilder,
    switch_setup::SwitchSetup,
};

pub(crate) use crate::gpio::sealed::Sealed;
