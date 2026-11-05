//! The pins of the selected board: the Arduino headers `D0`–`D13` and
//! `A0`–`A5`, the board-internal pins the HAL drives, and what each can do.
//!
//! Each header has a type (`D6`), a value for `const` setups
//! (`pin_map::d6`) and a [`Pins`](crate::gpio::Pins) field (`Pin<D6, Input>`). A header's
//! functions are trait impls on its type (listed under "Trait
//! Implementations" in its docs), so a failed driver bound names the
//! header: `` `A0` has no GPT output ``. Board-internal pins (`UsbMux`,
//! `Sci1Tx`, …) are types too, handed out by
//! [`Peripherals`](crate::Peripherals). The RA4M1 port pin behind any of
//! them (P111) is only a property, [`PinId::PORT_PIN`], for reading the
//! manual.
//!
//! The table below is written per port pin, because the functions belong
//! to the silicon and are the same on both boards: one row gives a port
//! pin's functions once, then the header (and any internal name) it has on
//! each board.
//!
//! Sources: RA4M1 User's Manual R01UH0887 Rev 1.10, Tables 19.5 (PORT0),
//! 19.6–19.7 (PORT1), 19.10 (PORT3), 19.12 (PORT4) and 19.13 (PORT5):
//! `PSEL` functions, `ASEL` (ADC/DAC) and `ISEL` (IRQ). Board columns: the
//! Arduino core's variant tables, transcribed in `src/tests/pins.csv` and
//! checked by `src/tests/pins.rs`. Function classes not yet in the table:
//! AGT, KINT, CTSU, SLCDC, GPT triggers and output-disable pins, SSIE,
//! ADTRG, USB.

use crate::gpio::{
    AdcPin, CanPin, DacPin, GptOutput, Gtioc, HeaderPin, IicPin, IrqPin, PinId, SciPin, Sealed,
    SpiPin, psel,
    role::{CanRx, CanTx, Cts, Miso, Mosi, Rspck, Rxd, Sck, Scl, Sda, Ssl0, Ssl1, Ssl2, Ssl3, Txd},
};

macro_rules! pin_table {
    // One capability onto `$ty`, then the rest.
    (@cap $ty:ident;) => {};
    (@cap $ty:ident; gtioc $timer:ident $output:ident, $($rest:tt)*) => {
        impl Gtioc for $ty {
            type Timer = ra4m1::$timer;
            const OUTPUT: GptOutput = GptOutput::$output;
            const PSEL: u8 = psel::GPT_GTIOC;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; adc $channel:literal, $($rest:tt)*) => {
        impl AdcPin for $ty {
            const CHANNEL: u8 = $channel;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; dac $channel:literal, $($rest:tt)*) => {
        impl DacPin for $ty {
            const CHANNEL: u8 = $channel;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; irq $irq:literal, $($rest:tt)*) => {
        impl IrqPin for $ty {
            const IRQ: u8 = $irq;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; sci $unit:ident $role:ident $psel:literal, $($rest:tt)*) => {
        impl SciPin<ra4m1::$unit, $role> for $ty {
            const PSEL: u8 = $psel;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; iic $unit:ident $role:ident, $($rest:tt)*) => {
        impl IicPin<ra4m1::$unit, $role> for $ty {
            const PSEL: u8 = psel::IIC;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; spi $unit:ident $role:ident, $($rest:tt)*) => {
        impl SpiPin<ra4m1::$unit, $role> for $ty {
            const PSEL: u8 = psel::SPI;
        }
        pin_table!(@cap $ty; $($rest)*);
    };
    (@cap $ty:ident; can $unit:ident $role:ident, $($rest:tt)*) => {
        impl CanPin<ra4m1::$unit, $role> for $ty {
            const PSEL: u8 = psel::CAN;
        }
        pin_table!(@cap $ty; $($rest)*);
    };

    // A row's capability group onto `$ty`.
    (@caps $ty:ident { $($cap:tt)* }) => {
        pin_table!(@cap $ty; $($cap)*);
    };

    // A pin type on one board: identity and capabilities.
    (@pin $board:literal $ty:ident $port:ident $port_digit:literal $index:literal $caps:tt) => {
        #[cfg(feature = $board)]
        impl Sealed for $ty {}

        #[cfg(feature = $board)]
        impl PinId for $ty {
            const NAME: &'static str = stringify!($ty);
            const PORT_PIN: &'static str = stringify!($port);
            const PORT: char = $port_digit;
            const PIN: u8 = $index;
        }

        #[cfg(feature = $board)]
        pin_table!(@caps $ty $caps);
    };

    // A header on one board.
    (@header $board:literal $field:ident $header:ident
        $port:ident $port_digit:literal $index:literal $caps:tt) => {
        #[cfg(feature = $board)]
        #[doc = concat!("Header ", stringify!($header), " (RA4M1 port pin ", stringify!($port),
            "). What it can do: the trait implementations below.")]
        #[derive(Clone, Copy, Debug)]
        pub struct $header;

        #[cfg(feature = $board)]
        #[doc = concat!("Header ", stringify!($header), ", for `const` setups.")]
        #[allow(non_upper_case_globals)]
        pub const $field: $header = $header;

        #[cfg(feature = $board)]
        impl HeaderPin for $header {}

        pin_table!(@pin $board $header $port $port_digit $index $caps);
    };

    // A board-internal pin on one board.
    (@internal $board:literal $name:ident
        $port:ident $port_digit:literal $index:literal $caps:tt) => {
        #[cfg(feature = $board)]
        #[doc = concat!("Board-internal pin ", stringify!($name), " (RA4M1 port pin ", stringify!($port),
            "), handed out by `Peripherals`. What it can do: the trait implementations below.")]
        #[derive(Clone, Copy, Debug)]
        pub struct $name;

        pin_table!(@pin $board $name $port $port_digit $index $caps);
    };

    // The table.
    (
        $(
            $port:ident: $port_digit:literal, $index:literal,
                [ $( $board:literal $field:ident $header:ident ),* ]
                $caps:tt
                $( internal [ $( $iboard:literal $internal:ident ),* ] )? ;
        )*
    ) => {
        $(
            $( pin_table!(@header $board $field $header $port $port_digit $index $caps); )*
            $( $( pin_table!(@internal $iboard $internal $port $port_digit $index $caps); )* )?
        )*

        /// Visits the compiled board map as `(field name, port digit, pin
        /// index)`, for the golden test against `pins.csv`. Host-only and
        /// allocation-free, like the test.
        #[cfg(test)]
        pub(crate) fn for_each_header(mut visit: impl FnMut(&'static str, char, u8)) {
            $( $(
                #[cfg(feature = $board)]
                visit(stringify!($field), $port_digit, $index);
            )* )*
        }
    };
}

pin_table! {
    P000: '0', 0, ["wifi" a1 A1, "minima" a1 A1] {
        adc 0, irq 6,
    };
    P001: '0', 1, ["wifi" a2 A2, "minima" a2 A2] {
        adc 1, irq 7,
    };
    P002: '0', 2, ["wifi" a3 A3, "minima" a3 A3] {
        adc 2, irq 2,
    };
    P014: '0', 14, ["wifi" a0 A0, "minima" a0 A0] {
        adc 9, dac 0,
    };
    P100: '1', 0, ["wifi" a5 A5, "minima" a5 A5] {
        gtioc GPT165 B, adc 22, irq 2,
        sci SCI0 Rxd 0b00100, sci SCI1 Sck 0b00101,
        iic IIC1 Scl, spi SPI0 Miso,
    };
    P101: '1', 1, ["wifi" a4 A4, "minima" a4 A4] {
        gtioc GPT165 A, adc 21, irq 1,
        sci SCI0 Txd 0b00100, sci SCI1 Cts 0b00101,
        iic IIC1 Sda, spi SPI0 Mosi,
    };
    P102: '1', 2, ["wifi" d13 D13, "minima" d5 D5] {
        gtioc GPT162 B, adc 20,
        sci SCI0 Sck 0b00100, sci SCI2 Txd 0b00101,
        spi SPI0 Rspck, can CAN0 CanRx,
    };
    P103: '1', 3, ["wifi" d10 D10, "minima" d4 D4] {
        gtioc GPT162 A, adc 19,
        sci SCI0 Cts 0b00100,
        spi SPI0 Ssl0, can CAN0 CanTx,
    };
    P104: '1', 4, ["wifi" d2 D2, "minima" d3 D3] {
        gtioc GPT321 B, irq 1,
        sci SCI0 Rxd 0b00100,
        spi SPI0 Ssl1,
    };
    P105: '1', 5, ["wifi" d3 D3, "minima" d2 D2] {
        gtioc GPT321 A, irq 0,
        spi SPI0 Ssl2,
    };
    P106: '1', 6, ["wifi" d4 D4, "minima" d6 D6] {
        gtioc GPT320 B,
        spi SPI0 Ssl3,
    };
    P107: '1', 7, ["wifi" d5 D5, "minima" d7 D7] {
        gtioc GPT320 A,
    };
    P109: '1', 9, ["minima" d11 D11] {
        gtioc GPT321 A,
        sci SCI1 Sck 0b00100, sci SCI9 Txd 0b00101,
        spi SPI1 Mosi, can CAN0 CanTx,
    } internal ["wifi" Sci9Tx];
    P110: '1', 10, ["minima" d12 D12] {
        gtioc GPT321 B, irq 3,
        sci SCI2 Cts 0b00100, sci SCI9 Rxd 0b00101,
        spi SPI1 Miso, can CAN0 CanRx,
    } internal ["wifi" Sci9Rx];
    P111: '1', 11, ["wifi" d6 D6, "minima" d13 D13] {
        gtioc GPT163 A, irq 4,
        sci SCI2 Sck 0b00100, sci SCI9 Sck 0b00101,
        spi SPI1 Rspck,
    };
    P112: '1', 12, ["wifi" d7 D7, "minima" d10 D10] {
        gtioc GPT163 B,
        sci SCI2 Txd 0b00100, sci SCI1 Sck 0b00101,
        spi SPI1 Ssl0,
    };
    P301: '3', 1, ["wifi" d0 D0, "minima" d0 D0] {
        gtioc GPT164 B, irq 6,
        sci SCI2 Rxd 0b00100, sci SCI9 Cts 0b00101,
        spi SPI1 Ssl2,
    };
    P302: '3', 2, ["wifi" d1 D1, "minima" d1 D1] {
        gtioc GPT164 A, irq 5,
        sci SCI2 Txd 0b00100,
        spi SPI1 Ssl3,
    };
    P303: '3', 3, ["wifi" d9 D9, "minima" d9 D9] {
        gtioc GPT167 B,
    };
    P304: '3', 4, ["wifi" d8 D8, "minima" d8 D8] {
        gtioc GPT167 A, irq 9,
    };
    P408: '4', 8, [] {
        gtioc GPT165 B, irq 7,
        sci SCI1 Cts 0b00100, sci SCI9 Rxd 0b00101,
        iic IIC0 Scl,
    } internal ["wifi" UsbMux, "minima" UsbMux];
    P410: '4', 10, ["wifi" d12 D12] {
        gtioc GPT166 B, irq 5,
        sci SCI0 Rxd 0b00100,
        spi SPI0 Miso,
    };
    P411: '4', 11, ["wifi" d11 D11] {
        gtioc GPT166 A, irq 4,
        sci SCI0 Txd 0b00100,
        spi SPI0 Mosi,
    };
    P501: '5', 1, [] {
        gtioc GPT162 B, adc 17, irq 11,
        sci SCI1 Txd 0b00101,
    } internal ["wifi" Sci1Tx, "minima" Sci1Tx];
    P502: '5', 2, [] {
        gtioc GPT163 B, adc 18, irq 12,
        sci SCI1 Rxd 0b00101,
    } internal ["wifi" Sci1Rx, "minima" Sci1Rx];
}

/// The pull-down the board itself puts on a port pin, in ohms (0: none).
/// A switch's pull-up must hold the pin above VIH (0.8 × VCC, datasheet
/// R01DS0355 Table 2.4) against it.
///
/// - **WiFi** (schematic ABX00087, 23/08/2023): only P102 (D13) has one —
///   R52, 100 kΩ, on the gate of Q3, which drives the SCK LED. A4/A5's I²C
///   pull-ups (R4/R5) are marked "NOT MOUNTED"; every other header goes
///   straight to the connector (D11, D12, D13 also to the 6-pin J3).
/// - **Minima:** no schematic in the repo — unverified; 0 for every pin.
pub(crate) const fn board_pull_down_ohms(port: char, pin: u8) -> u32 {
    #[cfg(feature = "wifi")]
    if port == '1' && pin == 2 {
        return 100_000;
    }
    let _ = (port, pin);
    0
}
