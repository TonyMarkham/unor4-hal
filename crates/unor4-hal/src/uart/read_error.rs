//! Why [`Uart::read_byte`](crate::uart::Uart::read_byte) has no byte.

/// Why [`Uart::read_byte`](crate::uart::Uart::read_byte) has no byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadError {
    /// RDRF = 0: nothing new since the last read.
    Empty,
    /// ORER: a byte arrived before the previous one was read; the lost
    /// byte is dropped, the receiver keeps working.
    Overrun,
    /// FER: the stop bit was not where expected — usually a baud
    /// mismatch on the other end.
    Framing,
    /// PER: parity check failed (cannot happen in 8N1, listed for
    /// completeness).
    Parity,
}

impl core::fmt::Display for ReadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Empty => "no byte received",
            Self::Overrun => "receive overrun",
            Self::Framing => "framing error",
            Self::Parity => "parity error",
        })
    }
}

/// `embedded-io` 0.7 (feature `uart-irq`) requires `core::error::Error`.
impl core::error::Error for ReadError {}
