//! Which part of the R3 validator is loaded.
//!
//! The checks no longer fit the R3's 32 KB in one image, so
//! `r3-validator.ino` builds as parts (`R3_PART`), each uploaded for its
//! own `validate` session. The sketch's help line names the part:
//! `r3-validator part <n> of <m>: see header`.

use regex::Regex;

/// How many parts the R3 validator builds as.
pub const R3_PARTS: u8 = 3;

/// The part named by the R3's help line in `text`, if it names one from 1
/// to [`R3_PARTS`].
#[must_use]
pub fn parse_part(text: &str) -> Option<u8> {
    let caps = Regex::new(r"r3-validator part (\d+) of (\d+)")
        .unwrap()
        .captures(text)?;
    let part: u8 = caps.get(1)?.as_str().parse().ok()?;
    let of: u8 = caps.get(2)?.as_str().parse().ok()?;
    (of == R3_PARTS && (1..=R3_PARTS).contains(&part)).then_some(part)
}
