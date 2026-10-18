//! The Arduino header pins.

use crate::gpio::{Input, pin::Pin};

/// Arduino header pins (UNO R4 WiFi map, verified against the Arduino
/// core's pin table). All fields start in reset-default input mode.
#[allow(missing_docs)]
pub struct Pins {
    pub d0: Pin<'3', 1, Input>, // RX
    pub d1: Pin<'3', 2, Input>, // TX
    pub d2: Pin<'1', 4, Input>,
    pub d3: Pin<'1', 5, Input>,
    pub d4: Pin<'1', 6, Input>,
    pub d5: Pin<'1', 7, Input>,
    pub d6: Pin<'1', 11, Input>,
    pub d7: Pin<'1', 12, Input>,
    pub d8: Pin<'3', 4, Input>,
    pub d9: Pin<'3', 3, Input>,
    pub d10: Pin<'1', 3, Input>,
    pub d11: Pin<'4', 11, Input>,
    pub d12: Pin<'4', 10, Input>,
    pub d13: Pin<'1', 2, Input>, // onboard LED (P102)
    pub a0: Pin<'0', 14, Input>,
    pub a1: Pin<'0', 0, Input>,
    pub a2: Pin<'0', 1, Input>,
    pub a3: Pin<'0', 2, Input>,
    pub a4: Pin<'1', 1, Input>, // SDA
    pub a5: Pin<'1', 0, Input>, // SCL
}

impl Pins {
    /// Consumes the GPIO-related PAC peripherals purely to guarantee
    /// exclusive ownership; register access goes through computed
    /// addresses.
    pub fn new(
        _port0: ra4m1::PORT0,
        _port1: ra4m1::PORT1,
        _port3: ra4m1::PORT3,
        _port4: ra4m1::PORT4,
        _pfs: ra4m1::PFS,
        _pmisc: ra4m1::PMISC,
    ) -> Self {
        Self {
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
            a0: Pin::new(),
            a1: Pin::new(),
            a2: Pin::new(),
            a3: Pin::new(),
            a4: Pin::new(),
            a5: Pin::new(),
        }
    }
}
