//! The Arduino header pins, each free or taken by a driver.

use crate::gpio::{Input, Pin, pin_map};

/// The Arduino header pins of the selected board (feature `wifi` or
/// `minima`), in [`Peripherals::pins`](crate::Peripherals).
///
/// Each field is `Pin<Dn, Input>` (reset-default input) while the pin is
/// free, and [`Taken`](crate::peripherals::Taken) once a driver built with
/// `with_peripherals` owns it: using it again is a type error, so a pin
/// can never have two owners. The type parameters are the fields' types;
/// firmware never writes them out.
pub struct Pins<
    A0 = Pin<pin_map::A0, Input>,
    A1 = Pin<pin_map::A1, Input>,
    A2 = Pin<pin_map::A2, Input>,
    A3 = Pin<pin_map::A3, Input>,
    A4 = Pin<pin_map::A4, Input>,
    A5 = Pin<pin_map::A5, Input>,
    D0 = Pin<pin_map::D0, Input>,
    D1 = Pin<pin_map::D1, Input>,
    D2 = Pin<pin_map::D2, Input>,
    D3 = Pin<pin_map::D3, Input>,
    D4 = Pin<pin_map::D4, Input>,
    D5 = Pin<pin_map::D5, Input>,
    D6 = Pin<pin_map::D6, Input>,
    D7 = Pin<pin_map::D7, Input>,
    D8 = Pin<pin_map::D8, Input>,
    D9 = Pin<pin_map::D9, Input>,
    D10 = Pin<pin_map::D10, Input>,
    D11 = Pin<pin_map::D11, Input>,
    D12 = Pin<pin_map::D12, Input>,
    D13 = Pin<pin_map::D13, Input>,
> {
    /// Header A0.
    pub a0: A0,
    /// Header A1.
    pub a1: A1,
    /// Header A2.
    pub a2: A2,
    /// Header A3.
    pub a3: A3,
    /// Header A4.
    pub a4: A4,
    /// Header A5.
    pub a5: A5,
    /// Header D0.
    pub d0: D0,
    /// Header D1.
    pub d1: D1,
    /// Header D2.
    pub d2: D2,
    /// Header D3.
    pub d3: D3,
    /// Header D4.
    pub d4: D4,
    /// Header D5.
    pub d5: D5,
    /// Header D6.
    pub d6: D6,
    /// Header D7.
    pub d7: D7,
    /// Header D8.
    pub d8: D8,
    /// Header D9.
    pub d9: D9,
    /// Header D10.
    pub d10: D10,
    /// Header D11.
    pub d11: D11,
    /// Header D12.
    pub d12: D12,
    /// Header D13.
    pub d13: D13,
}

impl Pins {
    /// Every header pin, free. Consumes the GPIO-related PAC peripherals
    /// purely to guarantee exclusive ownership; register access goes
    /// through computed addresses. Both boards claim the same ports.
    pub(crate) fn new(
        _port0: ra4m1::PORT0,
        _port1: ra4m1::PORT1,
        _port3: ra4m1::PORT3,
        _port4: ra4m1::PORT4,
        _pfs: ra4m1::PFS,
        _pmisc: ra4m1::PMISC,
    ) -> Self {
        Self {
            a0: Pin::new(),
            a1: Pin::new(),
            a2: Pin::new(),
            a3: Pin::new(),
            a4: Pin::new(),
            a5: Pin::new(),
            d0: Pin::new(),
            d1: Pin::new(),
            d2: Pin::new(),
            d3: Pin::new(),
            d4: Pin::new(),
            d5: Pin::new(),
            d6: Pin::new(),
            d7: Pin::new(),
            d8: Pin::new(),
            d9: Pin::new(),
            d10: Pin::new(),
            d11: Pin::new(),
            d12: Pin::new(),
            d13: Pin::new(),
        }
    }
}
