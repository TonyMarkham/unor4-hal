//! Why the baud calculator refuses a request.

/// Why [`calc_baud`](crate::uart::calc_baud) refused a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaudError {
    /// No register setting lands within
    /// [`MAX_ERROR_X1000`](crate::uart::MAX_ERROR_X1000) of the
    /// requested rate from this clock.
    OutOfTolerance,
    /// `baud == 0` was requested.
    InvalidBaud,
}
