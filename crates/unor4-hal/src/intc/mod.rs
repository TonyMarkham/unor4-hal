//! Interrupt infrastructure: the ICU/IELSR event system, the NVIC glue,
//! and the dispatch table behind the vector table (Slice 5).
//!
//! Hardware facts, Renesas RA4M1 hardware manual (R01UH0887EJ0100) unless the
//! source is named otherwise:
//! - The ICU register block sits at `0x4000_6000` (SVD `ICU::PTR`;
//!   cross-checked against FSP's `R_ICU_BASE` in the Arduino core's
//!   device header). Its 32 `IELSRn` registers at offset `0x300` each
//!   link one peripheral event to one NVIC interrupt line: **slot `n`
//!   is NVIC IRQ `n`**. The RA4M1 has no separate ICU enable/priority
//!   registers — enabling and prioritizing happen in the Cortex-M4
//!   NVIC. Vendor cross-check: FSP's `bsp_irq.h` enables with
//!   `NVIC->ISER`, sets priority with `NVIC->IP`, and touches the ICU
//!   only to clear the `IR` status flag.
//! - `IELSRn` layout: `IELS` in bits 7..0 = the event code, `IR` at
//!   bit 16 = the sticky request flag (SVD; FSP's
//!   `R_ICU_IELSR_IR_Pos` agrees).
//!   Writing 0 clears `IR` (writing 1 is prohibited — SVD field doc).
//!   The dispatcher clears `IR` at the top of every ISR, before running
//!   the registered handler, cross-checked against FSP's non-FIFO SCI
//!   ISR order. The peripheral handler services the actual source.
//! - Event codes are transcribed into [`Event`] from the manual's ICU
//!   event-code table, cross-checked entry-for-entry against two
//!   independent transcriptions (the `ra-hal` crate's `InterruptEvent`
//!   enum, which cites manual §13/§18, and a community table derived
//!   from the same chapter). Values that both agree on are fact; the
//!   slice 5 hardware run exercises the SCI2/SCI9 RXI/TXI/ERI entries on
//!   the board — until then the table is host-verified only, and the
//!   0xB7–0xDA range (SCE guard) is the only populated entry beyond the
//!   SPI block because the cross-checks stop there.
//! - Vector table: the `ra4m1` PAC's `rt` feature would normally supply
//!   the 32 IRQ vectors, but it is bound to `cortex-m-rt ^0.6`, which
//!   cannot coexist with this workspace's 0.7 runtime (both would emit
//!   `__INTERRUPTS`). Instead this module ships its own 32-entry table
//!   ([`vector_table`]) with dispatch built in. Firmware must build
//!   `cortex-m-rt` with its `device` feature: that swaps out the
//!   runtime's generic 240-entry `DefaultHandler` table for ours and
//!   makes `link.x` `INCLUDE device.x` (supplied empty by the HAL's
//!   build script — no weak aliases are needed). Mechanics per
//!   cortex-m-rt 0.7.7: `EXTERN(__INTERRUPTS)`,
//!   `KEEP(*(.vector_table.interrupts))`.
//! - IELSR interrupts use logical NVIC priority 12, encoded as 0xc0
//!   because the RA4M1 implements four priority bits. They do not nest
//!   each other. Enabling a slot preserves the CPU's global mask.
//! - `Peripherals::take` clears inherited ICU routes and NVIC state.
//!   A bootloader jump is not a reset: duplicate SCI9 routes violate
//!   manual §13.4.2 and caused one-shot interrupts. Main explicitly
//!   calls [`enable_interrupts`] outside critical sections after setup.
//! - Critical sections: [`critical_section`] is implemented by
//!   cortex-m's `critical-section-single-core` provider (PRIMASK
//!   save/disable/restore), enabled via the cargo feature on the
//!   `cortex-m` dependency — audited upstream code, not hand-rolled.
//!   It is ISR-safe: re-entering inside a handler just re-masks.
//!
//! [`critical_section`]: https://docs.rs/critical-section/latest/critical_section/

mod dispatch;
mod enable_interrupts;
mod event;
mod ielsr;
mod init;

// ---------------------------------------------------------------------------------------------- //

pub use dispatch::vector_table;
pub use enable_interrupts::enable_interrupts;
pub use event::Event;
pub use ielsr::Ielsr;

pub(crate) use init::init;

// `register_handler` has no caller until `uart-irq` is on (or the host
// tests run); `call_handler` and `ALL_EVENTS` are test-only.
#[cfg(test)]
pub(crate) use dispatch::call_handler;
#[cfg_attr(all(not(test), not(feature = "uart-irq")), allow(unused_imports))]
pub(crate) use dispatch::register_handler;
#[cfg(test)]
pub(crate) use event::ALL_EVENTS;
