//! SCI UART driver — polling mode, 8 data bits, no parity, 1 stop bit
//! (Slice 3: SCI2 on the header's D0/D1; Slice 4: SCI9 on the internal
//! monitor line P109/P110; Slice 6: SCI1 on the WiFi AT line P501/P502).
//! Any TX/RX pair the pin table gives the channel is accepted:
//! [`Uart::new`] bounds on [`SciPin`](crate::gpio::SciPin).
//!
//! Hardware facts, Renesas RA4M1 hardware manual (R01UH0888) unless the
//! source is named otherwise:
//! - SCI2 register block at `0x4007_0040`; SCI9 shares the same non-FIFO
//!   layout at `0x4007_0120` — from the RA4M1 SVD (`ra4m1` PAC), which
//!   aliases both engines' blocks to `sci2::RegisterBlock`. SCI1's block
//!   sits at `0x4007_0020` and the PAC types it as `sci0::RegisterBlock`
//!   (`pub use self::sci0 as sci1`); every register the driver touches is
//!   byte-identical between the `sci0` and `sci2` PAC modules at
//!   identical offsets, so [`SciChannel::block`] hands every engine's
//!   registers out through one `sci2`-typed view (evidence on the trait).
//!   That is also
//!   the source of every reset value relied on below (SCMR.CHR1 = 1
//!   makes `SMR.CHR = 0` mean 8 data bits; SSR.TDRE resets to 1).
//! - Module stop: `MSTPCRB` bit 29 releases SCI2, bit 22 releases SCI9,
//!   bit 30 releases SCI1 (SVD `MSTP` block at `0x4004_7000`,
//!   `MSTPCRB` first at offset `0x00`; the SVD names the fields "Serial
//!   Communication Interface 2/9/1 Module Stop", and manual §10.2.3
//!   carries the same bit list). `MSTP*` writes require `PRCR.PRC1`
//!   write-enable (key `0xA5`), like Slice 1's clock registers.
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
//!   cross-check data only, never transcribed into the
//!   crate) with FSP's error formula; FSP additionally considers
//!   undershooting candidates via absolute error, so its loop order
//!   differs — for the shipped bauds the results coincide. FSP's
//!   ABCSE rows (6-cycle coefficient) are left
//!   out: FSP restricts `ABCSE = 1` on some channels and the RA4M1's
//!   restriction list is unverified — nothing in the board's baud range
//!   needs them at 24 MHz.
//! - Pin functions: each pin's SCI signals and their `PSEL` are its
//!   [`SciPin`](crate::gpio::SciPin) impls in the
//!   [`pin_map`](crate::gpio::pin_map) table (manual §19 Tables
//!   19.6–19.13). The proven pairs: P302 = TXD2, P301 = RXD2 at
//!   `PSEL = 00100b` (Slice 3); P109 = TXD9, P110 = RXD9 at `00101b`
//!   (Slice 4); P501 = TXD1, P502 = RXD1 at `00101b` (Table 19.13; Slice
//!   6). Mostly the even channels (SCI0/2/…) sit at `00100b` and the odd
//!   ones (SCI1/9/…) at `00101b` — FSP's `IOPORT_PERIPHERAL_SCI0_2_4_6_8`
//!   / `_SCI1_3_5_7_9` and the Arduino core's `SCI_ODD_CFG` encode that
//!   rule — but the manual's tables have exceptions: Table 19.6 lists
//!   P102's `TXD2` under `00101b`, and Table 19.7 lists `SCK1` under
//!   `00100b` on P109 but `00101b` on P112. So `PSEL` is a per-pin
//!   property, and TX and RX may differ. The exceptions are not yet proven
//!   on the board. Board wiring (P109/P110 through the level translator to
//!   the ESP32-S3; P501/P502 board-internal to the ESP32-S3): ABX00087
//!   block diagram.
//!
//! No `embedded-hal` serial traits on the polling driver yet: eh 1.0's
//! serial traits live in the separate `embedded-hal-nb` crate, and the
//! polling `Uart` keeps the shape the course already teaches. The
//! interrupt-mode driver (`UartIrq`, feature `uart-irq`) carries the
//! `embedded-hal-nb` serial, `embedded-io`, and `core::fmt::Write`
//! impls from Slice 5 on.

mod baud_error;
mod baud_params;
mod baud_result;
mod calc_baud;
mod read_error;
mod read_result;
mod sci_channel;
mod uart;

#[cfg(feature = "uart-irq")]
mod ring;
#[cfg(feature = "uart-irq")]
mod uart_irq;

// ---------------------------------------------------------------------------------------------- //

pub use crate::uart::{
    baud_error::BaudError,
    baud_params::BaudParams,
    baud_result::BaudResult,
    calc_baud::{MAX_ERROR_X1000, calc_baud},
    read_error::ReadError,
    read_result::ReadResult,
    sci_channel::SciChannel,
    uart::Uart,
};

#[cfg(feature = "uart-irq")]
pub(crate) use crate::uart::{ring::Ring, uart::init_sci};

#[cfg(feature = "uart-irq")]
pub use crate::uart::uart_irq::UartIrq;

#[cfg(all(test, feature = "uart-irq"))]
pub(crate) use crate::uart::uart_irq::can_prime_tx;
