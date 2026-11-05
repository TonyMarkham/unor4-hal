//! Host-side unit tests for the crate's pure logic. Compiled only by
//! `cargo test` (see the `#[cfg(test)]` gate in lib.rs). The `ring`
//! tests need the `uart-irq` feature (`cargo test -p unor4-hal
//! --features uart-irq`).

mod adc;
mod builders;
mod clock;
mod count_stop;
mod delay;
mod dflash;
mod elc;
mod ena;
mod pins;
mod pwm;
mod recovery;
mod start;
mod step;
mod switch;
mod uart;
#[cfg(feature = "usb")]
mod usb;
mod wake;

mod intc;
#[cfg(feature = "uart-irq")]
mod ring;
#[cfg(feature = "uart-irq")]
mod uart_irq;
