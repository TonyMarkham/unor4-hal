//! The run-time builder: a checked setup plus the peripherals it claims.
// The claim chain names every intermediate peripherals type; firmware
// never writes them out.
#![allow(clippy::type_complexity)]

use crate::{
    builder::{Missing, Provided, Set, slot},
    elc::{ElcB, ElcC, ElcD},
    gpio::{GptOutput, Gtioc, Input, Pin, PinId},
    intc::Ielsr,
    peripherals::{Board, Claim, Release},
    pwm::{GptChannel, Width},
    step::{Divider, EnaChoice, EnaPart, Hardware, StepError, StepGenerator, StepSetup, TickDtc},
};

/// The run-time builder [`StepGenerator::new`] returns: a checked
/// [`StepSetup`], then the peripherals to take its hardware from.
///
/// ```no_run
/// # #[cfg(feature = "wifi")] {
/// # use embedded_hal::digital::PinState;
/// # use ra4m1::{GPT164, GPT165, GPT321};
/// # use unor4_hal::{
/// #     gpio::pin_map::{self, D4, D5, D8},
/// #     step::{CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup,
/// #            NoDivider, PulPinBuilder, PulSetup, StepGeneratorBuilder, StepSetup, TickBuilder,
/// #            TickSetup},
/// # };
/// # const PUL: PulSetup<D4, NoDivider> =
/// #     PulPinBuilder::new().with_pin(pin_map::d4).with_timing(5_000, 2_500).build();
/// # const DIR: DirSetup<D5> = DirPinBuilder::new()
/// #     .with_pin(pin_map::d5).with_setup(5_000).with_forward(PinState::High).build();
/// # const ENA: EnaSetup<D8> = EnaPinBuilder::new()
/// #     .with_pin(pin_map::d8).with_enabled(PinState::Low).with_lead(200_000_000).build();
/// # const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
/// # const TICK: TickSetup<GPT164, GPT165> =
/// #     TickBuilder::new().with_timers::<GPT164, GPT165>().build();
/// # const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
/// #     StepGeneratorBuilder::new().with_pul(PUL).with_dir(DIR).with_ena(ENA)
/// #         .with_counter(COUNTER).with_tick(TICK).build();
/// use unor4_hal::{Peripherals, step::StepGenerator};
///
/// let p = Peripherals::take().unwrap();
/// let Ok((mut stepper, p)) = StepGenerator::new(STEPPER).with_peripherals(p).build() else {
///     panic!("no free interrupt slot");
/// };
/// # }
/// ```
///
/// `build()` claims exactly what the setup names — the STEP pin and its
/// timer, the DIR pin, the ENA pin (with ENA), the counter, the tick
/// timers and the DTC — plus the ELC inputs it links (B, C and D), and
/// returns the rest of the peripherals with those parts taken. Using one of them again (another driver on D5, say) fails
/// to compile: `` `D5` is already used by another driver ``. Without
/// `with_peripherals`, `build()` fails to compile: `` setup incomplete:
/// `with_peripherals` not set ``.
pub struct StepGeneratorInit<P, D, Dp, C, T, N, E, Pp> {
    setup: StepSetup<P, D, Dp, C, T, N, E>,
    peripherals: Pp,
}

impl StepGenerator {
    /// Starts building a generator from a checked `setup` (a named `const`,
    /// see [`StepGeneratorBuilder`](crate::step::StepGeneratorBuilder)).
    // The entry of the run-time builder (`new(SETUP).with_peripherals(p)
    // .build()`): it returns the builder, and `build` returns the driver.
    #[allow(clippy::new_ret_no_self)]
    pub const fn new<P, D, Dp, C, T, N, E>(
        setup: StepSetup<P, D, Dp, C, T, N, E>,
    ) -> StepGeneratorInit<P, D, Dp, C, T, N, E, Missing> {
        StepGeneratorInit {
            setup,
            peripherals: Missing,
        }
    }
}

impl<P, D, Dp, C, T, N, E> StepGeneratorInit<P, D, Dp, C, T, N, E, Missing> {
    /// The peripherals to claim the setup's hardware from:
    /// [`Peripherals::take`](crate::Peripherals::take)'s, or what another
    /// `build()` gave back.
    pub fn with_peripherals<X>(
        self,
        peripherals: X,
    ) -> StepGeneratorInit<P, D, Dp, C, T, N, E, Set<X>> {
        StepGeneratorInit {
            setup: self.setup,
            peripherals: Set(peripherals),
        }
    }
}

impl<P, D, Dp, C, T, N, E, Pp> StepGeneratorInit<P, D, Dp, C, T, N, E, Pp> {
    /// Claims the setup's hardware and starts the generator: stopped, the
    /// STEP pin HIGH, DIR at the forward level, ENA (if any) at the level
    /// that disables the driver, the counter and the 2 ms tick running.
    /// Returns it with the rest of the peripherals.
    ///
    /// The bounds sit on this method, not on the `impl`: on the `impl` a
    /// missing slot hides the method (E0599) instead of naming it.
    ///
    /// # Errors
    /// [`StepError::NoIcuSlot`] when fewer than two interrupt slots are
    /// free (one for the tick's DTC, one for the count stop's wake-up),
    /// with the peripherals given back. No pin or timer is touched in that
    /// case; the DTC may be left started, with no slot linked to it. Every
    /// other check ran at compile time.
    // The error hands the peripherals back whole; `no_std` has no box.
    #[allow(clippy::result_large_err)]
    pub fn build<X, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11>(
        self,
    ) -> Result<(StepGenerator, R11), (StepError, X)>
    where
        P: Gtioc,
        D: Divider,
        Dp: PinId,
        C: GptChannel,
        T: GptChannel,
        N: GptChannel,
        E: EnaChoice,
        E::Part: EnaPart<E>,
        Pp: Provided<slot::with_peripherals, Value = X>,
        X: Claim<ra4m1::DTC, Part = ra4m1::DTC, Rest = R1>,
        R1: Release<ra4m1::DTC, Part = ra4m1::DTC, Restored = X>
            + Claim<P, Part = Pin<P, Input>, Rest = R2>,
        R2: Claim<P::Timer, Part = P::Timer, Rest = R3>,
        R3: Claim<Dp, Part = Pin<Dp, Input>, Rest = R4>,
        R4: Claim<C, Part = C, Rest = R5>,
        R5: Claim<T, Part = T, Rest = R6>,
        R6: Claim<N, Part = N, Rest = R7>,
        R7: Claim<E, Part = E::Part, Rest = R8>,
        R8: Claim<ElcB, Part = ElcB, Rest = R9>,
        R9: Claim<ElcC, Part = ElcC, Rest = R10>,
        R10: Claim<ElcD, Part = ElcD, Rest = R11>,
        R11: Board,
    {
        let setup = self.setup;
        let (dtc, rest) = self.peripherals.into_value().claim();
        // The tick's DTC slot first: the only thing that can fail, before
        // anything is claimed or written.
        let tick_dtc = match TickDtc::new(dtc, T::OVERFLOW_EVENT) {
            Ok(tick_dtc) => tick_dtc,
            Err(dtc) => return Err((StepError::NoIcuSlot, rest.release(dtc))),
        };
        // The count stop's wake-up slot (`stop_at`, `set_count_wake`).
        let Some(count_slot) = Ielsr::take_any() else {
            return Err((StepError::NoIcuSlot, rest.release(tick_dtc.release())));
        };
        let (step_pin, rest) = rest.claim();
        let (step_timer, rest) = rest.claim();
        let (dir_pin, rest) = rest.claim();
        let (counter, rest) = rest.claim();
        let (tick, rest) = rest.claim();
        let (number, rest) = rest.claim();
        let (ena, rest) = rest.claim();
        // The ELC inputs it links: held by the generator from here on
        // (`free` gives them back), so no other driver can link into them.
        let (_pulse_end, rest): (ElcB, _) = rest.claim();
        let (_tick, rest): (ElcC, _) = rest.claim();
        let (_count_stop, mut rest): (ElcD, _) = rest.claim();
        let hardware = Hardware {
            step: step_timer.block(),
            step_bits: <<P::Timer as GptChannel>::Width as Width>::BITS,
            step_tpcs: D::TPCS,
            step_divider: D::VALUE,
            output: P::OUTPUT,
            pulse_end: match P::OUTPUT {
                GptOutput::A => <P::Timer as GptChannel>::ELC_COMPARE_A,
                GptOutput::B => <P::Timer as GptChannel>::ELC_COMPARE_B,
            },
            counter: counter.block(),
            counter_mask: <C::Width as Width>::MAX_GTPR,
            count_match: C::ELC_COMPARE_B,
            count_event: C::COMPARE_B_EVENT,
            tick: tick.block(),
            tick_elc: T::ELC_OVERFLOW,
            ticker: number.block(),
            dir_port: Dp::PORT,
            dir_pin: Dp::PIN,
            low_counts: setup.pul.low_counts,
            min_high_counts: setup.pul.min_high_counts,
            dir_setup_counts: setup.dir.setup_counts,
            forward: setup.dir.forward,
            ena_pin: E::PIN,
            ena_enabled: setup.ena.enabled,
            ena_lead_ns: setup.ena.lead_ns,
        };
        let clocks = rest.clocks();
        let generator = StepGenerator::start(
            hardware,
            tick_dtc,
            count_slot,
            rest.elc(),
            &clocks,
            || {
                <P::Timer as GptChannel>::release_module_stop();
                C::release_module_stop();
                T::release_module_stop();
                N::release_module_stop();
            },
            move || {
                let _step = step_pin.into_alternate(P::PSEL);
            },
            move |level| {
                let _dir = dir_pin.into_output_in_state(level);
            },
            move |level| ena.into_output(level),
        );
        Ok((generator, rest))
    }
}
