//! The step generator's `Result` alias.

use crate::step::StepError;

/// The step generator's `Result`: `Ok(T)` or the [`StepError`] that
/// refused the call.
pub type StepResult<T> = Result<T, StepError>;
