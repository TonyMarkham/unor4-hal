//! The request-frame reader: the STX/ETX half of the link contract (see
//! the crate docs).

/// Frame start byte.
pub(crate) const STX: u8 = 0x02;

/// Frame end byte.
pub(crate) const ETX: u8 = 0x03;

/// Accumulates link bytes into `STX … ETX` frames. Bytes outside a frame
/// are ignored. A frame longer than [`FrameBuf::CAP`] is silently
/// cancelled and never dispatched: dispatch waits for the next STX.
pub(crate) struct FrameBuf {
    buf: [u8; Self::CAP],
    len: usize,
    in_frame: bool,
}

impl FrameBuf {
    /// Largest request the link accepts.
    pub(crate) const CAP: usize = 16;

    /// An empty reader, outside a frame.
    pub(crate) const fn new() -> Self {
        Self {
            buf: [0; Self::CAP],
            len: 0,
            in_frame: false,
        }
    }

    /// Feeds one link byte; returns the request payload once a non-empty
    /// frame just ended.
    pub(crate) fn feed(&mut self, byte: u8) -> Option<&[u8]> {
        match byte {
            STX => {
                self.in_frame = true;
                self.len = 0;
            }
            ETX if self.in_frame => {
                self.in_frame = false;
                if self.len > 0 {
                    return Some(&self.buf[..self.len]);
                }
            }
            _ if self.in_frame => {
                if self.len < Self::CAP {
                    self.buf[self.len] = byte;
                    self.len += 1;
                } else {
                    self.in_frame = false;
                }
            }
            _ => {}
        }
        None
    }
}
