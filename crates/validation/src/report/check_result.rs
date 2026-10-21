//! One recorded check result.

use crate::report::verdict::Verdict;

/// One check's name, verdict, and evidence line.
#[derive(Debug)]
pub struct CheckResult {
    /// Check name as shown in the table, e.g. "S2 button".
    pub check: String,
    /// The verdict.
    pub verdict: Verdict,
    /// The observed evidence.
    pub evidence: String,
}
