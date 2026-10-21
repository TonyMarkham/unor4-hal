# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-10-21

First release. An independent, `no_std` hardware abstraction layer for the
Arduino UNO R4 (Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 WiFi**
(default `wifi` pin map) and **UNO R4 Minima** (`minima` pin map) — exactly
one map per build, enforced with `compile_error!`. No fork, no copied code:
hardware facts are cited to the Renesas RA4M1 hardware manual and SVD; the
Arduino core and FSP are behavioral cross-check data only.

### Added

- **Clock** — HOCO 48 MHz initialization and the `Clocks` token resolving
  PCLKA/PCLKB/PCLKD/ICLK for the other drivers.
- **GPIO** — typestate pins (`Input`, `InputPullUp`, `Output`,
  `OutputOpenDrain`, `Alternate`, `Analog`) with a compile-time `Peripherals`
  exactly-once claim; both board pin maps shown in every pin's docs so the
  documentation never lies about the board you are not building. The compiled
  map is proven equal to a checked-in transcription of the Arduino core's
  board tables by a golden test (`src/tests/pins.rs` + `pins.csv`).
- **Delay** — cycle-accurate SysTick `DelayNs` implementation.
- **UART (polling)** — SCI2 (header D0/D1), SCI9 (the WiFi board's internal
  monitor line) and SCI1 (the WiFi AT line to the ESP32-S3) in 8N1 polling
  mode, with a baud calculator that refuses rates outside 3% of PCLKA
  (`calc_baud`).
- **`uart-irq` feature** — interrupt-driven `UartIrq` with ring buffers,
  carrying `embedded-hal-nb` serial, `embedded-io` and `core::fmt::Write`
  impls; 4 KiB bursts echo byte-exact with no overrun while a heartbeat
  continues on the main loop.
- **Interrupts (`intc`)** — the ICU/IELSR event table, event linking and a
  dispatch registry behind the vector table; the crate's `critical-section`
  provider.
- **PWM (`pwm`)** — GPT timer driver implementing `embedded-hal` 1.0
  `SetDutyCycle`; all six header PWM pins (D3, D5, D6, D9, D10, D11) have
  constructors with engine/chip-pin/output fixed at compile time and locked
  by a golden test; `calc_period` keeps every reachable period inside the
  16-bit `GTPR`.
- **ADC (`adc`)** — single-shot blocking reads on A0–A5 via the ADC140
  (`Channel` trait sealed to the six header pins), 12- or 14-bit resolution,
  and a host-tested `calc_scan_time` (`embedded-hal` 1.0 has no ADC trait;
  `Adc::read` is this crate's own API).
- **USB (`usb` feature)** — the USBFS module as a `usb_device::bus::UsbBus`
  adapter plus a blocking CDC-ACM wrapper over `usbd-serial`. Taking
  `UsbPort` explicitly flips the WiFi board's USB mux (disconnecting the
  ESP32-S3 bridge — documented behavior); recovery paths are `release`, the
  opt-in 1200-baud bootloader knock, or a double RESET.
- **Data flash (`dflash`)** — the 8 KB data flash as byte-programmed,
  block-erased storage: plain reads, byte programming (once per erase,
  blank-checked first — overwrites are refused), 1 KB block erase, and blank
  check by the hardware's blank-check command (an erased byte's read value is
  not guaranteed). Code flash is unreachable through the API; the
  mode-switch helpers run from SRAM via cortex-m-rt's `.data` copy, so no
  `memory.x` change is needed.

### Verified on hardware (UNO R4 WiFi)

Every item above was proven on the bench before release; the README records
each witnessed run, re-provable with the validation pair in
`arduino/validation/` + `crates/validation` (`validate all`, 16/16 on
2026-10-21). Highlights: SCI1 `AT` → `OK` from the stock bridge firmware;
PWM duty/period measured edge-by-edge by a controller board; ADC staircase
error within the datasheet budget; native CDC enumeration, echo and
bridge recovery; data-flash erase+rewrite byte-exact and a boot counter
that advanced once per boot — across resets and across **27 manual power
cycles** — with the log intact.

**UNO R4 Minima builds and passes the host test suite but is
hardware-unverified** — no Minima hardware exists in this project.

[0.1.0]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.0

## [0.1.1] - 2026-10-21

### Added

- Crate packaging metadata: `repository` link and a crate-facing `README.md`
  (`crates/unor4-hal/README.md`), so crates.io renders them. Published
  versions are immutable, so 0.1.0's page cannot gain these — 0.1.1 exists
  to carry them; no code changed.

[0.1.1]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.1
