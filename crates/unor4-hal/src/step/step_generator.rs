//! The step generator: a STEP pulse train, its hardware pulse count, the
//! 2 ms tick and tick-timed changes, on the hardware its setup names.

use crate::{
    clock::Clocks,
    elc::{Elc, ElcB, ElcC, ElcD, ElcInput, GptInput},
    gpio::{GptOutput, Gtioc, Input, Pin, PinId},
    intc::Ielsr,
    peripherals::{Board, Release},
    pwm::GptChannel,
    step::{
        EDGE_MARGIN, Ena, EnaChoice, GptRegs, Hardware, MAX_REVERSAL_LOW, MRB_CHAIN,
        PULSE_END_LATCH_DELAY, Pending, Schedule, StepError, StepResult, StepSetup, TICK_DIVISOR,
        TICK_TPCS, TICK_US, TickDtc, TickReport, calc_step_period, check_count_target,
        count_compare_value, first_cycle, idle_expired, lead_passed, missed_reversal_new_pulses,
        record, resync_schedule, start_lead,
    },
};

use core::{
    cell::Cell,
    ptr::{addr_of, addr_of_mut, read_volatile, write_volatile},
};
use embedded_hal::digital::PinState;
use ra4m1::gpt320::gtior::{GTIOA_A, GTIOB_A};

/// `GTCR.CST` (bit 0): the counter runs (§22.2.12).
const GTCR_CST: u32 = 1;

/// The ELC input that carries the STEP channel's compare match on its
/// output (each pulse's end): the counter counts it. Claimed by `build`.
const PULSE_END_INPUT: GptInput = ElcB::INPUT;

/// The ELC input that carries the tick channel's overflow (each 2 ms tick):
/// the counter and the STEP channel capture on it, the tick-number channel
/// counts it, and a start from stopped is triggered by it.
const TICK_INPUT: GptInput = ElcC::INPUT;

/// The ELC input that carries the counter's compare match B: the armed
/// count of [`StepGenerator::stop_at`]. The STEP channel stops on it
/// (`GTPSR.PSELCD`, SVD: "Counter stop is enable at the ELC_GPTD input")
/// only while a count stop is armed.
const COUNT_STOP_INPUT: GptInput = ElcD::INPUT;

/// Reads of `GTCR.CST` a settling count stop waits for the hardware stop,
/// which lands a few PCLKD cycles after the compare match.
const COUNT_STOP_SPINS: u32 = 1_000;

/// Read sets [`StepGenerator::resync`] takes when a wrap of the STEP
/// counter falls between its reads.
const RESYNC_READS: u32 = 3;

/// [`StepGenerator::queue`] refuses a rate this close (PCLKD counts, 40 µs)
/// to the next tick, so arming can never straddle it.
const QUEUE_GUARD: u32 = 1_920;

/// DTC record 1 (block mode, word, source incrementing — MRA = 10_10_10_00b;
/// manual §17.2.1): two words from RAM to the STEP channel's `GTPR`,
/// `GTPBR` (adjacent, 0x64/0x68, Table 22.4), destination incrementing (MRB
/// DM = 10b) and chained to record 2.
const MRA_BLOCK_WORDS: u8 = 0b1010_1000;
const MRB_DEST_INC: u8 = 0b0000_1000;
/// Block size 2 in both halves of CRA (§17.2.5), one block (CRB, §17.2.6).
const CRA_BLOCK_2: u16 = 0x0202;
/// DTC record 2 (normal mode, word, source and destination fixed —
/// MRA = 00_10_00_00b): the STEP channel's `GTCNT` back into RAM, right
/// after the write.
const MRA_WORD_FIXED: u8 = 0b0010_0000;

/// The values the tick DTC writes: `GTPR`, `GTPBR`.
static mut DTC_SOURCE: [u32; 2] = [0; 2];
/// The STEP channel's counter as the tick DTC read it right after its write.
static mut DTC_READBACK: u32 = 0;
/// The `PCNTR3` word a reversal's DTC chain writes to set DIR.
static mut DTC_DIR_VALUE: u32 = 0;
/// Source and destination of a reversal chain's delay record.
static mut DTC_SCRATCH: u32 = 0;

/// Port 0's `PCNTR3` (manual §19.2.3): port m's is `0x20 × m` further. A
/// 32-bit write sets the pins in bits 15:0 (`POSR`) and clears those in bits
/// 31:16 (`PORR`), so the CPU and the DTC set DIR in one access.
const PORT0_PCNTR3: u32 = 0x4004_0008;

/// A delay record of N words holds a DTC chain for 2N + 22…32 counts and
/// the first record lands 14–17 counts after the tick (both measured on the
/// board, Validation.md Slice 11 steps 5a/5b). Slack added to the measured maxima
/// for the latest DIR write.
const DIR_WRITE_SLACK: u32 = 120;
/// Block mode, word, source and destination fixed (MRA = 10_10_00_00b):
/// the reversal chain's delay record.
const MRA_BLOCK_FIXED: u8 = 0b1010_0000;

/// The "period" of rate 0: `GTPR = GTPBR = u32::MAX − 1`, so no further
/// pulse comes for ~89 s while the counter keeps the time since the last
/// one (the rule for the next start needs it). Long before that, an idle
/// generator is stopped ([`IDLE_STOP_SECONDS`]).
const IDLE_LEN: u32 = u32::MAX;

/// An idle generator (rate 0) whose last pulse is this many seconds old is
/// stopped at the next poll, long before its counter could wrap.
const IDLE_STOP_SECONDS: u32 = 1;

/// A STEP pulse train, built from a checked
/// [`StepSetup`](crate::step::StepSetup) by
/// [`StepGenerator::new`]: STEP on the setup's pin and timer output, DIR
/// on its DIR pin, a hardware pulse count on its counter, and the 2 ms tick
/// on its tick timers.
///
/// The STEP pin rests HIGH, each pulse is LOW for the setup's LOW time, and
/// a stopped generator holds the pin HIGH. [`StepGenerator::run`] starts a
/// rate while stopped; [`StepGenerator::queue`] changes it at the next 2 ms
/// tick, written by the DTC at the tick instant. The counter counts every
/// completed pulse in hardware ([`StepGenerator::pulses`]); the tick
/// latches that count and the STEP phase at one instant, and the
/// tick-number timer numbers the ticks ([`StepGenerator::poll_tick`]).
///
/// With an ENA pin in the setup the driver starts disabled:
/// [`StepGenerator::enable`] enables it, and a non-zero rate is refused
/// until the setup's lead has passed ([`StepError::Disabled`],
/// [`StepError::Enabling`]). Without one the generator is always enabled.
pub struct StepGenerator {
    step: &'static GptRegs,
    output: GptOutput,
    counter: &'static GptRegs,
    counter_mask: u32,
    ext_raw: Cell<u32>,
    ext_total: Cell<u32>,
    tick_gpt: &'static GptRegs,
    ticker: &'static GptRegs,
    tick_dtc: TickDtc,
    pclkd_hz: u32,
    low_counts: u32,
    min_high_counts: u32,
    period_counts: u32,
    tick_counts: u32,
    tick: u32,
    last_tick_raw: u16,
    last_pulses: u32,
    seen_tick: bool,
    changed: bool,
    schedule: Option<Schedule>,
    pending: Option<Pending>,
    lost: bool,
    dir_pcntr3: u32,
    dir_bit: u32,
    forward: PinState,
    negative: bool,
    dir_setup_counts: u32,
    delay_words: u32,
    dir_write_max: u32,
    dir_write_min: u32,
    position: i64,
    carry: Option<u32>,
    ena: Option<Ena>,
    count_slot: Ielsr,
    count_stop: Option<u32>,
    count_reached: bool,
    count_wake: bool,
}

impl StepGenerator {
    /// Takes the resolved hardware: releases the channels' module stop,
    /// claims an ICU slot for the tick's DTC, sets the STEP channel up
    /// stopped with the pin HIGH, routes its compare match on the STEP
    /// output (each pulse's end) to ELC input B, where the counter counts
    /// every completed pulse; starts the tick (the tick channel overflows
    /// every 2 ms, its overflow on ELC input C makes the counter and the STEP
    /// channel capture and the tick-number channel count); links the tick
    /// overflow to the ICU slot that starts the DTC (never the CPU) for
    /// queued changes. The timing registers are written before the STEP pin
    /// is muxed, so it goes from input straight to a HIGH output.
    ///
    /// The setup's builders checked every number at compile time, and the
    /// caller has already claimed the tick's ICU slot (`tick_dtc`), so this
    /// cannot fail.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn start(
        hw: Hardware,
        tick_dtc: TickDtc,
        mut count_slot: Ielsr,
        elc: &mut Elc,
        clocks: &Clocks,
        release_module_stop: impl FnOnce(),
        mux_step: impl FnOnce(),
        dir_output: impl FnOnce(PinState),
        ena_output: impl FnOnce(PinState),
    ) -> Self {
        let pclkd_hz = clocks.pclkd();
        let low_counts = hw.low_counts;
        let tick_counts = u64::from(pclkd_hz) * u64::from(TICK_US) / 1_000_000;
        let tick_counts = tick_counts as u32;
        // The reversal chain's delay record: long enough that the DIR write
        // lands after a pulse LOW at the tick (≤ low) or one that fires
        // inside the first record's latency (≤ 17 + low), with 32 counts to
        // spare: 14 + (2N + 22) ≥ low + 49.
        let delay_words = (low_counts + 16).div_ceil(2) + 8;
        let dir_write_min = 14 + 2 * delay_words + 22;
        let dir_write_max = 17 + 2 * delay_words + 32 + DIR_WRITE_SLACK;
        debug_assert!(low_counts <= MAX_REVERSAL_LOW && hw.min_high_counts >= 2 * EDGE_MARGIN);
        debug_assert!(hw.step_bits == 32 && hw.step_divider == 1 && hw.step_tpcs == 0);

        release_module_stop();
        let regs = hw.step;
        // Stopped, saw-wave PWM, PCLKD/1 (MD/TPCS only change while stopped,
        // §22.2.12). Every source and buffer register is rewritten: a
        // bootloader jump is not a reset.
        regs.gtcr
            .write(|w| unsafe { w.md()._000().tpcs().bits(hw.step_tpcs).cst().clear_bit() });
        regs.gtuddtyc.write(|w| w.ud().set_bit());
        // GTPR buffered by GTPBR (GTBER.PR = 01b): every overflow reloads
        // the steady period (§22.3.2.1), so a tick-written transition
        // period lasts one period only.
        regs.gtber.write(|w| unsafe { w.pr().bits(0b01) });
        regs.gtssr.write(|w| unsafe { w.bits(0) });
        regs.gtpsr.write(|w| unsafe { w.bits(0) });
        regs.gtcsr.write(|w| unsafe { w.bits(0) });
        regs.gtupsr.write(|w| unsafe { w.bits(0) });
        regs.gtdnsr.write(|w| unsafe { w.bits(0) });
        regs.gtpr.write(|w| unsafe { w.bits(u32::MAX) });
        regs.gtpbr.write(|w| unsafe { w.bits(u32::MAX) });
        regs.gtcnt.write(|w| unsafe { w.bits(0) });
        // The output's compare register ends each pulse; the other one
        // captures the phase at each tick (§22.2.10–§22.2.11). The output:
        // HIGH at count start, LOW at the cycle end, HIGH at the compare
        // match (GTIOx = 10110b, Table 22.5); HIGH while stopped (xDFLT = 1,
        // xHLD = 0). The other output stays disabled.
        match hw.output {
            GptOutput::B => {
                regs.gticasr.write(|w| w.aselcc().set_bit());
                regs.gticbsr.write(|w| unsafe { w.bits(0) });
                regs.gtccrb.write(|w| unsafe { w.bits(low_counts) });
                regs.gtior.write(|w| {
                    w.gtiob()
                        .variant(GTIOB_A::_10110)
                        .obdflt()
                        .set_bit()
                        .obhld()
                        .clear_bit()
                        .obe()
                        .set_bit()
                });
            }
            GptOutput::A => {
                regs.gticasr.write(|w| unsafe { w.bits(0) });
                regs.gticbsr.write(|w| w.bselcc().set_bit());
                regs.gtccra.write(|w| unsafe { w.bits(low_counts) });
                regs.gtior.write(|w| {
                    w.gtioa()
                        .variant(GTIOA_A::_10110)
                        .oadflt()
                        .set_bit()
                        .oahld()
                        .clear_bit()
                        .oae()
                        .set_bit()
                });
            }
        }
        mux_step();
        // DIR: a push-pull output at the forward level (positive rates).
        dir_output(hw.forward);
        // ENA: a push-pull output at the level that disables the driver.
        let ena = hw.ena_pin.map(|(port, pin)| {
            let port = u32::from(port as u8 - b'0');
            // The lead in tick-timer counts (PCLKD/4), rounded up.
            let lead_counts = (u64::from(hw.ena_lead_ns) * u64::from(pclkd_hz / TICK_DIVISOR))
                .div_ceil(1_000_000_000);
            Ena {
                pcntr3: PORT0_PCNTR3 + 0x20 * port,
                bit: 1 << pin,
                enabled: hw.ena_enabled,
                lead_counts,
                since: None,
                ready: Cell::new(false),
            }
        });
        ena_output(match &ena {
            Some(ena) => ena.disabled(),
            None => PinState::Low,
        });
        elc.link_input(PULSE_END_INPUT, hw.pulse_end);

        // The counter: an event counter on input B (GTUPSR.USELCB, §22.2.8;
        // event count, §22.3.1.1(4)), wrapping at its width (GTPR = max),
        // that also captures its count into GTCCRA at each tick (input C).
        // The STEP channel is stopped and its GTCNT preloads past the
        // compare value, so no compare match fires before the first pulse.
        let counter = hw.counter;
        reset_event_counter(counter, hw.counter_mask);
        counter.gtupsr.write(|w| w.uselcb().set_bit());
        counter.gticasr.write(|w| w.aselcc().set_bit());
        // Its compare match B is the count stop: to the STEP channel over
        // ELC input D, and to the CPU through its own ICU slot (enabled only
        // while a count stop is armed with the wake-up on).
        counter.gtccrb.write(|w| unsafe { w.bits(hw.counter_mask) });
        elc.link_input(COUNT_STOP_INPUT, hw.count_match);
        count_slot.link(hw.count_event);

        // The tick-number channel: counts ticks on input C (16-bit,
        // wrapping); poll_tick extends it to 32 bits.
        let ticker = hw.ticker;
        reset_event_counter(ticker, 0xFFFF);
        ticker.gtupsr.write(|w| w.uselcc().set_bit());

        // Both counters run from here on (event counting starts one PCLKD
        // after CST with TPCS = 000b, §22.3.1.1(4)).
        counter.gtcr.modify(|_, w| w.cst().set_bit());
        ticker.gtcr.modify(|_, w| w.cst().set_bit());

        // The tick itself: saw-wave at PCLKD/4, overflowing every 2 ms; its
        // overflow goes to input C.
        let tick = hw.tick;
        reset_event_counter(tick, tick_counts / TICK_DIVISOR - 1);
        tick.gtcr
            .write(|w| unsafe { w.md()._000().tpcs().bits(TICK_TPCS).cst().clear_bit() });
        elc.link_input(TICK_INPUT, hw.tick_elc);
        tick.gtcr.modify(|_, w| w.cst().set_bit());

        let port = u32::from(hw.dir_port as u8 - b'0');
        Self {
            step: regs,
            output: hw.output,
            counter,
            counter_mask: hw.counter_mask,
            ext_raw: Cell::new(0),
            ext_total: Cell::new(0),
            tick_gpt: tick,
            ticker,
            tick_dtc,
            pclkd_hz,
            low_counts,
            min_high_counts: hw.min_high_counts,
            period_counts: 0,
            tick_counts,
            tick: 0,
            last_tick_raw: 0,
            last_pulses: 0,
            seen_tick: false,
            changed: true,
            schedule: None,
            pending: None,
            lost: false,
            dir_pcntr3: PORT0_PCNTR3 + 0x20 * port,
            dir_bit: 1 << hw.dir_pin,
            forward: hw.forward,
            negative: false,
            dir_setup_counts: hw.dir_setup_counts,
            delay_words,
            dir_write_max,
            dir_write_min,
            position: 0,
            carry: None,
            ena,
            count_slot,
            count_stop: None,
            count_reached: false,
            count_wake: false,
        }
    }

    /// Takes the generator apart and puts its hardware back into `p`, the
    /// peripherals [`StepGenerator::new`]'s `build` returned (or what they
    /// became since), so another driver can claim it.
    ///
    /// It stops the pulse train ([`StepGenerator::stop`], so no pulse is
    /// cut), stops the counter and the tick, unlinks ELC inputs B, C and D
    /// and gives them back, returns both ICU slots and the DTC, disables the
    /// driver (ENA) and puts the pins
    /// back into input mode. An ENA pin in input mode floats: the driver
    /// then sees its own idle level (a DM542T, for one, is enabled with ENA
    /// unconnected). `setup` must be the [`StepSetup`] the generator was
    /// built from.
    ///
    /// # Errors
    /// The generator and `p`, untouched, when `setup` names hardware other
    /// than what the generator drives.
    // The error hands both back untouched, so the caller loses nothing;
    // `no_std` has no box to shrink it.
    #[allow(clippy::result_large_err, clippy::type_complexity)]
    pub fn free<P, D, Dp, C, T, N, E, X, Q1, Q2, R0, R1, R2, R3, R4, R5, R6, R7, R8>(
        mut self,
        setup: StepSetup<P, D, Dp, C, T, N, E>,
        mut p: X,
    ) -> Result<R8, (Self, X)>
    where
        P: Gtioc,
        Dp: PinId,
        C: GptChannel,
        T: GptChannel,
        N: GptChannel,
        E: EnaChoice,
        X: Board + Release<ElcD, Part = ElcD, Restored = Q1>,
        Q1: Release<ElcC, Part = ElcC, Restored = Q2>,
        Q2: Release<ElcB, Part = ElcB, Restored = R0>,
        R0: Release<E, Part = E::Part, Restored = R1>,
        R1: Release<N, Part = N, Restored = R2>,
        R2: Release<T, Part = T, Restored = R3>,
        R3: Release<C, Part = C, Restored = R4>,
        R4: Release<Dp, Part = Pin<Dp, Input>, Restored = R5>,
        R5: Release<P::Timer, Part = P::Timer, Restored = R6>,
        R6: Release<P, Part = Pin<P, Input>, Restored = R7>,
        R7: Release<ra4m1::DTC, Part = ra4m1::DTC, Restored = R8>,
    {
        let _ = setup;
        let port = u32::from(Dp::PORT as u8 - b'0');
        let same = core::ptr::eq(self.step, <P::Timer as GptChannel>::PTR)
            && self.output == P::OUTPUT
            && core::ptr::eq(self.counter, C::PTR)
            && core::ptr::eq(self.tick_gpt, T::PTR)
            && core::ptr::eq(self.ticker, N::PTR)
            && self.dir_pcntr3 == PORT0_PCNTR3 + 0x20 * port
            && self.dir_bit == 1 << Dp::PIN
            && match (&self.ena, E::PIN) {
                (None, None) => true,
                (Some(ena), Some((port, pin))) => {
                    ena.pcntr3 == PORT0_PCNTR3 + 0x20 * u32::from(port as u8 - b'0')
                        && ena.bit == 1 << pin
                }
                _ => false,
            };
        if !same {
            return Err((self, p));
        }
        self.stop();
        if let Some(ena) = &self.ena {
            ena.write(false);
        }
        for regs in [self.counter, self.tick_gpt, self.ticker] {
            regs.gtcr.modify(|_, w| w.cst().clear_bit());
        }
        self.step.gtior.write(|w| unsafe { w.bits(0) });
        p.elc().unlink_input(PULSE_END_INPUT);
        p.elc().unlink_input(TICK_INPUT);
        p.elc().unlink_input(COUNT_STOP_INPUT);
        self.count_slot.release();
        let dtc = self.tick_dtc.release();
        // Safety: the generator owned these parts since `build` claimed them
        // from `p`, and the check above proved they are the ones it drives;
        // it is consumed here, so each part exists once again, back in `p`.
        unsafe {
            Ok(p.release(ElcD::steal())
                .release(ElcC::steal())
                .release(ElcB::steal())
                .release(E::release())
                .release(N::steal())
                .release(T::steal())
                .release(C::steal())
                .release(Pin::<Dp, Input>::new().into_input())
                .release(<P::Timer as GptChannel>::steal())
                .release(Pin::<P, Input>::new().into_input())
                .release(dtc))
        }
    }

    /// The schedule at the latest tick (`phase`, tick number `raw`), rebuilt
    /// from the STEP channel's registers ([`resync_schedule`]). `None` when a
    /// newer tick has started, the tick-number timer may not have counted
    /// the latest tick yet (tick timer under 2 counts), or the counter
    /// overflowed during the reads: the next poll retries.
    fn resync(&self, phase: u32, raw: u16) -> Option<Schedule> {
        let regs = self.step;
        critical_section::with(|_| {
            // A wrap during the reads (the counter read again lower) makes
            // them disagree; reads taken just after it agree, so retake them
            // a few times rather than wait for the next poll, which at a
            // rate fitting the tick a whole number of times would land at
            // the same phase again.
            for _ in 0..RESYNC_READS {
                // The tick timer, then at once the STEP counter: one read
                // apart, so an overflow in between (a whole period, ≥ 50
                // counts) stands far outside the slack.
                let since = self.tick_gpt.gtcnt.read().bits();
                let count = regs.gtcnt.read().bits();
                let gtpr = regs.gtpr.read().bits();
                let gtpbr = regs.gtpbr.read().bits();
                let count_after = regs.gtcnt.read().bits();
                let raw_now = self.ticker.gtcnt.read().bits() as u16;
                if since < 2 || raw_now != raw {
                    return None;
                }
                if count_after >= count {
                    return resync_schedule(phase, count, since * TICK_DIVISOR, gtpr, gtpbr);
                }
            }
            None
        })
    }

    /// The STEP channel's capture register: the phase at each tick. The
    /// register the STEP output does not compare with (§22.2.10–§22.2.11).
    fn phase_capture(&self) -> u32 {
        match self.output {
            GptOutput::B => self.step.gtccra.read().bits(),
            GptOutput::A => self.step.gtccrb.read().bits(),
        }
    }

    /// Extends a raw counter value to 32 bits. Every read — a tick's
    /// captured count in `poll_tick`, a live count in `pulses` — moves the
    /// one extension state forward. The difference to the previous read is
    /// taken as signed in the counter's width: a tick's capture can be a few
    /// pulses older than a live read made after it. A 32-bit counter wraps
    /// exactly at 2³² with no reads needed; a 16-bit one needs a read at
    /// least every 32 768 pulses (0.25 s at the highest rate).
    fn extend_count(&self, raw: u32) -> u32 {
        let delta = raw.wrapping_sub(self.ext_raw.get()) & self.counter_mask;
        let half = (self.counter_mask >> 1) + 1;
        let signed = if delta >= half {
            delta.wrapping_sub(self.counter_mask).wrapping_sub(1)
        } else {
            delta
        };
        let total = self.ext_total.get().wrapping_add(signed);
        self.ext_raw.set(raw);
        self.ext_total.set(total);
        total
    }

    /// Starts the pulse train at `rate` pulses/s, the sign giving the
    /// direction (DIR at the DIR setup's forward level for positive rates); `0`
    /// stops it. Returns the period in PCLKD counts (`0` for a stop); the
    /// delivered rate is `pclkd / period`.
    ///
    /// DIR is set first, while the generator is stopped. The first pulse
    /// starts the minimum HIGH time after this call, or the DIR set-up time
    /// when this call changes the direction, whichever is longer. `GTCNT`
    /// is preloaded so the first overflow comes that soon, and past
    /// `GTCCRB`, so no compare match fires before the first pulse; when
    /// the wait does not fit in one period, the first period is longer
    /// (`first_cycle`).
    ///
    /// # Errors
    /// [`StepError::Running`] if the generator runs (stop it first);
    /// [`StepError::Disabled`] / [`StepError::Enabling`] with an ENA pin
    /// that is not enabled, or whose lead has not passed;
    /// [`StepError::RateTooHigh`] when the pulse shape does not fit. DIR
    /// is left as it was on every error.
    pub fn run(&mut self, rate: i32) -> StepResult<u32> {
        self.sync_count_stop();
        if rate == 0 {
            self.stop();
            return Ok(0);
        }
        if self.is_running() {
            return Err(StepError::Running);
        }
        self.check_enabled()?;
        let negative = rate < 0;
        let period = calc_step_period(
            self.pclkd_hz,
            rate.unsigned_abs(),
            self.low_counts,
            self.min_high_counts,
        )?;
        let flips = negative != self.negative;
        // The CPU writes DIR here, before the counter starts below, so the
        // set-up time runs from no later than the start.
        self.set_direction(negative);
        let lead = start_lead(flips, self.min_high_counts, self.dir_setup_counts, 0);
        let (first_top, preload) = first_cycle(period, self.low_counts, lead);
        let regs = self.step;
        regs.gtpr.write(|w| unsafe { w.bits(first_top) });
        regs.gtpbr.write(|w| unsafe { w.bits(period - 1) });
        regs.gtcnt.write(|w| unsafe { w.bits(preload) });
        regs.gtcr.modify(|_, w| w.cst().set_bit());
        self.period_counts = period;
        self.changed = true;
        self.lost = false;
        self.schedule = None;
        Ok(period)
    }

    /// Sets the rate for the next tick to `rate` pulses/s, the sign giving
    /// the direction. Returns the new period in PCLKD counts (`0` for rate
    /// 0). Not calling it before a tick holds the running rate and
    /// direction.
    ///
    /// - **Reversal** (the sign flips while pulses run): the same tick write,
    ///   plus DIR, by the rule agreed with the cart-pole rig — DIR changes
    ///   only while STEP is HIGH, and the next pulse comes at least
    ///   the DIR set-up time after it. The DTC chain holds the DIR
    ///   write back with a delay record (measured: it lands
    ///   [`StepGenerator::dir_write_window`] after the tick) so that a pulse
    ///   LOW at the tick, or one firing inside the write latency, has ended
    ///   first; the next pulse is placed `dir_setup_ns` after the latest
    ///   possible DIR write. Pulses are split by direction exactly in
    ///   [`TickReport::position`].
    /// - **Direction change while idle or stopped:** DIR is set at once (no
    ///   pulse is running), at least 40 µs before the tick.
    ///
    /// - **Running:** at the tick instant the DTC writes the period in
    ///   progress and the new steady period, by the rule agreed with the
    ///   cart-pole rig: the next pulse comes one new period after the last
    ///   one, or the minimum HIGH time after the tick if that time has
    ///   already passed. If the old period was due to end within the DTC's
    ///   write latency (~0.3 µs), its pulse fires first and the transition
    ///   period follows it; the DTC reads the counter back right after its
    ///   write, so [`StepGenerator::poll_tick`] knows which happened and
    ///   predictions stay exact.
    /// - **Rate 0:** the same write with a period of ~89 s: no further pulse,
    ///   while the counter keeps the time since the last one, so a later
    ///   non-zero rate still follows the rule. After 1 s
    ///   (`IDLE_STOP_SECONDS`) idle, `poll_tick` stops the counter.
    /// - **Stopped:** a hardware start at the tick (`GTSSR`, ELC input C,
    ///   manual §22.3.7.1), the counter preloaded so the first pulse comes
    ///   the minimum HIGH time after the tick.
    ///
    /// # Errors
    /// [`StepError::NotReady`] when the latest tick has not been polled, a
    /// running generator's schedule is not known yet (one full tick after
    /// [`StepGenerator::run`] or [`StepGenerator::stop`]), or a change is
    /// already queued; [`StepError::TooLate`] within 40 µs of the tick;
    /// [`StepError::Disabled`] / [`StepError::Enabling`] for a non-zero
    /// rate with an ENA pin that is not enabled, or whose lead has not
    /// passed;
    /// [`StepError::RateTooHigh`] for the rate;
    /// [`StepError::ReversalWithCountStop`] for a direction change while
    /// pulses run and a count stop ([`StepGenerator::stop_at`]) is armed.
    pub fn queue(&mut self, rate: i32) -> StepResult<u32> {
        self.sync_count_stop();
        if self.pending.is_some() || self.changed || !self.seen_tick {
            return Err(StepError::NotReady);
        }
        let negative = rate < 0;
        let magnitude = rate.unsigned_abs();
        if magnitude != 0 {
            self.check_enabled()?;
        }
        if !self.is_running() {
            return if magnitude == 0 {
                Ok(0)
            } else {
                self.queue_start(magnitude, negative)
            };
        }
        let schedule = self.schedule.ok_or(StepError::NotReady)?;
        let period = if magnitude == 0 {
            if self.period_counts == IDLE_LEN {
                return Ok(0);
            }
            IDLE_LEN
        } else {
            calc_step_period(
                self.pclkd_hz,
                magnitude,
                self.low_counts,
                self.min_high_counts,
            )?
        };
        let flips = magnitude != 0 && negative != self.negative;
        let idle = self.period_counts == IDLE_LEN;
        if flips && idle {
            // No pulse runs while idle: set DIR now, once any pulse that
            // fired at the zero tick has ended. Every end still due in this
            // tick is an old-direction pulse.
            let regs = self.step;
            while regs.gtcnt.read().bits() <= self.low_counts + EDGE_MARGIN {}
            let old_ends = self
                .predicted_pulses()
                .map_or(0, |due| due.wrapping_sub(self.last_pulses));
            self.carry = Some(old_ends);
            self.set_direction(negative);
        }
        let reverse = flips && !idle;
        if reverse && self.count_stop.is_some() {
            return Err(StepError::ReversalWithCountStop);
        }
        // The schedule at the target tick, before the change, and the
        // transition period: the next pulse one new period after the last
        // one, or min_high after the tick — for a reversal, dir_setup after
        // the latest possible DIR write instead; for a direction change
        // while idle (DIR written above, at least the queue guard before
        // the tick), also dir_setup after that write.
        let at_tick = schedule.advance(self.tick_counts);
        let lead = if reverse {
            self.dir_write_max + self.dir_setup_counts
        } else {
            start_lead(
                flips && idle,
                self.min_high_counts,
                self.dir_setup_counts,
                QUEUE_GUARD,
            )
        };
        let crossing = period.max(at_tick.phase.saturating_add(lead));
        let regs = self.step;
        let write = record(
            MRA_BLOCK_WORDS,
            MRB_CHAIN | MRB_DEST_INC,
            addr_of!(DTC_SOURCE) as u32,
            &regs.gtpr as *const _ as u32,
            CRA_BLOCK_2,
            1,
        );
        let readback = |chain: u8| {
            record(
                MRA_WORD_FIXED,
                chain,
                &regs.gtcnt as *const _ as u32,
                addr_of!(DTC_READBACK) as u32,
                1,
                1,
            )
        };
        let words = (self.delay_words & 0xFF) as u16;
        let delay = record(
            MRA_BLOCK_FIXED,
            MRB_CHAIN,
            addr_of!(DTC_SCRATCH) as u32,
            addr_of!(DTC_SCRATCH) as u32,
            (words << 8) | words,
            1,
        );
        let dir_write = record(
            MRA_WORD_FIXED,
            MRB_CHAIN,
            addr_of!(DTC_DIR_VALUE) as u32,
            self.dir_pcntr3,
            1,
            1,
        );
        let chain_change = [write, readback(0)];
        let chain_reverse = [write, delay, dir_write, readback(0)];
        let records: &[[u32; 4]] = if reverse {
            &chain_reverse
        } else {
            &chain_change
        };
        let dir_value = if self.level(negative) == PinState::High {
            self.dir_bit
        } else {
            self.dir_bit << 16
        };
        let tick_regs = self.tick_gpt;
        let ticker = self.ticker;
        let tick_top = self.tick_counts / TICK_DIVISOR - 1;
        let last_raw = self.last_tick_raw;
        let dtc = &mut self.tick_dtc;
        let target = critical_section::with(|_| {
            let remaining = (tick_top - tick_regs.gtcnt.read().bits()) * TICK_DIVISOR;
            if remaining < QUEUE_GUARD {
                return Err(StepError::TooLate);
            }
            let raw = ticker.gtcnt.read().bits() as u16;
            if raw != last_raw {
                // A tick happened that has not been polled yet.
                return Err(StepError::NotReady);
            }
            unsafe {
                write_volatile(addr_of_mut!(DTC_SOURCE[0]), crossing - 1);
                write_volatile(addr_of_mut!(DTC_SOURCE[1]), period - 1);
                write_volatile(addr_of_mut!(DTC_DIR_VALUE), dir_value);
            }
            dtc.arm(records);
            if ticker.gtcnt.read().bits() as u16 != raw && dtc.armed() {
                dtc.disarm();
                return Err(StepError::TooLate);
            }
            Ok(raw.wrapping_add(1))
        })?;
        self.pending = Some(Pending {
            tick_raw: target,
            crossing,
            period,
            start: false,
            reverse,
        });
        Ok(if period == IDLE_LEN { 0 } else { period })
    }

    /// [`StepGenerator::queue`] from stopped: set DIR (no pulse runs),
    /// preload the counter and the period registers (writable while
    /// stopped, §22.3.1.1(6)) and arm the hardware start on the tick
    /// (`GTSSR.SSELCC`). DIR is written before the guard check below, so at
    /// least the queue guard (40 µs) before the tick; the first pulse comes
    /// min_high after the tick, or later when a DIR change needs more of
    /// its set-up time ([`start_lead`], [`first_cycle`]).
    fn queue_start(&mut self, rate: u32, negative: bool) -> StepResult<u32> {
        let period = calc_step_period(self.pclkd_hz, rate, self.low_counts, self.min_high_counts)?;
        let flips = negative != self.negative;
        self.set_direction(negative);
        let lead = start_lead(
            flips,
            self.min_high_counts,
            self.dir_setup_counts,
            QUEUE_GUARD,
        );
        let (first_top, preload) = first_cycle(period, self.low_counts, lead);
        let regs = self.step;
        let tick_regs = self.tick_gpt;
        let ticker = self.ticker;
        let tick_top = self.tick_counts / TICK_DIVISOR - 1;
        let last_raw = self.last_tick_raw;
        let target = critical_section::with(|_| {
            let remaining = (tick_top - tick_regs.gtcnt.read().bits()) * TICK_DIVISOR;
            if remaining < QUEUE_GUARD {
                return Err(StepError::TooLate);
            }
            let raw = ticker.gtcnt.read().bits() as u16;
            if raw != last_raw {
                return Err(StepError::NotReady);
            }
            regs.gtpr.write(|w| unsafe { w.bits(first_top) });
            regs.gtpbr.write(|w| unsafe { w.bits(period - 1) });
            regs.gtcnt.write(|w| unsafe { w.bits(preload) });
            regs.gtssr.write(|w| w.sselcc().set_bit());
            if ticker.gtcnt.read().bits() as u16 != raw && !regs.gtcr.read().cst().bit_is_set() {
                regs.gtssr.write(|w| unsafe { w.bits(0) });
                return Err(StepError::TooLate);
            }
            Ok(raw.wrapping_add(1))
        })?;
        self.pending = Some(Pending {
            tick_raw: target,
            crossing: first_top + 1,
            period,
            start: true,
            reverse: false,
        });
        Ok(period)
    }

    /// The DIR pin level for a direction.
    fn level(&self, negative: bool) -> PinState {
        match (self.forward, negative) {
            (PinState::High, false) | (PinState::Low, true) => PinState::High,
            _ => PinState::Low,
        }
    }

    /// Sets DIR from the CPU — only while no pulse runs (stopped, idle).
    fn set_direction(&mut self, negative: bool) {
        let value = match self.level(negative) {
            PinState::High => self.dir_bit,
            PinState::Low => self.dir_bit << 16,
        };
        // PCNTR3: one atomic write sets (bits 15:0) or clears (bits 31:16)
        // the DIR pin, the same access the reversal's DTC write makes. The
        // pin is owned by this generator (claimed by `build`).
        unsafe { write_volatile(self.dir_pcntr3 as *mut u32, value) };
        self.negative = negative;
    }

    /// The window, in PCLKD counts after the tick, in which a reversal's
    /// DIR write lands: from the measured minimum of its DTC chain to the
    /// latest the HAL allows for (the next pulse is placed
    /// the DIR set-up time after the latter).
    pub fn dir_write_window(&self) -> (u32, u32) {
        (self.dir_write_min, self.dir_write_max)
    }

    /// The signed position in pulses since construction: completed pulses
    /// forward minus completed pulses backward, as of the latest tick.
    pub fn position(&self) -> i64 {
        self.position
    }

    /// Stops the pulse train; the pin ends HIGH, and no pulse is ever cut.
    /// A change queued for the next tick is cancelled.
    ///
    /// No further pulse starts once the longest period is written to
    /// `GTPR`/`GTPBR` (the rate-0 write: the next overflow is ~89 s away). A
    /// pulse that started before that write lands runs to its end and is
    /// counted; then the counter stops. Nothing here races the counter, so
    /// how fast the code runs (an unoptimised build included) only adds to
    /// the time taken, never cuts a pulse. Returns once the counter has
    /// stopped: at most the rest of a pulse in flight plus the code's own
    /// time. A tick DTC transfer already under way is let finish first, so
    /// it cannot rewrite the period afterwards. Does nothing when already
    /// stopped.
    pub fn stop(&mut self) {
        // A count stop that fired is settled; one still armed is cancelled.
        self.cancel_stop_at();
        self.tick_dtc.disarm();
        self.pending = None;
        let regs = self.step;
        // A start queued from stopped must not fire later.
        regs.gtssr.write(|w| unsafe { w.bits(0) });
        if regs.gtcr.read().bits() & GTCR_CST == 0 {
            return;
        }
        // A chain already running (at most a reversal's ~7 µs) ends first.
        while self.tick_dtc.busy() {}
        // The buffer first: an overflow between the two writes then reloads
        // the long period as well.
        regs.gtpbr.write(|w| unsafe { w.bits(IDLE_LEN - 1) });
        regs.gtpr.write(|w| unsafe { w.bits(IDLE_LEN - 1) });
        // A pulse in flight (or one that started before the write landed)
        // ends at `low` and is counted at `low + PULSE_END_LATCH_DELAY`.
        let done = self.low_counts + PULSE_END_LATCH_DELAY;
        while regs.gtcnt.read().bits() <= done {}
        regs.gtcr.modify(|_, w| w.cst().clear_bit());
        regs.gtpsr.write(|w| unsafe { w.bits(0) });
        self.period_counts = 0;
        self.changed = true;
        self.schedule = None;
    }

    /// Enables the driver: drives ENA to the setup's enabling level and
    /// starts the lead. A non-zero rate is refused with
    /// [`StepError::Enabling`] until the lead has passed, so the driver sees
    /// ENA at least that long before the first DIR change or pulse. The lead
    /// is measured in real time on the tick timer (83 ns counts), never
    /// short: [`StepGenerator::is_enabled`] turns true at most 2 counts
    /// (167 ns) after it. Already enabled, or no ENA pin: does nothing.
    pub fn enable(&mut self) {
        let stamp = self.tick_stamp();
        if let Some(ena) = &mut self.ena
            && ena.since.is_none()
        {
            ena.write(true);
            ena.since = Some(stamp);
            ena.ready.set(ena.lead_counts == 0);
        }
    }

    /// Disables the driver: drives ENA to the level that disables it. The
    /// driver then lets the motor go (no holding torque) and ignores
    /// pulses, so this needs a stopped generator. No ENA pin: does nothing.
    ///
    /// # Errors
    /// [`StepError::Running`] while the generator runs (stop it first).
    pub fn disable(&mut self) -> StepResult<()> {
        if self.is_running() {
            return Err(StepError::Running);
        }
        if let Some(ena) = &mut self.ena {
            ena.write(false);
            ena.since = None;
            ena.ready.set(false);
        }
        Ok(())
    }

    /// True when a non-zero rate may start: always without an ENA pin;
    /// with one, once [`StepGenerator::enable`] was called and its lead has
    /// passed.
    pub fn is_enabled(&self) -> bool {
        self.check_enabled().is_ok()
    }

    /// The ENA rule for a non-zero rate (see [`StepGenerator::enable`]).
    fn check_enabled(&self) -> StepResult<()> {
        let now = self.tick_stamp();
        let per_tick = u64::from(self.tick_counts / TICK_DIVISOR);
        let Some(ena) = &self.ena else {
            return Ok(());
        };
        let Some((tick0, count0)) = ena.since else {
            return Err(StepError::Disabled);
        };
        if !ena.ready.get() {
            let (tick1, count1) = now;
            // Ticks between the stamps, then the counts within them; the
            // 16-bit tick number wraps after 131 s, far past any lead
            // (≤ 4.29 s), and `ready` latches once the lead has passed.
            let ticks = u64::from(tick1.wrapping_sub(tick0));
            let elapsed = (ticks * per_tick + u64::from(count1)).saturating_sub(u64::from(count0));
            ena.ready.set(lead_passed(elapsed, ena.lead_counts));
        }
        if ena.ready.get() {
            Ok(())
        } else {
            Err(StepError::Enabling)
        }
    }

    /// The tick number's low 16 bits and the tick timer's count, read as
    /// one instant: re-read while a tick lands between the reads. A count
    /// under 2 (8 PCLKD counts after the overflow) is re-read too, so the
    /// tick number has surely counted that overflow (ELC event counting
    /// starts within one PCLKD, §22.3.1.1(4); the 2-count guard is margin,
    /// not measured).
    fn tick_stamp(&self) -> (u16, u32) {
        loop {
            let tick0 = self.ticker.gtcnt.read().bits() as u16;
            let count = self.tick_gpt.gtcnt.read().bits();
            let tick1 = self.ticker.gtcnt.read().bits() as u16;
            if tick0 == tick1 && count >= 2 {
                return (tick1, count);
            }
        }
    }

    /// Makes every 2 ms tick raise a CPU interrupt (`on = true`), so firmware
    /// can sleep in `WFI` between ticks; `false` turns it off again (the
    /// default after [`StepGeneratorBuilder::build`](crate::step::StepGeneratorBuilder::build)).
    ///
    /// The interrupt comes from the tick's own ICU slot: at an unarmed tick
    /// at once, at a tick with a queued change when the DTC has finished
    /// writing it (manual Table 13.5, §17.7). On the board a `WFI` loop saw
    /// every tick at most 92 PCLKD counts (1.9 µs) after it, over 2 500
    /// ticks of changing rates (Validation.md, Slice 11). It needs no handler: the
    /// HAL's vector table acknowledges it. Unmask CPU interrupts
    /// ([`crate::intc::enable_interrupts`]) so the acknowledgement runs;
    /// then a loop of `WFI`, [`StepGenerator::poll_tick`],
    /// [`StepGenerator::queue`] wakes once per tick (and for any other
    /// interrupt, such as a UART's), with nearly the whole tick left
    /// ([`StepGenerator::since_tick_counts`]) before the queue guard.
    pub fn set_tick_wake(&mut self, on: bool) {
        self.tick_dtc.set_cpu_interrupt(on);
    }

    /// Arms a stop at an exact pulse count: the pulse train stops in
    /// hardware when [`StepGenerator::pulses`] reaches `count`, with that
    /// pulse complete and no pulse after it, whatever the CPU is doing.
    ///
    /// The counter's compare match B reaches the STEP channel over ELC
    /// input D, which stops its counter (`GTPSR.PSELCD`). `GTCCRB` holds
    /// `count − 1`: the match fires on the count event that leaves the
    /// compared value (measured on the board), so it comes at the end of
    /// pulse `count`. The next pulse starts at
    /// least the minimum HIGH time (at least 1 µs) later, long after the
    /// stop. With [`StepGenerator::set_count_wake`] on, the same match also
    /// interrupts the CPU, so it ends a `WFI`.
    ///
    /// Arm it before or during a move, in either order with
    /// [`StepGenerator::queue`] / [`StepGenerator::run`]. A move of `n`
    /// pulses from rest:
    ///
    /// ```no_run
    /// # fn demo(stepper: &mut unor4_hal::step::StepGenerator, n: u32) -> unor4_hal::step::StepResult<()> {
    /// stepper.stop_at(stepper.pulses().wrapping_add(n))?;
    /// stepper.set_count_wake(true);
    /// // Queue the rate as usual; then sleep in WFI until the stop.
    /// while !stepper.stopped_at_count() {
    ///     cortex_m::asm::wfi();
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// The stop settles like [`StepGenerator::stop`]: no queued change
    /// survives, and `queue` is refused with `NotReady` until the next
    /// poll. [`StepGenerator::stopped_at_count`] reports it once. Arming
    /// again replaces an armed count; [`StepGenerator::stop`] and
    /// [`StepGenerator::cancel_stop_at`] cancel it.
    ///
    /// # Errors
    /// - [`StepError::CountNotAhead`]: `count` is not at least 1 ahead of
    ///   the pulses counted so far while no pulse can come (stopped, no
    ///   start queued), or at least 2 while pulses may run (one can end
    ///   while this arms); or it is behind (more than 2³¹ ahead).
    /// - [`StepError::CountTooFar`]: more than 65 535 ahead on a 16-bit
    ///   counter.
    /// - [`StepError::ReversalWithCountStop`]: a reversal is queued for the
    ///   next tick.
    pub fn stop_at(&mut self, count: u32) -> StepResult<()> {
        self.sync_count_stop();
        if self.pending.is_some_and(|pending| pending.reverse) {
            return Err(StepError::ReversalWithCountStop);
        }
        let regs = self.step;
        let counter = self.counter;
        let mask = self.counter_mask;
        let this = &*self;
        critical_section::with(|_| {
            let quiet = regs.gtcr.read().bits() & GTCR_CST == 0 && regs.gtssr.read().bits() == 0;
            check_count_target(count, this.pulses(), quiet, mask)?;
            // No stop on the old compare value while it changes; then the
            // new value, its flags cleared (`TCFB`, the ICU request), then
            // the stop source. At most one pulse ends inside this section
            // (it lasts well under the shortest period, 7.5 µs), so the
            // count is still ahead once the source is on.
            regs.gtpsr.write(|w| unsafe { w.bits(0) });
            counter
                .gtccrb
                .write(|w| unsafe { w.bits(count_compare_value(count, mask)) });
            counter.gtst.modify(|_, w| w.tcfb().clear_bit());
            this.count_slot.clear_request();
            regs.gtpsr.write(|w| w.pselcd().set_bit());
            Ok(())
        })?;
        self.count_stop = Some(count);
        self.count_reached = false;
        if self.count_wake {
            self.count_slot.enable();
        }
        Ok(())
    }

    /// Cancels an armed count stop ([`StepGenerator::stop_at`]); the train
    /// keeps running. If the train already stopped at the count, the stop
    /// is settled and [`StepGenerator::stopped_at_count`] reports it. A
    /// match that comes while the stop is being removed stops nothing: the
    /// cancel wins, and the count runs past the armed one. Nothing armed:
    /// does nothing.
    pub fn cancel_stop_at(&mut self) {
        if self.count_stop.is_none() {
            return;
        }
        // No hardware stop from here on.
        self.step.gtpsr.write(|w| unsafe { w.bits(0) });
        // The compare flag alone does not prove a stop: a match landing
        // between the write above and this read sets `TCFB` with the stop
        // source already off (measured: 15 of 2 000 swept cancels at
        // 55 733/s then reported a stop at a wrong count). The train
        // stopped only if the STEP counter stopped, a few PCLKD cycles
        // after the match.
        let regs = self.step;
        let mut stopped = false;
        if self.counter.gtst.read().tcfb().bit_is_set() {
            for _ in 0..COUNT_STOP_SPINS {
                if regs.gtcr.read().bits() & GTCR_CST == 0 {
                    stopped = true;
                    break;
                }
            }
        }
        if stopped {
            self.finish_count_stop();
        } else {
            self.counter.gtst.modify(|_, w| w.tcfb().clear_bit());
            self.count_stop = None;
            self.count_slot.disable();
        }
    }

    /// True once after an armed count stop ([`StepGenerator::stop_at`])
    /// stopped the train, and settles it: the generator is stopped and
    /// [`StepGenerator::pulses`] equals the armed count.
    pub fn stopped_at_count(&mut self) -> bool {
        self.sync_count_stop();
        core::mem::take(&mut self.count_reached)
    }

    /// Makes an armed count stop raise a CPU interrupt when it fires
    /// (`on = true`), so firmware can sleep in `WFI` until a move of an
    /// exact pulse count ends; `false` turns it off (the default). The
    /// interrupt comes from the counter's compare match B through its own
    /// ICU slot (claimed by the build), enabled only while a count is
    /// armed. It needs no handler: the HAL's vector table acknowledges it.
    /// Unmask CPU interrupts ([`crate::intc::enable_interrupts`]) so the
    /// acknowledgement runs; while they are masked a `WFI` returns at once
    /// for as long as the request is pending.
    pub fn set_count_wake(&mut self, on: bool) {
        self.count_wake = on;
        if self.count_stop.is_some() {
            if on {
                // A match since arming left the request set: it interrupts
                // at once.
                self.count_slot.enable();
            } else {
                self.count_slot.disable();
            }
        }
    }

    /// Settles an armed count stop that has fired (the counter's `TCFB`):
    /// true if it had. Every `&mut` entry point calls it, so no state
    /// outlives a hardware stop the CPU has not seen yet.
    fn sync_count_stop(&mut self) -> bool {
        if self.count_stop.is_none() || self.counter.gtst.read().tcfb().bit_is_clear() {
            return false;
        }
        self.finish_count_stop();
        true
    }

    /// After the count was reached: waits for the hardware stop, disarms
    /// the stop source and the wake-up, and leaves the generator stopped as
    /// [`StepGenerator::stop`] does.
    fn finish_count_stop(&mut self) {
        let regs = self.step;
        let mut spins = 0;
        while regs.gtcr.read().bits() & GTCR_CST != 0 && spins < COUNT_STOP_SPINS {
            spins += 1;
        }
        regs.gtpsr.write(|w| unsafe { w.bits(0) });
        self.count_stop = None;
        self.count_reached = true;
        self.count_slot.disable();
        if regs.gtcr.read().bits() & GTCR_CST != 0 {
            // The hardware stop never came (never seen on the bench): stop
            // the train the usual way rather than leave it running.
            self.stop();
            return;
        }
        // A tick change still armed must not rewrite the period of a
        // stopped train; one already written does nothing while stopped.
        while self.tick_dtc.busy() {}
        self.tick_dtc.disarm();
        self.pending = None;
        regs.gtssr.write(|w| unsafe { w.bits(0) });
        self.period_counts = 0;
        self.changed = true;
        self.schedule = None;
        self.lost = false;
    }

    /// PCLKD counts since the latest tick (the tick timer's counter × 4), from 0 to
    /// [`StepGenerator::tick_counts`] − 4. With [`StepGenerator::queue`]
    /// refusing the last 40 µs (1 920 counts at 48 MHz), this says how much
    /// of the tick is left for the work between ticks.
    pub fn since_tick_counts(&self) -> u32 {
        self.tick_gpt.gtcnt.read().bits() * TICK_DIVISOR
    }

    /// True while the counter runs (`GTCR.CST`).
    pub fn is_running(&self) -> bool {
        self.step.gtcr.read().cst().bit_is_set()
    }

    /// The steady period in PCLKD counts; `0` when stopped or idle (rate 0).
    /// The rate actually running is `pclkd / period` — after a tick without
    /// a queued rate it is the one held from before.
    pub fn period_counts(&self) -> u32 {
        if self.period_counts == IDLE_LEN {
            0
        } else {
            self.period_counts
        }
    }

    /// The low time in PCLKD counts (`GTCCRB`).
    pub fn low_counts(&self) -> u32 {
        self.low_counts
    }

    /// The tick length in PCLKD counts (96 000 at 48 MHz).
    pub fn tick_counts(&self) -> u32 {
        self.tick_counts
    }

    /// The schedule at the latest polled tick, when it is known exactly.
    pub fn schedule(&self) -> Option<Schedule> {
        self.schedule
    }

    /// Completed pulses since construction, counted by the counter in
    /// hardware (each compare match on the STEP output: the end of a
    /// pulse), wrapping at 2³². A 16-bit counter is extended in software:
    /// call this or [`StepGenerator::poll_tick`] at least every 32 768
    /// pulses (0.25 s at the highest rate). A pulse in progress counts when
    /// it ends; take differences with `wrapping_sub`. No CPU interrupt is
    /// involved, so interrupt load cannot lose a count.
    pub fn pulses(&self) -> u32 {
        self.extend_count(self.counter.gtcnt.read().bits())
    }

    /// The latest tick's latch, if a tick happened since the last call.
    ///
    /// The count and the phase in the report were captured by hardware at
    /// the tick's instant, so how late this is called does not matter. Call
    /// it once per tick (2 ms) while you change the rate: a rate is queued
    /// after a polled tick. At a steady rate it can wait as long as the CPU
    /// sleeps: the pulse train needs no CPU. Each call reports only the
    /// latest tick, and [`TickReport::skipped`] says how many were passed
    /// over. A 16-bit counter still needs a read (this or
    /// [`StepGenerator::pulses`]) at least every 32 768 pulses. After a skip
    /// the generator rebuilds its schedule from
    /// the STEP channel's registers at the first poll that can prove it
    /// (normally the same poll), and [`StepGenerator::queue`] works again; a
    /// change queued for a tick that was passed over still took effect in
    /// hardware (a reversal included: DIR and [`TickReport::position`] stay
    /// right). An idle generator (rate 0) stops 1 s after its last pulse
    /// whether or not ticks were missed. The tick number comes from the
    /// tick-number timer.
    pub fn poll_tick(&mut self) -> Option<TickReport> {
        let counter = self.counter;
        if counter.gtst.read().tcfa().bit_is_clear() {
            return None;
        }
        let (raw, count_raw, phase) = loop {
            // Clear the capture flag first: if another tick lands during the
            // reads it sets the flag again and the reads are retaken.
            counter.gtst.modify(|_, w| w.tcfa().clear_bit());
            let raw = self.ticker.gtcnt.read().bits() as u16;
            let count_raw = counter.gtccra.read().bits();
            let phase = self.phase_capture();
            if counter.gtst.read().tcfa().bit_is_clear() {
                break (raw, count_raw, phase);
            }
        };
        let pulses = self.extend_count(count_raw);
        let elapsed = u32::from(raw.wrapping_sub(self.last_tick_raw));
        let skipped = if self.seen_tick {
            elapsed.saturating_sub(1)
        } else {
            0
        };
        self.tick = self.tick.wrapping_add(elapsed);
        self.last_tick_raw = raw;
        // Position: the pulses that ended since the previous latch, all in
        // the current direction except those a reversal left over.
        let delta = i64::from(pulses.wrapping_sub(self.last_pulses));
        let sign = if self.negative { -1 } else { 1 };
        self.position += match self.carry.take() {
            Some(old) => -sign * i64::from(old) + sign * (delta - i64::from(old)),
            None => sign * delta,
        };
        self.last_pulses = pulses;
        self.seen_tick = true;

        // The schedule this tick should show if nothing changed since the
        // previous one.
        let expected = if self.changed || skipped != 0 {
            None
        } else {
            self.schedule.map(|before| before.advance(self.tick_counts))
        };
        let mut applied = false;
        let mut old_pulse_first = false;
        let mut dir_changed_at = None;
        let schedule = match self.pending.take() {
            Some(pending) if pending.start => {
                // A start from stopped: the hardware start fired at the tick
                // if the counter runs now. Either way the start source is
                // cleared, so a later stop stays stopped.
                self.step.gtssr.write(|w| unsafe { w.bits(0) });
                if pending.tick_raw == raw && self.is_running() {
                    applied = true;
                    self.lost = false;
                    self.period_counts = pending.period;
                    // A hardware start takes effect after one count clock
                    // (manual §22.9.4), so the counter at the tick instant
                    // is effectively one count behind the captured value
                    // (measured: the next tick's phase was 1 lower than a
                    // schedule from the captured value predicted). The
                    // period in progress is the first one, `crossing` long
                    // (longer than the steady period when a DIR change
                    // needed the wait, `first_cycle`).
                    Some(Schedule {
                        phase: phase.saturating_sub(1),
                        current: pending.crossing,
                        next: pending.period,
                        period: pending.period,
                    })
                } else {
                    self.lost = self.is_running();
                    None
                }
            }
            Some(pending) if pending.tick_raw == raw && !self.tick_dtc.armed() => {
                // The DTC ran at this tick. Its read-back says whether the
                // old period's pulse fired before the write (counter below
                // the captured phase) or after.
                applied = true;
                self.period_counts = pending.period;
                let readback = unsafe { read_volatile(addr_of!(DTC_READBACK)) };
                // A reversal's read-back comes after its delay record and DIR
                // write: in order the counter has advanced at least the
                // chain's minimum past the phase.
                let in_order_floor = if pending.reverse {
                    phase.saturating_add(self.dir_write_min)
                } else {
                    phase
                };
                old_pulse_first = readback < in_order_floor;
                if pending.reverse {
                    // DIR flipped: pulses that end in this tick but started
                    // before the flip (the one LOW at the tick, the one that
                    // fired inside the write latency) are old-direction.
                    let in_flight = phase < self.low_counts + PULSE_END_LATCH_DELAY;
                    self.carry = Some(u32::from(in_flight) + u32::from(old_pulse_first));
                    self.negative = !self.negative;
                    dir_changed_at = expected.map(|before| {
                        if old_pulse_first {
                            readback + (before.current - phase)
                        } else {
                            readback - phase
                        }
                    });
                }
                expected.map(|before| {
                    if old_pulse_first {
                        Schedule {
                            phase,
                            current: before.current,
                            next: pending.crossing,
                            period: pending.period,
                        }
                    } else {
                        Schedule {
                            phase,
                            current: pending.crossing,
                            next: pending.period,
                            period: pending.period,
                        }
                    }
                })
            }
            Some(pending) => {
                // The target tick was passed over. Either the transfer never
                // ran (still armed: cancel it), or it ran at that tick: the
                // change is in the hardware, and a reversal flipped DIR. The
                // schedule is rebuilt from the registers below.
                self.lost = true;
                if self.tick_dtc.armed() {
                    self.tick_dtc.disarm();
                } else {
                    applied = true;
                    self.period_counts = pending.period;
                    if pending.reverse {
                        // Every pulse since the previous latch was added in
                        // the old direction above; those after the reversal
                        // tick that started after the flip belong to the new
                        // one. The previous schedule was exact (a reversal
                        // is only queued with one) and the read-back
                        // survives, so the split is the polled-tick one.
                        if let Some(before) = self.schedule {
                            let readback = unsafe { read_volatile(addr_of!(DTC_READBACK)) };
                            let new = missed_reversal_new_pulses(
                                before,
                                self.tick_counts,
                                self.low_counts + PULSE_END_LATCH_DELAY,
                                readback,
                                self.dir_write_min,
                                delta as u32,
                            );
                            let old_sign = if self.negative { -1 } else { 1 };
                            self.position -= 2 * old_sign * i64::from(new);
                        }
                        self.negative = !self.negative;
                    }
                }
                None
            }
            None if self.changed && self.period_counts != 0 => {
                // run() since the previous tick: from the next tick on the
                // train is steady at the new period.
                None
            }
            None if self.period_counts != 0 && self.schedule.is_none() && !self.lost => {
                // One clean tick after run(): steady at the run period. Its
                // first period is longer than the steady one only when a
                // DIR change's set-up time did not fit (`first_cycle`); that
                // wait is at most 1 ms (`DirPinBuilder`), so the first
                // pulse came within 1 ms of run(), and this tick is more
                // than a tick (2 ms) after it.
                if skipped == 0 {
                    Some(Schedule::steady(phase, self.period_counts))
                } else {
                    None
                }
            }
            None => expected.map(|schedule| Schedule { phase, ..schedule }),
        };
        // Ticks passed over while the train ran unchanged lose the schedule;
        // ticks passed over while stopped, or before the first tick after a
        // run/stop, do not matter.
        if skipped != 0 && !self.changed && self.period_counts != 0 {
            self.lost = true;
        }
        let expected_phase = expected.map(|schedule| schedule.phase);
        self.schedule = if self.period_counts == 0 {
            None
        } else {
            schedule
        };
        self.changed = false;
        // A count stop that fired since the previous poll: settle it (the
        // pulses up to it are in the report above).
        self.sync_count_stop();
        // Ticks passed over: rebuild the schedule from the registers, so
        // `queue` works again without a stop.
        if self.lost
            && self.schedule.is_none()
            && self.is_running()
            && !self.tick_dtc.armed()
            && let Some(schedule) = self.resync(phase, raw)
        {
            self.schedule = Some(schedule);
            self.period_counts = schedule.period;
            self.lost = false;
        }
        // Idle (rate 0) with the last pulse over a second old: stop the
        // counter long before it could wrap (~89 s). Decided from the
        // registers and the captured phase alone — no schedule — so ticks
        // passed over cannot keep an idle generator running. No pulse is
        // due, so nothing about the count changes.
        let regs = self.step;
        if self.is_running()
            && idle_expired(
                phase,
                regs.gtpr.read().bits(),
                regs.gtpbr.read().bits(),
                IDLE_LEN,
                u64::from(self.pclkd_hz) * u64::from(IDLE_STOP_SECONDS),
            )
        {
            self.stop();
            self.changed = false;
            self.schedule = None;
            self.lost = false;
        }
        Some(TickReport {
            tick: self.tick,
            pulses,
            phase_counts: phase,
            skipped,
            applied,
            old_pulse_first,
            expected_phase,
            position: self.position,
            dir_changed_at,
        })
    }

    /// The pulse count expected at the next tick: from the latest tick's
    /// schedule ([`Schedule::ends_in`], with a pulse's end reaching the tick
    /// latch 2 counts after the low time, measured on the board), or the
    /// latched count when stopped
    /// throughout. `None` when it cannot be exact: no tick reported yet, a
    /// [`StepGenerator::run`] / [`StepGenerator::stop`] since the latest
    /// report, or an unknown schedule. A change queued for the next tick
    /// does not affect it (the change lands at that tick).
    pub fn predicted_pulses(&self) -> Option<u32> {
        if !self.seen_tick || self.changed {
            return None;
        }
        if self.period_counts == 0 {
            return Some(self.last_pulses);
        }
        let schedule = self.schedule?;
        let ends = schedule.ends_in(self.low_counts + PULSE_END_LATCH_DELAY, self.tick_counts);
        Some(self.last_pulses.wrapping_add(ends))
    }
}

/// Puts a GPT channel into a known stopped state: saw-wave, PCLKD/1, no
/// outputs, every hardware source cleared, no buffers, `GTPR = top`,
/// `GTCNT = 0`, status flags cleared. The caller then sets its count and
/// capture sources (or prescaler) and starts it.
fn reset_event_counter(regs: &GptRegs, top: u32) {
    regs.gtcr
        .write(|w| unsafe { w.md()._000().tpcs().bits(0).cst().clear_bit() });
    regs.gtuddtyc.write(|w| w.ud().set_bit());
    regs.gtber.write(|w| unsafe { w.bits(0) });
    regs.gtior.write(|w| unsafe { w.bits(0) });
    regs.gtssr.write(|w| unsafe { w.bits(0) });
    regs.gtpsr.write(|w| unsafe { w.bits(0) });
    regs.gtcsr.write(|w| unsafe { w.bits(0) });
    regs.gtupsr.write(|w| unsafe { w.bits(0) });
    regs.gtdnsr.write(|w| unsafe { w.bits(0) });
    regs.gticasr.write(|w| unsafe { w.bits(0) });
    regs.gticbsr.write(|w| unsafe { w.bits(0) });
    regs.gtpr.write(|w| unsafe { w.bits(top) });
    regs.gtcnt.write(|w| unsafe { w.bits(0) });
    regs.gtst.write(|w| unsafe { w.bits(0) });
}
