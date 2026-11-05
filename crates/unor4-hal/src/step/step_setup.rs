//! The whole step generator setup, cross-checked.

use crate::step::{CounterSetup, DirSetup, EnaSetup, NoEna, PulSetup, TickSetup};

/// A checked step generator setup: the PUL, DIR, optional ENA, counter and
/// tick parts, combined and cross-checked by
/// [`StepGeneratorBuilder`](crate::step::StepGeneratorBuilder) in a named
/// `const`. [`StepGenerator::new`](crate::step::StepGenerator::new) takes it
/// and claims exactly the hardware it names.
///
/// The type parameters are the parts' types: the PUL pin and divider, the
/// DIR pin, the counter, the tick and tick-number timers, and the ENA pin
/// ([`NoEna`] by default). A named `const` needs them spelled out on stable
/// Rust.
#[derive(Clone, Copy, Debug)]
pub struct StepSetup<P, D, Dp, C, T, N, E = NoEna> {
    pub(crate) pul: PulSetup<P, D>,
    pub(crate) dir: DirSetup<Dp>,
    pub(crate) _counter: CounterSetup<C>,
    pub(crate) _tick: TickSetup<T, N>,
    pub(crate) ena: EnaSetup<E>,
}
