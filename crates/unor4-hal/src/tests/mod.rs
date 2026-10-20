//! Host-side unit tests for the crate's pure logic. Compiled only by
//! `cargo test` (see the `#[cfg(test)]` gate in lib.rs). The `ring`
//! tests need the `uart-irq` feature (`cargo test -p unor4-hal
//! --features uart-irq`).

mod clock;
mod delay;
mod pins;
mod uart;

mod intc;
#[cfg(feature = "uart-irq")]
mod ring;
#[cfg(feature = "uart-irq")]
mod uart_irq;
