# unor4-hal

An independent hardware abstraction layer (HAL) for the Arduino UNO R4
(Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 Minima** and
**UNO R4 WiFi** boards.

- `no_std`, no allocation, no panics in the library — errors are returned
- typestate GPIO: a pin's mode is tracked in the type system, input and output
- two board pin maps — `wifi` (default) and `minima` — mutually exclusive at
  compile time; building with both or neither fails the build
- golden-table tested: every `cargo test` proves the compiled pin map equals a
  checked-in transcription of the boards' hardware facts
- cycle-accurate SysTick delay (`embedded-hal::delay::DelayNs`)
- HOCO 48 MHz clock initialization at startup

Design, requirements, and build order live in the repo's spec docs:
[HalSpec.md](HalSpec.md), [Plan.md](Plan.md), [Limitations.md](Limitations.md).

## Status

Early development — the API is unstable and the crate is not yet published
to crates.io. Verified on real hardware so far: 48 MHz clock init, GPIO
output, SysTick delay (1,584-byte blinky firmware), and SCI2 UART output
observed through a classic-UNO serial bridge. Slice 2 (input GPIO, both
board pin maps) is implemented and host-tested; its hardware run is
pending. Slice 3 (UART SCI2 polling, baud calculator host-tested) is
implemented and verified on the bridge (boot banner + tick stream); its
10-minute soak run is pending. Slice 4 (UART SCI9 — the monitor path, no
adapter) is implemented and host-tested; its hardware run is pending.

Planned next: interrupt support (ICU/IELSR), PWM, ADC, data flash, and
USB CDC-ACM.

## Install

One-time setup, in order:

1. **Rust** — install [rustup](https://rustup.rs) if you don't have it.
   This repo pins its toolchain in `rust-toolchain.toml`; the first `cargo`
   command you run inside the repo installs that exact toolchain
   automatically.
2. **The MCU target** — the RA4M1 is an Arm Cortex-M4F:

   ```powershell
   rustup target add thumbv7em-none-eabihf
   ```

3. **Firmware tools** — `cargo objcopy` (ELF → flashable binary) and
   `cargo size` come from cargo-binutils:

   ```powershell
   cargo install cargo-binutils
   ```

4. **The uploader** — [arduino-cli](https://docs.arduino.cc/arduino-cli/installation/):
    - Windows: 64-bit [MSI](https://downloads.arduino.cc/arduino-cli/arduino-cli_latest_Windows_64bit.msi)
      from Arduino's installation page, or the install script from Git Bash.
    - macOS/Linux: `brew install arduino-cli`
    - Confirm: `arduino-cli version`

## Get the code

```powershell
git clone https://github.com/TonyMarkham/unor4-hal.git
cd unor4-hal
```

The workspace has two crates: `crates/unor4-hal` (the library) and
`crates/test-firmware` (one flashable binary per slice — see
*Testing on hardware*).

## Test the library (no hardware needed)

From the repo root:

```powershell
cargo test -p unor4-hal
```

Expected: `16 passed; 0 failed` - the count grows with each slice. These
run on the host: clock math, the delay table, the golden pin-table
cross-check that proves the compiled board map matches the transcribed
hardware facts, and the SCI2 baud calculator (every shipped baud is
proven within 3% before any hardware run).

## Using the library

Blink the onboard LED:

```rust
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use unor4_hal::Peripherals;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let mut delay = p.delay;
    let mut led = p.pins.d13.into_output();

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

Firmware binary dependencies:

```toml
[dependencies]
# Until first crates.io publish — either a sibling checkout:
unor4-hal   = { path = "../unor4-hal" }
# ...or the git form:
# unor4-hal = { git = "https://github.com/TonyMarkham/unor4-hal.git" }
embedded-hal = "1.0.0"
cortex-m-rt = { version = "0.7.7", features = ["set-vtor"] }
panic-halt  = "1.0.0"
```

The hardware-proven version of the example above lives at
`crates/test-firmware/src/bin/slice01_blink.rs` — flash it per
*Testing on hardware*.

## Building firmware

> Build every slice binary:
>
> > [!IMPORTANT]
> >
> > Run every firmware command from `crates/test-firmware/`:
> > that folder's `.cargo/config.toml` pins the cross-compile target; from the
> > repo root the same commands silently build for the **host** and produce a
> > binary that cannot run on the board.
>
> ```powershell
> cargo build --release
> ```
>
> The WiFi pin map is the default; build the Minima map with:
>
> ```powershell
> cargo build --release --no-default-features --features minima
> ```

Flash safety: the Arduino bootloader occupies the first 16 KiB of flash.
Byte 0 of every converted image lands at flash `0x4000`, right where the
bootloader expects the sketch — the bootloader is never touched.

## Testing on hardware

One subsection per slice, updated in the same commit as the slice — the
durable record of "the board shows it". Each subsection is complete:
build, convert, flash, evaluate. The COM port is discovered once in the
preamble below and set as `$PORT` — nothing in the
per-slice commands is machine-specific.

Two rules for every upload, from the course's verified procedure:

- **Never upload cold** — double-tap the reset button first (LED fades =
  bootloader mode), then re-check the port with `arduino-cli board list`.
- If the upload dies mid-write with repeated `SAM-BA operation failed`,
  the DFU escape hatch in the course repo's `Basics.md` recovers it.

> ## Preamble
> **Before either test** (run after the double-tap reset: the
> bootloader can enumerate a different COM number than the sketch).
>
> Setting `$PORT` (the R4, used by every upload) and `$uartPort` (the
> slice 3 bridge UNO) here feeds every command below. Boards are picked
> by board name, not by port position — a second connected board cannot
> hijack `$PORT`.
> If a later flash fails on the port, re-run the preamble and flash again.
> The flashing flow is verified on Windows PowerShell
> On macOS/Linux the same steps apply, with
> the port discovered as `PORT=$(arduino-cli board list --format json | jq -r '.ports[0].address')`.
>
> > ### List the boards and find the UNO R4's port:
> > ```powershell
> > arduino-cli board list
> > ```
> > > Example Result:
> > > ```text
> > > Port    Protocol  Type                 Board Name           FQBN                           Core
> > > COM3    serial    Arduino UNO R4 WiFi  Arduino UNO R4 WiFi  arduino:renesas_uno:unor4wifi  arduino:renesas_uno
> > > COM4    serial    Arduino UNO          Arduino UNO          arduino:avr:uno                arduino:avr
> > > ```
> >
>
> > ### Set the port and verify it:
> > ```powershell
> > $PORT = (arduino-cli board list --format json | ConvertFrom-Json).detected_ports | Where-Object { $_.matching_boards.fqbn -eq "arduino:renesas_uno:unor4wifi" } | Select-Object -First 1 | ForEach-Object { $_.port.address }
> > ```
> > ### Proof:
> > ```pwsh
> > $PORT
> > ```
> > > ### Sample Result:
> > > ```pwsh
> > > COM3
> > > ```
>
> > ### List the boards and find the bridge UNO's port:
> > ```powershell
> > arduino-cli board list
> > ```
> > > Example Result:
> > > ```text
> > > Port    Protocol  Type                 Board Name           FQBN                           Core
> > > COM3    serial    Arduino UNO R4 WiFi  Arduino UNO R4 WiFi  arduino:renesas_uno:unor4wifi  arduino:renesas_uno
> > > COM4    serial    Arduino UNO          Arduino UNO          arduino:avr:uno                arduino:avr
> > > ```
> >
>
> > ### Set the bridge port and verify it:
> > ```powershell
> > $uartPort = (arduino-cli board list --format json | ConvertFrom-Json).detected_ports | Where-Object { $_.matching_boards.fqbn -eq "arduino:avr:uno" } | Select-Object -First 1 | ForEach-Object { $_.port.address }
> > ```
> > ### Proof:
> > ```pwsh
> > $uartPort
> > ```
> > > ### Sample Result:
> > > ```pwsh
> > > COM4
> > > ```

> ### Slice 1 — blink (`slice01_blink`)
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the firmware in release mode (size-optimized, LTO) and
> prints the final `text` size — the budget recorded in the binary's
> doc comment:
>
> ```pwsh
> cargo build --release --bin slice01_blink
> ```
>
> Converts the linked ELF into a raw binary image — the file format
> `arduino-cli upload` consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice01_blink -- -O binary slice01_blink.bin
> ```
>
> **Flash** — double-tap reset first (LED fade = bootloader mode), then:
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice01_blink.bin
> ```
>
> (Minima board: `--fqbn arduino:renesas_uno:minima`.)
>
> **Evaluate:**
>
> - **Pass:** the onboard LED (D13) blinks at exactly 1 Hz — 1 s on, 1 s
    >   off, steady, indefinitely.
> - **Fail → no blink at all:** re-check that the upload reported
    >   success; try the never-upload-cold reset again.
> - **Fail → blinks at the wrong speed:** the clock or SysTick init is
    >   wrong — stop and investigate before any other slice.
>
> **Verified:** see the binary's doc comment for the recorded `text`
> size (1,584 bytes at slice 1).

> ### Slice 2 — button (`slice02_button`)
>
> > [!IMPORTANT]
> > **Wiring:** a pushbutton between **D2 and GND** — nothing else. The
> > internal pull-up idles the line high, so pressed = low.
>
> **Build and convert** (from `crates/test-firmware/`, WiFi map):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the button-mirror firmware for the WiFi pin map:
>
> ```powershell
> cargo build --release --bin slice02_button
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> **Flash** (never-cold rule applies):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> **Evaluate:**
>
> - **Pass:** LED off at rest; press the button → LED on; release → LED
    >   off. Repeatable, immediate, no visible flicker.
> - Contact bounce is invisible — a *level* mirror has no edges to
    >   double-count; that is expected, not a bug.
> - **Fail → LED always on:** button wired to 5 V instead of GND, or the
    >   wrong header pin.
> - **Fail → LED never reacts:** confirm the WiFi image is flashed and
    >   the button is on D2 (P104 on the WiFi map).
>
> **Minima-map cross-check (optional but instructive):** build, convert
> and flash the Minima map — the same code now listens on P105, which on
> a WiFi board is header **D3**.
>
> Rebuilds the same firmware with the `minima` pin map selected:
>
> ```pwsh
> cargo build --release --no-default-features --features minima --bin slice02_button
> ```
>
> Overwrites `slice02_button.bin` with the Minima-map image (same
> output name, different pin table):
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> Flashes the Minima-map image over the still-set `$PORT`:
>
> ```pwsh
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> Move the button from D2 to D3: it works again. That is the pin-map
> switch, observed on hardware. **Then restore the WiFi image.**
>
> Rebuilds and overwrites `slice02_button.bin` with the default
> (WiFi-map) image:
>
> ```powershell
> cargo build --release --bin slice02_button
> ```
>
> Converts the restored WiFi-map ELF:
>
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
>
> Re-flashes the WiFi map so the board is back to its standard state
> (button on D2 again):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> **Verified:** pending hardware run.

> ### Slice 3 - UART SCI2 (`slice03_sci2`)
>
> > [!IMPORTANT]
> > **Test setup** (no USB-UART adapter needed): a classic UNO
> > (`arduino:avr:uno`) runs the relay sketch
> > `arduino/sketches/slice03-adapter/slice03-adapter.ino` and acts as
> > the USB-UART bridge. Wiring, 3 wires:
> >
> > - UNO **D0** <-> R4 **D1** (P302, the R4's TX)
> > - UNO **D1** <-> R4 **D0** (P301, the R4's RX)
> > - UNO **GND** <-> R4 **GND**
> >
> > The relay forwards both directions: anything the R4 sends appears
> > in the monitor on the UNO's USB port, and anything typed in the
> > monitor goes to the R4. Monitor runs at **115200 8N1**.
>
> **Install the bridge sketch** (one-time, from the repo root; the
> bridge UNO's `$uartPort` comes from the Preamble).
>
> Compiles the relay sketch for the classic UNO (AVR target):
>
> ```powershell
> arduino-cli compile --fqbn arduino:avr:uno arduino/sketches/slice03-adapter
> ```
>
> Installs it on the bridge UNO:
>
> ```powershell
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno arduino/sketches/slice03-adapter
> ```
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command below from `crates/test-firmware/`
>
> Compiles the SCI2 polling firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice03_sci2
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice03_sci2 -- -O binary slice03_sci2.bin
> ```
>
> **Flash** the R4 (upload port `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice03_sci2.bin
> ```
>
> **Evaluate:** open the **bridge UNO's** port at 115200 (not the R4's
> own USB port - this slice talks on the header pins):
>
> ```powershell
> arduino-cli monitor -p $uartPort --config baudrate=115200
> ```
>
> Then plug in (or reset) the R4 and watch the monitor.
>
> - **Pass:** one garbage byte at plug-in (expected — see below), then
    >   a boot banner and `tick N` once per second:
>
>   ```text
>   �unor4-hal slice 03 - SCI2 polling, 115200 8N1
>   tick 0
>   tick 1
>   tick 2
>   ```
>
>   Let it run for **10 minutes**: no garbage, no hang, no missing
    >   seconds.
> - **Expected, not a fail - one garbage byte before the banner when
    >   the R4 is plugged in:** while the R4's power rail ramps up, its
    >   unpowered TX pin clamps the shared line low and the receiving
    >   UART latches one junk byte. This happens before any code runs on
    >   the R4 and would appear with any firmware (or a bare USB-UART
    >   adapter). Exactly one byte at plug-in = pass; garbage *during*
    >   the tick stream = fail.
> - **Fail - nothing at all:** check the three wires (D0<->D1 crossed
    >   as listed above, GND common) and that the relay sketch is on the
    >   bridge UNO.
> - **Fail - garbage characters in the tick stream:** baud mismatch.
    >   Run `cargo test` - the calculator's 3% tolerance is proven on the
    >   host, so suspect the monitor's configured rate first.
>
> **Verified:** host-verified (9 baud-calculator tests, both board-map
> builds, `text` = 4,084 bytes); boot banner + ticks observed on the
> bridge (2026-10-18); 10-minute soak run pending.

> ### Slice 4 — UART SCI9, the monitor path (`slice04_sci9`)
>
> > [!IMPORTANT]
> > **Test setup** (no adapter, no second board): SCI9 talks on the
> > board-internal **P109 (TX) / P110 (RX)** pair, which runs through
> > the level translator to the ESP32-S3. The bridge firmware forwards
> > that line to the board's own USB port — the same port `$PORT` from
> > the Preamble. Monitor at **115200 8N1**.
> >
> > **Never open this port at 1200 or 2400 baud.** Those are the
> > bridge's magic rates: 1200 resets the RA4M1, 2400 forces it into
> > DFU boot mode (observed in the bridge firmware's USB line-coding
> > handler).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> Compiles the SCI9 polling firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice04_sci9
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice04_sci9 -- -O binary slice04_sci9.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice04_sci9.bin
> ```
>
> **Evaluate:** open the **R4's own port** (the bridge CDC port — the
> same `$PORT` the upload used) at 115200:
>
> ```powershell
> arduino-cli monitor -p $PORT --config baudrate=115200
> ```
>
> Then reset the board and watch the monitor.
>
> - **Pass:** a boot banner and `tick N` once per second, with no
    >   adapter and no second board connected:
>
>   ```text
>   unor4-hal slice 04 - SCI9 polling, 115200 8N1
>   tick 0
>   tick 1
>   tick 2
>   ```
>
> - **Expected, not a fail:** bridge/ESP32 diagnostic noise around
    >   reset — the bridge shares this port with its own logs. Garbage
    >   *during* the tick stream = baud mismatch (the bridge reconfigures
    >   its UART to the monitor's baud, so suspect the monitor config
    >   first).
> - **Fail — nothing at all:** confirm the WiFi image is flashed and
    >   the monitor is on the R4's port, not a second board.
>
> **This line is also the upload path.** The bootloader talks to the
> bridge over the same SCI9 pins, so the run is only complete once
> flashing is proven intact: after the tick stream, re-flash this same
> binary (double-tap reset, upload again). If the upload succeeds, the
> shared wire survived — record it under **Verified** below.
>
> **Verified:** pending hardware run.

## License

MIT — see [LICENSE.md](LICENSE.md).
