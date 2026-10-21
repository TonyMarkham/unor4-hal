//! ADC140 driver (Slice 8) — single-shot blocking reads on A0–A5.
//!
//! Hardware facts, Renesas RA4M1 hardware manual (R01UH0887EJ0100)
//! unless the source is named otherwise:
//! - One unit, 25 analog channels AN000–AN014 + AN016–AN025 (§35.1
//!   Table 35.1), successive approximation, register base
//!   `0x4005_C000` (§41.1; the PAC's `ADC140::PTR` agrees).
//! - Resolution is per-unit, `ADCER.ADPRC[1:0]`: `00b` = 12-bit (the
//!   reset value and the driver's default), `11b` = 14-bit (§35.2.11);
//!   writable only while stopped. Data is right-justified by reset
//!   (`ADRFMT = 0`); `ADDRy` is a 16-bit read-only register (§35.2.1).
//! - Clocks: PCLKB (module) and PCLKC/ADCLK (conversion), ratio
//!   1:1…8:1 or 1:2, 1:4 via `SCKDIVCR` (§35.1 Table 35.1 Note 1).
//!   Slice 1's clock tree gives PCLKB = 24 MHz, ADCLK = 48 MHz — a 1:2
//!   ratio, which sets the sampling-state minimum to ≥ 6 (`ADSSTRn`
//!   range 5–255, reset 0Dh; §35.2.14).
//! - Modes (§35.3.1): single scan converts the selected channels once;
//!   that is all this driver does. Trigger: **software** — `ADCSR.ADST`
//!   starts a scan regardless of `TRGE`/`EXTRG`/`TRSA` (§35.2.12, TRSA
//!   description), so no trigger setup exists or is needed.
//! - Completion: in single scan mode `ADST` remains 1 during the
//!   conversion and **the hardware clears it when the scan completes**,
//!   after which the unit waits (§35.3.2.1 step 4). The driver polls
//!   that self-clear, bounded ([`ReadError::Timeout`]); the interrupt
//!   path (IELSR 029h, `ADC140_ADI`) is future work.
//! - Reference: `ADHVREFCNT` (§35.2.31) selects AVCC0/VREFH0/internal
//!   on the high side, AVSS0/VREFL0 on the low side; its reset 0x00 is
//!   AVCC0/AVSS0. The driver leaves it at reset — on the UNO R4 that is
//!   the board's 5 V analog rail (the Arduino core's 0–5 V analog range
//!   claims it; the slice 8 rails test is the hardware proof).
//!   The internal-reference procedure (§35.7) is out of scope.
//! - Module stop: the ADC140 is initially stopped after reset;
//!   `MSTPCRD.MSTPD16 = 0` releases it (§10, MSTPCRD, p.175: bit 16
//!   targets the "14-Bit A/D Converter Module"), and **at least 1 µs
//!   must pass between release and the first conversion** (§35.8.5) —
//!   the constructor spends it as a cycle-counted spin at the fixed
//!   48 MHz ICLK.
//! - Pins: the six header analog inputs are the same chip pins on both
//!   board maps (the `pins.csv` A0–A5 rows are identical): A0 = P014 →
//!   AN009 (also DA0), A1 = P000 → AN000, A2 = P001 → AN001, A3 = P002
//!   → AN002, A4 = P101 → AN021, A5 = P100 → AN022 (manual §19 Tables
//!   19.5/19.6 `ASEL` rows; datasheet pin tables). Cross-checked
//!   entry-for-entry against the Arduino core's `pinmux.inc`. A0–A3
//!   select through `ADANSA0` and read `ADDR9`/`ADDR0–2`; A4/A5 select
//!   through `ADANSA1` and read `ADDR21`/`ADDR22`, with their sampling
//!   time in the shared `ADSSTRL` (Table 35.8).
//! - Accuracy budget (datasheet R01DS0355EJ0110 §2, Tables 2.42/2.43,
//!   high-speed mode): absolute accuracy ±5 LSB typ / ±20 LSB max
//!   (14-bit, high-precision channels AN000–AN004 — A1–A3; ±32 LSB max
//!   elsewhere), quantization ±0.5 LSB. The tables assume
//!   VCC = AVCC0, which this board does not satisfy (3.3 V digital,
//!   5 V analog rail) — the slice 8 hardware run measured the actual
//!   rails/staircase error and recorded it in the README.
//!
//! **Trait note:** `embedded-hal` 1.0
//! has no `adc` module (only `delay`, `digital`, `i2c`, `pwm`, `spi`;
//! `embedded-hal-nb` 1.0 only `serial`, `spi`), so there is no
//! ecosystem ADC trait to implement — [`Adc::read`] is this crate's own
//! API, the same stance the UART module takes outside `embedded-io`.

mod adc;
mod calc_scan_time;
mod channel;
mod read_error;
mod read_result;
mod resolution;
mod scan_time_params;
mod scan_time_result;

// ---------------------------------------------------------------------------------------------- //

pub use adc::Adc;
pub use calc_scan_time::calc_scan_time;
pub use channel::Channel;
pub use read_error::ReadError;
pub use read_result::ReadResult;
pub use resolution::Resolution;
pub use scan_time_params::ScanTimeParams;
pub use scan_time_result::ScanTime;
