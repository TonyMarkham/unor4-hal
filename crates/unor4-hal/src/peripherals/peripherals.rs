//! The ownership gate: everything claimed exactly once, at startup.

use crate::{
    clock::{self, Clocks},
    delay::Delay,
    elc::Elc,
    gpio::{
        Input, Pin, Pins, Sealed,
        pin_map::{self, Sci1Rx, Sci1Tx},
    },
    intc,
    peripherals::{Board, Claim, Release, Taken},
};

#[cfg(feature = "usb")]
use crate::gpio::pin_map::UsbMux;
#[cfg(feature = "wifi")]
use crate::gpio::pin_map::{Sci9Rx, Sci9Tx};

/// Generates [`Peripherals`] with one type parameter per claimable part
/// (the 20 header pins in `pins`, the eight GPT channels and the DTC in
/// `parts`, the eight ELC inputs in `inputs`), and a [`Claim`] and a
/// [`Release`] impl per part: claiming turns that part's type to
/// [`Taken`], releasing turns it back.
macro_rules! peripherals {
    (
        pins { $( $pf:ident: $pp:ident ),* $(,)? }
        parts { $( $gf:ident: $gp:ident = $gdoc:literal ),* $(,)? }
        inputs { $( $if:ident: $ip:ident = $idoc:literal ),* $(,)? }
    ) => {
        /// Claimed peripherals, handed out once by [`Peripherals::take`].
        ///
        /// - **Setups with builders** (the step generator, switches, the
        ///   wake timer) take their hardware with `with_peripherals(p)`. It claims exactly
        ///   what the setup names and gives back the rest, with those parts
        ///   marked [`Taken`] in its type, so handing the same pin or timer
        ///   to a second driver fails to compile, naming it.
        /// - **Drivers without builders** (UART, PWM, ADC, USB) take their
        ///   fields directly: `Uart::new(p.sci2, p.pins.d1, p.pins.d0, …)`.
        ///   A field another driver has taken holds [`Taken`] instead, and
        ///   passing it is a type error.
        ///
        /// Call the `with_peripherals` builders before moving fields out
        /// directly: a `Peripherals` with a field moved out can no longer be
        /// passed whole. The type parameters are the claimable fields'
        /// types; firmware never writes them out.
        pub struct Peripherals<
            $( $pp = Pin<pin_map::$pp, Input>, )*
            $( $gp = ra4m1::$gp, )*
            $( $ip = crate::elc::$ip, )*
        > {
            /// Resolved clock frequencies (HOCO 48 MHz).
            pub clocks: Clocks,
            /// The Arduino header pins; what each can do is in
            /// [`gpio::pin_map`](crate::gpio::pin_map).
            pub pins: Pins<$( $pp ),*>,
            /// SysTick-based blocking delay.
            pub delay: Delay,
            /// SCI2, unconfigured (the UART driver claims it once).
            pub sci2: ra4m1::SCI2,
            /// SCI9, unconfigured — the monitor path: P109/P110, wired to the
            /// ESP32-S3 bridge.
            pub sci9: ra4m1::SCI9,
            /// SCI1, unconfigured — the WiFi AT channel: P501/P502, wired to
            /// the ESP32-S3's AT server (115200 8N1).
            pub sci1: ra4m1::SCI1,
            $(
                #[doc = $gdoc]
                pub $gf: $gp,
            )*
            $(
                #[doc = $idoc]
                pub $if: $ip,
            )*
            /// ADC140 — the 14-bit A/D converter, unconfigured (the ADC
            /// driver claims it once). Each header's channel is in
            /// [`gpio::pin_map`](crate::gpio::pin_map).
            pub adc140: ra4m1::ADC140,
            /// The event link controller, started: the step generator links
            /// its events through it, and firmware can route its own.
            pub elc: Elc,
            /// P501 — SCI1 TX, wired to the ESP32-S3 (board-internal on the
            /// WiFi board; on the Minima the SWD connector's pin 23).
            pub sci1_tx: Pin<Sci1Tx, Input>,
            /// P502 — SCI1 RX, wired back from the ESP32-S3.
            pub sci1_rx: Pin<Sci1Rx, Input>,
            /// USBFS — the USB 2.0 Full-Speed module, unconfigured (feature
            /// `usb`; see `usb/mod.rs` for the mux rules).
            #[cfg(feature = "usb")]
            pub usbfs: ra4m1::USBFS,
            /// P408 — the WiFi board's USB mux control (board-internal;
            /// driving it hands the connector to the RA4M1 and disconnects
            /// the ESP32-S3 bridge).
            #[cfg(feature = "usb")]
            pub usb_mux: Pin<UsbMux, Input>,
            /// P109 — SCI9 TX to the ESP32-S3 (WiFi only: on the Minima this
            /// chip pin is header D11).
            #[cfg(feature = "wifi")]
            pub sci9_tx: Pin<Sci9Tx, Input>,
            /// P110 — SCI9 RX from the ESP32-S3 (WiFi only: on the Minima
            /// this chip pin is header D12).
            #[cfg(feature = "wifi")]
            pub sci9_rx: Pin<Sci9Rx, Input>,
        }

        impl<$( $pp, )* $( $gp, )* $( $ip, )*> Sealed
            for Peripherals<$( $pp, )* $( $gp, )* $( $ip, )*> {}

        impl<$( $pp, )* $( $gp, )* $( $ip, )*> Board
            for Peripherals<$( $pp, )* $( $gp, )* $( $ip, )*>
        {
            fn clocks(&self) -> Clocks {
                self.clocks
            }

            fn elc(&mut self) -> &mut Elc {
                &mut self.elc
            }
        }

        impl Peripherals {
            /// Takes the peripherals, once: `None` on any later call. The
            /// cortex-m guard runs first and can only succeed once — that is
            /// what makes the PAC's unguarded `steal()` below safe. Starts the
            /// HOCO 48 MHz clock, clears the interrupt routes a bootloader
            /// jump leaves behind, and starts the event link.
            pub fn take() -> Option<Self> {
                let core = cortex_m::Peripherals::take()?;
                // Safe: `take()` above succeeded exactly once, so no other
                // caller can reach this line — the single precondition
                // `steal()` demands.
                let device = unsafe { ra4m1::Peripherals::steal() };
                // A bootloader jump is not a hardware reset: discard
                // inherited ICU routes before the HAL allocates its own
                // interrupt slots.
                intc::init();
                // Keep the HAL's 32-entry IELSR vector table in the link
                // even for firmware that enables no interrupts (see
                // `intc::vector_table`).
                let _linked = intc::vector_table();
                let clocks = clock::init(device.SYSTEM);
                let pins = Pins::new(
                    device.PORT0,
                    device.PORT1,
                    device.PORT3,
                    device.PORT4,
                    device.PFS,
                    device.PMISC,
                );
                let delay = Delay::new(core.SYST, &clocks);
                Some(Self {
                    clocks,
                    pins,
                    delay,
                    sci2: device.SCI2,
                    sci9: device.SCI9,
                    sci1: device.SCI1,
                    $( $gf: device.$gp, )*
                    $( $if: crate::elc::$ip::new(), )*
                    adc140: device.ADC140,
                    elc: Elc::new(device.ELC),
                    sci1_tx: Pin::new(),
                    sci1_rx: Pin::new(),
                    #[cfg(feature = "usb")]
                    usbfs: device.USBFS,
                    #[cfg(feature = "usb")]
                    usb_mux: Pin::new(),
                    #[cfg(feature = "wifi")]
                    sci9_tx: Pin::new(),
                    #[cfg(feature = "wifi")]
                    sci9_rx: Pin::new(),
                })
            }
        }

        peripherals!(@pins [] [$( ($pf $pp) )*] [$( ($gf $gp) )* $( ($if $ip) )*]);
        peripherals!(@parts
            [$( ($pf $pp) )*]
            []
            [$( ($gf $gp ra4m1::$gp) )* $( ($if $ip crate::elc::$ip) )*]);
    };

    // One header pin at a time: `$cf`/`$cp` is claimed, the rest carry over.
    (@pins
        [$( ($bf:ident $bp:ident) )*]
        [($cf:ident $cp:ident) $( ($af:ident $ap:ident) )*]
        [$( ($gf:ident $gp:ident) )*]
    ) => {
        impl<$( $bp, )* $( $ap, )* $( $gp, )*> Claim<pin_map::$cp>
            for Peripherals<$( $bp, )* Pin<pin_map::$cp, Input>, $( $ap, )* $( $gp, )*>
        {
            type Part = Pin<pin_map::$cp, Input>;
            type Rest = Peripherals<$( $bp, )* Taken, $( $ap, )* $( $gp, )*>;

            fn claim(self) -> (Self::Part, Self::Rest) {
                let part = self.pins.$cf;
                let rest = Peripherals {
                    clocks: self.clocks,
                    pins: Pins {
                        $( $bf: self.pins.$bf, )*
                        $cf: Taken,
                        $( $af: self.pins.$af, )*
                    },
                    delay: self.delay,
                    sci2: self.sci2,
                    sci9: self.sci9,
                    sci1: self.sci1,
                    $( $gf: self.$gf, )*
                    adc140: self.adc140,
                    elc: self.elc,
                    sci1_tx: self.sci1_tx,
                    sci1_rx: self.sci1_rx,
                    #[cfg(feature = "usb")]
                    usbfs: self.usbfs,
                    #[cfg(feature = "usb")]
                    usb_mux: self.usb_mux,
                    #[cfg(feature = "wifi")]
                    sci9_tx: self.sci9_tx,
                    #[cfg(feature = "wifi")]
                    sci9_rx: self.sci9_rx,
                };
                (part, rest)
            }
        }

        impl<$( $bp, )* $( $ap, )* $( $gp, )*> Release<pin_map::$cp>
            for Peripherals<$( $bp, )* Taken, $( $ap, )* $( $gp, )*>
        {
            type Part = Pin<pin_map::$cp, Input>;
            type Restored = Peripherals<$( $bp, )* Pin<pin_map::$cp, Input>, $( $ap, )* $( $gp, )*>;

            fn release(self, part: Self::Part) -> Self::Restored {
                Peripherals {
                    clocks: self.clocks,
                    pins: Pins {
                        $( $bf: self.pins.$bf, )*
                        $cf: part,
                        $( $af: self.pins.$af, )*
                    },
                    delay: self.delay,
                    sci2: self.sci2,
                    sci9: self.sci9,
                    sci1: self.sci1,
                    $( $gf: self.$gf, )*
                    adc140: self.adc140,
                    elc: self.elc,
                    sci1_tx: self.sci1_tx,
                    sci1_rx: self.sci1_rx,
                    #[cfg(feature = "usb")]
                    usbfs: self.usbfs,
                    #[cfg(feature = "usb")]
                    usb_mux: self.usb_mux,
                    #[cfg(feature = "wifi")]
                    sci9_tx: self.sci9_tx,
                    #[cfg(feature = "wifi")]
                    sci9_rx: self.sci9_rx,
                }
            }
        }

        peripherals!(@pins
            [$( ($bf $bp) )* ($cf $cp)]
            [$( ($af $ap) )*]
            [$( ($gf $gp) )*]);
    };
    (@pins [$( $done:tt )*] [] [$( $parts:tt )*]) => {};

    // One GPT channel, the DTC or an ELC input at a time; `$ct` is its type.
    (@parts
        [$( ($pf:ident $pp:ident) )*]
        [$( ($bf:ident $bp:ident $bt:ty) )*]
        [($cf:ident $cp:ident $ct:ty) $( ($af:ident $ap:ident $at:ty) )*]
    ) => {
        impl<$( $pp, )* $( $bp, )* $( $ap, )*> Claim<$ct>
            for Peripherals<$( $pp, )* $( $bp, )* $ct, $( $ap, )*>
        {
            type Part = $ct;
            type Rest = Peripherals<$( $pp, )* $( $bp, )* Taken, $( $ap, )*>;

            fn claim(self) -> (Self::Part, Self::Rest) {
                let part = self.$cf;
                let rest = Peripherals {
                    clocks: self.clocks,
                    pins: self.pins,
                    delay: self.delay,
                    sci2: self.sci2,
                    sci9: self.sci9,
                    sci1: self.sci1,
                    $( $bf: self.$bf, )*
                    $cf: Taken,
                    $( $af: self.$af, )*
                    adc140: self.adc140,
                    elc: self.elc,
                    sci1_tx: self.sci1_tx,
                    sci1_rx: self.sci1_rx,
                    #[cfg(feature = "usb")]
                    usbfs: self.usbfs,
                    #[cfg(feature = "usb")]
                    usb_mux: self.usb_mux,
                    #[cfg(feature = "wifi")]
                    sci9_tx: self.sci9_tx,
                    #[cfg(feature = "wifi")]
                    sci9_rx: self.sci9_rx,
                };
                (part, rest)
            }
        }

        impl<$( $pp, )* $( $bp, )* $( $ap, )*> Release<$ct>
            for Peripherals<$( $pp, )* $( $bp, )* Taken, $( $ap, )*>
        {
            type Part = $ct;
            type Restored = Peripherals<$( $pp, )* $( $bp, )* $ct, $( $ap, )*>;

            fn release(self, part: Self::Part) -> Self::Restored {
                Peripherals {
                    clocks: self.clocks,
                    pins: self.pins,
                    delay: self.delay,
                    sci2: self.sci2,
                    sci9: self.sci9,
                    sci1: self.sci1,
                    $( $bf: self.$bf, )*
                    $cf: part,
                    $( $af: self.$af, )*
                    adc140: self.adc140,
                    elc: self.elc,
                    sci1_tx: self.sci1_tx,
                    sci1_rx: self.sci1_rx,
                    #[cfg(feature = "usb")]
                    usbfs: self.usbfs,
                    #[cfg(feature = "usb")]
                    usb_mux: self.usb_mux,
                    #[cfg(feature = "wifi")]
                    sci9_tx: self.sci9_tx,
                    #[cfg(feature = "wifi")]
                    sci9_rx: self.sci9_rx,
                }
            }
        }

        peripherals!(@parts
            [$( ($pf $pp) )*]
            [$( ($bf $bp $bt) )* ($cf $cp $ct)]
            [$( ($af $ap $at) )*]);
    };
    (@parts [$( $pins:tt )*] [$( $done:tt )*] []) => {};
}

peripherals! {
    pins {
        a0: A0, a1: A1, a2: A2, a3: A3, a4: A4, a5: A5,
        d0: D0, d1: D1, d2: D2, d3: D3, d4: D4, d5: D5, d6: D6,
        d7: D7, d8: D8, d9: D9, d10: D10, d11: D11, d12: D12, d13: D13,
    }
    parts {
        gpt320: GPT320 = "GPT320 — GPT32 channel 0 (32-bit). Which headers it drives is each pin's `Gtioc` fact in `gpio::pin_map`.",
        gpt321: GPT321 = "GPT321 — GPT32 channel 1 (32-bit).",
        gpt162: GPT162 = "GPT162 — GPT16 channel 2 (16-bit).",
        gpt163: GPT163 = "GPT163 — GPT16 channel 3 (16-bit).",
        gpt164: GPT164 = "GPT164 — GPT16 channel 4 (16-bit).",
        gpt165: GPT165 = "GPT165 — GPT16 channel 5 (16-bit).",
        gpt166: GPT166 = "GPT166 — GPT16 channel 6 (16-bit).",
        gpt167: GPT167 = "GPT167 — GPT16 channel 7 (16-bit).",
        dtc: DTC = "DTC — the data transfer controller (the step generator claims it for its tick-timed writes).",
    }
    inputs {
        elc_a: ElcA = "`ELC_GPTA` — an ELC input to the GPT, claimable ([`Elc::link`] takes it).",
        elc_b: ElcB = "`ELC_GPTB` — the step generator claims it (each pulse's end, counted).",
        elc_c: ElcC = "`ELC_GPTC` — the step generator claims it (the 2 ms tick).",
        elc_d: ElcD = "`ELC_GPTD` — the step generator claims it (the count stop).",
        elc_e: ElcE = "`ELC_GPTE` — an ELC input to the GPT, claimable.",
        elc_f: ElcF = "`ELC_GPTF` — an ELC input to the GPT, claimable.",
        elc_g: ElcG = "`ELC_GPTG` — an ELC input to the GPT, claimable.",
        elc_h: ElcH = "`ELC_GPTH` — an ELC input to the GPT, claimable.",
    }
}
