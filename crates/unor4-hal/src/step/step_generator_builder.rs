//! The compile-time builder that combines the step generator's parts.
// The builder's typestate is eleven parameters wide (seven for the parts'
// types, four slots); firmware never names it.
#![allow(clippy::type_complexity)]

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::{Gtioc, PinId},
    pwm::GptChannel,
    step::{
        CounterSetup, DirSetup, EnaChoice, EnaSetup, NoEna, PulSetup, StepSetup, StepWidth,
        TickSetup,
    },
};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// Combines the checked parts into a [`StepSetup`] and cross-checks them.
/// Write it as a named `const`, like the parts:
///
/// ```no_run
/// # #[cfg(feature = "wifi")] {
/// use embedded_hal::digital::PinState;
/// use ra4m1::{GPT164, GPT165, GPT321};
/// use unor4_hal::{
///     gpio::pin_map::{self, D4, D5, D8},
///     step::{
///         CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup,
///         NoDivider, PulPinBuilder, PulSetup, StepGeneratorBuilder, StepSetup, TickBuilder,
///         TickSetup,
///     },
/// };
///
/// const PUL: PulSetup<D4, NoDivider> =
///     PulPinBuilder::new().with_pin(pin_map::d4).with_timing(5_000, 2_500).build();
/// const DIR: DirSetup<D5> = DirPinBuilder::new()
///     .with_pin(pin_map::d5)
///     .with_setup(5_000)
///     .with_forward(PinState::High)
///     .build();
/// const ENA: EnaSetup<D8> = EnaPinBuilder::new()
///     .with_pin(pin_map::d8)
///     .with_enabled(PinState::Low)
///     .with_lead(200_000_000)
///     .build();
/// const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
/// const TICK: TickSetup<GPT164, GPT165> =
///     TickBuilder::new().with_timers::<GPT164, GPT165>().build();
///
/// const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
///     StepGeneratorBuilder::new()
///         .with_pul(PUL)
///         .with_dir(DIR)
///         .with_ena(ENA)          // optional
///         .with_counter(COUNTER)
///         .with_tick(TICK)
///         .build();
/// # }
/// ```
///
/// Checks, all failing `cargo check`:
/// - every required part is set (`` setup incomplete: `with_dir` not set
///   ``); ENA is optional;
/// - no GPT channel is used twice (STEP, counter, tick, tick number);
/// - DIR is not on the PUL pin, and ENA is on neither;
/// - the PUL timer is 32-bit (16-bit STEP is not built yet).
///
/// The counter on the STEP timer fails to compile:
///
/// ```compile_fail,E0080
/// use embedded_hal::digital::PinState;
/// use ra4m1::{GPT164, GPT165, GPT321};
/// use unor4_hal::{
///     gpio::pin_map::{self, D2, D7},
///     step::{
///         CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, NoDivider, PulPinBuilder,
///         PulSetup, StepGeneratorBuilder, StepSetup, TickBuilder, TickSetup,
///     },
/// };
/// // D2 is on GPT321 on both boards.
/// const PUL: PulSetup<D2, NoDivider> =
///     PulPinBuilder::new().with_pin(pin_map::d2).with_timing(5_000, 2_500).build();
/// const DIR: DirSetup<D7> = DirPinBuilder::new()
///     .with_pin(pin_map::d7).with_setup(5_000).with_forward(PinState::High).build();
/// const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
/// const TICK: TickSetup<GPT164, GPT165> =
///     TickBuilder::new().with_timers::<GPT164, GPT165>().build();
/// const STEPPER: StepSetup<D2, NoDivider, D7, GPT321, GPT164, GPT165> = StepGeneratorBuilder::new()
///     .with_pul(PUL).with_dir(DIR).with_counter(COUNTER).with_tick(TICK).build();
/// ```
///
/// A missing part fails to compile, naming it:
///
/// ```compile_fail,E0277
/// use ra4m1::{GPT162, GPT164, GPT165};
/// use unor4_hal::{
///     gpio::pin_map::{self, D2, D7},
///     step::{
///         CounterBuilder, CounterSetup, NoDivider, PulPinBuilder, PulSetup,
///         StepGeneratorBuilder, StepSetup, TickBuilder, TickSetup,
///     },
/// };
/// const PUL: PulSetup<D2, NoDivider> =
///     PulPinBuilder::new().with_pin(pin_map::d2).with_timing(5_000, 2_500).build();
/// const COUNTER: CounterSetup<GPT162> = CounterBuilder::new().with_timer::<GPT162>().build();
/// const TICK: TickSetup<GPT164, GPT165> =
///     TickBuilder::new().with_timers::<GPT164, GPT165>().build();
/// // no with_dir
/// const STEPPER: StepSetup<D2, NoDivider, D7, GPT162, GPT164, GPT165> = StepGeneratorBuilder::new()
///     .with_pul(PUL).with_counter(COUNTER).with_tick(TICK).build();
/// ```
pub struct StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Di, Co, Ti> {
    pul: PulSetup<P, D>,
    dir: DirSetup<Dp>,
    counter: CounterSetup<C>,
    tick: TickSetup<T, N>,
    ena: EnaSetup<E>,
    _slots: PhantomData<(Pu, Di, Co, Ti)>,
}

impl<P, D, Dp, C, T, N>
    StepGeneratorBuilder<P, D, Dp, C, T, N, NoEna, Missing, Missing, Missing, Missing>
{
    /// An empty setup. The parts' types come from the `with_*` calls and
    /// the `const`'s type.
    pub const fn new() -> Self {
        // Placeholders: `build` needs every slot set, so none of these
        // values can reach a `StepSetup`.
        Self {
            pul: PulSetup {
                low_counts: 0,
                min_high_counts: 0,
                _types: PhantomData,
            },
            dir: DirSetup {
                setup_counts: 0,
                forward: PinState::High,
                _pin: PhantomData,
            },
            counter: CounterSetup {
                _timer: PhantomData,
            },
            tick: TickSetup {
                _timers: PhantomData,
            },
            ena: EnaSetup::NONE,
            _slots: PhantomData,
        }
    }
}

impl<P, D, Dp, C, T, N> Default
    for StepGeneratorBuilder<P, D, Dp, C, T, N, NoEna, Missing, Missing, Missing, Missing>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P, D, Dp, C, T, N, E, Pu, Di, Co, Ti>
    StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Di, Co, Ti>
{
    /// The STEP (PUL) part ([`PulPinBuilder`](crate::step::PulPinBuilder)).
    pub const fn with_pul(
        self,
        pul: PulSetup<P, D>,
    ) -> StepGeneratorBuilder<P, D, Dp, C, T, N, E, Set<()>, Di, Co, Ti> {
        StepGeneratorBuilder {
            pul,
            dir: self.dir,
            counter: self.counter,
            tick: self.tick,
            ena: self.ena,
            _slots: PhantomData,
        }
    }

    /// The DIR part ([`DirPinBuilder`](crate::step::DirPinBuilder)).
    pub const fn with_dir(
        self,
        dir: DirSetup<Dp>,
    ) -> StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Set<()>, Co, Ti> {
        StepGeneratorBuilder {
            pul: self.pul,
            dir,
            counter: self.counter,
            tick: self.tick,
            ena: self.ena,
            _slots: PhantomData,
        }
    }

    /// The pulse counter ([`CounterBuilder`](crate::step::CounterBuilder)).
    pub const fn with_counter(
        self,
        counter: CounterSetup<C>,
    ) -> StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Di, Set<()>, Ti> {
        StepGeneratorBuilder {
            pul: self.pul,
            dir: self.dir,
            counter,
            tick: self.tick,
            ena: self.ena,
            _slots: PhantomData,
        }
    }

    /// The tick and tick-number timers
    /// ([`TickBuilder`](crate::step::TickBuilder)).
    pub const fn with_tick(
        self,
        tick: TickSetup<T, N>,
    ) -> StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Di, Co, Set<()>> {
        StepGeneratorBuilder {
            pul: self.pul,
            dir: self.dir,
            counter: self.counter,
            tick,
            ena: self.ena,
            _slots: PhantomData,
        }
    }

    /// Checks the setup (see the type's docs).
    pub const fn build(self) -> StepSetup<P, D, Dp, C, T, N, E>
    where
        P: Gtioc,
        <P::Timer as GptChannel>::Width: StepWidth,
        Dp: PinId,
        C: GptChannel,
        T: GptChannel,
        N: GptChannel,
        E: EnaChoice,
        Pu: Provided<slot::with_pul>,
        Di: Provided<slot::with_dir>,
        Co: Provided<slot::with_counter>,
        Ti: Provided<slot::with_tick>,
    {
        let step = <P::Timer as GptChannel>::CHANNEL;
        assert!(
            C::CHANNEL != step,
            "StepGeneratorBuilder: the counter uses the PUL timer"
        );
        assert!(
            T::CHANNEL != step,
            "StepGeneratorBuilder: the tick uses the PUL timer"
        );
        assert!(
            N::CHANNEL != step,
            "StepGeneratorBuilder: the tick number uses the PUL timer"
        );
        assert!(
            C::CHANNEL != T::CHANNEL,
            "StepGeneratorBuilder: the counter uses the tick timer"
        );
        assert!(
            C::CHANNEL != N::CHANNEL,
            "StepGeneratorBuilder: the counter uses the tick-number timer"
        );
        assert!(
            !(P::PORT == Dp::PORT && P::PIN == Dp::PIN),
            "StepGeneratorBuilder: DIR is on the PUL pin"
        );
        if let Some((port, pin)) = E::PIN {
            assert!(
                !(P::PORT == port && P::PIN == pin),
                "StepGeneratorBuilder: ENA is on the PUL pin"
            );
            assert!(
                !(Dp::PORT == port && Dp::PIN == pin),
                "StepGeneratorBuilder: ENA is on the DIR pin"
            );
        }
        StepSetup {
            pul: self.pul,
            dir: self.dir,
            _counter: self.counter,
            _tick: self.tick,
            ena: self.ena,
        }
    }
}

impl<P, D, Dp, C, T, N, Pu, Di, Co, Ti>
    StepGeneratorBuilder<P, D, Dp, C, T, N, NoEna, Pu, Di, Co, Ti>
{
    /// The optional driver enable ([`EnaPinBuilder`](crate::step::EnaPinBuilder)).
    /// Without it the generator is always enabled.
    pub const fn with_ena<E: PinId>(
        self,
        ena: EnaSetup<E>,
    ) -> StepGeneratorBuilder<P, D, Dp, C, T, N, E, Pu, Di, Co, Ti> {
        StepGeneratorBuilder {
            pul: self.pul,
            dir: self.dir,
            counter: self.counter,
            tick: self.tick,
            ena,
            _slots: PhantomData,
        }
    }
}
