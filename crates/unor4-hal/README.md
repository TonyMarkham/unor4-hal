# unor4-hal

An independent hardware abstraction layer (HAL) for the Arduino UNO R4
(Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 WiFi** (default
`wifi` pin map) and **UNO R4 Minima** (`minima` pin map) — exactly one map
per build, enforced at compile time.

`no_std`, no allocation. Setups built with the builders are refused at
`cargo check`; a few run-time constructors panic on a bad number instead
(`Uart::new` / `attach` on an unreachable baud, `Pwm::new` on an
unreachable frequency, `UartIrq::attach` with no free interrupt slot).
No fork, no copied code: hardware facts are cited to the Renesas RA4M1
hardware manual and SVD; the Arduino core and Renesas FSP are behavioral
cross-check data only.

## Features

- **Clock** — HOCO 48 MHz initialization; `Clocks` resolves PCLKA / PCLKB /
  PCLKD / ICLK for the other drivers
- **GPIO** — pins named by their Arduino header: `Pin<D6, Output>`. One
  pin table (`gpio::pin_map`) gives each header's functions (GPT output,
  ADC channel, SCI/IIC/SPI/CAN signals, IRQ), so drivers check their pins
  with trait bounds and a wrong pin fails `cargo check` with a message
  naming it (`` `A0` has no GPT output ``). Typestate modes (`Input`,
  `InputPullUp`, `Output`, `OutputOpenDrain`, `Alternate`, `Analog`),
  exactly-once `Peripherals` claim
- **Delay** — cycle-accurate SysTick `DelayNs`
- **UART** — any TX/RX pair the pin table gives an SCI channel, in polling
  8N1: SCI2 on header D1/D0, SCI9 on the WiFi monitor line, SCI1 on the
  WiFi AT line; baud calculator with 3% tolerance
- **`uart-irq`** — interrupt-driven `UartIrq` with ring buffers;
  `embedded-hal-nb`, `embedded-io` and `core::fmt::Write` impls
- **Interrupts (`intc`)** — ICU/IELSR event table, linking, dispatch
  registry; `critical-section` provider
- **PWM (`pwm`)** — GPT driver, `embedded-hal` 1.0 `SetDutyCycle`, on any
  of the 16 headers with a GPT output: `Pwm::new(p.gpt163, p.pins.d6, …)`;
  the channel the pin needs is a type check
- **ADC (`adc`)** — single-shot reads via the ADC140 on any pin with an ADC
  channel (A0–A5, plus WiFi D10/D13 and Minima D4/D5), 12- or 14-bit
  resolution
- **USB (`usb` feature)** — USBFS as a `usb_device::bus::UsbBus`, blocking
  CDC-ACM wrapper; explicit, documented USB-mux takeover on the WiFi board
- **Data flash (`dflash`)** — 8 KB: byte programming (once per erase,
  blank-checked first), 1 KB block erase, blank check; code flash is
  unreachable through the API
- **Event link (`elc`)** — the Event Link Controller: routes peripheral
  events to the GPT's eight ELC inputs (count, start/stop/clear, capture)
  with no CPU in the path; software events
- **Step generator (`step`)** — drives a stepper driver's STEP and DIR
  (and optionally ENA) from a signed rate. Each part is set up by a builder
  checked at `cargo check`: STEP on any header with a 32-bit GPT output,
  DIR and ENA on any header, any counter and tick timers. It has a fixed
  LOW time, a stop that never cuts a pulse, an exact hardware pulse count,
  and a hardware stop at an exact count. Rate changes are applied by the
  DTC at a 2 ms hardware tick, missed ticks are recovered, and the DIR
  rule is handled for you. The tick and the count stop can each wake the
  CPU from `WFI`, so firmware needs no polling
- **Switch inputs (`gpio::SwitchPinBuilder`)** — a switch between any
  header pin and GND, checked at `cargo check`: normally open or normally
  closed, internal or external pull-up, an optional IRQ glitch filter, a
  hardware-latched activation edge on pins with an `IRQn`, and an optional
  wake-up from `WFI` on that edge
- **Wake timer (`wake::WakeTimer`)** — a one-shot timed wait on a spare GPT
  channel that ends a `WFI` once: stopped by its own overflow in hardware,
  never shorter than asked, in the finest step that reaches it (20.8 ns up
  to 1.365 ms, up to 1.398 s on a 16-bit channel), readable with
  interrupts masked
- **ELC inputs claimed by type** — `ELC_GPTA`–`ELC_GPTH` are parts of
  `Peripherals`; a driver claims the ones it links, and `Elc::link` takes a
  claimed input, so two users of one input fail `cargo check`

## Install

```toml
[dependencies]
unor4-hal = "0.2"
```

Add `"usb"` and/or `"uart-irq"` to the `features` list as needed.

## Usage

The user guide is the
[repository README](https://github.com/TonyMarkham/unor4-hal#readme): a
firmware project from scratch, then a compiled example for every driver.
For the stepper (PUL, DIR, ENA) and switches it covers what each builder
accepts and refuses, what to call per 2 ms tick, and every error. The API
docs carry the same guides in the `step` and `gpio` modules.

## Hardware verification

Every feature is proven on a real UNO R4 WiFi before release; the
two-board validation rig records each witnessed run and re-proves all of
them in three sessions, one per part of the R3's checks ([Validation.md](https://github.com/TonyMarkham/unor4-hal/blob/main/Validation.md)).
**Run on the cart-pole rig, not on the validation rig** (its report,
2026-11-04): the step generator's ENA pin, normally-closed limit switches
on D2 and D3 with the edge wake-up, and the count stop on a 32-bit counter
(GPT321).

**UNO R4 Minima builds and passes the host test suite but is
hardware-unverified** — no Minima hardware exists in this project.

On the WiFi board, USB CDC and the ESP32-S3 bridge share one connector:
taking the USB port disconnects the bridge (explicit, documented API).

## License

MIT — see [LICENSE.md](https://github.com/TonyMarkham/unor4-hal/blob/main/LICENSE.md).
