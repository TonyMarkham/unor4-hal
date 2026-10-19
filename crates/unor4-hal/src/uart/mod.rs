//! SCI UART driver — polling mode, 8 data bits, no parity, 1 stop bit
//! (Slice 3: SCI2 on the header's D0/D1).
//!
//! Hardware facts, Renesas RA4M1 hardware manual (R01UH0888) unless the
//! source is named otherwise:
//! - SCI2 register block at `0x4007_0040` (SCI9 shares the same non-FIFO
//!   layout at `0x4007_0120`) — from the RA4M1 SVD (`ra4m1` PAC), which
//!   is also the source of every reset value relied on below (SCMR.CHR1
//!   = 1 makes `SMR.CHR = 0` mean 8 data bits; SSR.TDRE resets to 1).
//! - Module stop: `MSTPCRB` bit 29 releases SCI2 (SVD `MSTP` block at
//!   `0x4004_7000`, `MSTPCRB` first at offset `0x00`). `MSTP*` writes
//!   require `PRCR.PRC1` write-enable (key `0xA5`), like Slice 1's clock
//!   registers.
//! - The SCI's base clock is PCLKA (48 MHz after Slice 1's clock init);
//!   `SMR.CKS` divides it by `4^CKS`. Vendor cross-check: FSP's RA4M1
//!   `bsp_feature.h` sets `BSP_FEATURE_SCI_CLOCK` to
//!   `FSP_PRIV_CLOCK_PCLKA`, and FSP's r_sci_uart docs list PCLKA for
//!   the RA4M1 group.
//! - Baud rate equation (SCI chapter, bit-rate generator): the bit rate
//!   is `PCLKA / (coefficient × (BRR + 1))`, where the coefficient is
//!   32 base, 16 with `SEMR.BGDM`, 8 with `BGDM + ABCS`, multiplied by 4
//!   per `SMR.CKS` step. [`calc_baud`] searches a subset of the table
//!   in Renesas FSP's `r_sci_uart.c` (BSD-3-Clause — behavioral
//!   cross-check data only, HalSpec §1, never transcribed into the
//!   crate) with FSP's error formula; FSP additionally considers
//!   undershooting candidates via absolute error, so its loop order
//!   differs — for the shipped bauds the results coincide. FSP's
//!   ABCSE rows (6-cycle coefficient) are left
//!   out: FSP restricts `ABCSE = 1` on some channels and the RA4M1's
//!   restriction list is unverified — nothing in the board's baud range
//!   needs them at 24 MHz.
//! - Pin functions: P302 = TXD2, P301 = RXD2, both at `PSEL = 0x04`
//!   (I/O ports chapter, pin function tables; cross-checked against
//!   FSP's `IOPORT_PERIPHERAL_SCI0_2_4_6_8` and the Arduino core's
//!   even-config mapping in `cores/arduino/Serial.cpp`).
//!
//! No `embedded-hal` serial traits yet: eh 1.0's serial traits live in
//! the separate `embedded-hal-nb` crate, and traits plus ring buffers
//! land together with interrupt mode (Slice 5).

mod baud_error;
mod baud_params;
mod baud_result;
mod calc_baud;
mod read_error;
mod read_result;
mod uart;

// ---------------------------------------------------------------------------------------------- //

pub use baud_error::BaudError;
pub use baud_params::BaudParams;
pub use baud_result::BaudResult;
pub use calc_baud::{MAX_ERROR_X1000, calc_baud};
pub use read_error::ReadError;
pub use read_result::ReadResult;
pub use uart::Uart;
