# Limitations — what we found, in plain language

> *Copied from the owner's course repo (`arduino-embedded-rust`), where this
> analysis was done. References to "Part 7", roadmap items, and course docs
> point at that repo; the actionable outcome of this document is
> [HalSpec.md](HalSpec.md) in this repo.*

No steps in this file. It's a reference: what the jargon means, how the
board's serial wiring actually works, and a plain-language list of every
shortcoming we've confirmed while building Part 7. Written after that
research; each fact lists where it came from.

## First, the words

| Term | What it actually is |
|---|---|
| **Header, D0–D13, A0–A5** | The holes along the board's edge. "D" pins are digital, "A" pins are analog. These are Arduino's numbers — printed on the board, used by every tutorial. |
| **P102, P301, P501, …** | The *chip's* own name for a physical pin ("Port 1, pin 02"). Same piece of metal, different name. Header hole **D13** is chip pin **P102**. Both naming systems appear in datasheets and source code, which is half the confusion. |
| **UART ("serial port")** | The simplest way two chips talk: two wires — **TX** (transmit) and **RX** (receive) — sending bytes one at a time at an agreed speed ("baud", here 115200). Cross-wiring TX↔RX is the classic mistake. |
| **SCI2, SCI9, SCI1** | The RA4M1 chip contains several *independent* UART engines, numbered SCI0–SCI9. Think of them as several mail rooms inside one building — each mail room can only serve certain windows (pins). SCI2 serves D0/D1, SCI9 serves P109/P110, SCI1 serves P501/P502. |
| **USB CDC** | A USB device that *pretends* to be a serial port. This is why a "COM" port appears when you plug the board in. |
| **The bridge** | The ESP32-S3's day job. It runs Arduino firmware ("UNOR4USBBridge") that presents the board to your PC as a USB device and shuttles bytes between the PC and the main chip. |
| **Level translator** | A chip between the two processors. The RA4M1 speaks 5 V, the ESP32-S3 speaks 3.3 V — connecting them directly would damage something. |
| **Mux (analog switch)** | Tiny switch chips (U2/U6) that decide who the USB plug's wires connect to: the ESP32-S3 (default) or the RA4M1's own USB hardware (when a C++ sketch starts native USB). |
| **HAL** | "Hardware abstraction layer" — the Rust library layer that turns friendly Rust calls into register writes. |
| **PAC** | "Peripheral access crate" — auto-generated access to the chip's raw registers. Every feature, zero conveniences. Roadmap item 6 territory. |
| **Register** | A numbered mailbox inside the chip. Write the right value, the hardware acts. |
| **FSP** | Renesas's official C driver library — what Arduino's C++ core calls into underneath. |

## One pin, several names — the ones that matter here

| Where you'd touch it | Chip pin | UART engine | Where the wire goes |
|---|---|---|---|
| Header **D0** | P301 | SCI2 (RX) | header only — nowhere else |
| Header **D1** | P302 | SCI2 (TX) | header only — nowhere else |
| *(not on header)* | P109 | SCI9 (TX) | → level translator → ESP32-S3 → USB plug |
| *(not on header)* | P110 | SCI9 (RX) | ← ESP32-S3 ← USB plug |
| *(not on header)* | P501 | SCI1 (TX) | → ESP32-S3 (WiFi command channel) |
| *(not on header)* | P502 | SCI1 (RX) | ← ESP32-S3 (WiFi command channel) |
| Header **D13** | P102 | — | onboard LED |

P109/P110 and P501/P502 are wired directly between the two chips — you can't
touch them from the header.

## The board's three serial "phone lines"

Two computers share one USB plug, and there are three separate routes a
serial byte can travel:

1. **USB plug ↔ mux switch ↔ RA4M1's own USB hardware.** When a *C++* sketch
   uses `Serial`, the Arduino core flips the mux and the main chip talks USB
   directly. This is the path the Part 1 monitor used — BlinkWipe's `tick`
   lines traveled here.
2. **RA4M1 P109/P110 ↔ level translator ↔ ESP32-S3 ↔ USB plug.** The bridge's
   data line. Sketch *uploads* travel this route (the bridge resets the main
   chip into bootloader mode and forwards the transfer), and in C++ this is
   `Serial1`.
3. **SCI2 ↔ header holes D0/D1.** Goes to the outside world and *nothing
   else* — not to the USB plug, not to the ESP32-S3.

Source: the ABX00087 datasheet's block diagram, plus the Arduino core and
bridge firmware source (links at the bottom).

## The shortcomings, one by one

### 1. The serial monitor can't hear Rust code — by wiring, not by bug

The Rust library's serial port is SCI2, which lands on header holes D0/D1 —
phone line 3. The monitor you've been using listens at the USB plug — phone
lines 1 and 2. The bytes physically have no path between them.

This is not a Rust failure. A C++ sketch writing to the same pins (`Serial2`)
is equally silent. The fix in Part 7 puts a USB-UART bridge on D0/D1 —
anything that turns those header pins into their own USB port. Verified
cheapest option: an R3 running a 6-line relay sketch
(`arduino/sketches/slice03-adapter`), no purchase needed; a ~$5 USB-UART
adapter works identically. The longer-term fix (monitor with no extra
board) needs shortcoming #2 or #3 solved.

### 2. The Rust library implements 1 of the chip's ~6 UART engines — and it's the least convenient one

The RA4M1 has several SCI engines. The existing community HAL implements
exactly one: SCI2. The two it skips are precisely the useful ones:

- **SCI9** (P109/P110) — phone line 2. A Rust UART here would show up in
  `arduino-cli monitor` with no adapter.
- **SCI1** (P501/P502) — the WiFi command channel. The mission depends on it
  (see #4).

This isn't a bug — it's one volunteer's early-days library. The chip is
capable of it; the code just hasn't been written. The engines are similar
enough that extending the HAL is realistic (the whole existing SCI2 driver is
~280 lines).

### 3. The Rust library has no USB support at all

The RA4M1 has USB hardware, and the C++ core proves it works (phone line 1 —
it drives it with **tinyusb**, a widely used open-source C USB stack). The
Rust side has generic USB stacks (`usb-device` + `usbd-serial`), but the
chip-specific driver piece that connects a stack to this chip's USB hardware
has, as far as I could find, never been published. So from Rust today there
is no way to talk to the monitor — the bridge in Part 7 is the workaround.

Writing that missing driver is a real (multi-evening) project, made smaller
by the fact that tinyusb provably runs on this chip — it could be linked
into a Rust build through the C ABI instead of rewritten.

### 4. Consequence for the mission: the WiFi lesson needs something the library can't do yet

All WiFi commands go through the ESP32-S3 over SCI1 (phone line 2's cousin:
P501/P502). Since the HAL only drives SCI2, the WiFi lesson (roadmap item 9)
will require either extending the HAL or writing the UART at the register
level — which is exactly what roadmap item 6 (bare-metal mode) teaches. The
roadmap has been updated to say this explicitly so it doesn't surprise us at
step 9.

### 5. A documentation trap: the published docs show the *other* board's pin numbers

The HAL supports both UNO R4 boards behind "feature flags": `minima`
and `wifi`. The boards differ in which chip pin sits behind header
holes D2–D7 and D10–D13. The automatically generated docs pages are built
with the *Minima* feature — so if you read the HAL's documentation and the
pin numbers look wrong, that's why.

Your `Cargo.toml` enables `wifi` (a board-safety rule in the course
repo's AGENTS.md for exactly this reason), so your firmware uses the correct
numbers — confirmed against the Arduino core's own pin table, line for line.
Verified consequence: the Part 6 blinky (D13 = P102) drives the real onboard
LED.

## Escape hatches, ranked by effort

| Option | What it buys | Effort |
|---|---|---|
| USB-UART bridge on D0/D1 (**Part 7 — works now**; R3 + relay sketch, or a ~$5 adapter) | A serial window into Rust firmware | trivial (spare R3 or adapter, 3 wires) |
| Extend the HAL with SCI9 | Monitor output with no adapter | an evening or two |
| Extend the HAL with SCI1 | Unlocks the WiFi lesson (mission-critical) | similar to SCI9 |
| Link tinyusb via the C ABI | Native USB CDC from Rust (the C++ `Serial` experience) | multi-evening project |
| Full Renesas FSP via C bindings | Everything, the C++ way | days of integration; not worth it for this project |

## Where every fact above came from

- **ABX00087 product reference manual** (Arduino, rev. 18 Apr 2023) — block
  diagram showing P109/P110 ↔ ESP32 and P501/P502 ↔ ESP32, the mux/USB
  section, the two-chip overview.
- **ArduinoCore-renesas** (the C++ core) — `variants/UNOWIFIR4/variant.cpp`
  and `pins_arduino.h` (which UART object sits on which pins),
  `variants/UNOWIFIR4/pinmux.inc` (which SCI engine serves which pin),
  `cores/arduino/Arduino.h` (`Serial` = `SerialUSB`), `cores/arduino/Serial.cpp`
  (the core calls Renesas FSP C drivers underneath).
- **uno-r4-wifi-usb-bridge** (the ESP32-S3 firmware) — `UNOR4USBBridge.ino`:
  the bridge forwards its UART to the USB CDC port unconditionally, and
  toggles the main chip's reset/boot lines for uploads.
