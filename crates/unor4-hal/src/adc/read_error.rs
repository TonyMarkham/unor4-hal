//! Why [`Adc::read`](crate::adc::Adc::read) has no reading.

/// Why [`Adc::read`](crate::adc::Adc::read) has no reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadError {
    /// `ADCSR.ADST` did not self-clear within the poll bound. The
    /// conversion takes ~1.2–1.6 µs at this crate's clocks (manual
    /// §35.3.6),
    /// so the bound is thousands of times the expected wait — reaching
    /// it means the unit never ran, not that the wait was short.
    Timeout,
}
