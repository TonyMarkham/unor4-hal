//! The bench's USB identities, as proven on this bench (README,
//! All-in-one validation).

/// Arduino's USB vendor ID, shared by the R3 and the R4's ports.
pub const VID_ARDUINO: u16 = 0x2341;

/// The R3's USB product ID (classic UNO R3).
pub const PID_R3: u16 = 0x0043;

/// The R4 ESP32-S3 bridge's USB product ID.
pub const PID_BRIDGE: u16 = 0x1002;

/// The R4 native RA4M1 CDC's USB product ID, present while native USB is
/// taken.
pub const PID_NATIVE: u16 = 0x006D;
