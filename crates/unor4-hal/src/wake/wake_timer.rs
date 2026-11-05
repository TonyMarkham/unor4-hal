//! The one-shot wake timer.

use crate::{
    elc::{ElcInput, GptInput},
    intc::Ielsr,
    peripherals::{Board, Release},
    pwm::GptChannel,
    wake::{WakeClock, WakeError, WakeSetup, wake_shot},
};
use core::time::Duration;

/// `GTCR.CST` (bit 0): the counter runs (manual §22.2.12).
const GTCR_CST: u32 = 1;

/// A one-shot wake timer on one GPT channel: each [`WakeTimer::start`]
/// times one wait and ends a `WFI` once when it is over.
///
/// The channel counts the wait from 0 and overflows; the overflow reaches
/// the channel's own stop input (`GTPSR`) through the setup's ELC input,
/// so the counter stops in hardware, however late the CPU looks. The same
/// overflow raises the timer's interrupt (its own ICU slot), which ends a
/// `WFI`; the HAL acknowledges it, no handler needed. Unmask interrupts
/// ([`crate::intc::enable_interrupts`]) so the acknowledgement runs.
///
/// Built with [`WakeTimer::new`] from a [`WakeSetup`]; see the
/// [module docs](crate::wake) for the masked-sleep loop.
pub struct WakeTimer {
    regs: &'static ra4m1::gpt320::RegisterBlock,
    clock: WakeClock,
    input: GptInput,
    slot: Ielsr,
    /// A zero wait: it ended at once, reported by the next `expired()`.
    zero: bool,
}

impl WakeTimer {
    /// The parts `build` set up, in one place.
    pub(crate) fn from_parts(
        regs: &'static ra4m1::gpt320::RegisterBlock,
        clock: WakeClock,
        input: GptInput,
        slot: Ielsr,
    ) -> Self {
        Self {
            regs,
            clock,
            input,
            slot,
            zero: false,
        }
    }

    /// Starts a wait of `wait`, replacing one still running. The wait is
    /// never shorter than asked: the channel counts in the finest prescaler
    /// step that reaches it (20.8 ns up to 1.365 ms on a 16-bit channel,
    /// 21.3 µs up to 1.398 s), rounded up. An earlier wait's end, read or
    /// not, is forgotten: [`WakeTimer::expired`] reads false until this one
    /// ends. A zero wait has ended at once.
    ///
    /// # Errors
    /// [`WakeError::TooLong`] beyond [`WakeTimer::longest`]; the timer is
    /// then stopped, with nothing pending.
    pub fn start(&mut self, wait: Duration) -> Result<(), WakeError> {
        self.stop();
        self.forget();
        let Some(shot) = wake_shot(wait.as_nanos(), &self.clock)? else {
            self.zero = true;
            return Ok(());
        };
        let regs = self.regs;
        regs.gtcr
            .write(|w| unsafe { w.md()._000().tpcs().bits(shot.tpcs).cst().clear_bit() });
        regs.gtpr.write(|w| unsafe { w.bits(shot.gtpr) });
        regs.gtcnt.write(|w| unsafe { w.bits(0) });
        regs.gtcr.modify(|_, w| w.cst().set_bit());
        Ok(())
    }

    /// True once after a wait ended, then false until another ends. It reads
    /// the channel's overflow flag (`GTST.TCFPO`), which the HAL's interrupt
    /// acknowledgement leaves alone, so it works with interrupts masked:
    /// check it, `WFI` (which returns while the request is pending), then
    /// unmask.
    pub fn expired(&mut self) -> bool {
        if core::mem::take(&mut self.zero) {
            return true;
        }
        if self.regs.gtst.read().tcfpo().bit_is_set() {
            self.forget();
            true
        } else {
            false
        }
    }

    /// Stops a running wait; nothing fires. A wait that ended just before
    /// is forgotten too: [`WakeTimer::expired`] reads false afterwards.
    pub fn cancel(&mut self) {
        self.stop();
        self.forget();
    }

    /// True while a wait runs.
    pub fn is_armed(&self) -> bool {
        self.regs.gtcr.read().bits() & GTCR_CST != 0
    }

    /// The longest wait [`WakeTimer::start`] takes: the whole counter at
    /// PCLKD/1024 (1.398 s on a 16-bit channel).
    pub fn longest(&self) -> Duration {
        Duration::from_nanos(self.clock.longest_nanos)
    }

    /// Takes the timer apart and puts its channel and ELC input back into
    /// `p`, the peripherals [`WakeTimer::new`]'s `build` returned (or what
    /// they became since). It stops the channel, unlinks the input and
    /// returns the interrupt slot. `setup` must be the [`WakeSetup`] it was
    /// built from.
    ///
    /// # Errors
    /// The timer and `p`, untouched, when `setup` names another channel or
    /// input.
    // The error hands both back untouched; `no_std` has no box.
    #[allow(clippy::result_large_err)]
    pub fn free<T, I, X, R1, R2>(
        mut self,
        setup: WakeSetup<T, I>,
        mut p: X,
    ) -> Result<R2, (Self, X)>
    where
        T: GptChannel,
        I: ElcInput,
        X: Board + Release<I, Part = I, Restored = R1>,
        R1: Release<T, Part = T, Restored = R2>,
    {
        let _ = setup;
        if !core::ptr::eq(self.regs, T::PTR) || self.input != I::INPUT {
            return Err((self, p));
        }
        self.cancel();
        self.regs.gtpsr.write(|w| unsafe { w.bits(0) });
        p.elc().unlink_input(self.input);
        self.slot.release();
        // Safety: the timer owned these parts since `build` claimed them
        // from `p`, and the check above proved they are the ones it drives;
        // it is consumed here, so each exists once again, back in `p`.
        unsafe { Ok(p.release(I::steal()).release(T::steal())) }
    }

    /// Stops the counter (it stops itself at a wait's end too).
    fn stop(&self) {
        self.regs.gtcr.modify(|_, w| w.cst().clear_bit());
    }

    /// Forgets an ended wait, on a stopped counter: the overflow flag, the
    /// ICU request and the HAL's record of acknowledging it, and the NVIC's
    /// pending bit, so no stale end reads as a new one and no stray `WFI`
    /// wake follows.
    fn forget(&mut self) {
        self.regs.gtst.modify(|_, w| w.tcfpo().clear_bit());
        let _ = self.slot.take_request();
        self.slot.unpend();
        self.zero = false;
    }
}
