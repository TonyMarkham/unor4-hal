//! The six hardware dividers. Any can be set on a PUL setup;
//! [`DividerFor`](crate::step::DividerFor) decides which fit the STEP
//! timer's width.

use crate::{gpio::Sealed, step::Divider};

macro_rules! dividers {
    ( $( $ty:ident: $value:literal, $tpcs:literal; )* ) => {
        $(
            #[doc = concat!("PCLKD/", stringify!($value), " (`GTCR.TPCS = ", stringify!($tpcs), "`).")]
            #[derive(Clone, Copy, Debug)]
            pub struct $ty;

            impl Sealed for $ty {}

            impl Divider for $ty {
                const VALUE: u32 = $value;
                const TPCS: u8 = $tpcs;
                const INSTANCE: Self = $ty;
            }
        )*
    };
}

dividers! {
    Div1: 1, 0;
    Div4: 4, 1;
    Div16: 16, 2;
    Div64: 64, 3;
    Div256: 256, 4;
    Div1024: 1024, 5;
}
