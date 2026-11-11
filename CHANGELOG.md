# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-11-11

Changes since 0.1.2. Breaking: the pin model, the driver constructors,
`Peripherals` and the MSRV (see Changed). Every feature below is verified
on a UNO R4 WiFi unless it says otherwise; the witnessed runs, 33/33
checks over the validation pair's three sessions, are in the repository's
[Validation.md](https://github.com/TonyMarkham/unor4-hal/blob/main/Validation.md).
The UNO R4 Minima builds and passes the host tests, and is
hardware-unverified.

### Added

- **Step generator (`step`)** — drives a stepper driver's STEP (PUL) and
  DIR, and optionally ENA, from a signed rate.
  - **Setup, checked at `cargo check`:** one builder per part
    (`PulPinBuilder`, `DirPinBuilder`, `EnaPinBuilder`, `CounterBuilder`,
    `TickBuilder`), combined by `StepGeneratorBuilder::new().with_pul(PUL)
    .with_dir(DIR).with_ena(ENA).with_counter(COUNTER).with_tick(TICK)
    .build()` in a named `const` that cross-checks the parts. STEP goes on
    any header with a 32-bit GPT output, DIR and ENA on any header pin;
    the counter and tick timers are any free GPT channels. A missing part,
    a pin used twice or a 16-bit STEP timer fails `cargo check` with a
    message naming it.
  - **Run time:** `StepGenerator::new(SETUP).with_peripherals(p).build()`
    claims exactly the hardware the setup names (timers, pins, the DTC,
    ELC inputs B, C and D, two ICU slots) and returns the rest of `p`;
    `free(SETUP, p)` gives it back.
  - **Pulses:** a fixed LOW time; `run(rate)` and `queue(rate)` with signed
    rates; a `stop()` that never cuts a pulse; an exact hardware pulse
    count (`pulses()`) and signed `position()`.
  - **2 ms hardware tick:** queued rate changes and reversals are written
    by the DTC at the tick, with no CPU in the path. `poll_tick()` returns
    a `TickReport` (count and phase captured at one instant, whether the
    change applied), and `predicted_pulses()` the count due at the next
    tick. A rate of 0 stops at the tick; an idle generator stops itself
    after 1 s; ticks the firmware sleeps through are recovered.
  - **DIR rule:** every DIR change is written after any pulse in flight and
    at least the DIR set-up time (`DirPinBuilder`, up to 1 ms) before the
    next pulse, for `run`, `queue` and starts from stopped or idle.
  - **ENA:** `enable` / `disable` / `is_enabled`; until the lead time has
    passed, a non-zero rate returns `StepError::Disabled` / `Enabling`.
    The builder refuses an enabling level the board holds while the pin is
    undriven (WiFi D13 is pulled LOW by R52). Run on the cart-pole rig,
    not on the validation rig.
  - **Stop at an exact count:** `stop_at(count)` stops the STEP timer in
    hardware at the end of pulse `count`, through ELC input D;
    `cancel_stop_at()`, `stopped_at_count()`.
  - **Interrupt wake-ups:** `set_tick_wake(true)` wakes the CPU from `WFI`
    at every tick; `set_count_wake(true)` wakes it at the count stop, so
    firmware needs no polling.
  - `schedule()` returns the period `Schedule` at the latest polled tick.
- **Switch inputs (`gpio::SwitchPinBuilder`)** — a switch between any
  header pin and GND: a required contact type (`gpio::contact::No` /
  `Nc`), a required pull (`Pull::Internal` / `External(ohms)`, checked
  against the board's own loads: WiFi D13's 100 kΩ pull-down needs an
  external pull-up of at most 25 kΩ), an optional IRQ digital filter
  (`SwitchFilter`), a hardware-latched activation edge (`with_edge`, pins
  with an `IRQn`), and a wake-up from `WFI` on that edge (`with_wake`).
  `Switch::new(SETUP).with_peripherals(p).build()`; `is_active()`,
  `activated()`, `free(p)`. Normally-closed switches with the edge wake-up
  were run on the cart-pole rig.
- **Wake timer (`wake::WakeTimer`)** — a one-shot timed wait on any GPT
  channel that ends a `WFI` once. Its overflow stops its own counter in
  hardware through an ELC input. `WakeTimerBuilder::new()
  .with_timer::<GPT167>().with_input::<ElcE>().build()`, then
  `WakeTimer::new(WAKE).with_peripherals(p).build()`. `start(Duration)`
  picks the finest prescaler that reaches the wait and rounds up: 20.8 ns
  steps up to 1.365 ms, up to 1.398 s on a 16-bit channel
  (`WakeError::TooLong` past that). `expired()` reads the overflow flag,
  so it works with interrupts masked; `cancel()`, `is_armed()`,
  `longest()`, `free()`.
- **Event link (`elc`)** — the Event Link Controller: `Elc::link` /
  `unlink` route an `ElcEvent` to one of the GPT's eight ELC inputs, and
  `fire` raises software events. The inputs are `Peripherals` parts
  (`elc_a` … `elc_h`, types `ElcA` … `ElcH`) and `link` takes a claimed
  one, so two users of one input fail `cargo check`.
- **Header pin model (`gpio::pin_map`)** — one pin table with each port
  pin's functions (GPT output, ADC channel, DAC, IRQ, SCI/IIC/SPI/CAN
  signals with their `PSEL`) from manual Tables 19.5–19.13, and the header
  it is on per board. Drivers check pins with trait bounds (`Gtioc`,
  `AdcPin`, `SciPin`, `IrqPin`, …), so a wrong pin fails `cargo check`
  with a message naming it (`` `A0` has no GPT output ``).
  `gpio::HeaderPin` marks the header pins; DIR, ENA and switch pins
  require it.
- **PWM on all 16 headers with a GPT output**, and **ADC on AN019/AN020**
  (WiFi D10/D13, Minima D4/D5). PWM on the newly reachable headers, ADC on
  D10/D13 and the `PSEL` exceptions are not yet run on a board.
- **`builder`** — the shared slot types (`Missing`, `Set`, `Provided`,
  `slot`) behind the compile-time setups.
- **`peripherals`** — `Claim`, `Release`, `Taken`, `Board` for claiming
  parts of `Peripherals` by type. `Peripherals` gains `gpt162`, `gpt164`,
  `gpt165`, `dtc`, `elc` and `elc_a` … `elc_h`.
- **`intc`** — `enable_interrupts`, `Ielsr::disable` and
  `Ielsr::release`, `Event::gpio_irq`.
- **docs.rs** documents the `usb` and `uart-irq` APIs
  (`[package.metadata.docs.rs]`), building a single target.

### Changed

- **Pins are named by their Arduino header (breaking).**
  `gpio::Pin<const PORT: char, const PIN: u8, MODE>` is now
  `Pin<ID, MODE>`, where `ID` is the header's type (`p.pins.d6` is a
  `Pin<D6, Input>`) or a board-internal pin (`UsbMux`,
  `Sci1Tx`/`Sci1Rx`, WiFi `Sci9Tx`/`Sci9Rx`). `gpio::Pins` is generic
  over the 20 header pins' types, with the same field names.
- **Driver constructors take any valid pin (breaking).**
  - `Pwm::d3`/`d5`/`d6`/`d9`/`d10`/`d11` → `Pwm::new(timer, pin,
    freq_hz, clocks)`; the type is `Pwm<D6>`.
  - `Uart::sci2`/`sci9`/`sci1` → `Uart::new(sci, tx, rx, baud, clocks)`
    on any TX/RX pair the pin table gives the channel;
    `UartIrq::sci2`/`sci9`/`sci1` → `UartIrq::attach`. Each pin is muxed
    with its own `PSEL`.
  - `Adc::read` takes any pin with an ADC channel; `adc::Channel` is
    removed.
  - `UsbPort::take` and `take_usb_port` take the new `UsbMux` pin type.
- **`Peripherals` is generated with one type per header pin, GPT channel,
  the DTC and each ELC input (breaking).** A part claimed through
  `with_peripherals` becomes `Taken` in the returned `Peripherals`, so a
  second use fails to compile (`` `D5` is already used by another driver
  ``). Build the `with_peripherals` drivers before moving fields out
  directly: a partly moved `p` can't be passed whole.
- **Minimum supported Rust version raised from 1.85 to 1.88 (breaking).**
  The 0.1.x code already needed 1.88: the `usb` feature calls
  `<[T]>::as_chunks` (stable in 1.88), and `usb`/`uart-irq` use
  `if let … && let …` chains. 1.88 is the lowest version consistent with
  those uses; it is not proven by a 1.88 build (the bench builds with
  1.99).

### Removed

- **Debugging hooks that were public API (breaking).** They existed to
  debug the HAL during bring-up, not for firmware:
  - `UartIrq::txi_events()`, a count of transmit interrupts;
  - the `usb-trace` feature, with `usb::bus::start_diagnostic_trace()`
    and `usb::bus::diagnostic_trace()`, a RAM trace of USB transfers.

### Fixed

- **The published crate carries the MIT licence text.** Cargo packages
  only the crate's folder, so the repository's `LICENSE.md` never shipped
  in 0.1.x. A copy is now in the crate.
- **`Peripherals` had no `gpt162`**, so `Pwm::d10` (WiFi) and
  `Pwm::d5`/`d10` (Minima) could not be called.
- **The UART docs claimed `PSEL` is set per SCI channel class** (even
  channels `00100b`, odd `00101b`). The manual has exceptions: P102's
  `TXD2` is under `00101b` (Table 19.6), and `SCK1` is `00100b` on P109
  but `00101b` on P112 (Table 19.7). `PSEL` is now a per-pin property from
  the pin table.
- Broken intra-doc links; `cargo doc` is warning-free with default
  features, with `usb,uart-irq`, and on the Minima map.

[Unreleased]: https://github.com/TonyMarkham/unor4-hal/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.2.0

## [0.1.2] - 2026-10-21

### Added

- Crate packaging metadata: `keywords` (`arduino`, `embedded`, `hal`,
  `ra4m1`, `no-std`) and `categories` (`embedded`, `hardware-support`,
  `no-std`). crates.io reads these from the manifest at publish time and
  they cannot be added to an already-published version, so they land on
  this release.
- Manifest `rust-version = "1.85"` (inherited from the workspace), so
  users on older toolchains get a clear error instead of an opaque one.

[0.1.2]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.2

## [0.1.1] - 2026-10-21

### Added

- Crate packaging metadata: `repository` link and a crate-facing `README.md`
  (`crates/unor4-hal/README.md`), so crates.io renders them. Published
  versions are immutable, so 0.1.0's page cannot gain these — 0.1.1 exists
  to carry them; no code changed.

[0.1.1]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.1

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
