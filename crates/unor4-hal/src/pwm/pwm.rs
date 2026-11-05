//! The PWM pin driver: one claimed GPT channel, one GTIOC output.

use crate::{
    clock::Clocks,
    gpio::{GptOutput, Gtioc, Input, Pin},
    pwm::{GptChannel, PeriodParams, calc_period},
};

use core::convert::Infallible;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};
use ra4m1::gpt320::{
    gtior::{GTIOA_A, GTIOB_A},
    gtuddtyc::OADTY_A,
};

/// One claimed GPT channel driving one PWM output on pin `P`.
///
/// `P` is the pin ([`pin_map::D6`](crate::gpio::pin_map), …); which GPT
/// channel it needs, which output (`GTIOCnA`/`GTIOCnB`) and its `PSEL` are
/// its [`Gtioc`] impl in the pin table, so the type is all it takes:
/// `Pwm<D6>`.
///
/// The channel starts running at the end of the constructor
/// (`GTCR.CST = 1`), outputting the reset duty (1 tick high — visually
/// off, and it avoids the compare/cycle-end race a 0% start would hit);
/// take the duty from there with
/// [`SetDutyCycle::set_duty_cycle`]. [`Pwm::set_running`] stops and
/// restarts the counter.
pub struct Pwm<P: Gtioc> {
    _gpt: P::Timer,
    period_counts: u16,
}

impl<P: Gtioc> Pwm<P> {
    /// Claims `pin` and its GPT channel at `freq_hz`.
    ///
    /// `gpt` must be the channel the pin's output belongs to, and the pin
    /// must have a GPT output at all; both are type checks:
    /// `Pwm::new(p.gpt163, p.pins.d6, 1_000, &p.clocks)` on the WiFi board
    /// (D6 = P111 → GTIOC3A on GPT163). Consumes the PAC's channel and the
    /// pin in its reset-default input state (exactly-once claims, like
    /// everything `Peripherals::take` hands out).
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn new(gpt: P::Timer, pin: Pin<P, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        let params = calc_period(clocks.pclkd(), freq_hz)
            .expect("requested PWM frequency is unreachable from PCLKD");
        Self::init(gpt, pin, params)
    }

    /// Bring-up (manual §22.3.3.1 Fig 22.30's saw-wave PWM recipe, with
    /// every register rewritten rather than trusted from a bootloader
    /// handoff):
    ///
    /// 1. module stop released ([`GptChannel::release_module_stop`]);
    /// 2. pin muxed to the GTIOC function (`PSEL = 00011b`, peripheral
    ///    mode on — the PFS sequence Slice 3 proved);
    /// 3. counter stopped: `GTCR` = saw-wave PWM mode, the computed
    ///    prescaler, `CST = 0` (`MD`/`TPCS` are only writable while
    ///    stopped, §22.2.12);
    /// 4. `GTUDDTYC.UD = 1` — up-counting (§22.2.13);
    /// 5. `GTPR` = period − 1 (§22.2.20);
    /// 6. `GTIOR`: polarity `0b0_10_01`, output enabled, and
    ///    `OADFLT = 0` so a stopped counter parks the pin low;
    /// 7. `GTBER`: single-buffered compare (`CCRA/CCRB = 01b` — duty
    ///    writes go to `GTCCRC/GTCCRE` and transfer at the cycle end,
    ///    §22.2.17/§22.3.2.2) **and `BD[1] = 1`** — GPT's `GTPBR` resets
    ///    to `0xFFFF`, so a left-enabled period buffer would load
    ///    `GTPR = 0xFFFF` at the first overflow and silently stretch
    ///    the period;
    /// 8. compare registers preloaded with the reset duty (1 tick) in
    ///    both the active and buffer registers — no first-period glitch;
    /// 9. `GTCR.CST = 1` — counting.
    fn init(gpt: P::Timer, pin: Pin<P, Input>, params: PeriodParams) -> Self {
        P::Timer::release_module_stop();
        let _pin = pin.into_alternate(P::PSEL);

        let regs = gpt.block();
        // Counting stopped while MD/TPCS/GTPR are written (§22.2.12):
        // MD = 000b (saw-wave PWM), TPCS from the calculator, CST = 0.
        regs.gtcr
            .write(|w| unsafe { w.md()._000().tpcs().bits(params.tpcs).cst().clear_bit() });
        // Up-counting; duty-select bits (OADTY/OBDTY) left at 00b —
        // output depends on the compare match. (Reset value 0x1 already
        // has UD = 1; written explicitly so no reset value is trusted.)
        regs.gtuddtyc.write(|w| w.ud().set_bit());
        // The period. GTPBR would reload GTPR with its reset 0xFFFF at
        // the first overflow if its buffer were left enabled — disable
        // the period buffer (BD[1]) since the period never changes.
        regs.gtpr.write(|w| unsafe { w.bits(params.gpr) });
        // Pin function + output enable. Only the claimed output is
        // enabled. GTIOx[4:0] = 0b0_10_01: initial low, HIGH at the cycle
        // end, low at the compare match (Table 22.5) — the pin is high
        // from the cycle end through the match, so the compare value over
        // the period *is* the duty (FSP's r_gpt documents the same
        // convention; behavioral cross-check only).
        match P::OUTPUT {
            GptOutput::B => {
                regs.gtior.write(|w| {
                    w.gtiob()
                        .variant(GTIOB_A::_01001)
                        .obe()
                        .set_bit()
                        .obdflt()
                        .clear_bit()
                });
                // Single buffer: GTCCRE transfers into GTCCRB at the cycle
                // end (§22.2.17: CCRB[1:0] = 01b). BD[1] = 1: GTPR buffer off.
                regs.gtber
                    .write(|w| unsafe { w.ccrb()._01().bd().bits(0b10) });
                // Reset duty (1 tick) preloaded in both buffer and active
                // register — the first period is already the reset duty.
                regs.gtccre.write(|w| unsafe { w.bits(1) });
                regs.gtccrb.write(|w| unsafe { w.bits(1) });
            }
            GptOutput::A => {
                regs.gtior.write(|w| {
                    w.gtioa()
                        .variant(GTIOA_A::_01001)
                        .oae()
                        .set_bit()
                        .oadflt()
                        .clear_bit()
                });
                // Single buffer: GTCCRC transfers into GTCCRA at the cycle
                // end (§22.2.17: CCRA[1:0] = 01b). BD[1] = 1: GTPR buffer off.
                regs.gtber
                    .write(|w| unsafe { w.ccra()._01().bd().bits(0b10) });
                regs.gtccrc.write(|w| unsafe { w.bits(1) });
                regs.gtccra.write(|w| unsafe { w.bits(1) });
            }
        }
        // Counting.
        regs.gtcr.modify(|_, w| w.cst().set_bit());

        Self {
            _gpt: gpt,
            period_counts: params.period_counts() as u16,
        }
    }

    /// The duty in ticks: high from the cycle end through the compare
    /// match. `0` and the full period use the manual's dedicated
    /// mechanisms (see below); everything between is one buffered
    /// compare write.
    ///
    /// - `duty == 0`: forced via `GTUDDTYC.OADTY = 10b` (§22.3.6) — a
    ///   compare match at 0 coincides with the cycle end and the
    ///   cycle-end setting wins in saw-wave PWM mode (Table 22.5
    ///   Note 2), so no compare value produces 0%.
    /// - `duty >= max`: a compare value beyond `GTPR` never matches, so
    ///   the pin stays high from the cycle end onward.
    /// - Both transitions take effect at a period boundary: the compare
    ///   buffer transfers at the cycle end (§22.3.2.2) and `OADTY`
    ///   changes are reflected at an overflow while counting
    ///   (§22.3.6) — no glitches, no partial pulses.
    pub fn set_duty(&mut self, duty: u16) {
        let gpt = self._gpt.block();
        if duty == 0 {
            // Force 0%; OADTYR was left 0 at init, so releasing later
            // restores the GTIOA[3:2] cycle-end behavior.
            gpt.gtuddtyc.modify(|_, w| w.oadty().variant(OADTY_A::_10));
        } else {
            // Back to compare-match control, then hand the buffer the
            // new value (a value ≥ the period never matches → 100%).
            let counts = if duty >= self.period_counts {
                self.period_counts as u32
            } else {
                duty as u32
            };
            gpt.gtuddtyc.modify(|_, w| w.oadty().variant(OADTY_A::_00));
            match P::OUTPUT {
                GptOutput::B => gpt.gtccre.write(|w| unsafe { w.bits(counts) }),
                GptOutput::A => gpt.gtccrc.write(|w| unsafe { w.bits(counts) }),
            }
        }
    }

    /// Starts or stops the counter (`GTCR.CST`). While stopped the pin
    /// parks at its count-stop level (`GTIOR.OADFLT/OBDFLT = 0` — low).
    pub fn set_running(&mut self, running: bool) {
        self._gpt.block().gtcr.modify(|_, w| w.cst().bit(running));
    }

    /// Maximum duty value — the period in ticks.
    pub fn max_duty_cycle(&self) -> u16 {
        self.period_counts
    }
}

impl<P: Gtioc> ErrorType for Pwm<P> {
    type Error = Infallible;
}

impl<P: Gtioc> SetDutyCycle for Pwm<P> {
    fn max_duty_cycle(&self) -> u16 {
        Pwm::<P>::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty(duty);
        Ok(())
    }
}
