//! What the wake timer can refuse.

/// Why a wake timer could not be built or started.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeError {
    /// `build()`: no interrupt slot was free (it needs one).
    NoIcuSlot,
    /// `start()`: the wait is longer than the channel reaches at PCLKD/1024
    /// ([`WakeTimer::longest`](crate::wake::WakeTimer::longest): 1.398 s on
    /// a 16-bit channel). Re-arm in a loop for longer waits.
    TooLong,
}
