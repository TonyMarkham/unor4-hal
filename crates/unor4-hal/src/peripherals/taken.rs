//! The marker for a claimed pin or peripheral.

/// What a pin or peripheral field of [`Peripherals`](crate::Peripherals)
/// holds once a driver has claimed it. Passing it where the part is
/// expected (`Uart::new(p.sci2, …)` after another driver took SCI2) is a
/// type error: expected `SCI2`, found `Taken`.
#[derive(Debug)]
pub struct Taken;
