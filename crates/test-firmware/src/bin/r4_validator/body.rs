//! Reply bodies for the request link.

/// A reply body: `A–Z`, `0–9` and spaces only (see the link contract in
/// the crate docs).
pub(crate) struct Body {
    bytes: [u8; 64],
    len: usize,
}

impl Body {
    /// A body holding `text` (truncated to 64 bytes).
    pub(crate) fn new(text: &[u8]) -> Self {
        let mut body = Self {
            bytes: [0; 64],
            len: 0,
        };
        body.text(text);
        body
    }

    /// Appends raw text.
    pub(crate) fn text(&mut self, text: &[u8]) -> &mut Self {
        for &byte in text {
            if self.len < self.bytes.len() {
                self.bytes[self.len] = byte;
                self.len += 1;
            }
        }
        self
    }

    /// Appends a space and the decimal digits of `value`.
    pub(crate) fn num(&mut self, value: u32) -> &mut Self {
        let mut digits = [0u8; 10];
        let mut count = 0;
        let mut rest = value;
        loop {
            digits[count] = b'0' + (rest % 10) as u8;
            count += 1;
            rest /= 10;
            if rest == 0 {
                break;
            }
        }
        self.text(b" ");
        for index in (0..count).rev() {
            self.text(&[digits[index]]);
        }
        self
    }

    /// The accumulated body.
    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}
