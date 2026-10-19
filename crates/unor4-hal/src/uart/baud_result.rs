//! The result alias for the baud-rate calculator.

use crate::uart::BaudError;

use core::result::Result as CoreResult;

/// Result of [`calc_baud`](crate::uart::calc_baud): the resolved
/// setting, or [`BaudError`] explaining why the request was refused.
pub type BaudResult<T> = CoreResult<T, BaudError>;
