//! The PWM pin driver: one claimed GPT channel, one GTIOC output.

use crate::{
    clock::Clocks,
    gpio::{Input, Pin},
    pwm::{GptChannel, PeriodParams, calc_period},
};

use core::convert::Infallible;
use embedded_hal::pwm::{ErrorType, SetDutyCycle};
use ra4m1::gpt320::{
    gtior::{GTIOA_A, GTIOB_A},
    gtuddtyc::OADTY_A,
};

/// `PSEL` for the GTIOC pin-function group — every header PWM pin muxes
/// to its `GTIOCnA/nB` output at `PSEL[4:0] = 00011b` (manual §19 "I/O
/// Ports": PORT1 Tables 19.6/19.7, PORT3 Table 19.10, PORT4 Table
/// 19.12; cross-checked against the Arduino core's `pinmux.inc`).
const PSEL_GPT: u8 = 0b00011;

/// One claimed GPT channel driving one PWM output.
///
/// `S` is the PAC's peripheral marker for the channel, `PORT`/`PIN` are
/// the physical pin that carries the signal, and `IOCB` selects which
/// compare output the pin is muxed to (`false` = GTIOCnA / `GTCCRA`,
/// `true` = GTIOCnB / `GTCCRB`) — a compile-time fact from the manual's
/// pin-function table, fixed by the constructor.
///
/// The channel starts running at the end of the constructor
/// (`GTCR.CST = 1`), outputting the reset duty (1 tick high — visually
/// off, and it avoids the compare/cycle-end race a 0% start would hit);
/// take the duty from there with
/// [`SetDutyCycle::set_duty_cycle`]. [`Pwm::set_running`] stops and
/// restarts the counter.
pub struct Pwm<S, const PORT: char, const PIN: u8, const IOCB: bool> {
    _gpt: S,
    period_counts: u16,
}

// Per-map constructors. The two boards wire their header PWM pins to
// different chip pins/engines (manual §19 Tables 19.6/19.7/19.10/19.12,
// cross-checked against the Arduino core's `pinmux.inc`), so the
// constructors are
// feature-gated like the pin tables: exactly one board per build.
// `d9` is the exception — P303 is D9 on both maps, so its constructor
// is ungated.

#[cfg(feature = "wifi")]
impl Pwm<ra4m1::GPT321, '1', 5, false> {
    /// Claims header **D3** — WiFi: **P105 → GTIOC1A** on GPT321
    /// (manual §19 Table 19.7, PORT1 (2): `GTIOC1A` at
    /// `PSEL[4:0] = 00011b`). Minima: the same header pin is **P104 →
    /// GTIOC1B** (Table 19.6) — see the Minima `d3`.
    ///
    /// Consumes the PAC's `GPT321` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d3(gpt: ra4m1::GPT321, pin: Pin<'1', 5, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "minima")]
impl Pwm<ra4m1::GPT321, '1', 4, true> {
    /// Claims header **D3** — Minima: **P104 → GTIOC1B** on GPT321
    /// (manual §19 Table 19.6, PORT1 (1): `GTIOC1B` at
    /// `PSEL[4:0] = 00011b`). WiFi: the same header pin is **P105 →
    /// GTIOC1A** (Table 19.7) — see the WiFi `d3`.
    ///
    /// Consumes the PAC's `GPT321` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d3(gpt: ra4m1::GPT321, pin: Pin<'1', 4, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "wifi")]
impl Pwm<ra4m1::GPT320, '1', 7, false> {
    /// Claims header **D5** — WiFi: **P107 → GTIOC0A** on GPT320
    /// (manual §19 Table 19.6, PORT1 (1): `GTIOC0A` at
    /// `PSEL[4:0] = 00011b`). Minima: the same header pin is **P102 →
    /// GTIOC2B** on GPT162 (Table 19.6) — see the Minima `d5`.
    ///
    /// Consumes the PAC's `GPT320` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d5(gpt: ra4m1::GPT320, pin: Pin<'1', 7, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "minima")]
impl Pwm<ra4m1::GPT162, '1', 2, true> {
    /// Claims header **D5** — Minima: **P102 → GTIOC2B** on GPT162
    /// (manual §19 Table 19.6, PORT1 (1): `GTIOC2B` at
    /// `PSEL[4:0] = 00011b`). WiFi: the same header pin is **P107 →
    /// GTIOC0A** on GPT320 (Table 19.6) — see the WiFi `d5`.
    ///
    /// Consumes the PAC's `GPT162` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d5(gpt: ra4m1::GPT162, pin: Pin<'1', 2, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "wifi")]
impl Pwm<ra4m1::GPT163, '1', 11, false> {
    /// Claims header **D6** — WiFi: **P111 → GTIOC3A** on GPT163
    /// (manual §19 Table 19.7, PORT1 (2): `GTIOC3A` at
    /// `PSEL[4:0] = 00011b`). Minima: the same header pin is **P106 →
    /// GTIOC0B** on GPT320 (Table 19.6) — see the Minima `d6`.
    ///
    /// Consumes the PAC's `GPT163` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d6(gpt: ra4m1::GPT163, pin: Pin<'1', 11, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "minima")]
impl Pwm<ra4m1::GPT320, '1', 6, true> {
    /// Claims header **D6** — Minima: **P106 → GTIOC0B** on GPT320
    /// (manual §19 Table 19.6, PORT1 (1): `GTIOC0B` at
    /// `PSEL[4:0] = 00011b`). WiFi: the same header pin is **P111 →
    /// GTIOC3A** on GPT163 (Table 19.7) — see the WiFi `d6`.
    ///
    /// Consumes the PAC's `GPT320` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d6(gpt: ra4m1::GPT320, pin: Pin<'1', 6, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

impl Pwm<ra4m1::GPT167, '3', 3, true> {
    /// Claims header **D9** — **P303 → GTIOC7B** on GPT167, the same
    /// chip pin on both board maps (manual §19 Table 19.10, PORT3:
    /// `GTIOC7B` at `PSEL[4:0] = 00011b`).
    ///
    /// Consumes the PAC's `GPT167` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d9(gpt: ra4m1::GPT167, pin: Pin<'3', 3, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "wifi")]
impl Pwm<ra4m1::GPT162, '1', 3, false> {
    /// Claims header **D10** — WiFi: **P103 → GTIOC2A** on GPT162
    /// (manual §19 Table 19.6, PORT1 (1): `GTIOC2A` at
    /// `PSEL[4:0] = 00011b`). Minima: the same header pin is **P112 →
    /// GTIOC3B** on GPT163 (Table 19.7) — see the Minima `d10`.
    ///
    /// Consumes the PAC's `GPT162` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d10(gpt: ra4m1::GPT162, pin: Pin<'1', 3, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "minima")]
impl Pwm<ra4m1::GPT163, '1', 12, true> {
    /// Claims header **D10** — Minima: **P112 → GTIOC3B** on GPT163
    /// (manual §19 Table 19.7, PORT1 (2): `GTIOC3B` at
    /// `PSEL[4:0] = 00011b`). WiFi: the same header pin is **P103 →
    /// GTIOC2A** on GPT162 (Table 19.6) — see the WiFi `d10`.
    ///
    /// Consumes the PAC's `GPT163` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d10(
        gpt: ra4m1::GPT163,
        pin: Pin<'1', 12, Input>,
        freq_hz: u32,
        clocks: &Clocks,
    ) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "wifi")]
impl Pwm<ra4m1::GPT166, '4', 11, false> {
    /// Claims header **D11** — WiFi: **P411 → GTIOC6A** on GPT166
    /// (manual §19 Table 19.12, PORT4 (2): `GTIOC6A` at
    /// `PSEL[4:0] = 00011b`). Minima: the same header pin is **P109 →
    /// GTIOC1A** on GPT321 (Table 19.7) — see the Minima `d11`.
    ///
    /// Consumes the PAC's `GPT166` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d11(
        gpt: ra4m1::GPT166,
        pin: Pin<'4', 11, Input>,
        freq_hz: u32,
        clocks: &Clocks,
    ) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

#[cfg(feature = "minima")]
impl Pwm<ra4m1::GPT321, '1', 9, false> {
    /// Claims header **D11** — Minima: **P109 → GTIOC1A** on GPT321
    /// (manual §19 Table 19.7, PORT1 (2): `GTIOC1A` at
    /// `PSEL[4:0] = 00011b`). WiFi: the same header pin is **P411 →
    /// GTIOC6A** on GPT166 (Table 19.12) — see the WiFi `d11`.
    ///
    /// Consumes the PAC's `GPT321` (exactly-once claim, like everything
    /// `Peripherals::take` hands out) and the pin in its reset-default
    /// input state.
    ///
    /// # Panics
    /// If `freq_hz` is unreachable from PCLKD (see
    /// [`PeriodError`](crate::pwm::PeriodError)).
    pub fn d11(gpt: ra4m1::GPT321, pin: Pin<'1', 9, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        Self::new(gpt, pin, freq_hz, clocks)
    }
}

impl<S: GptChannel, const PORT: char, const PIN: u8, const IOCB: bool> Pwm<S, PORT, PIN, IOCB> {
    /// Shared constructor path: resolve the requested frequency, then
    /// bring the channel up. Every per-pin constructor lands here, so
    /// the exact-once claim and the register recipe stay in one place.
    fn new(gpt: S, pin: Pin<PORT, PIN, Input>, freq_hz: u32, clocks: &Clocks) -> Self {
        let params = calc_period(clocks.pclkd(), freq_hz)
            .expect("requested PWM frequency is unreachable from PCLKD");
        Self::init(gpt, pin, params)
    }

    /// Shared bring-up (manual §22.3.3.1 Fig 22.30's saw-wave PWM
    /// recipe, with every register rewritten rather than trusted from a
    /// bootloader handoff):
    ///
    /// 1. module stop released ([`GptChannel::release_module_stop`]);
    /// 2. pin muxed to the GTIOC function (`PSEL = 00011b`, peripheral
    ///    mode on — the PFS sequence Slice 3 proved);
    /// 3. counter stopped: `GTCR` = saw-wave PWM mode, the computed
    ///    prescaler, `CST = 0` (`MD`/`TPCS` are only writable while
    ///    stopped, §22.2.12);
    /// 4. `GTUDDTYC.UD = 1` — up-counting (§22.2.13);
    /// 5. `GTPR` = period − 1 (§22.2.20);
    /// 6. `GTIOR`: the polarity in [`GTIO_PWM`], output enabled, and
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
    fn init(gpt: S, pin: Pin<PORT, PIN, Input>, params: PeriodParams) -> Self {
        S::release_module_stop();
        let _pin = pin.into_alternate(PSEL_GPT);

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
        // Pin function + output enable. Both outputs get the polarity;
        // only the claimed one is enabled. GTIOx[4:0] = 0b0_10_01:
        // initial low, HIGH at the cycle end, low at the compare match
        // (Table 22.5) — the pin is high from the cycle end through the
        // match, so the compare value over the period *is* the duty
        // (FSP's r_gpt documents the same convention; behavioral
        // cross-check only).
        if IOCB {
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
        } else {
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
            if IOCB {
                gpt.gtccre.write(|w| unsafe { w.bits(counts) });
            } else {
                gpt.gtccrc.write(|w| unsafe { w.bits(counts) });
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

impl<S: GptChannel, const PORT: char, const PIN: u8, const IOCB: bool> ErrorType
    for Pwm<S, PORT, PIN, IOCB>
{
    type Error = Infallible;
}

impl<S: GptChannel, const PORT: char, const PIN: u8, const IOCB: bool> SetDutyCycle
    for Pwm<S, PORT, PIN, IOCB>
{
    fn max_duty_cycle(&self) -> u16 {
        Pwm::<S, PORT, PIN, IOCB>::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.set_duty(duty);
        Ok(())
    }
}
