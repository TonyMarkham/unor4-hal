//! The compile-time ENA (driver enable) setup builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::{HeaderPin, PinId, pin_map::board_pull_down_ohms},
    step::EnaSetup,
};
use core::marker::PhantomData;
use embedded_hal::digital::PinState;

/// Builds an [`EnaSetup`]: the driver's enable pin, the level that enables
/// the driver, and the lead time from enabling to the first DIR change or
/// STEP pulse. Generic on purpose: the values are the driver's, so the
/// firmware that knows its driver supplies them. Write it as a named
/// `const` and add it with
/// [`StepGeneratorBuilder::with_ena`](crate::step::StepGeneratorBuilder::with_ena);
/// a driver without a wired enable input leaves it out.
///
/// ```no_run
/// use embedded_hal::digital::PinState;
/// use unor4_hal::{gpio::pin_map::{self, D8}, step::{EnaPinBuilder, EnaSetup}};
///
/// const ENA: EnaSetup<D8> = EnaPinBuilder::new()
///     .with_pin(pin_map::d8)          // any header pin
///     .with_enabled(PinState::Low)    // the level that enables this driver
///     .with_lead(5_000)               // 5 µs from enable to the first DIR change or pulse
///     .build();
/// ```
///
/// Checks, all failing `cargo check`:
/// - the pin is an Arduino header (every header can drive an output); a
///   board-internal pin (`UsbMux`, the ESP32-S3 bridge's `Sci9Tx`, …) is
///   refused: `` `UsbMux` is a board-internal pin, not an Arduino header ``;
/// - every part is set, the enabling level included — it differs between
///   drivers, so it has no default: `` setup incomplete: `with_enabled` not
///   set ``;
/// - the driver is not enabled before the firmware runs. Until firmware
///   drives the pin (reset, the bootloader) it is an input with its
///   pull-up off (manual §19.2.5), so the board decides its level. Most
///   headers float, and then the driver's own default decides. WiFi D13 is
///   held LOW by R52 (100 kΩ, the SCK LED's transistor gate; schematic
///   ABX00087), so ENA there must not enable on LOW: `` with_enabled: the
///   board holds this pin LOW while it is not driven ``. Minima board loads
///   are unverified (no schematic in the repo);
/// - [`StepGeneratorBuilder`](crate::step::StepGeneratorBuilder) refuses ENA
///   on the PUL or the DIR pin.
///
/// The lead needs no bound of its own: the generator times it on the tick
/// timer (PCLKD/4, rounded up to whole counts) and the 16-bit tick number,
/// which wraps after 131 s; the longest lead, `u32::MAX` ns, is 4.29 s.
///
/// A board-internal pin, a missing enabling level and a forgotten lead
/// each fail to compile:
///
/// ```compile_fail,E0277
/// use embedded_hal::digital::PinState;
/// use unor4_hal::{gpio::pin_map::{self, UsbMux}, step::{EnaPinBuilder, EnaSetup}};
/// const ENA: EnaSetup<UsbMux> = EnaPinBuilder::new()
///     .with_pin(pin_map::UsbMux).with_enabled(PinState::Low).with_lead(5_000).build();
/// ```
///
/// ```compile_fail,E0277
/// use unor4_hal::{gpio::pin_map::{self, D8}, step::{EnaPinBuilder, EnaSetup}};
/// const ENA: EnaSetup<D8> =
///     EnaPinBuilder::new().with_pin(pin_map::d8).with_lead(5_000).build();
/// ```
///
/// ```compile_fail,E0277
/// use embedded_hal::digital::PinState;
/// use unor4_hal::{gpio::pin_map::{self, D8}, step::{EnaPinBuilder, EnaSetup}};
/// const ENA: EnaSetup<D8> =
///     EnaPinBuilder::new().with_pin(pin_map::d8).with_enabled(PinState::Low).build();
/// ```
///
/// The generator enforces the lead at run time: after
/// [`StepGenerator::enable`](crate::step::StepGenerator::enable), a non-zero
/// rate is refused with [`StepError::Enabling`](crate::step::StepError::Enabling)
/// until the lead has passed.
pub struct EnaPinBuilder<P, E, L> {
    enabled: PinState,
    lead_ns: u32,
    _slots: PhantomData<(P, E, L)>,
}

impl EnaPinBuilder<Missing, Missing, Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self {
            enabled: PinState::Low,
            lead_ns: 0,
            _slots: PhantomData,
        }
    }
}

impl Default for EnaPinBuilder<Missing, Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, E, L> EnaPinBuilder<P, E, L> {
    /// The ENA pin: any header from [`pin_map`](crate::gpio::pin_map),
    /// e.g. `pin_map::d8`.
    pub const fn with_pin<Id: HeaderPin>(self, _pin: Id) -> EnaPinBuilder<Set<Id>, E, L> {
        EnaPinBuilder {
            enabled: self.enabled,
            lead_ns: self.lead_ns,
            _slots: PhantomData,
        }
    }

    /// The pin level that enables the driver; the other level disables it.
    pub const fn with_enabled(self, enabled: PinState) -> EnaPinBuilder<P, Set<()>, L> {
        EnaPinBuilder {
            enabled,
            lead_ns: self.lead_ns,
            _slots: PhantomData,
        }
    }

    /// The shortest time from enabling the driver to the first DIR change
    /// or STEP pulse, ns (0 for none; up to `u32::MAX` ns, 4.29 s).
    pub const fn with_lead(self, lead_ns: u32) -> EnaPinBuilder<P, E, Set<()>> {
        EnaPinBuilder {
            enabled: self.enabled,
            lead_ns,
            _slots: PhantomData,
        }
    }

    /// Checks the setup (see the type's docs).
    pub const fn build<Id>(self) -> EnaSetup<Id>
    where
        P: Provided<slot::with_pin, Value = Id>,
        E: Provided<slot::with_enabled>,
        L: Provided<slot::with_lead>,
        Id: PinId + HeaderPin,
    {
        let held_low = board_pull_down_ohms(Id::PORT, Id::PIN) != 0;
        assert!(
            !(held_low && matches!(self.enabled, PinState::Low)),
            "with_enabled: the board holds this pin LOW while it is not driven (reset, \
             bootloader), so a driver enabled on LOW would run before the firmware does"
        );
        EnaSetup {
            enabled: self.enabled,
            lead_ns: self.lead_ns,
            _pin: PhantomData,
        }
    }
}
