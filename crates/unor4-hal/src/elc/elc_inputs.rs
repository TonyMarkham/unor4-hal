//! The eight claimable ELC inputs, one zero-sized type each (generated,
//! like the header pins in `gpio::pin_map`).

use crate::elc::{ElcInput, GptInput, elc_input::Sealed};

macro_rules! elc_inputs {
    ( $( $name:ident = $input:ident, $elsr:literal; )* ) => {
        $(
            #[doc = concat!(
                "The GPT's ELC input `ELC_GPT", stringify!($input), "` (`", $elsr,
                "`, manual Table 18.2) as a claimable part: a field of ",
                "[`Peripherals`](crate::Peripherals), claimed by the driver that links an ",
                "event into it."
            )]
            #[derive(Debug)]
            pub struct $name {
                _private: (),
            }

            impl $name {
                /// The one token, made by `Peripherals::take`.
                pub(crate) const fn new() -> Self {
                    Self { _private: () }
                }
            }

            impl Sealed for $name {}

            impl ElcInput for $name {
                const INPUT: GptInput = GptInput::$input;

                unsafe fn steal() -> Self {
                    Self::new()
                }
            }
        )*
    };
}

elc_inputs! {
    ElcA = A, "ELSR0";
    ElcB = B, "ELSR1";
    ElcC = C, "ELSR2";
    ElcD = D, "ELSR3";
    ElcE = E, "ELSR4";
    ElcF = F, "ELSR5";
    ElcG = G, "ELSR6";
    ElcH = H, "ELSR7";
}
