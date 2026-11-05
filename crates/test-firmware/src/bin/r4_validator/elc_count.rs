//! Slice 11 step 1: the ELC routes software event 0 to GPT166's count-up
//! input, and GPT166 must count every event exactly. (GPT321 counted here
//! until Slice 11 step 3 gave it to the step generator's pulse count.)

use crate::{body::Body, requests::argument};

use unor4_hal::{
    elc::{Elc, ElcA, ElcEvent, SoftwareEvent},
    pwm::GptChannel,
};

/// Largest event count the R3 may request: GPT166's counter is 16-bit.
const MAX_EVENTS: u32 = 60_000;
/// Software events fired after the route is removed (the control).
const CONTROL_EVENTS: u32 = 10;

/// The `L<n>` request: GPT166 as an event counter on `ELC_GPTA`
/// (`GTUPSR.USELCA`, manual §22.2.8), `ELC_GPTA` linked to
/// `ELC_SWEVT0`; `n` software events fired; then the route removed and
/// [`CONTROL_EVENTS`] more fired. Reply `L <n> <count> <after>`: `count`
/// must equal `n` and `after` must equal `count`. The counter is stopped
/// and the count source cleared afterwards. `input` is the claimed
/// `ELC_GPTA` (`Peripherals::elc_a`), which `Elc::link` requires.
pub(crate) fn elc_count(req: &[u8], elc: &mut Elc, input: &ElcA, gpt: &ra4m1::GPT166) -> Body {
    let wanted = argument(req, 100).min(MAX_EVENTS);
    ra4m1::GPT166::release_module_stop();
    let regs = gpt.block();
    // Stopped, saw-wave, TPCS = 000b: an event-count start then waits only
    // one PCLKD (§22.3.1.1(4)).
    regs.gtcr.write(|w| unsafe { w.bits(0) });
    regs.gtdnsr.write(|w| unsafe { w.bits(0) });
    regs.gtupsr.write(|w| w.uselca().set_bit());
    regs.gtpr.write(|w| unsafe { w.bits(u32::MAX) });
    regs.gtcnt.write(|w| unsafe { w.bits(0) }); // allowed only while stopped
    elc.link(input, ElcEvent::SoftwareEvent0);
    regs.gtcr.modify(|_, w| w.cst().set_bit());
    for _ in 0..wanted {
        elc.fire(SoftwareEvent::Zero);
    }
    let count = regs.gtcnt.read().bits();
    // Control: with the route removed and the counter still running, more
    // software events must not count — so `count` came through the ELC.
    elc.unlink(input);
    for _ in 0..CONTROL_EVENTS {
        elc.fire(SoftwareEvent::Zero);
    }
    let after = regs.gtcnt.read().bits();
    regs.gtcr.modify(|_, w| w.cst().clear_bit());
    regs.gtupsr.write(|w| unsafe { w.bits(0) });
    let mut body = Body::new(b"L");
    body.num(wanted).num(count).num(after);
    body
}
