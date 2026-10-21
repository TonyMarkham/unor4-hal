//! GPT PWM driver (Slice 7) — saw-wave PWM on the header PWM pins,
//! D6 first.
//!
//! Hardware facts, Renesas RA4M1 hardware manual (R01UH0887EJ0100) unless
//! the source is named otherwise:
//! - The GPT is 2× GPT32 (channels 0–1) + 6× GPT16 (channels 2–7)
//!   (§22.1, Table 22.1). All eight channels share one address space,
//!   `0x4007_8000 + 0x0100 × m` with m = 0..7, and (Table 22.4, p.391)
//!   one register layout: `GTWP` at +0x00, `GTCR` at +0x2C,
//!   `GTUDDTYC` at +0x30, `GTIOR` at +0x34, `GTBER` at +0x40,
//!   `GTCCRA` at +0x4C, `GTCCRC` at +0x54, `GTPR` at +0x64. The PAC
//!   splits them into two identically-shaped blocks — `gpt320` for
//!   GPT320/GPT321, `gpt162` for GPT162–GPT167 — with the same field
//!   order and 32-bit register types (checked field-for-field in the
//!   generated sources); [`GptChannel`] retypes them onto the
//!   `gpt320::RegisterBlock` view the way [`crate::uart::SciChannel`]
//!   retypes the SCI blocks.
//! - The **count clock is PCLKD** (§22.1 Table 22.1 "Core clock: PCLKD";
//!   §22.2 Table 22.2 "Count clock: PCLKD…"; §8 Table 8.2 "PCLKD — GPT
//!   count clock"), prescalable per channel by 1/4/16/64/256/1024 via
//!   `GTCR.TPCS[2:0]` (§22.2.12, p.412). Slice 1's clock init leaves
//!   PCLKD at 48 MHz ([`crate::clock::Clocks::pclkd`]).
//! - Saw-wave PWM mode is `GTCR.MD[2:0] = 000b`; `MD`/`TPCS` must be
//!   set while the counter is stopped (`GTCR.CST = 0`, §22.2.12). The
//!   saw period is `(GTPR + 1)` count ticks (§22.2.20, p.426); GPT16's
//!   `GTPR` is 16-bit effective (upper bits read 0, writes ignored).
//! - Output polarity comes from `GTIOR.GTIOx[4:0]` (§22.2.14, p.415;
//!   Table 22.5, p.418): b4 = initial level at count start, b3:b2 = the
//!   transition at the cycle end (overflow), b1:b0 = the transition at
//!   the compare match. This driver uses `0b0_10_01` — initial low,
//!   **high at cycle end, low at compare match** — so the pin is high
//!   from the cycle end through the match and the duty (high fraction)
//!   is the compare value divided by the period. That is also FSP's PWM
//!   convention ("the PWM output signal is high at the beginning of the
//!   cycle and low at the end", FSP r_gpt module docs — behavioral
//!   cross-check only).
//! - **Duty 0% and 100% cannot both come from the compare register**:
//!   when a compare match coincides with the cycle end, the cycle-end
//!   setting (b3:b2) wins in saw-wave PWM mode (Table 22.5 Note 2), so
//!   compare = 0 produces 100%, not 0%. The dedicated mechanism is
//!   `GTUDDTYC.OADTY` (§22.2.13, p.413; §22.3.6, p.465): `10b` forces
//!   duty 0%, `11b` duty 100%, applied at a cycle end while counting.
//!   [`Pwm::set_duty_cycle`] maps duty 0 to `OADTY = 10b` and the top
//!   duty to a compare value beyond `GTPR` (never matches → always
//!   high); everything between goes through the compare register.
//! - Glitch-free updates: `GTBER.CCRA[1:0] = 01b` single-buffers
//!   `GTCCRA` against `GTCCRC`, transferred at the cycle end
//!   (§22.2.17, p.423; §22.3.2.2; Fig 22.29). `OADTY` changes are also
//!   reflected at a cycle end (§22.3.6), so every duty transition is
//!   period-aligned.
//! - Module stop: `MSTPCRD.MSTPD6 = 0` releases GPT162–GPT167,
//!   `MSTPCRD.MSTPD5 = 0` releases GPT320–GPT321 (§10 Low Power Modes,
//!   MSTPCRD, p.175; reset value = all modules stopped). `MSTP*` writes
//!   require `PRCR.PRC1` write-enable, like the SCI module-stop
//!   releases. POEG (`MSTPD14`) is only needed for GTRETRGn trigger
//!   input (Table 22.2 Note 1) — not for plain PWM output.
//! - `GTWP` (write protection, §22.2.1, p.392) resets to 0 — GPT
//!   registers are writable out of reset; unlike `PRCR`/`PWPR` no
//!   unlock sequence is needed. (After a bootloader handoff the driver
//!   still rewrites every register it relies on; it never trusts
//!   inherited state — the bootloader leaves D13's GPT162 running as
//!   PWM, per the Arduino core's `variant.cpp` note.)
//! - Pin functions: every header PWM pin muxes to a `GTIOCnA/nB` output
//!   at `PSEL[4:0] = 00011b` (§19 "I/O Ports": PORT1 Table 19.6/19.7,
//!   PORT3 Table 19.10, PORT4 Table 19.12). D6 — WiFi = P111 →
//!   `GTIOC3A` (GPT163), Minima = P106 → `GTIOC0B` (GPT320). The full
//!   six-pin map, cross-checked entry-for-entry against the Arduino
//!   core's `pinmux.inc` (behavioral reference only), is locked by the
//!   golden test in `src/tests/pwm.rs`.
//!
//! The public trait is `embedded-hal` 1.0's
//! [`embedded_hal::pwm::SetDutyCycle`] (the
//! multi-channel `Pwm` trait is embedded-hal 0.2 legacy and does not
//! exist in 1.0). Duty is `u16`, so the period is capped at 65 535
//! ticks — [`calc_period`] enforces that, which also keeps every
//! reachable period inside GPT16's 16-bit `GTPR`.

mod calc_period;
mod gpt_channel;
mod period_error;
mod period_params;
mod period_result;
mod pwm;

// ---------------------------------------------------------------------------------------------- //

pub use calc_period::calc_period;
pub use gpt_channel::GptChannel;
pub use period_error::PeriodError;
pub use period_params::PeriodParams;
pub use period_result::PeriodResult;
pub use pwm::Pwm;

pub(crate) use gpt_channel::{release_module_stop_gpt16, release_module_stop_gpt32};
