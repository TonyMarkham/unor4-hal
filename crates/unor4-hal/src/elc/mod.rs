//! Event Link Controller (Slice 11, step 1) — hardware-to-hardware event
//! routing into the GPT, with no CPU in the path.
//!
//! Hardware facts, Renesas RA4M1 User's Manual R01UH0887 Rev 1.10 (§18
//! unless noted):
//! - The ELC links an event signal from one module to an action in
//!   another. `ELSRn.ELS[7:0]` selects the event for destination `n`
//!   (§18.2.3); `0` disables the link. The event numbers are the same
//!   table the ICU uses (Table 18.3; e.g. `GPT0_CCMPB = 058h`).
//! - Destinations `ELSR0`–`ELSR7` are the GPT's eight ELC inputs
//!   `ELC_GPTA`–`ELC_GPTH` (Table 18.2), shared by all eight GPT channels.
//!   A channel acts on an input through its own select registers — count
//!   up (`GTUPSR.USELCx`, §22.2.8), start/stop/clear (`GTSSR`/`GTPSR`/
//!   `GTCSR`), input capture (`GTICASR`/`GTICBSR`). This module only routes;
//!   the GPT drivers choose the action.
//! - The eight inputs are claimable parts of [`Peripherals`](crate::Peripherals)
//!   ([`ElcA`] … [`ElcH`]): [`Elc::link`] and [`Elc::unlink`] take a claimed
//!   input, so two users of one input fail to compile.
//! - `ELCR.ELCON` enables every link at once (§18.2.1). The ELC starts in
//!   module stop after reset; `MSTPCRC.MSTPC14 = 0` releases it (§18.4.3,
//!   §10), a `PRCR.PRC1`-protected write like the SCI and GPT releases.
//! - Software events: writing `SEG = 1` to `ELSEGR0`/`ELSEGR1` while
//!   `WE = 1` raises `ELC_SWEVT0`/`ELC_SWEVT1` (`053h`/`054h`); `WE` is only
//!   writable with `WI = 0` written in the same access (§18.2.2).
//! - Delay: an event between two modules on the same clock arrives in
//!   0 cycles; across clock domains, 1–2 cycles of the slower clock
//!   (§18.4.4, Table 18.5). GPT to GPT is PCLKD to PCLKD: 0 cycles.
//!
//! Left out on purpose: the non-GPT destinations (`ELSR8`/`9` ADC, `12`
//! DAC, `14`–`17` port groups, `18` CTSU) and the port event groups —
//! nothing in this crate drives them yet.

mod elc;
mod elc_event;
mod elc_input;
mod elc_inputs;
mod gpt_input;
mod software_event;

// ---------------------------------------------------------------------------------------------- //

pub use crate::elc::{
    elc::Elc,
    elc_event::ElcEvent,
    elc_input::ElcInput,
    elc_inputs::{ElcA, ElcB, ElcC, ElcD, ElcE, ElcF, ElcG, ElcH},
    gpt_input::GptInput,
    software_event::SoftwareEvent,
};

#[cfg(test)]
pub(crate) use crate::elc::{elc_event::ALL_ELC_EVENTS, gpt_input::ALL_GPT_INPUTS};
