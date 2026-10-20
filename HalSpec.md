# HalSpec — specification for an independent UNO R4 HAL

> *References to "roadmap item N" and "course" point at the owner's course
> repo (`arduino-embedded-rust`); its roadmap numbers the exercises that
> this crate unblocks.*

Requirements for a from-scratch HAL crate for the Arduino UNO R4 (RA4M1),
written to close the gaps documented in [Limitations.md](Limitations.md).
This is a specification, not an implementation — the owner writes all code;
this document defines *what* to build and *how to verify it*.

## 1. Why and on what terms

**Why:** existing community support for this board covers only SCI2 — no
USB, no interrupt infrastructure, no data flash. The project needs SCI1
(WiFi AT channel — mission-critical), SCI9 (monitor without an adapter), and
eventually USB CDC.

**Licensing stance:**

- **No forking.** No code copied from any existing implementation —
  Arduino's C++ core included (license status is not the issue — mixing
  origins is). This is an independent implementation.
- **Primary source of truth: the Renesas RA4M1 hardware manual** (register
  descriptions, the SCI baud-rate formula, the IELSR event table, USBFS
  registers) and the RA4M1 SVD. Hardware *facts* — register addresses, pin
  mappings, formulas — come from Renesas documentation, not from anyone's
  code.
- **Behavioral reference only:** Arduino's core and bridge firmware may be
  *observed* (what baud the monitor bridge uses, when the mux is flipped) and
  consulted to answer "how does the board behave," but not transcribed.
- **Dependency audit before the first release:** confirm the license of every
  dependency — notably the `ra4m1` PAC (svd2rust output; verify the SVD's
  terms) and `usb-device`/`usbd-serial` (MIT/Apache-2.0 — verify at
  implementation time, don't trust this line).

## 2. Goals and non-goals

**Goals (each maps to a course shortcoming or roadmap item):**

1. All three board-relevant UART engines: **SCI2** (D0/D1), **SCI9**
   (P109/P110 — monitor/upload path), **SCI1** (P501/P502 — WiFi AT channel).
2. **USB device support** (CDC-ACM) on the RA4M1's USBFS, so the monitor path
   works with no adapter.
3. **Interrupt infrastructure** (ICU/IELSR event system) — required for
   IRQ-driven UART and USB, and for roadmap items 2 and 5.
4. GPIO, PWM (GPT), ADC, delay — feature parity with the existing HAL so the
   course can run on this crate end to end.
5. **Data flash access** (the RA4M1's 8 KB EEPROM-equivalent) — needed to
   store WiFi credentials (credential-storage option C in the course repo's
   AGENTS.md).
6. Both board pin maps (Minima / WiFi) with a mechanism that makes the
   "docs show the other board" trap impossible.
7. Project hygiene that survives: license files, CI, tests, changelog.

**Non-goals (explicitly out of scope):**

- Async/embassy support — polling plus IRQ ring buffers is sufficient for
  this project's POC; design so it can be added later.
- WiFi/BLE stacks — they live on the ESP32-S3 or in the protocol/app layer.
- LED matrix driver — separate crate on top of this HAL (roadmap item 8);
  the HAL only needs to expose the matrix's pins as ordinary GPIO.
- Host-side tooling (`crates/host-tool`) — separate crate.

## 3. Crate identity and layout

- Working name: `unor4-hal` (confirm the name is free on crates.io before
  first publish).
- Edition 2024, `#![no_std]`, `#![no_main]`-agnostic (no runtime in the
  library; `cortex-m-rt` is the app's choice).
- Features: `minima` and `wifi` (pin maps). **`wifi` is the default feature**
  so that plain `cargo doc`/docs.rs shows the board this project uses.
  Building with both features enabled must be a **compile error**
  (`compile_error!`) — a board is a physical fact; "both at once" is always a
  mistake.
- Layout:

```
src/
  lib.rs        // crate docs, feature gates, Peripherals
  clock.rs      // clock tree init + Clocks handle
  gpio.rs       // typestate pins, both pin maps
  intc.rs       // ICU/IELSR event linking + IRQ dispatch
  uart.rs       // SCI driver (SCI1/2/9)
  usb.rs        // USBFS device (behind feature "usb")
  pwm.rs        // GPT channels
  adc.rs        // ADC140
  dflash.rs     // data flash (8 KB)
  delay.rs      // SysTick blocking delay
```

## 4. Core design rules

1. **Ownership:** `Peripherals::take() -> Option<Peripherals>` singleton.
   Each peripheral is claimed exactly once; drivers consume their hardware
   token and the pins they use. No `Send` on drivers unless proven safe.
2. **Errors, not panics:** the library returns `Result` everywhere and never
   panics, blocks indefinitely on its own, or allocates. Panic *strategy*
   (halt, print-to-UART) belongs to the application.
3. **Typestate pins:** keep the proven pattern — `Pin<PORT, PIN, Mode>` with
   `into_output()`, `into_input_pullup()`, `into_alternate(psel)`, etc.
   Register-level detail: `PmnPFS` writes must unlock `PWPR` and relock, in a
   critical section.
4. **Traits:** implement `embedded-hal` 1.0 (`digital`, `delay`, `spi`, `i2c`
   as applicable), `embedded-hal-nb`, and `embedded-io`/`embedded-io` 0.6 for
   UART and USB. `core::fmt::Write` on every byte sink.
5. **Register access only through the PAC** (`ra4m1` crate), except where the
   PAC is missing something (e.g. `PWPR` unlock sequence) — then raw
   `volatile` access with a comment citing the hardware manual section.
6. **MSRV:** pin and document it. CI proves it.

## 5. Module specifications

### 5.1 Clock (`clock.rs`)

- Bring-up identical in effect to the existing HAL's verified behavior:
  HOCO 48 MHz as system clock; return a `Clocks` struct exposing
  `sysclk()`, `pclka()`, `pclkb()` (Hz) for driver math.
- Expose module-stop control (MSTPCRB/MSTPCRD) as an internal API used by
  drivers (SCI, USBFS, GPT, ADC), replacing the ad-hoc per-driver
  enable/disable code.
- **Open question OQ-1:** which clock feeds USBFS (48 MHz requirement) and
  what the register sequence is — resolve from the hardware manual's USBFS
  and clock chapters before implementing `usb.rs`.

### 5.2 GPIO (`gpio.rs`)

- Feature-gated pin maps for both boards, generated **from a checked-in
  table** (`pins.csv`: board, d-pin, port, pin) that itself is derived from
  the UNO R4 WiFi/Minima pin assignment — with the Arduino core's
  `variant.cpp` used as *cross-check data* in CI (see §7), not as copied
  code.
- Correctness mechanism for the docs trap: every pin's doc comment shows the
  mapping for **both** boards, e.g. `d2: WiFi=P104, Minima=P105`.

### 5.3 Interrupts (`intc.rs`)

- Implement IELSR event linking: an `Event` enum (per the hardware manual's
  ICU chapter), `link(event, irq)` with allocation tracking, and
  `#[cortex_m_rt::interrupt]` trampolines dispatching to registered
  handlers. — Slice 5 implementation: the PAC's `rt` feature is unusable
  (it binds `cortex-m-rt ^0.6`, conflicting with the workspace's 0.7), so
  the HAL supplies its own 32-entry `__INTERRUPTS` table with built-in
  dispatch and firmware builds cortex-m-rt with the `device` feature. The
  NVIC provides enable/pending directly — the RA4M1 ICU has no IER/IPR
  registers of its own (cross-checked against FSP's `bsp_irq.h`).
  `Peripherals::take` disables inherited NVIC lines, clears their pending
  bits, and removes bootloader ICU event routes before allocating slots.
  Duplicate event links are rejected (manual R01UH0887EJ0100 §13.4.2).
  Main explicitly calls unsafe `intc::enable_interrupts()` outside critical
  sections after attaching drivers; slot enables preserve PRIMASK.
- Provide a `critical_section` implementation (the `critical-section`
  crate) — the PAC and any shared state depend on it. — Provided via
  cortex-m's `critical-section-single-core` provider (PRIMASK), enabled
  as a cargo feature on the existing dependency: audited upstream code
  instead of a hand-rolled `unsafe impl`.
- **Open question OQ-2:** exact IELSR/IRQ numbering table for the RA4M1 —
  transcribe from the hardware manual's ICU chapter into `intc.rs` as a
  const table with a unit test asserting uniqueness and bounds.

### 5.4 UART (`uart.rs`) — the centerpiece

- One generic SCI driver, instantiated for **SCI2, SCI9, SCI1**. Per-pin
  PSEL values and module-stop bits come from the hardware manual's pin
  function chapter; the pin types enforce engine/pin matching at compile
  time exactly as the existing HAL does for SCI2.
- Two modes per engine:
  - `polling` (default): blocking `write_byte`/`read_byte` — same shape the
    course already teaches.
  - `irq` (feature `uart-irq`): TX/RX ring buffers (const-generic size,
    `critical_section`-guarded) served by the SCI interrupts linked through
    `intc.rs`. This is what the WiFi AT channel needs — concurrent,
    non-dropping send/receive while the app loop runs.
- Format: 8N1 fixed at first; config struct (`data_bits`, `parity`,
  `stop_bits`) as a fast-follow.
- Baud math: implement the RA4M1 SCI asynchronous formula from the hardware
  manual (BRR with ABCS/BGDM semantics, smallest CKS that fits), reject
  requests whose error exceeds 3% with `InvalidBaudRate`. Unit-test the
  calculator on the host against known-good pairs (115200@48MHz etc.) — the
  pairs are arithmetic facts, re-derivable from the formula.
- API sketch (illustrative, not code to paste):

```rust
let mut serial = Serial::new(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks)?;
writeln!(serial, "tick {}", n)?;          // impl core::fmt::Write
serial.write_all(b"AT\r\n")?;             // impl embedded-io Write
let n = serial.read(&mut buf)?;           // impl embedded-io Read (blocking)
```

### 5.5 USB device (`usb.rs`, feature `usb`)

- Implement a `usb-device` USB bus adapter for the RA4M1 USBFS, and build
  CDC-ACM on top via `usbd-serial`. This gives: the monitor path with no
  adapter, and later a clean host↔board packet channel for `host-tool`.
- Must handle, per the hardware manual + board:
  - the **USB mux** (drive P408 high to route the plug to the RA4M1) behind
    an explicit `take_usb_port()` call — never implicit, because it
    disconnects the ESP32-S3 bridge (uploads!). Document the trade-off in
    the API docs and reference the SJ1 solder jumper.
  - VBUS/USB interrupt wiring through `intc.rs`.
- **Open question OQ-3:** does the existing `usb-device` ecosystem have an
  RA4M1/RA USBFS adapter already? (Search before writing one.) If none:
  this is the largest single work item in the spec; budget accordingly and
  consider contributing it upstream to `usb-device`'s ecosystem.
- Alternative documented in passing: linking tinyusb through the C ABI
  (proven on this silicon by the C++ core). Spec prefers the pure-Rust
  `usb-device` path for coherence, but this remains the fallback.

### 5.6 PWM (`pwm.rs`)

- GPT channel driver covering at least the header PWM pins (D3, D5, D6, D9,
  D10, D11) with `embedded-hal` 1.0's `pwm::SetDutyCycle` (the
  multi-channel `Pwm` trait is embedded-hal 0.2 legacy and does not exist
  in 1.0 — §6 amended; Slice 7 finding). `PwmD6`-equivalent API for the
  roadmap item 3 breathing LED: period + duty in ticks, duty 0..=period.
  — Slice 7: implemented for **all six header PWM pins** (D3, D5, D6,
  D9, D10, D11 — per-map constructors named by header pin, engine/pin/
  output fixed at compile time, surface-locked by the golden
  constructor test in `src/tests/pwm.rs`), with the single-buffered
  compare update and the `GTUDDTYC.OADTY` 0%/100% mechanism.
  Hardware-proven on WiFi D6 (GPT163) — the driver's register recipe is
  common code, so the other pin/engine pairs are compile-checked but
  not board-run (no bench wires to them; noted in README slice 7).

### 5.7 ADC (`adc.rs`)

- ADC140 driver for A0–A5, single-shot blocking read
  `read(pin) -> Result<u16, Error>` with configurable resolution (12-bit
  default). A0's DAC function is out of scope
  until needed.
  — Amended 2026-10-19 (slice 8 research): **`embedded-hal` 1.0 has no
  `adc` module** (docs.rs: 1.0.0 ships only `delay`, `digital`, `i2c`,
  `pwm`, `spi`; `embedded-hal-nb` 1.0 only `serial`, `spi`), so there is
  no ecosystem ADC trait to implement — the driver exposes its own
  infallible `read()` API, as the UART module does outside `embedded-io`.

### 5.8 Data flash (`dflash.rs`)

- The RA4M1's 8 KB data flash as byte-granular (or 1KB-block-erased) storage:
  `read(offset, buf)`, `write(offset, buf)` (program-only bits), `erase()`.
- Sufficient for storing one WiFi credential set. The AT-command
  provisioning flow (credential-storage option C in the course repo's
  AGENTS.md) lives in the app layer.

### 5.9 Delay (`delay.rs`)

- SysTick blocking `DelayNs` — carry the existing behavior; unit of record:
  milliseconds and microseconds.

## 6. Trait compliance matrix (CI-enforced)

| Module | embedded-hal 1.0 | embedded-hal-nb | embedded-io | core::fmt::Write |
|---|---|---|---|---|
| GPIO    | digital In/Out | — | — | — |
| UART    | — | serial Read/Write | Read/Write | ✔ |
| USB CDC | — | — | Read/Write | ✔ |
| PWM     | pwm SetDutyCycle | — | — | — |
| ADC     | — (no adc trait in e-h 1.0) | — | — | — |
| Delay   | DelayNs | — | — | — |

## 7. Testing and CI (what makes this not hobby-grade)

1. **Build matrix:** `{minima, wifi} × {usb on, usb off} × release/debug` —
   all green on every push (GitHub Actions, `rust-toolchain.toml` pinned).
2. **Golden pin-table test:** a checked-in JSON/CSV of the Arduino core's
   variant pin tables (facts, transcribed) + a unit test asserting our
   `pins.csv` matches it. This is the mechanism that would have caught the
   docs.rs trap automatically.
3. **Host unit tests** (run without hardware): baud calculator, ring buffer,
   IELSR table integrity, pin table cross-check.
4. **Docs build:** `cargo doc` with both features in CI; deny
   `missing_docs` on public items; every module has at least one compiled
   example (`examples/`) that type-checks against a fake target.
5. **Size budget:** CI prints `cargo size` on the blinky example and fails if
   it regresses past a threshold — keeps the "small firmware" property that
   motivated this whole course (1,584 bytes baseline).
6. **HIL (optional, later):** a test firmware + PowerShell host script
   exercising SCI2 loopback and CDC echo on the real board, run manually.

## 8. Project hygiene

- `LICENSE.md` (MIT) committed **on day one** (the absence of a license file
  is precisely what made the fork option unattractive).
- `CHANGELOG.md`, semantic versioning, git tags per release.
- All dependencies license-audited and listed in the README with their
  licenses.

## 9. Milestones mapped to the course

| Milestone | Contents | Unblocks |
|---|---|---|
| M0 | Scaffold, clock, GPIO (both maps), delay | Parts 2–6 run on the new crate |
| M1 | UART SCI2 (polling) | the course's serial lesson (adapter path) |
| M1.5 | UART SCI9 (polling) | Monitor with no adapter |
| M2 | intc.rs + IRQ UART ring buffers; SCI1 | WiFi AT channel — roadmap item 9 becomes implementable |
| M3 | PWM, ADC | Roadmap items 3–4 |
| M4 | USB CDC (usb-device adapter) | Monitor over USB; host-tool packet channel |
| M5 | Data flash | Credential storage (option C) |

M2 is the mission-critical one; M4 is the largest single item (see OQ-3).

## 10. Open questions (resolve before/at implementation)

- **OQ-1** USBFS clock source and init sequence (hardware manual).
- **OQ-2** Full IELSR event/IRQ table for the RA4M1 (hardware manual, ICU
  chapter). — Resolved in Slice 5: the table is transcribed into
  `src/intc/event.rs` (the `Event` enum), cross-checked entry-for-entry
  against two independent transcriptions of the manual's ICU chapter
  (the `ra-hal` crate's `InterruptEvent` and a community table). The
  SCI2/SCI9 RXI/TXI/ERI entries are exercised on hardware by slice 5's
  run; the 0xB7–0xDA range stays untranscribed (only 0xDB is attested)
  until a driver needs it.
- **OQ-3** Existing RA4M1 `usb-device` adapters — search; if none, decide
  build vs. tinyusb fallback.
- **OQ-4** SCI FIFO usage (some SCI modes have FIFOs; the polling path
  doesn't need them — confirm IRQ path requirements from the manual). —
  Slice 5 answer: the IRQ path uses the non-FIFO `RDR`/`TDR` registers
  only, same as polling; FIFO modes stay out of scope.
- **OQ-5** `ra4m1` PAC license and completeness (does it expose USBFS,
  IELSR/ICU, data flash registers?). — Slice 5 partial answer: ICU and
  IELSR are exposed, MIT OR Apache-2.0. But the PAC's `rt` feature is
  bound to `cortex-m-rt ^0.6` and cannot coexist with this workspace's
  0.7 runtime, so the HAL supplies its own 32-entry vector table and
  firmware builds cortex-m-rt with the `device` feature instead.
- **OQ-6** Whether SCI9 output actually appears in the monitor — verify on
  hardware early. — Verified on UNO R4 WiFi in Slice 5 (2026-10-19):
  sustained interrupt-driven output and all 4096 paced host bytes echoed
  correctly through the bridge. Repeated uploads over that same USB path
  succeeded. Unpaced USB bursts are not reliable in this setup; continuous
  4 KiB UART bursts are independently verified through the R3/SCI2 rig.
- **OQ-7** Can the RA4M1 drive the AT channel — the M2/M6 question the
  WiFi roadmap depends on. — Verified on UNO R4 WiFi in Slice 6
  (2026-10-19): SCI1 in IRQ mode (P501/P502, `PSEL[4:0] = 00101b` per
  manual §19 Table 19.13, module stop released via `MSTPCRB.MSTPB30`,
  manual §10.2.3) sent `AT\r\n` at 115200 8N1 to the ESP32-S3's AT
  server (stock bridge firmware, SudoMaker/chAT — behavioral reference)
  and received exactly `OK\r\n` on 9/9 witnessed rounds (one complete
  6-round run, one partial 3-round capture), with zero overruns and zero
  receive errors. Uploads over the shared USB
  path succeeded before and after. Evidence and reproduction: README
  slice 6; verification chain: `Research/slice-06-research.md`.
- **OQ-8** GPT PWM duty mechanics — polarity, the 0%/100% extremes, and
  the inherited GPT state after a bootloader handoff. — Verified on
  UNO R4 WiFi in Slice 7 (2026-10-19), measured edge-by-edge by the R3
  rig (`arduino/sketches/slice07-measure`): with
  `GTIOR.GTIOx[4:0] = 0b0_10_01` (high at cycle end, low at compare
  match) the high fraction is exactly the compare value over `GTPR + 1`
  (25/50/75% measured at 250/500/750 per-mille ±1 tick of 48 000);
  duty 0% is only reachable via `GTUDDTYC.OADTY` and 100% via a
  never-matching compare — a compare match coinciding with the cycle
  end loses to the cycle-end setting, exactly as Table 22.5 Note 2
  predicts; both extremes measured as DC lines. The driver rewrites
  every GPT register (counting `GTPR`'s buffer off via `GTBER.BD[1]`,
  since `GTPBR` resets to `0xFFFF` and would otherwise stretch the
  first period), so no bootloader-inherited state is trusted.
  Evidence and reproduction: README slice 7; verification chain:
  `Research/slice-07-research.md`.
- **OQ-9** The ADC140's reference rail and achievable accuracy on the
  UNO R4 — the board runs the analog domain at 5 V while the digital
  VCC is 3.3 V, which does not satisfy the datasheet accuracy tables'
  `VCC = AVCC0` condition. — Verified on UNO R4 WiFi in Slice 8
  (2026-10-20), measured by the R3 rig (`arduino/sketches/
  slice08-source`, R3 D5 → R4 A0): both rails land exactly on the code
  range (duty 0 → 0–7 LSB, duty 255 → 4095 with min=max — solid DC),
  and the PWM staircase 0/25/50/75/100% reads 7/1045/2051/3072/4095
  against expected 0/1028/2040/3068/4095 — worst error +17 LSB
  ≈ 21 mV, inside the datasheet's ±32 LSB max for the normal-precision
  channel (and within ±20 LSB for the three windows below the top
  step). Window means repeat within ±2 LSB across rounds; ~10 M
  conversions with zero errors across all six channels. Evidence and
  reproduction: README slice 8; verification chain:
  `Research/slice-08-research.md`.

## 11. Implementer's source list

- Renesas RA4M1 hardware manual (R01UH0887 — register chapters: SCI, USBFS,
  ICU/IELSR, GPT, ADC140, MSTP, clock tree) and the RA4M1 datasheet.
- RA4M1 SVD (source of the `ra4m1` PAC).
- ABX00087 product reference manual — board wiring facts (block diagram,
  mux, level translator).
- ArduinoCore-renesas — behavioral reference and pin-table cross-check data
  only; nothing transcribed.
- Limitations.md — the gap list this spec closes.
