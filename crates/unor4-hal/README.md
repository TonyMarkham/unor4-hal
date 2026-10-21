# unor4-hal

An independent hardware abstraction layer (HAL) for the Arduino UNO R4
(Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 WiFi** (default
`wifi` pin map) and **UNO R4 Minima** (`minima` pin map) — exactly one map
per build, enforced at compile time.

`no_std`, no allocation, no panics in the library — errors are returned.
No fork, no copied code: hardware facts are cited to the Renesas RA4M1
hardware manual and SVD; the Arduino core and Renesas FSP are behavioral
cross-check data only.

## Features

- **Clock** — HOCO 48 MHz initialization; `Clocks` resolves PCLKA / PCLKB /
  PCLKD / ICLK for the other drivers
- **GPIO** — typestate pins (`Input`, `InputPullUp`, `Output`,
  `OutputOpenDrain`, `Alternate`, `Analog`), exactly-once `Peripherals`
  claim, both board pin maps shown in every pin's docs
- **Delay** — cycle-accurate SysTick `DelayNs`
- **UART** — SCI2 (header D0/D1), SCI9 (WiFi monitor line), SCI1 (WiFi AT
  line) in polling 8N1; baud calculator with 3% tolerance
- **`uart-irq`** — interrupt-driven `UartIrq` with ring buffers;
  `embedded-hal-nb`, `embedded-io` and `core::fmt::Write` impls
- **Interrupts (`intc`)** — ICU/IELSR event table, linking, dispatch
  registry; `critical-section` provider
- **PWM (`pwm`)** — GPT driver, `embedded-hal` 1.0 `SetDutyCycle`, all six
  header PWM pins with engine/chip-pin/output fixed at compile time
- **ADC (`adc`)** — single-shot reads on A0–A5 via the ADC140, 12- or
  14-bit resolution
- **USB (`usb` feature)** — USBFS as a `usb_device::bus::UsbBus`, blocking
  CDC-ACM wrapper; explicit, documented USB-mux takeover on the WiFi board
- **Data flash (`dflash`)** — 8 KB: byte programming (once per erase,
  blank-checked first), 1 KB block erase, blank check; code flash is
  unreachable through the API

## Install

```toml
[dependencies]
unor4-hal = "0.1"
```

Add `"usb"` and/or `"uart-irq"` to the `features` list as needed.

## Hardware verification

Every feature was proven on a real UNO R4 WiFi before release; the
[repository README](https://github.com/TonyMarkham/unor4-hal) records each
witnessed run and the two-board validation rig that re-proves all of them
in one command.

**UNO R4 Minima builds and passes the host test suite but is
hardware-unverified** — no Minima hardware exists in this project.

On the WiFi board, USB CDC and the ESP32-S3 bridge share one connector:
taking the USB port disconnects the bridge (explicit, documented API).

## License

MIT — see [LICENSE.md](LICENSE.md).
