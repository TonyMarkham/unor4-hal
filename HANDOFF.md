# HANDOFF — unor4-hal (rewritten 2026-11-04, before a context compaction)

Read AGENTS.md first: rules, grants, bench. This file is the working state
on top of it. **Nothing is committed; the owner commits (rule 6).**

## Where things stand

- **Version 0.2.0, unreleased.** crates.io has 0.1.2.
  - Slice 11 (the step generator) plus two follow-ups are built, recorded
    in the README, and proven on the board: `validate all` 24/24.
  - Follow-up 1: `stop()` could cut a pulse; fixed by writing the longest
    period first.
  - Follow-up 2: `set_tick_wake` lets the 2 ms tick wake the CPU from WFI.
- **The old one-off `StepGenerator::d4` is gone** (owner: "NO FUCKING WAY
  AM I GOING TO PUBLISH THIS NAIVE BULLSHIT!"); the builder set below
  replaced it. 0.2.0 is not published until the builder work is finished
  and recorded.
- **The validator image now counts pulses on 16-bit GPT162** (not GPT321)
  and builds its generator from `step_run::SETUP`; the README's Slice 11
  text and check table still say GPT321 / `d4` — update with the records.
- **Done 2026-11-04:** two POCs outside the repo (`D:\temp\stepgen-poc`,
  `D:\temp\stepgen-poc2`), then the HAL moved onto POC 2's **header pin
  model**: `Pin<D6, MODE>`, one pin table (`gpio/pin_map.rs`) with
  hard-coded capabilities, `Pwm::new` / `Uart::new` / `UartIrq::attach` /
  `Adc::read` checked by trait bounds. `validate all` 24/24 (README,
  CHANGELOG). Setups for compile-time number checks must be named `const`
  items: `cargo check` skips inline `const { }` blocks (POC 1, V1).
- **In progress (grant in AGENTS.md, owner "EXACTLY!"):** the full step
  generator builder set, `StepGeneratorDesign.md` §4.3/§9. Done and
  proven: §9 step 1 (GPT facts, all ELC events); step 2 (`crate::builder`,
  `PulPinBuilder`/`DirPinBuilder`/`CounterBuilder`/`TickBuilder`/
  `StepSetup`, `StepGenerator::builder(SETUP)`, `d4`/`StepConfig`/
  `ConfigInvalid` gone, `NoIcuSlot` added); step 3 internals (any channel,
  output A/B, DIR any pin via its PCNTR3) — `validate all` 24/24 on D4/D5
  through the builder; 21 compile-fail/pass cases in `D:\temp\hal-cases`
  (`cargo test` there) all match; `compile_fail` doc tests in the crate.
  Step 4 proven: 16-bit counter (validator counts on GPT162), extension
  continuous and signed on every read (`pulses()` and `poll_tick` share
  `ext_raw`/`ext_total` Cells; a read needed every 32 768 pulses) — first
  try lost exactly 2¹⁶ because `pulses()` extended from the last poll only;
  `slices S11` 8/8 after the fix (count 71 162, wake 214 055 = R3).
  Release API written (compiles, clippy clean, not bench-run yet):
  `StepGenerator::free(self, SETUP, &mut elc) -> Result<StepParts, Self>`
  (checks the setup names the hardware it drives, stops without cutting a
  pulse, unlinks ELC B/C, `TickDtc::release` → new `Ielsr::release`, pins
  back to input, tokens via new hidden `GptChannel::steal`). Left: a
  validator request that frees and rebuilds on another configuration. Bench wiring: STEP D5 (output
  A, GPT320) counted on R3 D7 (PCINT23, new R3 check) with DIR on D4 read
  on R3 D8; STEP D6 (16-bit GPT163) on R3 D2 (INT0). Then step 5 (16-bit STEP: `StepWidth`
  bound refuses it at compile time until then; /1024 gives a non-integer
  tick in timer counts — refuse it; latch delay and DTC constants in timer
  counts; rate 0 without an 89 s period), records (README, CHANGELOG, crate
  README, design doc). The owner was furious the builder was left
  out after the pin model: never narrow the scope again without asking.
- **ENA built, host-proven, not bench-run (2026-11-04).** The owner asked
  for it ("We should have an ENA builder in the step module") and said the
  builder must be generic, with the driver-specific values coming from the
  firmware. Recorded as a widening of the builder grant.
  - What's built:
    - `EnaPinBuilder` with `with_pin` / `with_enabled` / `with_lead`;
    - `EnaSetup`;
    - `NoEna`, the default 7th parameter of `StepSetup` and `StepParts`;
    - `StepSetup::with_ena`, which cross-checks that ENA is on neither the
      PUL nor the DIR pin;
    - the hand-over's `.ena(pin)` (`EnaPart<E>`: required only with ENA);
    - `enable` / `disable` / `is_enabled`;
    - `StepError::Disabled` and `Enabling`, which a non-zero `run`/`queue`
      returns until enabled and the lead has passed (lead timed on the tick
      timer and the tick number);
    - `free` disables the driver and returns the ENA pin as an input.
  - **Owner, "I don't see any validation in ena_pin_builder":** added the
    new `gpio::HeaderPin` capability, generated only for header pins. ENA
    *and* DIR `with_pin` now refuse board-internal pins: `UsbMux`, the ESP32
    bridge's `Sci9Tx`/`Sci9Rx`, and `Sci1Tx`/`Sci1Rx`. Every header can
    drive an output; the input-only P200/P214/P215 are on no header (manual
    §19.2).
  - The enabling level has no default.
  - **Owner, "NOW it feels like you COMPLETELY IGNORED my concern about the
    ENA Pin Builder":** added a board check. Until firmware drives it
    (reset, bootloader) an undriven pin floats, except WiFi D13, which R52
    (100 kΩ, SCK LED gate, schematic ABX00087) holds LOW. So ENA on D13
    cannot enable on LOW (`board_pull_down_ohms` in `pin_map`). Harness
    cases added.
  - The lead has no bound: the longest, 4.29 s, is far under the tick
    number's 131 s wrap.
  - The validator maps the new errors to codes 7 and 8.
  - Proof: host check, clippy and test matrices; firmware clippy and build;
    `D:\temp\hal-cases` 34/34 (13 new ENA/header cases, messages verified).
    One fix along the way: the hand-over's private `ena` field was renamed
    to `ena_pin`, so calling `.ena` on a setup without ENA no longer gets
    the misleading hint "private field, not a method".
  - Bench: needs a new wire, proposed as R4 D8 ↔ R3 D6 (design §9 step 6).
    Waiting on the owner's wiring.
- **`SwitchPinBuilder` built, host-proven, not bench-run (2026-11-04,
  grant in AGENTS.md, owner "Do it").**
  - New files in `gpio`: `contact` (`Contact`, `No`, `Nc`: active level and
    `IRQMD`), `Pull`, `SwitchFilter` (`FCLKSEL`), `SwitchPinBuilder`,
    `SwitchSetup<P, C>`, `Switch<P, C>`, `SwitchError`.
  - Hooks: `pin::configure_input` (pull-up and `ISEL`);
    `Ielsr::is_linked`; `Event::gpio_irq`; slot markers `with_contact` and
    `with_pull`.
  - `Switch::new`: `is_active` / `is_inactive` / `free`.
  - `Switch::with_edge` (only for `IrqPin` pins):
    - order per §13.4.4: filter off, `ISEL`, `IRQCRi`, then the ICU slot;
    - the edge is latched in `IELSRn.IR`, no CPU interrupt;
    - `activated()` reads and clears it;
    - errors return the pin.
  - **Owner, "AGAIN no validator, can a switch be configured on any
    pin?":** on the silicon every header has a pull-up. On the WiFi board
    D13's R52 pull-down beats the worst-case internal pull-up (50 kΩ
    against 100 kΩ gives 0.67 VCC, below VIH = 0.8 VCC, datasheet
    R01DS0355 Tables 2.4/2.10). So `Pull` became `Internal` /
    `External(ohms)`, and `build` asserts the pull-up holds VIH against
    the board's pull-down (D13: Internal refused, External at most 25 kΩ)
    and that External is not 0 Ω. Minima: no schematic, so no loads
    recorded (unverified). Host test: only WiFi D13 has a board
    pull-down. Harness 54/54.
  - The filter is a glitch filter: 8 µs at most with PCLKB 24 MHz.
    Debouncing is the firmware's job.
  - Several headers share IRQ numbers (A3/A5 IRQ2, A4/D2 IRQ1, A1/D0
    IRQ6). That is a run-time `IrqInUse`, because separate setups cannot
    see each other at compile time.
  - Proof: 4 host tests, 3 `compile_fail` doctests, 11 harness cases
    (45/45 in all); host clippy, tests and docs; firmware clippy.
  - Bench not done: needs a wire to a switch pin, or the R3 acting as the
    switch.
- **Builder tests in the repo (2026-11-04).** The owner found that no test in
  the repo called any builder ("WRITE FUCKING TESTS RIGHT FUCKING NOW").
  - Added `src/tests/builders.rs`: 34 host tests (6 of them board-gated).
    The builders are `const fn`s, so each test calls one at run time,
    catches the `assert!` panic and compares the **whole** message; value
    tests check what each builder produces.
  - They caught three garbled messages (a lost `\` continuation in my edit
    script left a run of spaces mid-message), in `EnaPinBuilder` and twice
    in `SwitchPinBuilder`. Fixed, and the tests now pass on all six feature
    sets.
  - The scratch harness in `D:\temp\hal-cases` never caught them, because
    it matched message fragments.
  - Found while writing the tests: two checks can never fire at the fixed
    48 MHz PCLKD. DIR set-up `≤ u32::MAX / 4` counts is unreachable (the
    longest set-up is 206 158 431 counts). Two of `TickBuilder`'s asserts
    (a whole 2 ms tick, a fit in `GTPR`) are unreachable too.
  - Still without full-message tests: the type-level refusals
    (`on_unimplemented` messages, mismatched types). Only `compile_fail`
    doc tests with error codes cover them. Full-message compile tests need
    a `Cargo.toml` change (`trybuild` dev-dependency, or a cases package),
    which the grants exclude: asked the owner, not yet answered.
- **Missed-tick recovery, verified on the board (2026-11-04).** The
  cart-pole session reported, from reading the code, that one missed tick
  while idle stops the idle stop forever and fires a stray pulse every
  89.48 s. The owner: "FIx your lazy, slop".
  - Fixed in `step_generator.rs` `poll_tick`, with the arithmetic in the
    new `step/recovery.rs` (12 host tests):
    1. The idle stop reads `GTPR`/`GTPBR` and the captured phase; it needs
       no schedule.
    2. The schedule is rebuilt from the registers when the STEP counter
       proves no overflow since the tick: `GTPR + 1` was in progress,
       `GTPBR + 1` follows.
    3. A reversal the DTC applied at a missed tick is split exactly (the
       previous schedule plus the read-back) and applied: direction and
       `position`.
  - New bench check `S11 skip`: R4 request `k1`/`k2`/`k3`, R3 `K`, CLI line
    "S11 skip". 3/3 passed, then `validate all` 25/25.
  - A stall that ends anywhere in a tick usually has a STEP overflow before
    the poll, so the rebuild waits for the next tick (one refused `queue`,
    `NotReady`). A stall that ends just after a tick rebuilds in the same
    poll.
  - Records: README follow-up 3, the check table row, the witnessed run;
    CHANGELOG Fixed. Reply to cart-pole: drafted for the owner to paste.
- **User docs (2026-11-04).** The owner: "Do you expect users to infer
  from code?" Before this, nothing showed how to control the generator.
  - The `step` module docs now have:
    1. a parts table: what each builder supports and refuses;
    2. a control table: what to call, and when;
    3. every `queue` error and what to do about it;
    4. a complete DM542 example with ENA, a `no_run` doctest that compiles.
  - The `gpio` module docs gained a switch guide (table, run-time use,
    debounce, IRQ sharing, a compiled two-limit-switch example).
  - The `StepGeneratorBuilder` example is compiled now, not `ignore`d.
  - The crate README gained a Usage section, the step line was corrected
    (not "WiFi D4 only"), and the "every feature proven" claim now names
    what isn't run on the board: ENA, switches, non-GPT320 STEP, `free`.
  - `lib.rs` gained a "Where to start" section.
  - The root README Slice 11 record has a "Superseded API" note.
- **README rewritten as a user guide; slice records moved to
  `Validation.md` (2026-11-04).** The owner: "the current README is SHIT!
  It should NOT be slice based", then "COMPLETELY RE-WRITE IT", and "I
  want examples for PUL, DIR, ENA, and Switch". The grant is recorded in
  AGENTS.md.
  - `Validation.md` holds the old README's "Testing on hardware" (lines
    180–1830), unchanged under a new header.
  - The new `README.md` covers:
    - status, then getting started (deps, `.cargo/config.toml`,
      `memory.x`, `build.rs`, a blink program, build/upload);
    - pins;
    - stepper sections for PUL, DIR, ENA and counter/tick, each with a
      compiled example, what it accepts and refuses, and its run-time
      behaviour;
    - the control table, the `queue` errors, and a complete stepper + ENA
      + two limit switches program;
    - switches, UART (polled and IRQ), PWM, ADC, data flash, USB;
    - the features table, testing, the licence table and MSRV (kept).
  - Its 13 `rust,no_run` blocks are compiled by `just test`
    (`ReadmeExamples` in `lib.rs`, `cfg(all(doctest, wifi, usb,
    uart-irq))`).
  - AGENTS.md rules now point slice records, upload commands and the
    All-in-one validation subsection at `Validation.md`.
  - Code comments citing "README, Slice N" were repointed, except
    `usb/bus.rs` (outside every grant), plus `Slice11Plan.md` and
    `CHANGELOG.md` history.
  - The crate README links to the guide.
- **Builder-shaped setup and `with_peripherals`, verified on the board
  (2026-11-04).** The owner: "FIX YOUR BULLSHIT!", answering the proposed
  shape. The grant is in AGENTS.md.
  - **Setup:** `StepGeneratorBuilder::new().with_pul().with_dir()
    .with_ena().with_counter().with_tick().build()` replaces
    `StepSetup::new(…).with_ena(…)`. The old hand-over type of that name is
    gone.
  - **Run time:** `StepGenerator::new(SETUP).with_peripherals(p).build()`
    (`step_generator_init.rs`, `StepGeneratorInit`) claims the setup's
    hardware and returns `(generator, rest)`. `free(SETUP, p)` releases it.
  - **Switches:** `Switch::new(SETUP).with_peripherals(p).build()`
    (`switch_init.rs`, `SwitchInit`); the edge latch is in the setup
    (`with_edge`, and `with_filter` needs it).
  - **`Peripherals`:** new module `peripherals/` (`Peripherals` generated
    by a macro, `Claim`/`Release` impls per pin, GPT channel and the DTC,
    `Taken`, `Board`). `Pins` is generic (`gpio/pins.rs`), `elc` is the
    started `Elc`, and `TickDtc::new` returns the DTC on failure.
  - **Proof:** host matrix 139 tests; harness 52/52 (double claim →
    `` `D5` is already used by another driver ``); README and module
    doctests compile; firmware clippy on both boards; `validate all` 25/25.
  - **Known limit:** once a field is moved out directly, `p` is partly
    moved and can't go to `with_peripherals` or `free` again. Documented in
    the README ("Order").
  - **Git index:** I ran `git mv` by mistake (rule 6 broken), which staged
    the rename `crates/unor4-hal/src/peripherals.rs ->
    peripherals/old_peripherals.rs`. That file has since been deleted.
    Unstage with `git restore --staged crates/unor4-hal/src/peripherals.rs
    crates/unor4-hal/src/peripherals/old_peripherals.rs`.
- **Limit switches: design history (2026-11-04).**
  The cart-pole has two limit switches.
  - I proposed one builder rather than `NoPinBuilder` + `NcPinBuilder`, then
    sketched it as step-specific (sides, a GTPSR hardware stop, `StepSetup`
    cross-checks).
  - Owner: "TOO specific!", then "SwitchPinBuilder is better".
  - Agreed shape: a generic `SwitchPinBuilder` in `gpio`, not `step`:
    - `with_pin` (`HeaderPin`);
    - `with_contact(No | Nc)`, required, with marker types, giving
      `SwitchSetup<D2, Nc>`;
    - `with_pull(Internal | External)`, required (the RA4M1 has pull-ups
      only, `PmnPFS.PCR`);
    - optional `with_filter`, only for `IrqPin` pins (IRQ digital filter,
      §13.2.1);
    - at run time, `is_active()` and an optional activation edge.
  - Limit behaviour stays in the firmware.
  - Waiting on the owner's grant before writing it.

## Why the 0.2.0 `step` module was naive (history; replaced by the builders)

What was in `crates/unor4-hal/src/step/` before the builders:
- **One wiring only.** `StepGenerator::d4` is the only constructor:
  STEP = WiFi D4 (P106, GPT320 B), DIR = D5 (P107). The DTC writes DIR
  through hard-coded `PORT1_PCNTR3` and `DIR_BIT = 1 << 7`. No Minima
  constructor.
- **Fixed timers.** GPT321 is the pulse counter, which wastes the second
  32-bit timer: a 16-bit counter extended in software would do. GPT164 is
  the tick and GPT165 the tick number; the DTC and ELC inputs B and C are
  claimed. `Pwm::d3` and `Pwm::d5` can't coexist with it.
- **No low-level layer.** The agreed two-layer API (low-level blocks
  configured once, plus the slim loop calls) was never written into
  `Slice11Plan.md`, so only the slim half exists.
- **Errors are bare codes.** `StepError::ConfigInvalid` folds six causes
  into one, and `StepError` has no `Display`/`Error` impls, unlike the
  `uart/read_error.rs` prototype. All checks run at run time; none is
  reported by `cargo check`.
- **Constants measured on one configuration only:** the latch delay, DTC
  latencies, start delay, DIR window. A different timer pair may differ.
- **How it happened:** the agent turned cart-pole's wiring (R1: "PUL on
  D4") into the HAL's API without checking it against the public-audience
  rule. The owner found out by reading the CHANGELOG.

The module works and is proven on the bench for cart-pole's wiring. The
README, CHANGELOG, crate README and `Publishing.md` still present it as
part of 0.2.0. They need correcting once the redesign lands, or if `step`
is pulled from the release.

## Why prototype first

- **Prove the compile-time API before breaking the HAL.** The redesign
  rests on language behaviour not yet confirmed on this toolchain:
  - whether `cargo check` (not only `cargo build`) reports the const
    asserts;
  - `on_unimplemented` messages naming the pin;
  - typestate `build()` messages;
  - `const fn` reading sealed-trait constants.

  Design §4.7 lists them as V1, V3, V5 and V6. A separate crate answers
  them with real compiler output; if one fails, the design changes before
  any HAL code does.
- **Keep the working HAL intact meanwhile.** The current `step` module
  and its 24/24 bench proof stay untouched until the prototype shows the
  API works. Then the HAL is changed in reviewed steps (design §9).
- **Let the owner review a working API, not a document.** This addresses
  the cause of the one-off: an unreviewed design.

## Proposed MVP (awaiting the owner's grant)

- **Location:** `crates/stepgen-proto/`, `publish = false`. Its own
  `[workspace]` table, so the root `Cargo.toml` and the HAL are untouched.
- **Dependencies:** `unor4-hal` and `ra4m1` by path. New dev-dependency:
  `trybuild`, licence-audited first.
- **Scope:** the compile-time layer only, host-side, no hardware.
  - `TimerPin` facts trait on `Pin<PORT, PIN, …>`, per port pin (§19
    tables). Proposed; the owner hasn't confirmed this part.
  - `pin_map` markers for both board maps.
  - `PulPinBuilder`: typestate, dividers as types, `on_unimplemented`
    messages, const asserts for the nanosecond timings.
  - A hand-over builder skeleton that type-checks pins and tokens against
    the setup.
- **Proof:** valid setups compile; trybuild tests show every invalid one
  failing `cargo check` with its message. This answers experiments V1, V3,
  V5 and V6 (design §4.7).
- **Grant asked:** write `crates/stepgen-proto/**` including its
  `Cargo.toml`, run `cargo check`/`cargo test` for it, record the results
  in the design doc. Excludes the HAL, the root `Cargo.toml`, firmware and
  git chores.

## Design decisions so far (StepGeneratorDesign.md)

- One generator at a time.
- STEP on any timer-capable header pin, 16- or 32-bit; DIR on any GPIO.
- Two public layers: low-level blocks for setup, and the slim loop API
  (`poll_tick` / `queue(rate)` / `stop`). Agreed with the owner on
  2026-11-03 but never written down then, which is how the one-off
  happened.
- **Validation happens at compile time.** The owner wants errors in
  `cargo check`, never on the board.
  - Research: const `panic!` is not guaranteed in `cargo check`
    (RFC 3477, rust-lang/rust#99682).
  - So pins, timers, widths, dividers, required parts and double use are
    type errors, with `#[diagnostic::on_unimplemented]` messages that
    name the pin.
  - Only the numeric timings use const asserts, which needs experiment V1.
- **`pin_map`:** public, sealed, generated by the existing per-board
  `pin_map!` macro. Markers are lowercase (`pin_map::d4`).
- **Builder shape (owner's):**
  `PulPinBuilder::new().with_pin(pin_map::d6).with_type(16).with_divider(Div16).with_timing(..).build()`
  in a `const { }` block. Then `StepGenerator::builder().pul(pul,
  p.pins.d6, p.gpt163)…build(&mut elc)?`.
- **Run-time errors** only for run-time resources: a free ICU slot, the
  ELC inputs. They use the module's existing `StepError`/`StepResult`,
  following the `uart/read_error.rs` + `read_result.rs` prototype. Today's
  `ConfigInvalid` is removed.
- **Timer width:** the agent answered the owner's "extend Pin with the
  timer size?" with a sealed trait on `Pin` (silicon facts per port pin),
  not a new `Pin` parameter. The owner hasn't replied yet.
- **Still open:** `StepError` `Display` hand-written or `thiserror`
  (`default-features = false`; needs a `Cargo.toml` line from the owner).

## Open issues (owner decisions unless noted)

1. **Licences.** `unicode-ident` is "(MIT OR Apache-2.0) AND Unicode-3.0".
   It is already a transitive HAL dependency (`cortex-m` →
   `cortex-m-macros` → `proc-macro2`). The README and AGENTS.md claim
   "only MIT or MIT/Apache-2.0" is therefore false, and transitive
   dependencies were never audited.
2. **Panics.** The library panics in the `Pwm`/`Uart`/`UartIrq`
   constructors and the USB bus (`expect`/`panic!`), against the README's
   "no panics in the library". Fixing it changes signatures; it needs a
   grant.
3. **AGENTS.md MSRV line.** It says "MSRV pinned in rust-toolchain.toml and
   proven by CI". False: the toolchain is `stable` and there is no CI.
4. **`Publishing.md` items:** no licence file in the package (copy
   `LICENSE.md` into `crates/unor4-hal/` or use `license-file`), and the
   docs.rs metadata (needs a `Cargo.toml` grant).
5. **Cart-pole.** It accepted the 5a/5b deviations and the R13 5.5 µs
   stop. Replies on (a) the count fix and (b) WFI wake were drafted in
   chat; the owner sends them. The current API works for them (D4/D5);
   the redesign changes their constructor.
6. **Unverified:** the tick waking WFI with `PRIMASK` set.

## Grants (AGENTS.md "Current grants")

- In force:
  - bench runs (board list, upload of the validation pair, validation);
  - the step generator design document (`StepGeneratorDesign.md`; docs
    only, no code).
- Every other grant has expired. Code work needs a new grant.

## How the owner wants to work (learned the hard way)

- **No decision box, ever.** Call `wrap_up` with `recap` only (AGENTS.md
  rule 10). Ask questions in plain chat, **one at a time**.
- **Don't ask what a stated principle already answers.** Once the owner
  said "builder with validators", apply it.
- **Never let a design go unreviewed.** Write agreements into the docs
  immediately. Get approval before code. Name a rejected substitution as
  one (rule 8).
- **Short replies, open issues first.** No editorializing. Research before
  claims; label anything unverified; cite sources.
- **Reproduce a reported problem first** (rule 7), and debug by
  observation (rule 9). Example: the miscount was found with a frozen-
  counter experiment, not theory.
- **Tool limits on this machine:**
  - The scratchpad, memory folder and session transcript are blocked.
  - Put temporary files under `target/` (git-ignored).
  - Large inline Python heredocs in Bash fail to parse; write a file under
    `target/` and run it.

## Bench and commands

R4 on COM3, R3 on COM4 (check with `arduino-cli board list`). Wires and
rules are in AGENTS.md. The R3 now has only `z` (wake) free among
lowercase commands.

```powershell
just clippy; just test                                   # host matrix (repo root)
# in crates/test-firmware:
just clippy
cargo objcopy --release --features usb,uart-irq --bin r4_validator -- -O binary r4_validator.bin
# repo root:
arduino-cli compile --fqbn arduino:avr:uno --build-path target/validation-r3-build arduino/validation/r3-validator
arduino-cli upload -p COM3 --fqbn arduino:renesas_uno:unor4wifi --input-file crates/test-firmware/r4_validator.bin
arduino-cli upload -p COM4 --fqbn arduino:avr:uno --input-file target/validation-r3-build/r3-validator.ino.hex --verify
cargo run --release -p validation -- all                 # or: -- slices S11
```

Use `$ErrorActionPreference = 'Stop'` so a failed build never uploads an
old image. Measured Slice 11 facts and every run are in the README, Slice
11 record.
