//! The result alias for the period calculator.

use crate::pwm::PeriodError;

use core::result::Result as CoreResult;

/// Result of [`calc_period`](crate::pwm::calc_period): the resolved
/// setting, or [`PeriodError`] explaining why the request was refused.
pub type PeriodResult<T> = CoreResult<T, PeriodError>;
