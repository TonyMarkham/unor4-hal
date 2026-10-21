//! R3 output framing.

/// Strips the R3's `\x02…\x03` output frames from a link capture.
#[must_use]
pub fn strip_frames(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut framed = false;
    for ch in text.chars() {
        match ch {
            '\x02' => framed = true,
            '\x03' => framed = false,
            _ if !framed => out.push(ch),
            _ => {}
        }
    }
    out
}
