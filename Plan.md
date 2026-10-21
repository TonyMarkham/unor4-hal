# Plan.md — build order, each step proven on the board

Build order for this repo: one slice at a time, each ending in something
**observable on real hardware**. No slice is "done" because it compiles —
it's done when the board shows it. Structure follows HalSpec.md's
milestones; course-repo roadmap items are noted where a slice unblocks them.

## Workspace layout (established in Slice 0)

```
unor4-hal/
├── Cargo.toml              # workspace root: members, profiles, workspace deps
├── crates/
│   ├── unor4-hal/          # the library (this repo's product)
│   └── test-firmware/      # r4_validator, the R4 side of the validation pair — built and flashed locally
├── arduino/validation/      # r3-validator sketch (host side: the `validate` CLI, crates/validation)
└── AGENTS.md, HalSpec.md, Limitations.md, Plan.md
```

- `test-firmware` owns everything board-specific: `memory.x` (FLASH ORIGIN
  `0x00004000` — bootloader safety, see AGENTS.md), `.cargo/config.toml`
  (target pin), `build.rs`. The library crate stays target-agnostic and
  board-agnostic except for the pin tables.
- Every slice adds its checks to the **validation pair**:
  - `test-firmware/src/bin/r4_validator/main.rs`;
  - `arduino/validation/r3-validator`;
  - the `validate` CLI (`crates/validation`) for the PC-only checks.

  The pair is the regression suite (rules 2–3 below). Until 2026-10-20
  each slice added its own binary. The "Binary:" lines in the slice
  entries below name those retired programs: Slices 1–8 restore from git
  `6649d4c`, while the Slice 9 programs were never committed.
- The library never flashes; only `test-firmware` produces images. Upload
  procedure: course repo, BlinkEmbedded Step 5 (never upload cold).

## The slices

| # | Slice | HalSpec milestone | Observable |
|---|---|---|---|
| 0 | Scaffold | — | CI green on an empty workspace |
| 1 | Clock + GPIO + delay | M0 | LED blinks |
| 2 | Input GPIO, both pin maps | M0 | Button lights LED |
| 3 | UART SCI2 polling | M1 | `tick` lines on the bridge monitor |
| 4 | UART SCI9 | M1.5 | `tick` lines in the USB monitor, no adapter |
| 5 | Interrupts + IRQ UART | M2 | Echo while LED keeps steady rhythm |
| 6 | UART SCI1 (AT channel) | M2 | `AT` → `OK` from the ESP32-S3 |
| 7 | PWM | M3 | Breathing LED on D6 |
| 8 | ADC | M3 | A0 voltage tracking, printed live |
| 9 | USB CDC | M4 | New COM port, echo, no adapter, no bridge |
| 10 | Data flash | M5 | Counter survives a power cycle |

### Slice 0 — Scaffold

Workspace manifests, `crates/unor4-hal` (empty lib) + `crates/test-firmware`
(empty bin), `rust-toolchain.toml` (MSRV pinned), both LICENSE files,
GitHub Actions running the HalSpec §7 matrix on placeholder content.

**Done when:** CI is green on both boards' feature combinations and the
license files predate any code commit.

### Slice 1 — Clock + GPIO + delay → the board lives

`clock.rs` (HOCO 48 MHz, `Clocks`), `gpio.rs` (typestate pins, WiFi map
first), `delay.rs` (SysTick `DelayNs`). Binary: `slice01_blink.rs` — D13
(P102) at 1 Hz.

**Observable:** the onboard LED blinks once per second.
**Done when:** size is printed and recorded in the slice's doc comment
(the budget line for CI from then on).

### Slice 2 — Input + both pin maps → the board listens

`gpio.rs` completed: `into_input_pullup()`, `read()`, the Minima map
compile-locked behind its feature, pin-table cross-check test (HalSpec §7.2).
Binary: `slice02_button.rs` — D2 button (pull-up) mirrors to D13.

**Observable:** pressing the button lights the LED; releasing extinguishes.
**Done when:** the golden-table test proves our map equals the transcribed
core tables, and the binary works with both feature builds.

### Slice 3 — UART SCI2 → first words

`uart.rs`: the SCI driver, polling mode, 8N1, baud calculator with 3% check
+ host unit tests. Binary: `slice03_sci2.rs` — boot banner + `tick N` at
1 Hz on D0/D1.

**Observable:** `tick N` lines stream in the bridge monitor — an R3
running the relay sketch `slice03-adapter` (removed 2026-10-20;
restorable from git `6649d4c`), wired
D0<->D1 crossed to the R4 (README slice 3).
**Done when:** baud unit tests pass on the host and the board prints for
10 minutes without garbage or a hang.

### Slice 4 — UART SCI9 → the monitor, no adapter

Same driver, instantiated for SCI9 (P109/P110), with its PSEL and module-stop
bits from the hardware manual. Binary: `slice04_sci9.rs`.

**Observable:** `tick N` lines appear in `arduino-cli monitor` on the
board's own USB port — with no adapter attached.
**Done when:** OQ-6 is answered with hardware evidence (the bridge really
forwards SCI9), recorded in HalSpec.md. Upload must still work afterwards —
this slice shares the wire with the bootloader's path.

### Slice 5 — Interrupts → the chip reacts while it works

`intc.rs`: IELSR event table + linking + dispatch (OQ-2 resolved from the
manual), `critical-section` provider, IRQ-mode UART with ring buffers
(feature `uart-irq`). Binary: `slice05_echo.rs` — uppercase echo at 115200
while D13 keeps a 1 Hz heartbeat.

**Observable:** typed characters echo back instantly, and the heartbeat LED
shows no stutter while the echo runs — proof the main loop isn't polling.
**Done when:** a continuous 4 KiB UART burst echoes byte-for-byte without
errors/overflows while the heartbeat continues. On the WiFi board, use
`slice05_rig` with the R3 controller for the continuous SCI2 burst and a
separate paced SCI9 USB test: the ESP32 bridge drops unpaced host USB
bursts before they reach the HAL, so a monitor paste is not a valid
ring-buffer test. The reproducible command and hardware results are
recorded in README's Slice 5 section (WiFi verified 2026-10-19).

### Slice 6 — UART SCI1 → the door to WiFi

SCI1 instantiation (P501/P502) + IRQ mode. Binary: `slice06_at.rs` — opens
the AT channel to the ESP32-S3, sends `AT`, prints everything received for
30 seconds.

**Observable:** the ESP32-S3 bridge answers `OK` (and its banner) —
displayed via the slice 4 monitor path.
**Done when:** OQ from the spec "can the RA4M1 drive the AT channel" is
answered with hardware evidence. This unblocks the course's WiFi roadmap
item — the mission's hard dependency.
— Done: verified on WiFi 2026-10-19. 9/9 witnessed `AT` rounds answered
exactly `OK\x0d\x0a` (one complete 6-round run, one partial 3-round
capture; the partial run's own `total=24` report covers its unwitnessed
rounds), `ov=0 er=0`; the run is re-triggerable via
the bench gate line (the R3 sketch `slice06-gate` cycles R3 D3 → R4
D3, the firmware re-arms on the rising edge; sketch removed 2026-10-20,
restorable from git `6649d4c`). Reproduction commands,
transcript, and failure triage: README slice 6. Verification chain:
`Research/slice-06-research.md`. Recorded as HalSpec OQ-7.

### Slice 7 — PWM → the LED breathes

`pwm.rs`: GPT timer driver, `embedded-hal` 1.0 `SetDutyCycle` (the 0.2
`Pwm` trait does not exist in 1.0 — HalSpec §6 amended), all six header
PWM pins (D3, D5, D6, D9, D10, D11 — surface-locked by the golden
constructor test), D6 hardware-proven first. Binary:
`slice07_breathe.rs` — sine-table breathing on D6. Measured, not
eyeballed: D6 → R3 D2 (INT0), the R3 sketch `slice07-measure` (removed
2026-10-20; restorable from git `6649d4c`) measures duty/period
edge-by-edge (owner decision 2026-10-19: no LED).

**Observable:** the R3 reports the duty staircase (0/25/50/75/100%,
period exactly 1000 µs) and the sine breathing profile from the wire.
**Done when:** duty resolution measured stable at 1 kHz+ with no flicker,
and heartbeat + breathing run simultaneously (two GPT channels or GPT + SysTick).
— Done: verified on WiFi 2026-10-19. Aligned round: every commanded duty
within ±1 tick of 48 000 (±4 per-mille), period exactly 1000 µs, 0%/100%
measured as DC lines (flat low/high, no edges), heartbeat ticks
continuous through gate/staircase/breathing. All six pins addressable;
hardware proof on WiFi D6, other pin/engine pairs compile-checked (no
bench wires — noted in README slice 7). Reproduction commands,
transcript, and triage: README slice 7. Verification chain:
`Research/slice-07-research.md`. Recorded as HalSpec OQ-8.

### Slice 8 — ADC → the chip senses

`adc.rs`: ADC140 single-shot reads, A0–A5. Binary: `slice08_adc.rs` —
prints A0 as raw + millivolts at 4 Hz.

**Observable:** jumper A0 to GND → ~0 mV; to 5 V → ~5000 mV; values move
live as a pot turns (if available).
**Done when:** readings within datasheet tolerance at both rails, printed
through the slice 4 monitor path.
— Done: verified on WiFi 2026-10-20, measured by the R3 rig
(R3 sketch `slice08-source`, removed 2026-10-20 and restorable from git
`6649d4c`; R3 D5 → R4 A0, one additive bench
wire): five 1.5 s averaged windows (631 650 samples each) over the R3's
PWM duty staircase read 7/1045/2051/3072/4095 against expected
0/1028/2040/3068/4095 — worst error +17 LSB ≈ 21 mV, inside the
datasheet's ±32 LSB budget for the channel class; both rails land
exactly (0–7 LSB / 4095 solid DC); window means repeat within ±2 LSB;
~10 M conversions across all six channels with zero errors. All six
pins addressable (sealed `Channel` trait); hardware proof on WiFi A0
(accuracy instrumented), A1–A5 converted on hardware but value-untested
(nothing drives them — noted in README slice 8). `embedded-hal` 1.0 has
no `adc` trait — the driver's own `read()` API (HalSpec §5.7/§6
amended). Reproduction commands, transcript, and the bring-up triage
(open-wire signature → wire parked in R3 D4): README slice 8.
Verification chain: `Research/slice-08-research.md`. Recorded as HalSpec
OQ-9.

### Slice 9 — USB CDC → the board becomes its own monitor

`usb.rs`: `usb-device` adapter for the RA4M1 USBFS (OQ-3 answered first:
search for an existing RA adapter; if none, this is the big build — HalSpec
§5.5), CDC-ACM via `usbd-serial`, explicit `take_usb_port()` mux handling.
Binary: `slice09_cdc.rs` — enumerates as a CDC device, echoes, prints ticks.

**Observable:** a **new** COM port appears without any adapter; opening it
shows ticks; the bridge's port is gone while held (documented behavior).
**Done when:** `lsusb`-equivalent (Device Manager) shows the descriptor,
native CDC echo and ticks work, and *upload still works* after replugging
(bridge recovery). **Owner amendment, 2026-10-20:** the hour-long soak is
cancelled; no replacement soak duration is required. Largest slice —
HalSpec's OQ-3 gates starting it.

**Status 2026-10-20 (WiFi):** the following were witnessed:

- the descriptor and native COM port appearing;
- echo and ticks working with the bridge port gone;
- recovery without hands: the opt-in 1200-baud knock (bootloader holds),
  followed by a standard upload on the bridge port.

An upload after a **physical replug** was also witnessed (21:19). Every
done-when item is therefore met on WiFi. Minima remains
hardware-unverified. Details and commands: README, Slice 9.

### Slice 10 — Data flash → the board remembers

`dflash.rs`: 8 KB data flash read/write/erase with wear-minimal API.
Binary: `slice10_remember.rs` — increments a persisted counter each boot.

**Observable:** the printed boot count increments across power cycles —
unplug, replug, the number remembers.
**Done when:** 100 power cycles with zero corruption, and an erase+rewrite
cycle verified byte-exact.

## Rules that hold across all slices

1. **One observable per slice.** If a slice needs two to be convincing, it's
   two slices.
2. **Nothing ships from the library without the validation pair proving
   it.** A `unor4-hal` module lands in the same commit as the matching
   checks in the validation pair:
   - a request in `crates/test-firmware/src/bin/r4_validator/main.rs`;
   - a check in `arduino/validation/r3-validator/r3-validator.ino`;
   - a check in the `validate` CLI when only a PC can observe it.
3. **The validation pair is the regression suite.** It keeps every
   finished slice's check, so one run of `validate all` re-proves all of
   them. A check is removed only together with the feature it proves.
   The built `.bin` image is a local build artifact (`*.bin` is
   gitignored); regenerate it from source any time.

   *Amended 2026-10-20 (owner):* the per-slice programs were retired in
   favour of the pair. The Slice 1–8 programs restore with
   `git restore --source 6649d4c -- crates/test-firmware/src/bin/<name>.rs`
   (plus `src/heartbeat.rs` for the four that use it). The Slice 9
   programs, `noop` and `slice08_req_res_reset` were never committed and no
   longer exist.
4. **Hardware facts get cited** (manual section) or **get marked unverified
   until a slice proves them** (OQ-1…OQ-6 all close this way).
5. **The course repo stays the consumer.** At slice 3 the course's Serial.md
   can be re-verified against this crate; at slice 6 the WiFi lesson
   unblocks. Switching the course's dependency happens when the owner
   chooses, not as part of a slice.
