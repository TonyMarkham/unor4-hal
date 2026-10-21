//! The collected results of a validation run: recording, artifact files,
//! and the check table.

use crate::report::{check_result::CheckResult, timestamp::stamp, verdict::Verdict};

use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
};

/// Every check result of a run, in the order the checks ran.
pub struct Report {
    results: Vec<CheckResult>,
    stamp: String,
}

impl Default for Report {
    fn default() -> Self {
        Self::new()
    }
}

impl Report {
    /// A report with no results yet, stamped for this run.
    #[must_use]
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            stamp: stamp(),
        }
    }

    /// Records one check result.
    pub fn add(&mut self, check: &str, verdict: Verdict, evidence: &str) {
        self.results.push(CheckResult {
            check: check.to_owned(),
            verdict,
            evidence: evidence.to_owned(),
        });
    }

    /// Appends raw text (an R3 or monitor dump) to `<stamp>-<kind>.log` in
    /// the artifact directory.
    ///
    /// # Errors
    /// When the artifact file cannot be written.
    pub fn append_log(&self, kind: &str, text: &str) -> std::io::Result<()> {
        let dir = artifacts_dir();
        fs::create_dir_all(&dir)?;
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join(format!("{}-{kind}.log", self.stamp)))?;
        file.write_all(text.as_bytes())
    }

    /// How many checks recorded FAIL.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.results
            .iter()
            .filter(|result| result.verdict.is_failed())
            .count()
    }

    /// Writes the table and summary to `<stamp>.txt` and prints both.
    ///
    /// # Errors
    /// When the artifact file cannot be written.
    pub fn finish(&self) -> std::io::Result<()> {
        let table = table(&self.results);
        let failed = self.failed();
        let summary = format!(
            "{}/{} checks passed ({} failed)",
            self.results.len() - failed,
            self.results.len(),
            failed
        );
        let dir = artifacts_dir();
        fs::create_dir_all(&dir)?;
        fs::write(
            dir.join(format!("{}.txt", self.stamp)),
            format!("{table}{summary}\n"),
        )?;
        println!("{table}{summary}");
        Ok(())
    }
}

/// The run's artifact directory: `<workspace>/target/validation`.
#[must_use]
pub fn artifacts_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap_or(Path::new("."))
        .join("target")
        .join("validation")
}

/// Formats the results as the check table, evidence wrapped so no
/// line exceeds 200 columns.
///
/// `pub(crate)`: the host tests in `src/tests` exercise it; it is not
/// part of the CLI's API.
#[must_use]
pub(crate) fn table(results: &[CheckResult]) -> String {
    const WIDTH: usize = 200;
    let check_w = results
        .iter()
        .map(|result| result.check.len())
        .chain(std::iter::once("Check".len()))
        .max()
        .unwrap_or(5);
    let evidence_w = WIDTH.saturating_sub(check_w + 11).max(20);
    let mut out = String::new();
    out.push_str(&format!(
        "{:<check_w$}  {:<7}  Evidence\n",
        "Check", "Verdict"
    ));
    out.push_str(&"-".repeat(check_w + 11 + evidence_w));
    out.push('\n');
    for result in results {
        let lines = wrap(&result.evidence, evidence_w);
        out.push_str(&format!(
            "{:<check_w$}  {:<7}  {}\n",
            result.check,
            result.verdict,
            lines.first().map(String::as_str).unwrap_or_default()
        ));
        for continuation in lines.iter().skip(1) {
            out.push_str(&format!("{:<check_w$}  {:<7}  {}\n", "", "", continuation));
        }
    }
    out
}

/// Wraps `text` at `width` on spaces; an over-long word is not split.
///
/// `pub(crate)`: the host tests in `src/tests` exercise it; it is not
/// part of the CLI's API.
#[must_use]
pub(crate) fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split(' ') {
        if !line.is_empty() && line.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() || lines.is_empty() {
        lines.push(line);
    }
    lines
}
