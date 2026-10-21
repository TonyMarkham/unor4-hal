//! Check verdicts.

use std::fmt;

/// The outcome of one validation check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The check demonstrated the slice.
    Pass,
    /// The check failed or produced no result.
    Fail,
    /// The R3 validator reported the check as not applicable.
    NotApplicable,
}

impl Verdict {
    /// Parses the verdict word from an R3 result line ("PASS", "FAIL",
    /// "NA"); anything else counts as a pass.
    #[must_use]
    pub fn parse(word: &str) -> Self {
        match word {
            "FAIL" => Self::Fail,
            "NA" => Self::NotApplicable,
            _ => Self::Pass,
        }
    }

    /// True when the verdict counts as failed.
    #[must_use]
    pub fn is_failed(self) -> bool {
        self == Self::Fail
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::NotApplicable => "NA",
        })
    }
}
