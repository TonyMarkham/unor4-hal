//! The compile-time STEP (PUL) setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::Gtioc,
    pwm::{GptChannel, Width},
    step::{
        Divider, DividerFor, EDGE_MARGIN, MAX_REVERSAL_LOW, NoDivider, PulSetup, counts_ceil,
        counts_floor,
    },
};
use core::marker::PhantomData;

/// Builds a [`PulSetup`]: the STEP pin, its timer's divider and the pulse
/// shape. Write it as a named `const` — `cargo check` evaluates those, not
/// inline `const { }` blocks — and every check below fails `cargo check`:
///
/// ```no_run
/// # #[cfg(feature = "wifi")] {
/// use unor4_hal::{gpio::pin_map::{self, D4}, step::{NoDivider, PulPinBuilder, PulSetup}};
///
/// const PUL: PulSetup<D4, NoDivider> = PulPinBuilder::new()
///     .with_pin(pin_map::d4)          // WiFi D4: GPT320, 32-bit, output B
///     .with_timing(5_000, 2_500)      // LOW 5 µs, HIGH at least 2.5 µs
///     .build();
/// # }
/// ```
///
/// - the pin has a GPT output: `` `A0` has no GPT output ``;
/// - the divider fits the timer width (a 32-bit timer takes none, a 16-bit
///   one `Div4`…`Div1024`): `` `Div16` cannot clock PUL pin `D4`: its timer
///   is `Bits32` ``;
/// - pin and timing are set: `` setup incomplete: `with_timing` not set ``;
/// - LOW is at least one timer count and at most 400 PCLKD counts (8.3 µs:
///   a reversal's delayed DIR write must outlast a pulse); HIGH is at least
///   one timer count and 1 µs (the tick's DTC write must land first); the
///   shape fits the timer's longest period; a stated width matches.
///
/// A pin without a GPT output, a divider on a 32-bit timer and a LOW time
/// over the DIR window each fail to compile:
///
/// ```compile_fail,E0277
/// use unor4_hal::{gpio::pin_map::{self, A0}, step::{NoDivider, PulPinBuilder, PulSetup}};
/// const PUL: PulSetup<A0, NoDivider> =
///     PulPinBuilder::new().with_pin(pin_map::a0).with_timing(5_000, 2_500).build();
/// ```
///
/// ```compile_fail,E0277
/// // D3 is on 32-bit GPT321 on both boards.
/// use unor4_hal::{gpio::pin_map::{self, D3}, step::{Div16, PulPinBuilder, PulSetup}};
/// const PUL: PulSetup<D3, Div16> = PulPinBuilder::new()
///     .with_pin(pin_map::d3).with_divider(Div16).with_timing(5_000, 2_500).build();
/// ```
///
/// ```compile_fail,E0080
/// use unor4_hal::{gpio::pin_map::{self, D2}, step::{NoDivider, PulPinBuilder, PulSetup}};
/// const PUL: PulSetup<D2, NoDivider> =
///     PulPinBuilder::new().with_pin(pin_map::d2).with_timing(9_000, 2_500).build();
/// ```
///
/// Type parameters are the slots: pin `P` (`Missing` or `Set<pin>`),
/// divider `D`, timing `T` (`Missing` or `Set<()>`; the values live in the
/// builder so the `const fn` can read them).
pub struct PulPinBuilder<P, D, T> {
    width: u8,
    low_ns: u32,
    min_high_ns: u32,
    _slots: PhantomData<(P, D, T)>,
}

impl PulPinBuilder<Missing, NoDivider, Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self {
            width: 0,
            low_ns: 0,
            min_high_ns: 0,
            _slots: PhantomData,
        }
    }
}

impl Default for PulPinBuilder<Missing, NoDivider, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, D, T> PulPinBuilder<P, D, T> {
    /// The STEP pin: any pin with a GPT output (a header from
    /// [`pin_map`](crate::gpio::pin_map), e.g. `pin_map::d4`).
    pub const fn with_pin<Id: Gtioc>(self, _pin: Id) -> PulPinBuilder<Set<Id>, D, T> {
        PulPinBuilder {
            width: self.width,
            low_ns: self.low_ns,
            min_high_ns: self.min_high_ns,
            _slots: PhantomData,
        }
    }

    /// Optional cross-check: the timer width the firmware expects, 16 or 32.
    pub const fn with_type(self, bits: u8) -> Self {
        assert!(bits == 16 || bits == 32, "with_type: 16 or 32");
        Self {
            width: bits,
            ..self
        }
    }

    /// The STEP timer's count-clock divider; any of the six can be set, and
    /// `build()` checks it against the timer's width.
    pub const fn with_divider<D2: Divider>(self, _divider: D2) -> PulPinBuilder<P, D2, T> {
        PulPinBuilder {
            width: self.width,
            low_ns: self.low_ns,
            min_high_ns: self.min_high_ns,
            _slots: PhantomData,
        }
    }

    /// The pulse shape: LOW time and minimum HIGH time, in nanoseconds,
    /// each rounded up to whole timer counts.
    pub const fn with_timing(self, low_ns: u32, min_high_ns: u32) -> PulPinBuilder<P, D, Set<()>> {
        PulPinBuilder {
            width: self.width,
            low_ns,
            min_high_ns,
            _slots: PhantomData,
        }
    }

    /// Checks the setup (see the type's docs for every check).
    pub const fn build<Id>(self) -> PulSetup<Id, D>
    where
        P: Provided<slot::with_pin, Value = Id>,
        T: Provided<slot::with_timing>,
        Id: Gtioc,
        D: Divider + DividerFor<Id, <Id::Timer as GptChannel>::Width>,
    {
        let width = <<Id::Timer as GptChannel>::Width as Width>::BITS;
        let max_gtpr = <<Id::Timer as GptChannel>::Width as Width>::MAX_GTPR;
        if self.width != 0 {
            assert!(
                self.width == width,
                "with_type: stated width differs from the PUL pin's timer"
            );
        }
        let low = counts_ceil(self.low_ns, D::VALUE);
        let high = counts_ceil(self.min_high_ns, D::VALUE);
        assert!(
            counts_floor(self.low_ns, D::VALUE) >= 1,
            "with_timing: LOW time under 1 timer count at this divider"
        );
        assert!(
            counts_floor(self.min_high_ns, D::VALUE) >= 1,
            "with_timing: HIGH time under 1 timer count at this divider"
        );
        assert!(
            low * D::VALUE as u64 <= MAX_REVERSAL_LOW as u64,
            "with_timing: LOW time over 8.3 us (400 PCLKD counts, the DIR write window)"
        );
        assert!(
            high * D::VALUE as u64 >= 2 * EDGE_MARGIN as u64,
            "with_timing: minimum HIGH time under 1 us (the tick's DTC write must land first)"
        );
        assert!(
            low + high <= max_gtpr as u64,
            "with_timing: the pulse shape is longer than the timer's longest period"
        );
        PulSetup {
            low_counts: low as u32,
            min_high_counts: high as u32,
            _types: PhantomData,
        }
    }
}
