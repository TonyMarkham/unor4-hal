//! The compile-time switch input builder.

use crate::{
    builder::{Missing, Provided, Set, slot},
    gpio::{
        HeaderPin, IrqPin, PinId, Pull, SwitchFilter, SwitchSetup, contact::Contact,
        pin_map::board_pull_down_ohms,
    },
};

/// The internal pull-up's highest value, ohms (datasheet R01DS0355 Rev
/// 1.10, Table 2.10: 10–50 kΩ).
const INTERNAL_PULL_UP_MAX_OHMS: u64 = 50_000;
use core::marker::PhantomData;

/// Builds a [`SwitchSetup`]: a switch between a header pin and GND — a
/// limit switch, a button, a door contact. Generic: what the switch means
/// is the firmware's business. Write it as a named `const` (`cargo check`
/// evaluates those):
///
/// ```no_run
/// use unor4_hal::gpio::{
///     Pull, SwitchFilter, SwitchPinBuilder, SwitchSetup,
///     contact::Nc,
///     pin_map::{self, D2},
/// };
///
/// const LIMIT: SwitchSetup<D2, Nc> = SwitchPinBuilder::new()
///     .with_pin(pin_map::d2)                  // any header pin
///     .with_contact(Nc)                       // No or Nc: which level is "active"
///     .with_pull(Pull::Internal)              // or Pull::External(ohms)
///     .with_edge()                            // optional, IRQ pins only: latch presses
///     .with_filter(SwitchFilter::PclkbDiv64)  // optional, with with_edge
///     .with_wake()                            // optional, with with_edge: wake from WFI
///     .build();
/// ```
///
/// Checks, all failing `cargo check`:
/// - the pin is an Arduino header, not a board-internal pin: `` `…::UsbMux`
///   is a board-internal pin, not an Arduino header ``;
/// - the contact type and the pull are set — neither has a default:
///   `` setup incomplete: `with_contact` not set ``;
/// - a filter only on a pin with an external interrupt (`IRQn`), and only
///   after the pin: `` `…::A0` has no external interrupt ``. The edge latch
///   (`with_edge`) and the wake-up (`with_wake`) have the same bound, and
///   a filter and a wake-up each need the edge latch: `` with_wake: the
///   wake-up is the edge latch's interrupt; add with_edge() ``;
/// - the pull-up holds the pin above VIH (0.8 × VCC, datasheet R01DS0355
///   Table 2.4) against what the board itself puts on the pin. Every
///   header can be a switch on the silicon (every port pin has a pull-up,
///   manual §19.2.5), but on the WiFi board D13 carries a 100 kΩ pull-down
///   (R52, the SCK LED's transistor gate; schematic ABX00087). The internal
///   pull-up (up to 50 kΩ) only reaches 0.67 × VCC against it, so D13 needs
///   `Pull::External` of at most 25 kΩ: `` with_pull: the board pulls this
///   pin down; the internal pull-up (10-50 kOhm) cannot hold it above VIH
///   ``.
///   Minima board loads are unverified (no schematic in the repo);
/// - an external pull-up is not 0 Ω (that ties the pin to VCC, and a
///   pressed NO switch would short the supply).
///
/// A missing contact type, a missing pull and a filter on a pin without an
/// IRQ each fail to compile (the board-load cases are per board, so they
/// are checked by the harness outside the repo, not as doc tests):
///
/// ```compile_fail,E0277
/// use unor4_hal::gpio::{Pull, SwitchPinBuilder, SwitchSetup, contact::No, pin_map::{self, D2}};
/// const S: SwitchSetup<D2, No> =
///     SwitchPinBuilder::new().with_pin(pin_map::d2).with_pull(Pull::Internal).build();
/// ```
///
/// ```compile_fail,E0277
/// use unor4_hal::gpio::{SwitchPinBuilder, SwitchSetup, contact::No, pin_map::{self, D2}};
/// const S: SwitchSetup<D2, No> =
///     SwitchPinBuilder::new().with_pin(pin_map::d2).with_contact(No).build();
/// ```
///
/// ```compile_fail,E0277
/// use unor4_hal::gpio::{
///     Pull, SwitchFilter, SwitchPinBuilder, SwitchSetup, contact::No, pin_map::{self, A0},
/// };
/// const S: SwitchSetup<A0, No> = SwitchPinBuilder::new()
///     .with_pin(pin_map::a0)
///     .with_contact(No)
///     .with_pull(Pull::Internal)
///     .with_edge()
///     .with_filter(SwitchFilter::Pclkb)
///     .build();
/// ```
pub struct SwitchPinBuilder<P, C, U> {
    pull: Pull,
    filter: Option<SwitchFilter>,
    irq: Option<u8>,
    wake: bool,
    _slots: PhantomData<(P, C, U)>,
}

impl SwitchPinBuilder<Missing, Missing, Missing> {
    /// An empty setup.
    pub const fn new() -> Self {
        Self {
            pull: Pull::Internal,
            filter: None,
            irq: None,
            wake: false,
            _slots: PhantomData,
        }
    }
}

impl Default for SwitchPinBuilder<Missing, Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P, C, U> SwitchPinBuilder<P, C, U> {
    /// The switch's pin: any header from [`pin_map`](crate::gpio::pin_map),
    /// e.g. `pin_map::d2`.
    pub const fn with_pin<Id: HeaderPin>(self, _pin: Id) -> SwitchPinBuilder<Set<Id>, C, U> {
        SwitchPinBuilder {
            pull: self.pull,
            filter: self.filter,
            irq: self.irq,
            wake: self.wake,
            _slots: PhantomData,
        }
    }

    /// The contact type, [`No`](crate::gpio::contact::No) or
    /// [`Nc`](crate::gpio::contact::Nc): decides which level is active.
    pub const fn with_contact<Ct: Contact>(self, _contact: Ct) -> SwitchPinBuilder<P, Set<Ct>, U> {
        SwitchPinBuilder {
            pull: self.pull,
            filter: self.filter,
            irq: self.irq,
            wake: self.wake,
            _slots: PhantomData,
        }
    }

    /// Where the pull-up comes from.
    pub const fn with_pull(self, pull: Pull) -> SwitchPinBuilder<P, C, Set<()>> {
        SwitchPinBuilder {
            pull,
            filter: self.filter,
            irq: self.irq,
            wake: self.wake,
            _slots: PhantomData,
        }
    }

    /// Latch the edge into the active level in hardware, so a press
    /// shorter than the poll interval is not missed
    /// ([`Switch::activated`](crate::gpio::Switch::activated)). Only for a
    /// pin with an external interrupt (`IRQn`), so the pin comes first.
    pub const fn with_edge<Id>(self) -> Self
    where
        P: Provided<slot::with_pin, Value = Id>,
        Id: IrqPin,
    {
        Self {
            pull: self.pull,
            filter: self.filter,
            irq: Some(Id::IRQ),
            wake: self.wake,
            _slots: PhantomData,
        }
    }

    /// The IRQ digital filter for the activation edge (see
    /// [`SwitchFilter`]). Only for a pin with an external interrupt, so the
    /// pin comes first; it acts on the edge latch, so it needs
    /// [`with_edge`](SwitchPinBuilder::with_edge).
    pub const fn with_filter<Id>(self, filter: SwitchFilter) -> Self
    where
        P: Provided<slot::with_pin, Value = Id>,
        Id: IrqPin,
    {
        Self {
            pull: self.pull,
            filter: Some(filter),
            irq: self.irq,
            wake: self.wake,
            _slots: PhantomData,
        }
    }

    /// Wake the CPU on the activation edge: the edge latch's interrupt is
    /// enabled, so the edge ends a `WFI`
    /// ([`cortex_m::asm::wfi`](https://docs.rs/cortex-m/0.7/cortex_m/asm/fn.wfi.html)).
    /// [`Switch::activated`](crate::gpio::Switch::activated) still reports
    /// it. Only for a pin with an external interrupt, so the pin comes
    /// first; it needs [`with_edge`](SwitchPinBuilder::with_edge). Unmask
    /// CPU interrupts ([`crate::intc::enable_interrupts`]) so the HAL's
    /// vector table can acknowledge the request; while they are masked a
    /// `WFI` returns at once for as long as the request is pending.
    pub const fn with_wake<Id>(self) -> Self
    where
        P: Provided<slot::with_pin, Value = Id>,
        Id: IrqPin,
    {
        Self {
            pull: self.pull,
            filter: self.filter,
            irq: self.irq,
            wake: true,
            _slots: PhantomData,
        }
    }

    /// Checks the setup (see the type's docs).
    pub const fn build<Id, Ct>(self) -> SwitchSetup<Id, Ct>
    where
        P: Provided<slot::with_pin, Value = Id>,
        C: Provided<slot::with_contact, Value = Ct>,
        U: Provided<slot::with_pull>,
        Id: PinId + HeaderPin,
        Ct: Contact,
    {
        let pull_down = board_pull_down_ohms(Id::PORT, Id::PIN) as u64;
        match self.pull {
            Pull::Internal => assert!(
                pull_down == 0 || pull_down >= 4 * INTERNAL_PULL_UP_MAX_OHMS,
                "with_pull: the board pulls this pin down; the internal pull-up (10-50 kOhm) \
                 cannot hold it above VIH (0.8 x VCC): use Pull::External"
            ),
            Pull::External(ohms) => {
                assert!(
                    ohms != 0,
                    "with_pull: an external pull-up of 0 Ohm ties the pin to VCC"
                );
                assert!(
                    pull_down == 0 || pull_down >= 4 * ohms as u64,
                    "with_pull: the external pull-up is too weak for the board's pull-down on \
                     this pin: at most a quarter of it (25 kOhm on WiFi D13)"
                );
            }
        }
        assert!(
            self.filter.is_none() || self.irq.is_some(),
            "with_filter: the filter acts on the edge latch; add with_edge()"
        );
        assert!(
            !self.wake || self.irq.is_some(),
            "with_wake: the wake-up is the edge latch's interrupt; add with_edge()"
        );
        SwitchSetup {
            pull: self.pull,
            filter: self.filter,
            irq: self.irq,
            wake: self.wake,
            _types: PhantomData,
        }
    }
}
