//! Run results: recording, artifact files, and the check table.

mod check_result;
mod results;
mod timestamp;
mod verdict;

// ---------------------------------------------------------------------------------------------- //

pub use crate::report::{results::Report, verdict::Verdict};

#[cfg(test)]
pub(crate) use crate::report::{
    check_result::CheckResult,
    results::{table, wrap},
    timestamp::stamp,
};
