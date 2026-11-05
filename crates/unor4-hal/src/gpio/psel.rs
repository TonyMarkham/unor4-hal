//! `PmnPFS.PSEL[4:0]` values that are the same on every pin carrying the
//! function (RA4M1 User's Manual Tables 19.5–19.13). SCI's value varies by
//! pin and is written per row in the [`pin_map`](crate::gpio::pin_map)
//! table.

/// GPT outputs `GTIOCnA`/`GTIOCnB`.
pub const GPT_GTIOC: u8 = 0b00011;
/// SPI.
pub const SPI: u8 = 0b00110;
/// IIC.
pub const IIC: u8 = 0b00111;
/// CAN.
pub const CAN: u8 = 0b10000;
