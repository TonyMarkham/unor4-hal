# Step generator — design for owner review

Status: **draft, revised 2026-11-04 after two POCs and the HAL's header pin
model.** The pin model (§4.2) is built and proven (`validate all` 24/24);
the step generator's builder is not. Implementation starts only after the
owner approves this revision and grants the code work.

## 0. Why this document exists

The step generator in the 0.2.0 draft (`crates/unor4-hal/src/step/`) works
on the bench, but it is a one-off for the cart-pole rig:

- STEP only on WiFi D4 (GPT320) and DIR only on D5. The DIR register and
  bit are hard-coded.
- Its support timers are fixed (GPT321 count, GPT164 tick, GPT165 tick
  number). The DTC and the ELC inputs B and C are claimed.
- No Minima constructor.
- One fixed-pin constructor does all the setup.

On 2026-11-03 the owner and the agent agreed on a layered API. Owner:
"GOOD CALL, offer a low level, order-configurable micro-managed calls, or
a slim shim…", then "I think I like the low-level API, for cart-pole, rate
STEP Rate can be configured in setup, then loop is just pulse rate". That
agreement was never written into `Slice11Plan.md`, and the low-level layer
was not built. The owner found the result in the CHANGELOG. This document
replaces that design. Until it lands, 0.2.0 is not published with the
current `step` module.

## 1. Requirements

| # | Requirement | Source |
| --- | --- | --- |
| O1 | One step generator at a time (Q1 answered: the owner chose one when offered one, two or three) | owner, 2026-11-04 |
| O2 | STEP on any header pin the silicon supports, on a 16- or 32-bit timer. Any other choice fails `cargo check` with a message naming the pin (§4) | owner, 2026-11-04 |
| O3 | DIR on any GPIO pin | owner, 2026-11-04 |
| O4 | Two layers: low-level blocks configured once in setup, and a slim per-loop call (`queue(rate)`) | owner, 2026-11-03 |
| O5 | Slice 11's requirements R1–R15 (`Slice11Plan.md` §2) still hold, except where a 16-bit STEP timer cannot meet one; §5 lists each such case | Slice 11 |
| O6 | Everything here proven on the board before release, both timer widths, on the existing bench wiring | AGENTS.md |

## 2. Hardware facts

Sources: Renesas RA4M1 User's Manual R01UH0887 Rev 1.10 (`Research/`).

- **Timers.** Eight GPT channels: GPT320 and GPT321 are 32-bit; GPT162–
  GPT167 are 16-bit (§22.1: "two GPT32 channels … six GPT16 channels";
  §22.2: "For GPT16m … GTCNT is a 16-bit register"). Channel n drives
  `GTIOCnA`/`GTIOCnB`: 0→GPT320, 1→GPT321, 2→GPT162 … 7→GPT167.
- **Pin mux.** `PmnPFS.PSEL = 00011b` selects the GPT function on every
  pin below (Tables 19.6, 19.7, 19.10, 19.12).
- **Clock divider.** `GTCR.TPCS` picks PCLKD/1, /4, /16, /64, /256 or
  /1024, and "must be set while the GTCNT operation is stopped" (§22.2.12).
  A running timer cannot change divider.
- **DIR writes.** Each port's `PCNTR3` sets the pins in bits 15:0 and
  clears those in bits 31:16 in one 32-bit write (§19.2.3). Port m's
  `PCNTR3` is at `0x4004_0008 + 0x20 × m`, consistent with the driver's
  port spacing and the measured `PORT1_PCNTR3 = 0x4004_0028`. The DTC can
  write it for any pin.

### Header pins with a GPT output

| Header | WiFi port → output (timer) | Minima port → output (timer) | Other board use |
| --- | --- | --- | --- |
| D0 | P301 → GTIOC4B (GPT164, 16) | same | SCI2 RX (UART) |
| D1 | P302 → GTIOC4A (GPT164, 16) | same | SCI2 TX (UART) |
| D2 | P104 → GTIOC1B (**GPT321, 32**) | P105 → GTIOC1A (**GPT321, 32**) | — |
| D3 | P105 → GTIOC1A (**GPT321, 32**) | P104 → GTIOC1B (**GPT321, 32**) | — |
| D4 | P106 → GTIOC0B (**GPT320, 32**) | P103 → GTIOC2A (GPT162, 16) | — |
| D5 | P107 → GTIOC0A (**GPT320, 32**) | P102 → GTIOC2B (GPT162, 16) | — |
| D6 | P111 → GTIOC3A (GPT163, 16) | P106 → GTIOC0B (**GPT320, 32**) | — |
| D7 | P112 → GTIOC3B (GPT163, 16) | P107 → GTIOC0A (**GPT320, 32**) | — |
| D8 | P304 → GTIOC7A (GPT167, 16) | same | — |
| D9 | P303 → GTIOC7B (GPT167, 16) | same | — |
| D10 | P103 → GTIOC2A (GPT162, 16) | P112 → GTIOC3B (GPT163, 16) | — |
| D11 | P411 → GTIOC6A (GPT166, 16) | P109 → GTIOC1A (**GPT321, 32**) | Minima: SCI9 TX |
| D12 | P410 → GTIOC6B (GPT166, 16) | P110 → GTIOC1B (**GPT321, 32**) | Minima: SCI9 RX |
| D13 | P102 → GTIOC2B (GPT162, 16) | P111 → GTIOC3A (GPT163, 16) | onboard LED |
| A0–A3 | no GPT output | no GPT output | — |
| A4 | P101 → GTIOC5A (GPT165, 16) | same | I²C SDA |
| A5 | P100 → GTIOC5B (GPT165, 16) | same | I²C SCL |

Q4 answered (owner 2026-11-04, "builder with validators"): every pin in
this table is a supported STEP pin, including those with another board
use. A pin already owned by another driver (e.g. D0/D1 by the UART)
can't be handed to the builder, since each pin exists once. Each `StepPin`
impl's docs name the other use.

## 3. Layers (O4)

### 3.1 Low level — configured once, in setup

Each block owns its hardware (PAC tokens and pins, exactly once) and is
usable on its own. The firmware chooses and orders them.

| Block | Owns | Does |
| --- | --- | --- |
| `StepOutput` | a STEP pin and its GPT channel (pairing validated at compile time, §4) | pulse shape (low time, min high), rate → period, divider (16-bit, §5), start, a stop that never cuts a pulse |
| `PulseCounter<C>` | any GPT channel `C` | counts the STEP channel's pulse ends through the ELC; 16-bit counts extended in software |
| `Tick<T, N>` | a tick channel `T` (+ optional tick-number channel `N`) | the 2 ms hardware tick, its ELC event, the latch of count and phase |
| `TickWriter` | the DTC | one-shot chained writes at the next tick: period, buffer, DIR, read-back |
| `Dir<P, N>` | any GPIO pin | DIR level; gives the DTC its `PCNTR3` address and bit |
| `Elc` | the ELC (exists) | routes events between the blocks |

### 3.2 Slim layer — what the loop calls

`StepGenerator` is built in setup from the blocks above. The loop calls
only:

```rust
let t = stepper.poll_tick();   // latched count, position, what applied
stepper.queue(rate)?;          // signed rate for the next tick
stepper.stop();                // trip: never cuts a pulse
```

plus `predicted_pulses()`, `position()` and `set_tick_wake()`. This is the
API cart-pole already uses; only its constructor changes.

Q5 answered: both layers public, as agreed on 2026-11-03.

## 4. Validation at compile time (O2, O3)

Owner, 2026-11-04: the validators must "throw if invalid", and "that
throw should be caught in cargo check" — "THIS is Rust's super power!".
Everything known when the firmware is written is checked by the compiler.
A bad setup fails `cargo check` with a message naming the pin, and never
reaches the board.

### 4.1 Mechanism: type errors, not panics (research, 2026-11-04)

- **A `panic!` in const evaluation is not a reliable `cargo check`
  error.**
  - RFC 3477, the `cargo check` policy: "`cargo check` only catches some
    subset of the possible compilation errors", and a complete answer
    needs `cargo build`.
  - A failing const inside generic code passes `cargo check` and fails
    only `cargo build` (rust-lang/rust#99682, open).
  - Unused `const _: () = assert!(…)` items "are not guaranteed to fail
    the build" (heapless#351).
  - Const panic messages take only a literal or one `&str` (Rust 1.57.0
    release notes).
- **Type errors are always reported by `cargo check`.** The embedded
  practice is to encode which pin can do what as traits: the Embedded Rust
  Book's static guarantees, Embassy's per-function pin traits.
  `#[diagnostic::on_unimplemented]` (stable since Rust 1.78, per the
  1.78.0 release notes) replaces "trait not satisfied" with a message the
  HAL writes, naming the pin.
- **So:** pins, timers, widths, dividers and required parts are checked
  as types. Only the nanosecond timing numbers need a const assertion
  (§4.4).
- **Measured on Rust 1.99 (POC 1, `D:\temp\stepgen-poc`):** a failing
  `assert!` in a **named `const` item** fails `cargo check`, whether the
  item is used, unused or `const _`. In an **inline `const { }` block**
  (and in any const inside a generic function, rust-lang/rust#99682) it
  passes `cargo check` and fails only `cargo build`. So setups are named
  `const` items (§4.3). That `cargo check` evaluates them is current
  behaviour; RFC 3477 does not promise it.

### 4.2 `pin_map`: public, sealed pin facts (owner, 2026-11-04)

**Built 2026-11-04 as the HAL's header pin model**, which supersedes the
marker design below: a pin is `Pin<D6, MODE>`, `pin_map::D6` is the
header's type and `pin_map::d6` its value; one table per port pin
(`crates/unor4-hal/src/gpio/pin_map.rs`) gives every function, so the
`StepPin` facts below are the `Gtioc` trait (`Timer`, `OUTPUT`, `PSEL`).
Owner: "the pin_map should be the user-facing pin id, like A0, D3", so
messages name headers. The text below is kept as the original design.


- **`gpio::pin_map`** holds one zero-sized marker per header pin:
  `pin_map::d0` … `pin_map::d13`, `pin_map::a0` … `pin_map::a5`. Owner:
  "pin_map feels right, because we already build them for the featured
  board". The existing private `pin_map!` macro, which generates `Pins`
  for the selected board map, generates the markers from the same table,
  so markers and `Pins` can't disagree. Each marker's type carries its port
  and pin (`PinMark<'1', 6>` for WiFi D4, `PinMark<'1', 3>` for Minima D4).
  The names are lowercase to match `p.pins.d4`, which needs
  `#[allow(non_upper_case_globals)]`.
- **Public** (owner: "make it pub so users of the HAL can make proper
  builders"). The fact traits behind the markers are public: header name,
  port, pin; and, for pins that can drive a timer output, timer channel,
  A/B output, width, `PSEL`, and the compare/capture registers and ELC
  events (§4.6). So are the timer facts (channel number, width) for all
  eight GPT tokens. Firmware can bound on them and read them in its own
  `const fn` builders.
- **Sealed.** Outside code can use the traits but not implement them, as
  with `pwm::GptChannel`. The facts come from the manual's tables (§2);
  letting firmware declare a pin capable would let every builder program
  the wrong hardware.

### 4.3 The builders

Every part has its own validating builder, written as a named `const`
(§4.1), and one `StepSetup` combines them and cross-checks them. No part is
limited to one timer or one output (owner, 2026-11-04: "EXACTLY!"). The
hand-over then passes the real hardware, whose types must match the setup.

```rust
// setup: named consts, all checked by cargo check
const PUL: PulSetup<D6, Div16> = PulPinBuilder::new()
    .with_pin(pin_map::d6)          // any pin with a GPT output, A or B, 16- or 32-bit
    .with_type(16)                  // optional: stated width vs the pin's timer (§4.4)
    .with_divider(Div16)            // any of the six; checked against the width (§5)
    .with_timing(5_000, 2_500)      // LOW / min-HIGH ns vs the divider (§4.4)
    .build();
const DIR: DirSetup<D7> = DirPinBuilder::new()
    .with_pin(pin_map::d7)          // any header pin
    .with_setup(5_000)              // ns from a DIR change to the next pulse
    .with_forward(PinState::High)   // the level for positive rates
    .build();
const ENA: EnaSetup<D8> = EnaPinBuilder::new() // optional: the driver's enable input
    .with_pin(pin_map::d8)          // any header pin
    .with_enabled(PinState::Low)    // the level that enables this driver (no default)
    .with_lead(5_000)               // ns from enable to the first DIR change or pulse
    .build();
const COUNTER: CounterSetup<GPT166> = CounterBuilder::new()
    .with_timer::<GPT166>()         // any GPT; a 16-bit one is extended in software
    .build();
const TICK: TickSetup<GPT164, GPT165> = TickBuilder::new()
    .with_timers::<GPT164, GPT165>() // tick + tick number, any GPTs
    .build();                       // checks a whole 2 ms tick at the tick timer's divider
const SETUP: StepSetup<D6, Div16, D7, GPT166, GPT164, GPT165, D8> =
    StepSetup::new(PUL, DIR, COUNTER, TICK) // cross-checks the four parts
        .with_ena(ENA);                     // optional; cross-checks ENA

// hand-over: real pins and timers; their types must match SETUP
let mut stepper = StepGenerator::builder(SETUP)
    .pul(p.pins.d6, p.gpt163)       // D6's own pin and timer
    .dir(p.pins.d7)
    .counter(p.gpt166)
    .tick(p.gpt164, p.gpt165)
    .dtc(p.dtc)
    .ena(p.pins.d8)                 // exists only with ENA in the setup, and then required
    .build(&mut elc, &p.clocks)?;   // Err only for run-time resources (§4.5)
stepper.enable();                   // ENA to the enabling level; rates wait out the lead
```

**ENA** (owner, 2026-11-04: "We should have an ENA builder in the step
module"; "The builder needs to be generic, the use in the implementing
firmware is specific to the stepper driver"). Drivers differ in ENA
polarity and timing, so the builder fixes none of it:

- **DM542T** (StepperOnline V4.0, per a third-party copy of the manual; not
  checked against the vendor PDF): 4.5–24 V on ENA disables the drive and
  0–0.5 V enables it; unconnected, it is enabled. ENA must lead DIR by at
  least 5 µs, and enabling takes at least 200 ms.
- **DM542E** (Leadshine): the level that enables depends on how the input is
  wired (NPN vs PNP).
- **DM542S** (Cloudray): ENA active cuts the phase current, and the polarity
  is configurable.

So the firmware passes the level and a lead covering every rule of its own
driver. For the DM542T that is 200 ms, which also covers 5 µs. The
generator builds with the driver disabled. `enable()` drives the enabling
level and timestamps it on the tick timer (PCLKD/4) and the tick number.
`run`/`queue` with a non-zero rate return `Disabled` or `Enabling` until
the lead has passed. `disable()` refuses while running. `free()` disables
the driver and returns the pin as an input. Rate 0 and `stop` are always
allowed. A setup without `with_ena` has the type parameter `NoEna`, which
is the default, so six-parameter setups are unchanged: there is no `.ena`
method, and the generator is always enabled.

Slots are `Missing` or `Set<T>`; one sealed trait, `Provided<Slot>`, with
`Slot` a marker named after the method, gives every "not set" message. The
slot bounds sit on the `build` method, not its `impl`: on the `impl` a
missing slot hides the method (E0599) instead of naming it (found in POC
1 and POC 2). A named `const` needs its type spelled out on stable Rust,
hence `StepSetup<…>`'s six parameters (seven with ENA).

| Part | Check | How | Reported by `cargo check` as |
| --- | --- | --- | --- |
| PUL | the pin has a timer output | `Gtioc` bound | `` `A0` has no GPT output `` |
| PUL | divider fits the timer width; `/1` refused | `DividerFor<P, Width>` | `` `Div16` cannot clock PUL pin `D4`: its timer is `Bits32` `` |
| PUL | LOW ≥ 1 count, ≤ the DIR write window (400 PCLKD counts); min-HIGH ≥ 2 × the edge margin; stated width | const asserts | `with_timing: …`, `with_type: …` |
| DIR, ENA | the pin is an Arduino header, not a board-internal pin (`UsbMux`, the ESP32-S3 bridge's `Sci9Tx`/`Sci9Rx`, `Sci1Tx`/`Sci1Rx`); every header can drive an output (the input-only P200/P214/P215 are on no header, manual §19.2) | `HeaderPin` bound | `` `…::UsbMux` is a board-internal pin, not an Arduino header `` |
| DIR | set-up ≥ 1 PCLKD count and fits the period maths | const assert | `with_setup: …` |
| ENA | the enabling level is set (no default: drivers differ) and the lead is set | `Provided<slot>` | `` setup incomplete: `with_enabled` not set `` |
| ENA | the board does not hold the pin at the enabling level while it is undriven (reset, bootloader): WiFi D13 is pulled LOW by R52 (100 kΩ, schematic ABX00087) | const assert on `board_pull_down_ohms` | `with_enabled: the board holds this pin LOW while it is not driven …` |
| StepSetup | ENA on neither the PUL nor the DIR pin | const assert on `PORT`/`PIN` | `StepSetup: ENA is on the DIR pin`, … |
| hand-over | `.ena(pin)` given exactly when the setup has ENA | `EnaPart<E>` bound / method only for a pin | `` setup incomplete: `ena` not set ``; without ENA: `` doesn't satisfy `NoEna: PinId` `` |
| COUNTER, TICK | the type is a GPT timer | `GptChannel` bound | `` `X` is not a GPT timer `` |
| TICK | a whole 2 ms tick at the tick timer's prescaler; tick ≠ tick number | const asserts | `TickBuilder: …` |
| StepSetup | no timer channel used twice (PUL, counter, tick, tick number) | const assert on `CHANNEL` | `StepSetup: the counter uses the PUL timer`, … |
| StepSetup | DIR not on the PUL pin | const assert on `PORT`/`PIN` | `StepSetup: DIR is on the PUL pin` |
| every builder | every required part set | `Provided<slot>` | `` setup incomplete: `with_setup` not set `` |
| hand-over | each pin and timer is the one set up | parameter types from `SETUP` | mismatched types: expected `Pin<D6, _>`, found `Pin<D7, Input>` |
| hand-over | no token used twice | ownership | use of moved value |

Const-assert messages are fixed strings (a const panic takes one `&str`),
so they name the part and the rule, not the pin; type-level checks name
the pin.
### 4.4 The numbers: const assertions

Two checks compare values, not types: `.with_type(16)` against the pin's
width, and the nanosecond timings against the divider's resolution. They
are `assert!`s in the `const fn` builder, inside the setup's `const { … }`
block in a non-generic function. RFC 3477 promises only `cargo build`
reports them. Whether `cargo check` does in practice on this toolchain is
experiment V1 (§4.7). If it doesn't, these two checks still stop the build
(`cargo build`), but not `cargo check`. That's recorded, not hidden.
Messages are ready-made strings (`timing: low time under 1 count at
/1024`).

### 4.5 Run time: only what the firmware's state decides

`StepGenerator::builder().build()` returns `StepResult<StepGenerator>` for
the two things only known when it runs: a free ICU slot for the tick's DTC,
and the two ELC inputs not already linked by other code. `run`/`queue`
refusals stay run-time too. The error type is the module's existing
`StepError` (owner: "the step module already has a step_error"), on the
`uart/read_error.rs` / `uart/read_result.rs` prototype:
- typed variants with context;
- `core::fmt::Display` and `core::error::Error`;
- `StepResult<T> = CoreResult<T, StepError>`.

Today's `ConfigInvalid` goes away; its causes move to §4.3/§4.4, or
become these specific run-time variants.

Open: `StepError`'s `Display` hand-written like the prototype, or
`#[derive(thiserror::Error)]`. thiserror needs `thiserror = { version =
"2", default-features = false }` in `crates/unor4-hal/Cargo.toml` (owner)
and a licence-table row. Its compile-time dependencies include
`unicode-ident`, (MIT OR Apache-2.0) AND Unicode-3.0, already in the HAL's
tree through `cortex-m` → `cortex-m-macros` and never audited.

### 4.6 A or B output

The pulse end is the compare match of the register matching the output:
`GTCCRB` for a B output (as built), `GTCCRA` for an A output. The phase
latch at the tick captures into the other register. The ELC event that
counts pulse ends follows: `GPTn_CCMPB` or `GPTn_CCMPA`. All eight
channels' events reach the ELC (Table 18.3, checked 2026-11-04):
`GPTn_CCMPA` = `057h + 8n`, `GPTn_CCMPB` = `058h + 8n`, `GPTn_OVF` =
`05Dh + 8n`. So any channel can be the counter or the tick.

### 4.7 To verify before relying on it

One-file experiments, run before step 1:

| # | Question |
| --- | --- |
| V1 | Does `cargo check` report a failing `assert!` in a `const { … }` block in a non-generic function on Rust 1.99? (RFC 3477 promises only `cargo build`.) |
| V3 | Can a `const fn` generic over a sealed trait read its associated constants inside `const { … }`? |
| V5 | Does `cargo check` show the `on_unimplemented` message, naming the pin, for a bad `.with_pin` and a mismatched `.pul` token? |
| V6 | Does a typestate `build()` that doesn't exist yet show the `on_unimplemented` message (`setup incomplete: …`)? |

Results, Rust 1.99 (POC 1 and POC 2, each case compiled by a harness that
compares outcome and message):

| # | Result |
| --- | --- |
| V1 | **Partly.** Named `const` items: yes, used or not. Inline `const { }` blocks and generic functions: no, only `cargo build`. Design changed to named consts. |
| V3 | Yes. Firmware's own `const fn` reads `Gtioc`/`GptTimer` constants. |
| V5 | Yes for `.with_pin` (`` `A0` has no GPT output ``). The `.pul` pin/timer checks are plain type mismatches naming both types, which replaced POC 1's message-only helper traits. |
| V6 | Yes, with the bounds on the method: `` setup incomplete: `with_pin` not set ``. A projection of the slot type in the signature, or bounds on the `impl`, gives E0599 instead. |

## 5. 16-bit STEP timers

| Divider | Longest period | Slowest rate | Resolution |
| --- | --- | --- | --- |
| PCLKD/1 | 65 536 counts = 1.37 ms | 733 /s | 20.8 ns |
| PCLKD/4 | 5.46 ms | 183 /s | 83 ns |
| PCLKD/16 | 21.8 ms | 46 /s | 333 ns |
| PCLKD/64 | 87.4 ms | 11.4 /s | 1.33 µs |
| PCLKD/256 | 350 ms | 2.9 /s | 5.33 µs |
| PCLKD/1024 | 1.40 s | 0.72 /s | 21.3 µs |

(48 MHz PCLKD.)

- **The divider is fixed at construction** (§22.2.12). Q3 answered, owner
  2026-11-04: a builder setting, "with validators, so pin deps catch
  errors". The compile-time validators (§4) check it against the STEP pin's timer:
  - 16-bit STEP pin without a divider setting: error;
  - divider below /4: error (the rate-0 handling below needs ≥ /4);
  - divider setting on a 32-bit STEP pin: error, not silently ignored;
  - pulse shape that doesn't fit the divided counts: error.

  Q3b answered, owner 2026-11-04: "Can it not be configured by the user to
  ANYTHING and the validators sanitize and throw if wrong: Throw with well
  formatted messages". So `.divider(…)` accepts any of the six hardware
  dividers, /1 included, and the compile-time validators reject the
  combinations that can't work (§4.3, §4.4).
- **Pulse shape:** low time and minimum high time round up to whole
  divided counts.
- **Rate 0 and idle** differ from 32-bit, where the longest period is
  ~89 s. On 16-bit it can be shorter than a tick (1.37 ms at /1), so
  "longest period" cannot hold pulses off between polls. Proposal:
  - require divider ≥ /4 (5.46 ms);
  - the HAL stops the counter at the first poll after a rate-0 tick;
  - it keeps the last pulse's time in software and preloads `GTCNT` on
    the next start, so R8 (next pulse one new period after the last) still
    holds.

  To prove on the board.
- **`stop()`** keeps the race-free design. It writes the longest period,
  waits out a pulse in flight (≤ low time), then stops. That waits far less
  than the shortest longest-period (1.37 ms).
- **DTC writes** to 16-bit-effective registers: word writes, upper bits
  ignored (§22.2.20, per `GptChannel`'s docs). **Unverified for the DTC**
  until the board shows it.

## 6. Measured constants to re-measure

These were measured on one configuration only (GPT320 STEP, GPT321 count,
GPT164/165 tick, P107 DIR). Each is re-measured for a 16-bit STEP timer
and a second 32-bit one before any claim:

- `PULSE_END_LATCH_DELAY` (2 counts);
- DTC first-record latency (14–17 counts) and delay-record timing
  (2N + 22…32);
- hardware-start delay (1 count);
- DIR write window (326–342 counts).

If they differ by configuration, they become per-configuration values,
not one constant.

## 7. Resources

One generator uses: the STEP timer, a count timer, a tick timer (+ an
optional tick-number timer), the DTC, two ELC GPT inputs and one ICU slot.
That leaves 4–5 of the 8 GPT channels for PWM and other users. Which ones
depends on the firmware's choice (Q2).

## 8. Bench proof — existing wiring, no new wires

| STEP pin (WiFi) | Timer | R3 input | Timing precision |
| --- | --- | --- | --- |
| D4 | GPT320, 32-bit | D8 (ICP1, hardware capture) | 0.5 µs |
| D3 | GPT321, 32-bit | D3 (INT1) | interrupt latency, a few µs (to measure) |
| D6 | GPT163, 16-bit | D2 (INT0, Slice 7 rig) | interrupt latency, a few µs (to measure) |
| D5 | GPT320, 32-bit | D7 (DIR wire) | polled; counts only |

DIR on D5 (read on R3 D7) when STEP is elsewhere, and on D4 (read on
R3 D8) when STEP is on D6.

- **Exact counts** are checked on every row. **Timing** is precise only
  on D4. The rate limit of the INT0/INT1 rows is measured first.
- **Release APIs.** The validator image must switch D6 between Slice 7's
  `Pwm` and the generator, and D4 between configurations, so `Pwm` and
  the blocks need `free()`/`release()` that return their pins and tokens.
- **Minima:** compiles and passes host tests; no hardware run. Labelled
  unverified.

## 9. Build steps (owner review between steps)

0. **Done:** language experiments V1, V3, V5, V6 (§4.7) in POC 1 and POC
   2; the header pin model in the HAL (§4.2), `validate all` 24/24.

The generator's internals hard-code their hardware in about 45 places:
GPT320 output B (`GTCCRB`, ELC `GPT0_CCMPB`), GPT321 as counter, GPT164 /
GPT165 as tick, DIR as `PORT1.PCNTR3` bit 7; `ElcEvent` lists only those
events. All of it is generalised; the order below only sequences the work
and its bench proof. Nothing is released until all of it is proven.

1. **GPT facts and ELC events.** `GptChannel` gains `CHANNEL` and `Width`
   (`Bits16`/`Bits32`), and its module-stop release per channel;
   `ElcEvent` gains every channel's `GPTn_CCMPA`, `GPTn_CCMPB` and
   `GPTn_OVF` (Table 18.3: `057h + 8n`, `058h + 8n`, `05Dh + 8n`). Host
   tests against the table.
2. **The five builders and the hand-over** (§4.3), replacing
   `StepGenerator::d4` and `StepConfig`; the numeric checks move from run
   time into the consts, so `StepError::ConfigInvalid` becomes the
   run-time `NoIcuSlot`. Host tests: every rejection as a case
   (`compile_fail` doc tests in the crate; messages checked by a harness
   outside the repo, as in POC 2).
3. **Generalised 32-bit internals:** STEP on any 32-bit channel and either
   output (compare and capture registers and the pulse-end event follow
   the output), the counter on the other 32-bit channel, the tick on any
   pair, DIR on any pin (the DTC writes the DIR pin's `PCNTR3`). Bench:
   all S11 checks on D4/D5 through the builder (regression), then STEP on
   D3 (GPT321, read by R3 D3 — new R3 check, rate limit measured first)
   with GPT320 counting.
4. **16-bit counter**, extended in software from the per-tick latch.
   Bench: S11 with the counter on GPT166.
5. **16-bit STEP** with dividers and the rate-0 handling of §5, the
   constants of §6 re-measured per configuration. Bench: STEP on D6 (R3
   D2, INT0), divider /4 and one more, all S11 checks, rate 0 and restart.
6. **ENA** (§4.3): `EnaPinBuilder`, `StepSetup::with_ena`, the hand-over's
   `.ena`, and `enable`/`disable` with the lead enforced by the generator.
   `HeaderPin` keeps DIR and ENA off the board-internal pins. Host-proven
   2026-11-04: host matrix, `compile_fail` doctests, and 13 new harness
   cases (34/34). Bench: needs a new wire, proposed as R4 D8 (ENA) ↔ R3
   D6 (PCINT22). The R3 checks the disabled level after build and the
   enabling level after `enable`. Using Timer1, it checks that the first
   STEP edge on ICP1 comes at least the lead after the ENA edge, and that
   a rate is refused before then.
7. **Docs:** README record, CHANGELOG, crate README, `Publishing.md`.
Cart-pole uses STEP D4 / DIR D5: only its constructor call changes, to the
builders above.

## 10. Questions for the owner

| Q | Question |
| --- | --- |
| Q1 | **Answered 2026-11-04:** one generator at a time (owner chose it from the agent's offer of one, two or three). |
| Q2 | **Answered 2026-11-04:** chosen by the firmware within the supported limits, through a builder whose validators run at compile time (§4). |
| Q3 | **Answered 2026-11-04:** any of the six dividers can be set; the compile-time validators check it against the STEP pin's timer and fail `cargo check` with a short message (§4, §5). |
| Q4 | **Answered 2026-11-04:** yes, every timer-capable pin. Ownership prevents a clash with another driver, and the docs name each pin's other use. |
| Q5 | **Answered:** both layers public (owner, 2026-11-03). |

All questions answered. The design waits for the owner's approval and a
code grant.
