//! One-shot wake timer: a wait that ends a `WFI` once, on its own GPT
//! channel, so firmware that sleeps between events needs no polling and
//! no borrowed timer.
//!
//! # How it works
//!
//! - **One shot, in hardware.** The channel counts the wait up from 0 and
//!   overflows. Its overflow reaches its own counter-stop select
//!   (`GTPSR.PSELCx`, manual §22.2.6) through one of the GPT's ELC inputs
//!   (manual §18), so the counter stops itself: it cannot fire twice,
//!   whatever the CPU is doing. The step generator's count stop works the
//!   same way.
//! - **The finest step for each wait.** The prescaler can only change while
//!   the counter is stopped (`GTCR.TPCS`, §22.2.12), and between waits it
//!   is, so each [`WakeTimer::start`] picks the finest step that reaches the
//!   wait and rounds up: never shorter than asked.
//!
//! | Wait up to (16-bit channel) | Step |
//! | --- | --- |
//! | 1.365 ms | 20.8 ns (PCLKD/1) |
//! | 5.46 ms | 83.3 ns (/4) |
//! | 21.8 ms | 0.333 µs (/16) |
//! | 87.4 ms | 1.33 µs (/64) |
//! | 349.5 ms | 5.33 µs (/256) |
//! | 1.398 s | 21.3 µs (/1024) |
//!
//!   A 32-bit channel reaches 89.5 s at 20.8 ns, 25 h at /1024. Longer
//!   waits than [`WakeTimer::longest`]: re-arm in a loop.
//! - **Waking the CPU.** The overflow also raises the timer's interrupt
//!   (one ICU slot), which ends a `WFI`; the HAL acknowledges it. Unmask
//!   interrupts once ([`crate::intc::enable_interrupts`]).
//! - **Reading the end.** [`WakeTimer::expired`] reads the overflow flag
//!   (`GTST.TCFPO`, §22.2.16), which only software clears: it works with
//!   interrupts masked, where the acknowledgement has not run yet.
//!   [`WakeTimer::start`] and [`WakeTimer::cancel`] forget an earlier end,
//!   read or not.
//!
//! # Setup and build
//!
//! A named `const` setup ([`WakeTimerBuilder`]) names the channel and the
//! ELC input; [`WakeTimer::new`] claims both from the
//! [`Peripherals`](crate::Peripherals). Either one another driver holds fails
//! to compile, naming it: the step generator holds ELC inputs B, C and D.
//!
//! ```no_run
//! use core::time::Duration;
//! use unor4_hal::{
//!     Peripherals,
//!     elc::ElcE,
//!     intc,
//!     wake::{WakeSetup, WakeTimer, WakeTimerBuilder},
//! };
//!
//! const WAKE: WakeSetup<ra4m1::GPT167, ElcE> = WakeTimerBuilder::new()
//!     .with_timer::<ra4m1::GPT167>()
//!     .with_input::<ElcE>()
//!     .build();
//!
//! let p = Peripherals::take().unwrap();
//! let Ok((mut wake, _p)) = WakeTimer::new(WAKE).with_peripherals(p).build() else {
//!     panic!("no free interrupt slot");
//! };
//! unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups
//!
//! // Sleep 1 ms. Interrupts masked around the check, so an end landing
//! // between the check and the WFI still ends the WFI (it returns while a
//! // request is pending) instead of being slept through.
//! wake.start(Duration::from_millis(1)).unwrap();
//! loop {
//!     cortex_m::interrupt::disable();
//!     if wake.expired() {
//!         unsafe { cortex_m::interrupt::enable() };
//!         break;
//!     }
//!     cortex_m::asm::wfi();
//!     unsafe { cortex_m::interrupt::enable() }; // the acknowledgement runs here
//! }
//! ```
//!
//! | Call | What it does |
//! | --- | --- |
//! | [`WakeTimer::start`] | starts a wait, replacing one still running; [`WakeError::TooLong`] past [`WakeTimer::longest`]; a zero wait has ended at once |
//! | [`WakeTimer::expired`] | true once after a wait ended |
//! | [`WakeTimer::cancel`] | stops a wait; nothing fires, and an end just before is forgotten |
//! | [`WakeTimer::is_armed`] | a wait is running |
//! | [`WakeTimer::free`] | gives the channel and the input back |

mod wake_clock;
mod wake_error;
mod wake_setup;
mod wake_shot;
mod wake_timer;
mod wake_timer_builder;
mod wake_timer_init;

// ---------------------------------------------------------------------------------------------- //

pub use crate::wake::{
    wake_error::WakeError, wake_setup::WakeSetup, wake_timer::WakeTimer,
    wake_timer_builder::WakeTimerBuilder, wake_timer_init::WakeTimerInit,
};

// ---------------------------------------------------------------------------------------------- //

pub(crate) use crate::wake::{wake_clock::WakeClock, wake_shot::wake_shot};

#[cfg(test)]
pub(crate) use crate::wake::wake_shot::WakeShot;
