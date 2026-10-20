//! SCI UART driver — polling mode, 8 data bits, no parity, 1 stop bit
//! (Slice 3: SCI2 on the header's D0/D1; Slice 4: SCI9 on the internal
//! monitor line P109/P110; Slice 6: SCI1 on the WiFi AT line P501/P502).
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
//!   cross-check data only, HalSpec §1, never transcribed into the
//!   crate) with FSP's error formula; FSP additionally considers
//!   undershooting candidates via absolute error, so its loop order
//!   differs — for the shipped bauds the results coincide. FSP's
//!   ABCSE rows (6-cycle coefficient) are left
//!   out: FSP restricts `ABCSE = 1` on some channels and the RA4M1's
//!   restriction list is unverified — nothing in the board's baud range
//!   needs them at 24 MHz.
//! - Pin functions: P302 = TXD2, P301 = RXD2 at `PSEL = 0x04`; P109 =
//!   TXD9, P110 = RXD9 at `PSEL = 0x05`; P501 = TXD1, P502 = RXD1 at
//!   `PSEL = 0x05` (manual §19, Table 19.13 "Register settings for
//!   input/output pin function (PORT5)": `TXD1/MOSI1/SDA1` and
//!   `RXD1/MISO1/SCL1` at `PSEL[4:0] = 00101b`). The RA4M1 assigns one
//!   `PSEL`
//!   per function *class*, not per pin: even channels (SCI0/2/4/6/8)
//!   share `0x04`, odd channels (SCI1/3/5/7/9) share `0x05` — FSP's
//!   `r_ioport_api.h` encodes exactly these two values
//!   (`IOPORT_PERIPHERAL_SCI0_2_4_6_8` / `IOPORT_PERIPHERAL_SCI1_3_5_7_9`)
//!   as the `PFS.PSEL` payload. Cross-checked against the Arduino core,
//!   which routes every `pinmux.inc` entry marked `SCI_ODD_CFG` — P109,
//!   P110, P501 and P502 included — to the odd-class constant
//!   (`cores/arduino/Serial.cpp`). Board wiring (P109/P110 to the level
//!   translator and ESP32-S3; P501/P502 to the ESP32-S3): ABX00087 block
//!   diagram, transcribed in Limitations.md. The even-class value is
//!   Slice-3 hardware-proven; the odd-class value is proven by Slice 4's
//!   monitor run.
//!
//! No `embedded-hal` serial traits on the polling driver yet: eh 1.0's
//! serial traits live in the separate `embedded-hal-nb` crate, and the
//! polling `Uart` keeps the shape the course already teaches. The
//! interrupt-mode driver ([`UartIrq`], feature `uart-irq`) carries the
//! `embedded-hal-nb` serial, `embedded-io`, and `core::fmt::Write`
//! impls from Slice 5 on (HalSpec §6).

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

pub use baud_error::BaudError;
pub use baud_params::BaudParams;
pub use baud_result::BaudResult;
pub use calc_baud::{MAX_ERROR_X1000, calc_baud};
pub use read_error::ReadError;
pub use read_result::ReadResult;
pub use sci_channel::SciChannel;
pub use uart::Uart;

#[cfg(feature = "uart-irq")]
pub(crate) use ring::Ring;
#[cfg(feature = "uart-irq")]
pub(crate) use uart::{
    PSEL_SCI1, PSEL_SCI2, PSEL_SCI9, init_sci, release_module_stop_sci1, release_module_stop_sci2,
    release_module_stop_sci9,
};

#[cfg(feature = "uart-irq")]
pub use uart_irq::UartIrq;

#[cfg(all(test, feature = "uart-irq"))]
pub(crate) use uart_irq::can_prime_tx;
