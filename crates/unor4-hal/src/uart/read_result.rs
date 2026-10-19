//! The result alias for receive operations.

use crate::uart::ReadError;

use core::result::Result as CoreResult;

/// Result of a receive operation: a byte, or [`ReadError`] explaining
/// why none is available.
pub type ReadResult<T> = CoreResult<T, ReadError>;
