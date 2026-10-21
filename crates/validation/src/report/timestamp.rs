//! A UTC timestamp for artifact file names.

use std::time::{SystemTime, UNIX_EPOCH};

/// A `yyyyMMdd-HHmmss` (UTC) stamp for artifact file names.
#[must_use]
pub fn stamp() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (hour, minute, second) = ((secs / 3_600) % 24, (secs / 60) % 60, secs % 60);
    let days = secs / 86_400;
    // Howard Hinnant's civil_from_days: a Unix day number to (y, m, d).
    let z = i64::try_from(days).unwrap_or(0) + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { year + 1 } else { year };
    format!("{year:04}{month:02}{day:02}-{hour:02}{minute:02}{second:02}")
}
