//! The claimed Event Link Controller.

use crate::elc::{ElcEvent, ElcInput, GptInput, SoftwareEvent};

/// `PRCR` key + PRC1 write-enable (`MSTP*` registers are PRC1-protected,
/// manual §12; the same unlock the SCI and GPT module-stop releases use).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

/// `ELCR` with `ELCON = 1` (bit 7): every link enabled (§18.2.1).
const ELCR_ELCON: u8 = 1 << 7;
/// `ELSEGRn` with `WI = 0`, `WE = 1` (bit 6): makes `SEG` writable
/// (§18.2.2: `WE` is only written while the access writes `WI = 0`).
const ELSEGR_WE: u8 = 1 << 6;
/// `ELSEGRn` with `WE = 1` and `SEG = 1` (bit 0): raises the event.
const ELSEGR_FIRE: u8 = ELSEGR_WE | 1;

/// The Event Link Controller, claimed exactly once.
///
/// [`Elc::new`] releases module stop, clears every GPT input route (a
/// bootloader jump is not a reset, so inherited links are discarded) and
/// enables linking. Routes then change one input at a time with
/// [`Elc::link`] / [`Elc::unlink`].
pub struct Elc {
    elc: ra4m1::ELC,
}

impl Elc {
    /// Claims the ELC: `MSTPCRC.MSTPC14 = 0`, `ELSR0`–`ELSR7 = 0`, then
    /// `ELCR.ELCON = 1`.
    pub fn new(elc: ra4m1::ELC) -> Self {
        release_module_stop_elc();
        let this = Self { elc };
        for input in [
            GptInput::A,
            GptInput::B,
            GptInput::C,
            GptInput::D,
            GptInput::E,
            GptInput::F,
            GptInput::G,
            GptInput::H,
        ] {
            this.elsr(input).write(|w| unsafe { w.bits(0) });
        }
        this.elc.elcr.write(|w| unsafe { w.bits(ELCR_ELCON) });
        this
    }

    /// Routes `event` to the GPT input `input` (`ELSRn.ELS = event`).
    /// Replaces whatever `input` carried before. `input` is the claimed part
    /// (a field of [`Peripherals`](crate::Peripherals), e.g. `p.elc_a`), so
    /// no driver that claimed it can be disturbed.
    pub fn link<I: ElcInput>(&mut self, input: &I, event: ElcEvent) {
        let _ = input;
        self.link_input(I::INPUT, event);
    }

    /// Disconnects the claimed GPT input `input` (`ELSRn.ELS = 0`, "event
    /// output disabled", §18.2.3).
    pub fn unlink<I: ElcInput>(&mut self, input: &I) {
        let _ = input;
        self.unlink_input(I::INPUT);
    }

    /// [`Elc::link`] by input number, for drivers that hold the claimed
    /// input themselves (the step generator, the wake timer).
    pub(crate) fn link_input(&mut self, input: GptInput, event: ElcEvent) {
        self.elsr(input)
            .write(|w| unsafe { w.bits(u16::from(event.code())) });
    }

    /// [`Elc::unlink`] by input number; see [`Elc::link_input`].
    pub(crate) fn unlink_input(&mut self, input: GptInput) {
        self.elsr(input).write(|w| unsafe { w.bits(0) });
    }

    /// The event currently routed to `input`, as its raw `ELS` code
    /// (`0` = none).
    pub fn linked(&self, input: GptInput) -> u8 {
        self.elsr(input).read().bits() as u8
    }

    /// Raises one software event (§18.2.2): write `WE = 1` with `WI = 0`,
    /// then `SEG = 1` with `WE = 1`.
    pub fn fire(&mut self, event: SoftwareEvent) {
        let register = match event {
            SoftwareEvent::Zero => &self.elc.elsegr0,
            SoftwareEvent::One => &self.elc.elsegr1,
        };
        register.write(|w| unsafe { w.bits(ELSEGR_WE) });
        register.write(|w| unsafe { w.bits(ELSEGR_FIRE) });
    }

    /// The `ELSRn` register behind a GPT input.
    fn elsr(&self, input: GptInput) -> &ra4m1::elc::ELSR {
        match input {
            GptInput::A => &self.elc.elsr0,
            GptInput::B => &self.elc.elsr1,
            GptInput::C => &self.elc.elsr2,
            GptInput::D => &self.elc.elsr3,
            GptInput::E => &self.elc.elsr4,
            GptInput::F => &self.elc.elsr5,
            GptInput::G => &self.elc.elsr6,
            GptInput::H => &self.elc.elsr7,
        }
    }
}

/// Takes the ELC out of module stop (`MSTPCRC.MSTPC14 = 0`; manual §10,
/// MSTPCRC: "b14 MSTPC14 Event Link Controller Module Stop").
fn release_module_stop_elc() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrc.modify(|_, w| w.mstpc14()._0());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}
