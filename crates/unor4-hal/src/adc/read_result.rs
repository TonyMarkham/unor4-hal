//! The result alias for single-shot conversions.

use crate::adc::ReadError;

use core::result::Result as CoreResult;

/// Result of a single-shot conversion: a raw reading, or
/// [`ReadError`] explaining why there is none.
pub type ReadResult<T> = CoreResult<T, ReadError>;
