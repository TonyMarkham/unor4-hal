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
- interrupt infrastructure: the ICU/IELSR event system, a dispatch table
  behind the vector table, and an IRQ-mode UART with ring buffers
  (`embedded-hal-nb` / `embedded-io` / `core::fmt::Write` impls)

Design, requirements, and build order live in the repo's spec docs:
[HalSpec.md](HalSpec.md), [Plan.md](Plan.md), [Limitations.md](Limitations.md).

## Status

Early development — the API is unstable and the crate is not yet published
to crates.io. Verified on real hardware so far: 48 MHz clock init, GPIO
output, SysTick delay (blinky firmware), SCI2 UART output observed through
a R3 serial bridge, and SCI9 UART output through the board's own
USB monitor port. Slice 2 (input GPIO, both board pin maps) is implemented
and host-tested; its hardware run is pending. Slice 3's 10-minute soak run
is pending. Slice 4's upload-path re-verification is pending. Slice 5
(interrupts: ICU/IELSR event table, dispatch, IRQ-mode UART) is
hardware-verified on WiFi: simultaneous SCI2/SCI9 uppercase echo,
eight continuous 4 KiB SCI2 bursts, and an uninterrupted heartbeat.
Slice 6 (UART SCI1, the WiFi AT channel) is hardware-verified on WiFi:
the ESP32-S3's AT server answered `OK` to 9/9 witnessed `AT` rounds with
`ov=0 er=0` (README, slice 6). The IRQ-enabled host suite contains
33 tests. Minima is build/host-tested, not hardware-tested.

Planned next: UART SCI1 (the WiFi AT channel), PWM, ADC, data flash, and
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
cargo test -p unor4-hal --features uart-irq
```

Expected: `32 passed; 0 failed` - the count grows with each slice. These
run on the host: clock math, the delay table, the golden pin-table
cross-check that proves the compiled board map matches the transcribed
hardware facts, the SCI2 baud calculator (every shipped baud is
proven within 3% before any hardware run), the IELSR event-table
integrity checks, and the IRQ UART's ring buffers. (`cargo test -p
unor4-hal` without the feature runs 24 of them.)

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
# `device` is required since Slice 5: unor4-hal supplies the real
# 32-entry IELSR vector table, which replaces cortex-m-rt's generic
# 240-entry default.
cortex-m-rt = { version = "0.7.7", features = ["set-vtor", "device"] }
panic-halt  = "1.0.0"
```

The hardware-proven version of the example above lives at
`crates/test-firmware/src/bin/slice01_blink.rs` — flash it per
*Testing on hardware*.

## Building firmware

> Build the firmware: `test-firmware` has one binary, `r4_validator`, the
> R4 side of the validation pair (see All-in-one validation below).
>
> > [!IMPORTANT]
> >
> > Run every firmware command from `crates/test-firmware/`:
> > that folder's `.cargo/config.toml` pins the cross-compile target; from the
> > repo root the same commands silently build for the **host** and produce a
> > binary that cannot run on the board.
>
> ```powershell
> cargo build --release --features usb,uart-irq
> ```
>
> The WiFi pin map is the default; build the Minima map with:
>
> ```powershell
> cargo build --release --no-default-features --features minima,usb,uart-irq
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

> [!NOTE]
> **2026-10-20:** the per-slice programs and R3 sketches were retired in
> favour of the validation pair (All-in-one validation, at the end of this
> section). One `validate all` run re-proves Slices 1–9.
>
> The Slice 1–8 subsections below are the historical record. To repeat
> one exactly as written, run its commands in a worktree of commit
> `6649d4c`, where every program, script and sketch it names still exists:
>
> ```powershell
> git worktree add ..\unor4-hal-6649d4c 6649d4c
> ```
>
> The Slice 9 per-slice programs were never committed, so they cannot be
> recovered.

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
> slice 3 bridge R3) here feeds every command below. Boards are picked
> by board name, not by port position — a second connected board cannot
> hijack `$PORT`.
> If a later flash fails on the port, re-run the preamble and flash again.
> The flashing flow is verified on Windows PowerShell
> On macOS/Linux the same steps apply, with
> the port discovered as `PORT=$(arduino-cli board list --format json | jq -r '.ports[0].address')`.
>
> > ### List the boards and find the R4's port:
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
> > ### List the boards and find the bridge R3's port:
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
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S1.
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
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S2.
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
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S3.
>
> > [!IMPORTANT]
> > **Test setup** (no USB-UART adapter needed): an R3
> > (`arduino:avr:uno`) runs the relay sketch
> > `arduino/sketches/slice03-adapter/slice03-adapter.ino` and acts as
> > the USB-UART bridge. Wiring, 3 wires:
> >
> > - R3 **D0** <-> R4 **D1** (P302, the R4's TX)
> > - R3 **D1** <-> R4 **D0** (P301, the R4's RX)
> > - R3 **GND** <-> R4 **GND**
> >
> > The relay forwards both directions: anything the R4 sends appears
> > in the monitor on the R3's USB port, and anything typed in the
> > monitor goes to the R4. Monitor runs at **115200 8N1**.
>
> **Install the bridge sketch** (one-time, from the repo root; the
> bridge R3's `$uartPort` comes from the Preamble).
>
> Compiles the relay sketch for the R3 (AVR target):
>
> ```powershell
> arduino-cli compile --fqbn arduino:avr:uno arduino/sketches/slice03-adapter
> ```
>
> Installs it on the bridge R3:
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
> **Evaluate:** open the **bridge R3's** port at 115200 (not the R4's
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
    >   bridge R3.
> - **Fail - garbage characters in the tick stream:** baud mismatch.
    >   Run `cargo test` - the calculator's 3% tolerance is proven on the
    >   host, so suspect the monitor's configured rate first.
>
> **Verified:** host-verified (9 baud-calculator tests, both board-map
> builds, `text` = 4,084 bytes); boot banner + ticks observed on the
> bridge (2026-10-18); 10-minute soak run pending.

> ### Slice 4 — UART SCI9, the monitor path (`slice04_sci9`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S4.
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

> ### Slice 5 — interrupts: IRQ-mode UART echo + heartbeat (`slice05_echo`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S5.
>
> **Automated two-board regression (`slice05_rig`) — preferred:** connect
> R3 D0 ↔ R4 D1, R3 D1 ↔ R4 D0, D3 ↔ D3, and GND ↔ GND.
> Both boards remain connected over USB. The R4's D1 is TX and D0 is RX;
> the same is true for the R3. The controller uses D3 as an active-low
> test gate. No D2 button is needed for this automated binary.
>
> From `crates/test-firmware/`:
>
> ```powershell
> powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice05.ps1
> ```
>
> The execution-policy override applies only to this PowerShell process.
> The script discovers both ports by FQBN using `arduino-cli board list`,
> builds the reference/controller sketches and Rust firmware, and uploads
> all three in order. The reference sketch first leaves the R4's D0/D1
> as inputs so its TX driver cannot interfere with the R3 bootloader.
> Never send host commands to the R3 port while the R4 drives its RX pin.
>
> - `slice05-reference`: Arduino-core SCI9 register dump and echo
>   reference at 9600; also the safe staging image for R3 uploads.
> - `slice05-controller`: autonomous R3 checker at 115200.
>   It generates eight **continuous 4096-byte UART bursts**, receives the
>   uppercase echoes simultaneously, and compares every returned byte.
> - `slice05_rig`: both SCI2 and SCI9 in IRQ mode at 115200, both using
>   the public default rings (256 RX/128 TX). The main loop toggles
>   D13 every half-second using elapsed SysTick cycles, not loop counts.
> - The host independently sends 4096 paced bytes through the SCI9 USB
>   bridge and checks the complete uppercase response. Both paths must
>   have zero receive errors/overflows, no duplicate/inherited ICU routes,
>   and consecutive heartbeat reports about one second apart.
>
> The ESP32-S3 bridge dropped host USB bursts in this setup; that is not
> a HAL ring-size test. The unpaced burst test therefore uses the R3 wire,
> while SCI9 RX is checked separately with individually paced USB bytes.
> `arduino-cli monitor` local echo is not evidence of a board response.
>
> Logs are saved under `target/slice05-hardware/{r4,uno}.log`. To repeat
> against the already-flashed pair, add `-SkipBuild -SkipUpload`.
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, Arduino
> renesas_uno core 1.6.0. All eight rounds returned `sent=4096 received=4096
> mismatches=0` (32768 bytes); concurrent SCI9 echo matched all 4096 bytes.
> Both UART error/overflow counters stayed zero, `stale=0 routes=6
> duplicates=0`, and 25 heartbeat reports continued with observed arrival
> intervals of 0.993–1.016 seconds using the default rings. Repeated uploads succeeded without
> disconnecting the wires. LED electrical/visual timing was not measured;
> the automated heartbeat timing assertion measures serial report arrival.
>
> **Bugs fixed:** the bootloader left SCI9 event links in ICU slots 4–7.
> Linking those events again in slots 0–3 violated RA4M1 hardware manual
> R01UH0887EJ0100 §13.4.2 and caused one-shot IRQ delivery. HAL startup now
> clears inherited routes/NVIC state, and linking rejects duplicate events.
> A second race primed TDR while TXI already owned it, overwriting bytes;
> priming now requires idle TX interrupts. Per-byte TIE toggles, ISR polling,
> the bit-band experiment, and the unused TEI interrupt were removed.
>
> **Application startup:** after attaching IRQ peripherals, main calls
> `unsafe { unor4_hal::intc::enable_interrupts() };` outside any critical
> section. The bootloader leaves PRIMASK set. Enabling an individual IELSR
> slot no longer clears that global mask behind the caller's back.
>
> **Standalone manual example (`slice05_echo`):**
>
> > [!IMPORTANT]
> > **Test setup** (no adapter, no second board): the echo runs on the
> > same board-internal SCI9 monitor line as slice 4 — **P109 (TX) /
> > P110 (RX)**, forwarded by the ESP32-S3 bridge to the board's own
> > USB port (`$PORT` from the Preamble). Monitor at **115200 8N1**.
> > The same never-open-at-1200/2400-baud rule as slice 4 applies.
> > Wiring: the slice 2 pushbutton between **D2 and GND**. The firmware
> > waits until D2 reads low; it does not detect monitor attachment or
> > require a new button-press edge. This is not a host/board handshake.
> >
> > This binary needs the `uart-irq` feature (it is not a default):
> > the build command below enables it.
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> > >
> > > Run every command in this section from `crates/test-firmware/`
>
> Compiles the IRQ-mode echo firmware (release, size-optimized):
>
> ```powershell
> cargo build --release --bin slice05_echo --features uart-irq
> ```
>
> Converts the ELF into the raw binary the uploader consumes:
>
> ```pwsh
> cargo objcopy --release --bin slice05_echo --features uart-irq -- -O binary slice05_echo.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` from the Preamble):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice05_echo.bin
> ```
>
> **Evaluate:** upload, then open the R4's own port at 115200 (the same
> `$PORT` the upload used):
>
> ```powershell
> arduino-cli monitor -p $PORT --config baudrate=115200
> ```
>
> If the firmware is still waiting at its D2 gate, **press the D2
> pushbutton** after connecting the monitor. If the gate has already
> passed, ticks are already running and the one-time banner may have
> been sent before the monitor connected. The firmware has no monitor-
> attachment handshake or command to restart its tick count.
>
> - **Manual monitor check — ongoing heartbeat and counters:**
>
>   ```text
>   tick 10 ov=0 er=0
>   tick 11 ov=0 er=0
>   tick 12 ov=0 er=0
>   ```
>
>   The first visible tick need not be 1. Missing the banner is not, by
>   itself, a failure. Check that ticks continue and observe D13 for the
>   heartbeat; the main loop schedules its toggles every half-second.
> - **Typed text is not an echo test:** `arduino-cli monitor` locally
>   displays input, so seeing lowercase text such as `tony` does not
>   establish that the RA4M1 received or returned it. Typed text and board
>   output can share a display line; do not expect the clean separate
>   `hello` / `HELLO` transcript previously shown here. The firmware
>   uppercases bytes it actually receives, but this monitor command does
>   not establish instant per-keystroke delivery through the host/bridge.
>   Use the automated two-board test for verified byte-for-byte SCI9 echo.
> - **Counter interpretation:** `ov=0 er=0` means no RX-ring overflow or
>   receive-error event has been counted. It does not prove that typed
>   bytes reached the board or were echoed successfully.
> - **Burst verification:** use the automated two-board test above.
>   Pasting 4 KB into this bridge's monitor does not reliably deliver
>   those bytes to the RA4M1, regardless of the HAL's ring capacity.
> - **Expected around reset:** bridge/ESP32 diagnostic noise, as in
>   slice 4. Continued malformed board output or increasing counters
>   needs investigation; do not classify it as harmless reset noise.
> - **Fail — no echo but ticks fine:** distinguish host/bridge delivery
>   from the HAL receive path using the automated test and its counters.
> - **Fail — echo works but LED stutters:** investigate foreground
>   blocking or excessive ISR time; the heartbeat uses elapsed cycles.
>
> **Regression note:** this slice replaces cortex-m-rt's generic
> 240-entry interrupt table with the HAL's real 32-entry IELSR table.
> Every earlier binary got *smaller* (the table shrank by 832 bytes and
> grew by 32 one-purpose trampolines): blink 1,584 → 1,452 `text`,
> SCI2/SCI9 4,084/4,088 → 3,960 (historical measurements). Current WiFi
> release sizes from `cargo size --release --bin <name> --features uart-irq`:
> `slice05_echo`: text=9064, data=0, bss=668;
> `slice05_rig`: text=11508, data=0, bss=1072 bytes.
>
> **Verification scope:** automated rig verified above. The owner's
> standalone monitor capture showed ticks 9–15 with `ov=0 er=0` and mixed
> input/banner/heartbeat text on the first displayed line. That capture
> confirms ongoing heartbeat reports, not uppercase RX echo or D2-gate
> timing. The standalone button interaction remains unverified.

> ### Slice 6 — the WiFi AT channel (`slice06_at`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S6.
>
> Proves the mission-critical claim: the RA4M1 drives **SCI1**
> (P501 = TX, P502 = RX, board-internal to the ESP32-S3) and the
> ESP32-S3's AT server answers. The firmware sends `AT\r\n` once every
> 5 seconds for 30 seconds per run and relays every received byte to
> the SCI9 monitor path, with non-printables shown as `\xHH`.
>
> **Hardware facts** (full chain in `Research/slice-06-research.md`):
> P501/P502 mux to TXD1/RXD1 at `PSEL[4:0] = 00101b` (manual
> R01UH0887EJ0100 §19, Table 19.13); `MSTPCRB.MSTPB30` releases SCI1
> (manual §10.2.3); the stock bridge firmware implements its AT server
> via SudoMaker/chAT at 115200 8N1 (`uno-r4-wifi-usb-bridge`,
> `UNOR4USBBridge.ino` — behavioral reference).
>
> **Test setup:** no adapter, no wiring beyond the bench. The R4's
> monitor is `$PORT` at **115200 8N1** (`arduino-cli monitor` asserts
> DTR, which the bridge's CDC requires for output — a raw `SerialPort`
> reader must set `DtrEnable` or it will capture nothing). The run is
> gated by the bench line R3 D3 → R4 D3: `slice06-gate` (on the R3) holds
> the line high 30 s, low 50 s, cycling; the R4
> firmware runs one test per falling edge and re-arms on the rising
> edge, so one flash serves many runs. Upload the R4 first (its D0/D1
> stay inputs; the R3's upload shares its own D0/D1 with the cross-wire).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice06_at --features uart-irq
> cargo objcopy --release --bin slice06_at --features uart-irq -- -O binary slice06_at.bin
> ```
>
> **Flash** (never-cold rule applies; `$PORT` = the R4):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice06_at.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice06-gate
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice06-gate
> ```
>
> **Evaluate:** open the R4's port at 115200 during a high phase (dots
> mark the waiting firmware); the next falling edge prints:
>
> ```text
> gate open
> giving the ESP32-S3 AT server 2 s to settle...
> round 0: sending AT\r\n -> OK\x0d\x0a
> round 0 done: 4 bytes
> ...
> round 5: sending AT\r\n -> OK\x0d\x0a
> round 5 done: 4 bytes
> done: total=24 ov=0 er=0
> waiting for the gate to rise to re-arm
> ```
>
> The ESP32-S3 answers exactly `OK\r\n` — no command echo, no banner.
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, Arduino renesas_uno
> core 1.6.0. One complete run (6/6 rounds) and one partial capture
> (rounds 3–5, 3/3) — **9/9 witnessed rounds** answered exactly
> `OK\x0d\x0a`, `ov=0 er=0` throughout; the partial run's own report
> (`total=24`) shows its unwitnessed rounds also received 4 bytes each.
> Uploads succeeded before and after the runs over the same
> USB path. WiFi map only — P501/P502 exist on the Minima on its SWD
> connector, untested there. Size (WiFi, `uart-irq`): `slice06_at`
> text=14888, data=0, bss=668.
>
> **Fail — silence after `gate open`:** the AT server runs from the
> ESP32-S3's boot; power-cycle the whole board once (fresh ESP32 boot)
> before investigating further, then check `er=` (nonzero suggests a
> baud/format mismatch) and the PSEL value.
>
> **Fail — dots but no gate open:** the R3 side is not driving D3 low;
> verify the gate sketch is on the R3 and the D3↔D3 wire is seated.
>
> ### Slice 7 — PWM, the LED breathes (`slice07_breathe`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S7.
>
> Proves the GPT PWM driver: saw-wave PWM on **D6** at exactly 1 kHz
> (PCLKD/1, `GTPR = 47999`, 48 000 duty steps) with a sine duty profile,
> while the D13 heartbeat keeps ticking. The measurement is instrumented:
> D6 is wired to the R3's **D2 (INT0)** and the R3 sketch
> `slice07-measure` measures duty and period edge-by-edge — no LED, no
> eyeballing. Per gate round (R3 D3 → R4 D3, the slice 6 pattern) the
> firmware runs a 1 s settle, a duty staircase (0/25/50/75/100%, 2 s per
> step — the two extremes exercise `GTUDDTYC.OADTY` and the
> never-matching-compare mechanisms), then sine breathing (2.048 s
> cycle) until the next gate edge.
>
> **Hardware facts** (full chain in `Research/slice-07-research.md`):
> WiFi D6 = P111 → `GTIOC3A` on GPT163 at `PSEL[4:0] = 00011b` (manual
> §19 Tables 19.6/19.7/19.10/19.12, cross-checked entry-for-entry
> against the Arduino core's `pinmux.inc`); `MSTPCRD.MSTPD6` releases
> GPT162–167 (manual §10, p.175); the count clock is PCLKD = 48 MHz
> (manual §22.1/§8 Table 8.2); saw period = `GTPR + 1` (§22.2.20); duty
> 0%/100% via `GTUDDTYC.OADTY` / never-matching compare, because a
> compare match at the cycle end loses to the cycle-end setting in
> saw-wave mode (Table 22.5 Note 2, §22.3.6). The public trait is
> `embedded-hal` 1.0 `SetDutyCycle` (HalSpec §6 amended — the 0.2
> multi-channel `Pwm` trait does not exist in 1.0).
>
> **The full PWM surface (HalSpec §5.6):** all six header PWM pins are
> addressable, named by header pin with the engine/pin/output fixed at
> compile time — `Pwm::d3`, `d5`, `d6`, `d9`, `d10`, `d11` (D9 shares
> one constructor across both maps; the rest are per-map, like the pin
> tables). The mapping table is locked by the golden constructor test
> (`src/tests/pwm.rs`), which asserts every constructor's exact
> signature against manual §19 Tables 19.6/19.7/19.10/19.12. Hardware
> proof: WiFi D6 (GPT163) — the register recipe is common code, but the
> other pin/engine pairs have no bench wires and are compile-checked
> only. (D8 also carries `GTIOC7A` per Table 19.10 — outside the six
> spec pins; one more constructor if ever wanted.)
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice07_breathe
> cargo objcopy --release --bin slice07_breathe -- -O binary slice07_breathe.bin
> ```
>
> **Flash** (R4 first, then the R3 rig; never-cold rule applies;
> `$PORT` = the R4, `$uartPort` = the R3):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice07_breathe.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice07-measure
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice07-measure
> ```
>
> **Evaluate:** open both ports at 115200. The R4 prints the staircase
> steps and continuous `tick N` heartbeats; the R3 prints, per 10 s
> window, twenty 500 ms duty samples (`H`/`L` = flat high/low — the
> DC-line evidence for the 0%/100% steps) plus the aggregate. One
> aligned round (R3 `W3` against the R4's second staircase):
>
> ```text
> R4: step 0 duty=0 of 48000 ... step 4 duty=48000 of 48000
> R3: 0 L L L 252 252 248 248 501 500 500 501 752 752 748 752 749 H H H
> R3: W3 duty=500/1000 (min 244 max 756) period=1000us edges=6013
> ```
>
> Reading: period exactly 1000 µs; 6013 edges = exactly the ~6 s of the
> three edged steps; every commanded duty lands within ±1 tick of
> 48 000 (±4 per-mille); the extremes are DC lines (no edges), which is
> the OQ-P2 answer; breathing windows sweep 59→1000 per-mille
> (their period mean reads 1007–1019 µs because the R3's ISR merges
> edges when the pulse gaps at the sine extremes drop below the ISR
> duration — a rig artifact, not a GPT defect).
>
> **Verified on 2026-10-19:** R4 `COM3`, R3 `COM4`, simultaneous
> capture of one aligned round plus breathing windows; duty staircase
> exact at 1 kHz, flat-line extremes confirmed, heartbeat continuous
> (ticks 456–516 with no gap) through gate/staircase/breathing. Uploads
> succeeded before and after. WiFi map verified; Minima image builds
> (`gpt0b`/P106 path) but is hardware-untested — no Minima on the bench.
> Size (WiFi): `slice07_breathe` text=12700, data=0, bss=4.
>
> **Fail — R3 windows show `period=1333us` and stale-looking samples:**
> the R3 was reset mid-round (its first window after a serial open
> inherits the R4's previous phase). The next aligned round is the
> evidence — check `W3` onward.
>
> **Fail — no `H`/`L` and duty stuck near one value:** the D6→D2 wire
> is off, or the R4 is not past its banner (check the R4 log for
> `gate open`).

> ### Slice 8 — ADC, the chip senses (`slice08_adc`)
>
> *Historical record, retired 2026-10-20:* repeat it in the `6649d4c` worktree (note at the top of Testing on hardware). Current check: `arduino/validation/`, S8.
>
> Proves the ADC140 driver: single-shot, software-triggered, blocking
> reads on **A0** (raw + millivolts at 4 Hz) through the slice-4 SCI9
> monitor path, while the D13 heartbeat keeps ticking. The measurement
> is instrumented: R3 **D5** drives a PWM duty staircase straight into
> R4 **A0** (R3 sketch `slice08-source`), and per gate round
> (R3 D3 → R4 D3, the slice 6/7 pattern) the firmware averages five
> 1.5 s windows — 631 650 samples each — one window per duty step. The
> mean of a PWM wave over whole periods is duty × VREF, so the window
> means trace the staircase with no RC filter.
>
> **Hardware facts** (full chain in `Research/slice-08-research.md`):
> A0–A5 are the same chip pins on both boards — A0 = P014 → **AN009**
> (also DA0), A1 = P000 → AN000, A2 = P001 → AN001, A3 = P002 → AN002,
> A4 = P101 → AN021, A5 = P100 → AN022 (manual §19 Tables 19.5/19.6
> `ASEL` rows, cross-checked entry-for-entry against the Arduino
> core's `pinmux.inc`; the golden test in `src/tests/adc.rs` locks the
> table). `MSTPCRD.MSTPD16` releases the ADC140 (manual §10, p.175)
> with the §35.8.5 ≥ 1 µs settle; the software trigger needs no
> trigger setup (`ADCSR.ADST` works regardless of `TRGE`/`EXTRG`/
> `TRSA`, §35.2.12); `ADST` **self-clears** at scan end in single scan
> mode (§35.3.2.1 step 4) — the driver polls that self-clear, bounded;
> the reference is the reset selection AVCC0/AVSS0 (§35.2.31) = the
> board's 5 V analog rail; sampling states stay at the reset 0Dh,
> valid at this crate's 1:2 PCLKB:PCLKC ratio (≥ 6 states, §35.2.14).
> Conversion ≈ 1206 ns (12-bit high-speed, `calc_scan_time`, host
> tested). **There is no ADC trait in `embedded-hal` 1.0** — the
> driver exposes its own `read()` API (HalSpec §5.7/§6 amended
> 2026-10-19), the same stance the UART module takes outside
> `embedded-io`.
>
> **The full analog surface (HalSpec §5.7):** all six header pins are
> addressable through the sealed `Channel` trait (one `const AN` per
> pin type; only analog-state pins type-check). Hardware proof: A0
> end-to-end (accuracy instrumented), and A1–A5 converted through the
> same driver during the bring-up diagnostic (~13 M conversions each,
> zero errors — their *values* float because nothing drives them;
> only A0's accuracy is measured).
>
> **Build and convert** (from `crates/test-firmware/`):
>
> ```powershell
> cargo build --release --bin slice08_adc
> cargo objcopy --release --bin slice08_adc -- -O binary slice08_adc.bin
> ```
>
> **Flash** (R4 first, then the R3 rig; never-cold rule applies;
> `$PORT` = the R4, `$uartPort` = the R3):
>
> ```powershell
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice08_adc.bin
> arduino-cli compile --fqbn arduino:avr:uno ..\..\arduino\sketches\slice08-source
> arduino-cli upload -p $uartPort --fqbn arduino:avr:uno ..\..\arduino\sketches\slice08-source
> ```
>
> **Evaluate:** open both ports at 115200. Between rounds the R4
> prints `a0 raw=… mv=…` at 4 Hz — with the sketch driving duty 0
> between rounds this sits at ~0 mV; jumper A0 → 5 V / 3V3 / GND for
> the rails check. Per round the R3 prints its commanded duties and
> the R4 prints the five window lines. One round (2026-10-20):
>
> ```text
> R3: duty 0 / duty 64 / duty 127 / duty 191 / duty 255
> R4: W0 n=631650 mean=7     min=0    max=53   err=0
>     W1 n=631650 mean=1045   min=0    max=4095 err=0
>     W2 n=631650 mean=2051   min=0    max=4095 err=0
>     W3 n=631650 mean=3072   min=0    max=4095 err=0
>     W4 n=631650 mean=4095   min=4095 max=4095 err=0
> ```
>
> Reading: the staircase 0 → 1045 → 2051 → 3072 → 4095 against the
> expected duty/255 × 4095 = 0 → 1028 → 2040 → 3068 → 4095; worst
> error +17 LSB ≈ 21 mV, inside the datasheet's ±32 LSB (±39 mV) max
> for A0's precision class (and W1–W3 are inside the ±20 LSB
> high-precision line); window means repeat within ±2 LSB across
> rounds; W0/W4 are DC lines (rail-to-rail ends solid), W1–W3's
> min/max 0/4095 is the instantaneous PWM ripple the averaging is
> for. Heartbeat continuous (`tick N` every second through gate,
> staircase, and windows).
>
> **Verified on 2026-10-20:** R4 `COM3`, R3 `COM4`, three aligned
> rounds captured simultaneously; staircase within tolerance at both
> rails (0 and 4095 exactly — the datasheet-tolerance done-when),
> ~10 M conversions with `err=0`, uploads succeeded before and after.
> WiFi map verified; Minima image builds (A0–A5 are the same chip pins
> on both maps) but is hardware-untested — no Minima on the bench.
> Size (WiFi): `slice08_adc` text=15108, data=0, bss=4.
>
> **Fail — every analog reading wanders ~800–3400 raw and no `W`
> window is ever at a rail, while the R3 log shows the staircase:**
> the D5→A0 path is open. A solid duty-255 (5 V DC) can only read full
> scale on a connected pin, so this signature means the wire is not on
> the driven pin. Triage history: the R3 end was parked in **D4** —
> not a PWM pin on the ATmega328p (PWM: D3/D5/D6/D9/D10/D11) — moving
> it to D5 fixed everything in one round. `slice08_probe` (20-pin
> digital scan) finds where a signal actually lands if this recurs.
>
> **Fail — `a0 err=timeout`:** the ADC140 never completed a scan; dump
> `ADCSR` before theorizing (the poll bound is thousands of times the
> computed conversion time — reaching it is a fault, not a short
> wait).

### Slice 9 — native USB CDC (verified on WiFi)

> [!NOTE]
> **Historical record.** None of the Slice 9 programs and scripts named
> below were ever committed:
>
> - `slice09_cdc.rs` and `test-slice09-cdc.ps1`;
> - `slice09_rig.rs` and `test-slice09.ps1`;
> - `noop.rs` and the other `slice09_*` probes.
>
> All of them were deleted on 2026-10-20, so the commands in this
> subsection cannot be repeated. The results stand as recorded. The current
> Slice 9 check is `arduino/validation/` (S9, knock, `!`/`@`); its R4 image arms
> the same knock and serves the same echo and ticks.

**Slice binary:** `crates/test-firmware/src/bin/slice09_cdc.rs`. It prints a
banner on the SCI9 monitor, leaves the connector with the bridge for a 3 s
upload window, takes it, then enumerates as a native CDC port
(VID 2341 / PID 006D, product `UNO R4 slice09`). It echoes what it receives
and prints `tick N` about once per second. The opt-in bootloader knock is
armed.

Build from `crates/test-firmware/`, then upload through the bridge:

```powershell
cargo objcopy --release --features usb --bin slice09_cdc -- -O binary slice09_cdc.bin
arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice09_cdc.bin
```

Evaluate from `crates/test-firmware/`. The script finds the native port by
USB identity, not by a remembered COM number:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09-cdc.ps1 -Knock
```

It checks the following:

- **Ticks:** every line is `tick N`, the numbers are consecutive, and no gap
  exceeds 1.5 s.
- **Echo:** printable payloads of 1–4096 bytes come back exactly, with tick
  lines stripped first.
- **Knock (`-Knock`):** a 1200-baud open of the native port makes the
  native port disappear and the bridge (PID 1002) return.

**Uploading while the slice binary holds the connector** takes two steps.
Either open the native port at 1200 baud (the knock) or double-tap RESET.
Then run a standard `arduino-cli upload` on the bridge port that returns.

A single command aimed at the native port fails with
`No device found on COMx`, including with
`--upload-property upload.wait_for_upload_port=true`. The bootloader holds,
but the bridge forwards `bossac` only after its own 1200-baud touch.
Uploads to an R4 already held in bootloader also failed whenever that touch
was disabled. This was observed with both a knock hold and a physical
double-tap hold.

**Diagnostic rig, retired 2026-10-20.** None of its three parts was ever
committed, and all were deleted, so the rig cannot be rebuilt. The results
below were recorded with it.

The rig's parts were:

- R3: `slice09-rig.ino` (removed).
- R4: `slice09_rig.rs`, features `usb,uart-irq` (removed).
- Windows host: `test-slice09.ps1` (removed).

The R3 is the stable host endpoint while the R4 changes USB identity. The
pair negotiates its own diagnostic contract (`H` + raw version 9), then allows
one outstanding request. Replies carry a sequence, length, hex payload and
checksum. Hex encoding keeps register values from becoming commands.

Both R3 serial links run at **9600 8N1**. The R4 releases SCI2 TX (P302) to
input between replies.

**Shared-RX limitation:** the PC's bytes and the R4's TX share the R3's only
RX input. While any R4 program keeps SCI2 TX driven, PC-written R3 commands,
`!`/`@` included, do not reach the R3. This was reproduced with the R3
sketch `utility-req-res-reset` (since removed, never committed) +
`slice08_req_res_reset`: rows kept flowing and
`!!!`/`@@@` had no effect. With the R4 held in bootloader, `!!!` worked.
So the R3 reset path only works when the R4 releases or idles that pin.

R3 commands, sent without line endings:

| Command | Effect |
| --- | --- |
| `h` | renegotiate |
| `?` | snapshot |
| `u` | take native USB |
| `r` | detach and release |
| `t` | trace record |
| `v` | toggle native ticks |
| `w` | enable the USB interrupt bell |
| `b` | arm the bootloader knock |
| `x` | read installed bootloader bytes (read-only) |
| `y` | diagnostic: write the boot magic and read it back |
| `z` | diagnostic: bare R4 software reset |
| `!` | single RESET pulse |
| `@` | double RESET pulse |

The 94-byte snapshot also reports the reset cause captured at boot
(RSTSR0/2/1, manual §5.2) and the tick flag. The host script uses the tick
flag to switch ticks on or off rather than blindly toggling them.

Build/stage sequence, as it was run (none of it can be repeated now):

1. Upload `noop.bin` to the R4 first, so its UART TX cannot disturb the AVR
   bootloader.
2. Compile and upload the R3 sketch `slice09-rig` with `--verify`.
3. Upload `slice09_rig.bin` to the R4.

```powershell
cargo objcopy --release --features usb,uart-irq --bin slice09_rig -- -O binary slice09_rig.bin
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Echo -R3Port $uartPort
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Soak -R3Port $uartPort
powershell -NoProfile -ExecutionPolicy Bypass -File .\test-slice09.ps1 -Mode Echo -R3Port $uartPort -EnableWake
```

`Soak` is a functional tick run with a 30 s default. **Owner amendment,
2026-10-20:** the hour-long soak requirement is cancelled and no
replacement duration is required. Logs and results go to
`target/slice09-hardware/`.

**Witnessed on WiFi, 2026-10-20:**

`slice09_cdc`:

- Native COM port present while the bridge port was absent.
- 9 consecutive ticks (maximum gap 1.03 s) and 48 exact echoes
  (21,744 bytes each direction).
- Knock: native gone and bridge back after 1.19 s. A standard upload on the
  bridge then succeeded, and the native port returned with ticks and echo
  passing again.

`slice09_rig`:

- Polled echo: 210 exact cases, 73,450 bytes each direction.
- Interrupt bell: 210 cases, 2,850 bell interrupts.
- 30 s tick run: 29 consecutive ticks, maximum gap 1.07 s.
- SCI2 `ov=0 er=0` throughout.
- Two-step upload: knock, R4 silent (held) 3 s later, then a standard
  bridge upload succeeded.

R3 D4 → R4 RESET:

- `!` restarts the R4.
- `@` returns the bridge with the bootloader held.

Build matrix:

- Release firmware builds for WiFi and Minima, with and without
  `usb,uart-irq`, have no warnings.
- 58 host tests pass on each map.

**Bootloader knock fix.** The knock first failed: the R4 came back running
the application, with RSTSR1.SWRF = 1, so no RES-pin reset followed. The
cause:

- The installed bootloader is byte-identical to the `dfu_wifi.hex` that
  ArduinoCore-renesas 1.6.0 ships.
- At offset 0x21EA it compares one 32-bit load of `VBTBKR0..3` with
  `0x07738135`.
- On this silicon that load returns `VBTBKR0` as the most significant byte.
  Bytes written `35 81 73 07` read back as `0x35817307`.

The HAL now writes the magic big-endian, still with 8-bit accesses
(§11.2.6). After that change, a bare software reset holds the bootloader.

The same disassembly answers part of OQ-USB-1. The `VBTBKR[1] = 40` marker
that `UsbPort::take` writes asks the bootloader to drive P408 high, delay,
drive it low and clear the byte at the next boot.

Earlier on 2026-10-20, the R3 trace exposed two USB defects that were then
fixed:

- A control-OUT status acknowledgement must use CCPL even while CTSQ still
  reads data stage 3 (§27.2.25 / Figure 27.16).
- Class OUT pipes need arming after configuration.

**Physical replug, witnessed 2026-10-20 21:19:**

1. The owner unplugged and replugged the R4 with `slice09_cdc` installed.
2. The bridge appeared during the upload window, then native COM7.
3. Ticks (5 consecutive) and echo (16 exact cases) passed.
4. The knock returned the bridge after 1.2 s.
5. A standard `arduino-cli upload` on the bridge succeeded.
6. The native port came back and passed again.

**Still open:**

- UNO R4 Minima is hardware-unverified, including its knock.
- `cargo doc` with `-D warnings` still fails on two pre-existing links
  outside USB (`adc/adc.rs` `ADST_POLL_LIMIT`, `pwm/mod.rs`
  `Pwm::set_duty_cycle`).
- `Research/slice-09-research.md` keeps its original §10.1, §10.6 and §10.7
  statements, each followed by a dated correction.

### All-in-one validation — Slices 1–9 (`arduino/validation/`)

The validation pair re-proves Slices 1–9 on the existing bench without
swapping firmware between slices. It has three parts:

- `arduino/validation/r3-validator/r3-validator.ino`: the R3 controller.
- `crates/test-firmware/src/bin/r4_validator/main.rs`: one R4 image built on
  this HAL, with features `usb,uart-irq`.
- the `validate` CLI (`crates/validation`): the host side. It also checks
  the two slices only a PC can see: Slice 4 on the bridge monitor and
  Slice 9 on the native USB port.

The R3 and R4 exchange framed requests (`STX` body `ETX`) at 9600 8N1 on
SCI2. R4 replies contain only `A–Z`, `0–9` and spaces, while PC commands to
the R3 are lowercase, `?`, `!` or `@`. That keeps the shared R3 RX
unambiguous, and the R4 releases SCI2 TX between replies, so `!`/`@` always
reach the R3. Both source files document the full contract.

Each check:

| Slice | How it is checked | Pass condition |
| --- | --- | --- |
| 1 | The R4 drives D3 with the HAL `Delay` (20 ms phases); the R3 times the edges on INT1 | 22 edges, every phase 20 000 µs ± 2 % |
| 2 | The R3 drives D3 low, then high; the R4 reads it | reads 0, then 1 |
| 2 (button) | The R3 presses R4 D3 like a switch to GND (LOW for 200 ms, then released to input); the R4 times it | press ~100 ms after the request, held ~200 ms |
| 3 | 10 identify round trips over SCI2 | 10/10 |
| 4 | The R4 prints a token on SCI9; the host reads the bridge port | token seen |
| 5 | 4096-byte burst through the interrupt-driven SCI2 echo | byte-exact, `OV 0 ER 0` |
| 5 (traits) | The same burst, echoed only through the HAL's `embedded-io` 0.7 `Read`/`Write` on `&UartIrq` (blocking `read`, `write_all`, `flush`) | byte-exact, `OV 0 ER 0` |
| 6 | `AT` to the ESP32-S3 over SCI1 | `OK` (received bytes shown in hex) |
| 7 | D6 PWM at 0/25/50/75/100 %, measured by the R3 on INT0 | period 1000 ± 10 µs, duty ± 10/1000, 0/100 % flat |
| 8 | R3 D5 duty 0/64/127/191/255 into A0; the R4 averages 200 ms windows | mean within ± 82 LSB (2 % of 12-bit full scale) |
| 9 | Native CDC after R3 `u`, while the bridge is gone | ticks consecutive, max gap ≤ 1.5 s, exact echoes, state Configured |
| knock | 1200-baud open of the native port | bridge back, R4 silent (bootloader held) |
| `!` / `@` | R3 RESET pulses | `!` restarts the validator; `@` holds the bootloader |

Build from `crates/test-firmware/`. Like every binary there, the image uses
the crate's `memory.x` (FLASH ORIGIN 0x4000):

```powershell
cargo objcopy --release --features usb,uart-irq --bin r4_validator -- -O binary r4_validator.bin
```

Flash, from the repo root. Upload the R3 while the R4's SCI2 TX is quiet:
the R4 in bootloader, or already running `r4_validator`.

```powershell
arduino-cli compile --fqbn arduino:avr:uno --build-path target/validation-r3-build arduino/validation/r3-validator
arduino-cli upload -p $uartPort --fqbn arduino:avr:uno --input-file target/validation-r3-build/r3-validator.ino.hex --verify
arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file crates/test-firmware/r4_validator.bin
```

Run, from the repo root. The CLI finds the R3 (2341:0043), the bridge
(2341:1002) and the native port (2341:006D) by USB identity:

```powershell
cargo run --release -p validation -- all
```

R4 D2 is free/open: the push button is no longer part of the bench
(AGENTS.md). The R3 acts as the Slice 2 button on the existing D3 line.

`validate all --skip-usb` leaves out Slice 9 and the knock. Per-slice runs:
`cargo run --release -p validation -- slices S2` (also `s4`, `usb`,
`reset --single|--double`, `identify`). Without the CLI, open the R3 at
9600 and send `a` for S1–S8, or one letter per check (`?` lists them).
Results go to `target/validation/`.

**Witnessed on WiFi, 2026-10-20:** 14/14 checks passed in one host run, on
the HAL ported to `embedded-io` 0.7.1:

- **S1:** phases 20 008–20 012 µs.
- **S2 button:** press seen 100 ms after the request, held 199 ms.
- **S5:** 4096/4096 bytes, `OV 0 ER 0`.
- **S5 traits:** 4096/4096 bytes through `embedded_io::Read`/`Write`,
  `OV 0 ER 0`.
  - The blocking `read` path runs at least once per run: the R4 calls
    `read` before the R3 starts sending.
  - How often it waited after that is not instrumented.
- **Host tests (same day):** 60 pass on each map with `usb,uart-irq`.
  - Two of them check the 0.7 write contract on the CDC port: trait
    `write` returns a non-zero count and trait `write_all` completes.
  - The CDC `Ok(0)` guard itself cannot be reached through
    `usbd-serial` 0.2.2, which reports a full buffer as `WouldBlock`.
- **S6:** `OK\r\n`.
- **S7:** duty 249/499/749 per mille at 998–1000 µs.
- **S8:** means 10/1047/2055/3070/4095.
- **S9:** 5 consecutive ticks and 6152 echoed bytes each direction, with the
  bridge absent.
- **Knock, `!` and `@`:** passed.

Minima builds, but is hardware-unverified.

Two earlier runs showed an S6 reply of 7 bytes with no `OK` (the first run,
before the hex dump existed), and an SCI2 `er=235` total. That total came
after an R3 upload, whose 115200-baud traffic reaches the R4's 9600 RX; the
burst check reports per-check deltas instead.

## License

MIT — see [LICENSE.md](LICENSE.md).
