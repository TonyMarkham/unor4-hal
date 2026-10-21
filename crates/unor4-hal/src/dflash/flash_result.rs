//! The result alias for data flash operations.

use crate::dflash::FlashError;

use core::result::Result as CoreResult;

/// Result of a data flash operation: the value, or [`FlashError`]
/// explaining why there is none.
pub type FlashResult<T> = CoreResult<T, FlashError>;
