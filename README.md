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
output, and SysTick delay (1,584-byte blinky firmware). Slice 2 (input
GPIO, both board pin maps) is implemented and host-tested; its hardware
run is pending.

Planned next: UART on all three board-relevant SCI channels (header pins,
USB-bridge monitor path, WiFi AT channel), interrupt support (ICU/IELSR),
PWM, ADC, data flash, and USB CDC-ACM.

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

Expected: `7 passed; 0 failed` — the count grows with each slice. These
run on the host: clock math, the delay table, and the golden pin-table
cross-check that proves the compiled board map matches the transcribed
hardware facts.

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
> bootloader can enumerate a different COM number than the sketch, and
> this assumes the UNO R4 is the only board connected).
> 
> Setting `$PORT` here will feed every upload below.
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
> > > ```
> >
>
> > ### Set the port and verify it:
> > ```powershell
> > $PORT = (arduino-cli board list --format json | ConvertFrom-Json).detected_ports[0].port.address
> > ```
> > ### Proof:
> > ```pwsh
> > $PORT
> > ```
> > > ### Sample Result:
> > > ```pwsh
> >>  COM3
> > > ```

> ### Slice 1 — blink (`slice01_blink`)
>
> **Build and convert** (from `crates/test-firmware/`):
>
> > [!IMPORTANT]
> >
> > Run every command in this section from `crates/test-firmware/`
>
> ```pwsh
> cargo build --release --bin slice01_blink
> ```
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
> ```powershell
> cargo build --release --bin slice02_button
> ```
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
> a WiFi board is header **D3**:
>
> ```pwsh
> cargo build --release --no-default-features --features minima --bin slice02_button
> ```
> ```pwsh
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> ```
> ```pwsh
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> Move the button from D2 to D3: it works again. That is the pin-map
> switch, observed on hardware. **Then restore the WiFi image** — rebuild
> and re-flash the default map:
>
> ```powershell
> cargo build --release --bin slice02_button
> cargo objcopy --release --bin slice02_button -- -O binary slice02_button.bin
> arduino-cli upload -p $PORT --fqbn arduino:renesas_uno:unor4wifi --input-file slice02_button.bin
> ```
>
> **Verified:** pending hardware run.

## License

MIT — see [LICENSE.md](LICENSE.md).
