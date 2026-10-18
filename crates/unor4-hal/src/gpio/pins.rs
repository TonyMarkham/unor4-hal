//! The Arduino header pins — one map per board, selected by crate feature.
//!
//! Correctness chain (HalSpec §5.2, §7.2): both maps derive from the
//! Arduino core's board tables, transcribed into `pins.csv` (cross-check
//! data only, HalSpec §1 — nothing copied). The golden test in
//! `src/tests/pins.rs` proves the compiled map equals that transcription.
//! The WiFi map is additionally hardware-proven: slice 1 blinks
//! D13 = P102 on the real board.
//!
//! Docs-trap mechanism: every pin's doc shows the mapping for **both**
//! boards, regardless of which feature is active.

use crate::gpio::{Input, pin::Pin};

/// Stamps out one board's `Pins` struct, its constructor, and the
/// `PORT_PINS` table the golden test checks against `pins.csv`.
///
/// Single source of truth: the struct fields' *types* and the test table
/// are generated from the same entries, so the compiled map and its test
/// shadow cannot drift apart.
macro_rules! pin_map {
    (
        $(#[$meta:meta])*
        $( $field:ident : $port:literal , $index:literal ; $doc:literal ),* $(,)?
    ) => {
        $(#[$meta])*
        pub struct Pins {
            $(
                #[doc = $doc]
                pub $field: Pin<$port, $index, Input>,
            )*
        }

        impl Pins {
            /// Consumes the GPIO-related PAC peripherals purely to
            /// guarantee exclusive ownership; register access goes through
            /// computed addresses. Both boards claim the same ports — the
            /// signature stays feature-independent on purpose.
            pub fn new(
                _port0: ra4m1::PORT0,
                _port1: ra4m1::PORT1,
                _port3: ra4m1::PORT3,
                _port4: ra4m1::PORT4,
                _pfs: ra4m1::PFS,
                _pmisc: ra4m1::PMISC,
            ) -> Self {
                Self {
                    $( $field: Pin::new(), )*
                }
            }
        }

        /// The compiled map as `(field name, port letter, pin index)`.
        /// `pub(crate)`: read by the golden test in `src/tests/pins.rs`
        /// without joining the public API. Host-only (`cfg(test)`) —
        /// firmware never needs it.
        #[cfg(test)]
        pub(crate) const PORT_PINS: [(&str, char, u8); 20] = [
            $( (stringify!($field), $port, $index), )*
        ];
    };
}

// ---------------------------------------------------------------------------------------------- //

#[cfg(feature = "wifi")]
pin_map! {
    /// Arduino header pins — UNO R4 **WiFi** map (feature `wifi`).
    ///
    /// All fields start in reset-default input mode.
    d0: '3', 1; "D0 — WiFi = P301, Minima = P301. SCI2 RX (UART).",
    d1: '3', 2; "D1 — WiFi = P302, Minima = P302. SCI2 TX (UART).",
    d2: '1', 4; "D2 — WiFi = P104, Minima = P105.",
    d3: '1', 5; "D3 — WiFi = P105, Minima = P104.",
    d4: '1', 6; "D4 — WiFi = P106, Minima = P103.",
    d5: '1', 7; "D5 — WiFi = P107, Minima = P102.",
    d6: '1', 11; "D6 — WiFi = P111, Minima = P106.",
    d7: '1', 12; "D7 — WiFi = P112, Minima = P107.",
    d8: '3', 4; "D8 — WiFi = P304, Minima = P304.",
    d9: '3', 3; "D9 — WiFi = P303, Minima = P303.",
    d10: '1', 3; "D10 — WiFi = P103, Minima = P112.",
    d11: '4', 11; "D11 — WiFi = P411, Minima = P109 (SCI9 TX on Minima).",
    d12: '4', 10; "D12 — WiFi = P410, Minima = P110 (SCI9 RX on Minima).",
    d13: '1', 2; "D13 — WiFi = P102 (onboard LED), Minima = P111.",
    a0: '0', 14; "A0 — WiFi = P014, Minima = P014. Also DAC0.",
    a1: '0', 0; "A1 — WiFi = P000, Minima = P000.",
    a2: '0', 1; "A2 — WiFi = P001, Minima = P001.",
    a3: '0', 2; "A3 — WiFi = P002, Minima = P002.",
    a4: '1', 1; "A4/SDA — WiFi = P101, Minima = P101.",
    a5: '1', 0; "A5/SCL — WiFi = P100, Minima = P100.",
}

#[cfg(feature = "minima")]
pin_map! {
    /// Arduino header pins — UNO R4 **Minima** map (feature `minima`).
    ///
    /// All fields start in reset-default input mode.
    d0: '3', 1; "D0 — WiFi = P301, Minima = P301. SCI2 RX (UART).",
    d1: '3', 2; "D1 — WiFi = P302, Minima = P302. SCI2 TX (UART).",
    d2: '1', 5; "D2 — WiFi = P104, Minima = P105.",
    d3: '1', 4; "D3 — WiFi = P105, Minima = P104.",
    d4: '1', 3; "D4 — WiFi = P106, Minima = P103.",
    d5: '1', 2; "D5 — WiFi = P107, Minima = P102.",
    d6: '1', 6; "D6 — WiFi = P111, Minima = P106.",
    d7: '1', 7; "D7 — WiFi = P112, Minima = P107.",
    d8: '3', 4; "D8 — WiFi = P304, Minima = P304.",
    d9: '3', 3; "D9 — WiFi = P303, Minima = P303.",
    d10: '1', 12; "D10 — WiFi = P103, Minima = P112.",
    d11: '1', 9; "D11 — WiFi = P411, Minima = P109 (SCI9 TX on Minima).",
    d12: '1', 10; "D12 — WiFi = P410, Minima = P110 (SCI9 RX on Minima).",
    d13: '1', 11; "D13 — WiFi = P102 (onboard LED), Minima = P111.",
    a0: '0', 14; "A0 — WiFi = P014, Minima = P014. Also DAC0.",
    a1: '0', 0; "A1 — WiFi = P000, Minima = P000.",
    a2: '0', 1; "A2 — WiFi = P001, Minima = P001.",
    a3: '0', 2; "A3 — WiFi = P002, Minima = P002.",
    a4: '1', 1; "A4/SDA — WiFi = P101, Minima = P101.",
    a5: '1', 0; "A5/SCL — WiFi = P100, Minima = P100.",
}
