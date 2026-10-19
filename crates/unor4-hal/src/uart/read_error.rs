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
