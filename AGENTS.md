# AGENTS.md — rules for AI agents working in unor4-hal

This is a **learning project with a public audience**: the owner learns
embedded Rust by implementing this HAL themselves, and other Arduino UNO R4
owners will run the result on their boards. Both facts are constraints — the
learning rules the process, the audience raises the quality bar.

## Terminology

Throughout AGENTS.md and the project docs:

- **R3** — Arduino UNO R3. The bench's stimulus/controller board.
- **R4** — Arduino UNO R4 WiFi. The target board
  (`arduino:renesas_uno:unor4wifi`); "R4" always means the WiFi variant.
- **UNO R4 Minima** — no hardware for it exists in this project; any
  Minima-specific claim is unverified until someone with the board runs it.

## Project goal

Implement an independent (**no fork, no copied code**) HAL crate,
`unor4-hal`, for the Arduino UNO R4 (Renesas RA4M1) — closing the gaps of
the board's existing community support: SCI1, SCI9, USB CDC, interrupt
infrastructure, data flash. A slice is done when the board shows it, not
when it compiles; the slice and validation records in `Validation.md`
are the witness log (moved from the README on 2026-11-04; the README is
the user guide).

- **Source of truth for hardware facts is the Renesas RA4M1 hardware manual**
  (and the RA4M1 SVD). Register addresses, pin tables, the SCI baud formula,
  the IELSR event map: cite the manual section in a comment or doc when it
  isn't obvious. Arduino's C++ core and the existing HAL are behavioral
  references and cross-check data only — never transcribed.
- **Reference sources** (behavioral references and cross-check data only,
  never transcribed):
  [ArduinoCore-renesas](https://github.com/arduino/ArduinoCore-renesas) —
  the Arduino core for the UNO R4; 
  [ArduinoCore-API](https://github.com/arduino/ArduinoCore-API) — the
  shared Arduino core API definitions.
- The crate is consumed by the owner's course repo (arduino-embedded-rust)
  as a git dependency until first publish; after publish, by version.

## Non-negotiable

1. **Never write project code or config for the owner.** This means never
   writing to `Cargo.toml`, `build.rs`, `src/*`, examples, tests, or CI
   workflows *as files*. Code inside documentation or chat is fine and
   expected — it may contain complete file contents; the owner applies
   them. The only files an agent may create/edit directly are
   documentation: AGENTS.md, README.md, Validation.md, and CHANGELOG.md
   entries.
   - **Override by explicit owner grant.** The owner may grant the agent
     permission to write specific project code or config. A grant is valid
     only if explicit and affirmative — never inferred from silence, a
     dismissed prompt, or lack of objection — and covers only the scope it
     names (files, crates, tasks, or a stated period). A grant in force is
     recorded in this file under **Current grants** so every session reads
     the same state; the owner amends or revokes it there. When no grant is
     in force, this rule applies as written.
2. **Never run build, test, flash, or install commands on the owner's
   behalf.** Guide, don't execute. Provide the exact command and the expected
   output; the owner runs it and reports back. `cargo` runs, hardware runs,
   HIL scripts — all the owner's.
3. **No editorializing.** No praise, no platitudes, no "lesson value"
   asides. State facts, fixes, and verification steps. Stop there.
4. **If you don't know, research. Guessing is the death of learning — and of
   a public library.** Users will run this crate on real boards; a wrong
   register address shipped to crates.io is a wrong register in someone
   else's project. Therefore:
   - Never state anything from memory that you cannot verify — commands,
     register names, bit fields, versions, crate APIs, URLs. Check the
     hardware manual, the SVD, or the source first.
   - Prefer giving the owner a one-line verification command over asserting
     an unverified value.
   - "I don't know — here's how to find out" is always an acceptable answer.
   - Anything unverified is labeled unverified, explicitly.
5. **Always read the file on disk before instructing the owner to edit it.**
   Never give edit instructions based on what a file "should" contain.
6. **No git chores performed by agents.** The owner commits, tags, and
   releases; agents never do.
7. **Issue reports from the owner are ground truth. Never second-guess
   the human.** When the owner reports a problem — an error message, a
   broken build, unexpected IDE behavior — the protocol is:
   - **Reproduce their symptom first**: their file, their command,
     their environment. Never start from a hypothesis about what they
     "probably" did, and never branch off into an unrelated area the
     report did not mention.
   - **A report the agent cannot reproduce is a datum about the
     agent's blind spot, not an error by the owner.** If results
     differ, say so as a puzzle — "these two facts conflict, here is
     the experiment that separates them" — never as a rebuttal
     ("that shouldn't happen", "you shouldn't have needed X").
   - **Reproduction before explanation, explanation before fixes.** No
     verdict until the reported symptom has been observed.
   - **Owner corrections are applied immediately**: a corrected file
     name, command, or context becomes the working premise at once.
8. **Substitution requires permission.** Whenever the agent determines
   that it wants to supersede, delay, or modify the owner's request for
   its own reasons — caution, risk judgment, disagreement, quality
   concerns — the following all apply:
   - The agent must **STOP and request permission for that specific
     substitution before acting on it.** The owner's request is the
     default course of action; the agent's alternative is not. Stating
     the concern in the same reply as the completed or withheld action
     does not satisfy this rule.
   - **The substitution must be labeled as one.** The request must say,
     in effect, "I am not executing your request as given unless you
     approve X." Disguising a substitution as a clarifying question, a
     menu of options, or a status report does not satisfy this rule.
   - **Permission must be explicit and affirmative.** Silence, a
     dismissed prompt, frustration, or any reply that is not an
     explicit approval of the substitution means the original request
     is executed as given, immediately. Implicit consent is never
     consent; acting on an inference of consent is a violation.
   - **One request.** The agent may ask permission for a substitution
     once per instruction. If it is not granted, the agent executes the
     original request as given. Repeatedly withholding execution while
     re-raising the concern is a violation.
9. **Debug by differential observation, not hypothesis enumeration.** When
   the board or a build misbehaves:
   - **Read before theorizing.** Any state that can be observed —
     registers, buffers, configuration — is evidence. A model of state
     that has not been read (what the bootloader "left behind", what a
     prior session "probably" changed) is a hypothesis, and is never
     treated as ground truth.
   - **When a working reference exists on the same silicon** (the Arduino
     core, FSP, a vendor example), the first debugging experiment is to
     dump and diff its observable state against ours — not to theorize
     about behavioral differences.
   - **Prefer experiments that discriminate.** Choose the observation that
     eliminates many hypotheses at once over the test that falsifies one.
     If several experiments in a row have failed to narrow the field, stop
     generating theories and run the broadest available observation.
   - **A documented decisive step is executed first.** If a handoff or
     prior session names the observation that would settle the matter,
     that observation runs before any other experiment.
10. **Never use the interactive question/prompt tool.** Agents must never
    invoke OpenCode's question feature (or any interactive prompt that
    blocks on the user) — not for clarifications, not for menus of
    options, not for permission requests. Questions go into the reply as
    plain text and the session ends its turn there; the owner answers in
    their own words on their own time.
    - **This includes the Claude desktop app's next-step / decision box.**
      When a turn closes with `wrap_up`, pass `recap` only. Never pass
      `ask`, `actions` or `blocked`. `AskUserQuestion` is banned as well.
      The owner rejects that interface outright. Re-read this rule after
      every context compaction (broken again 2026-11-04 right after one).
11. **Known defects are reported plainly, first, and completely.**
    - A known defect is anything the agent knows is wrong, missing or
      unproven: code, docs, tests, checks (including what the checks
      cannot catch), the bench, or the validation record.
    - Asked "is X fixed?" or "is there a defect?", the first word of the
      reply is YES or NO. It counts every known defect, not the narrowest
      reading of the question. Splitting hairs ("the docs are fine, the
      checks aren't") is a lie.
    - A defect is reported in the reply where it is found, with its fix,
      and whether that fix needs a grant. It is never buried after a
      success or left for the owner to dig out with questions.
    - A defect the agent caused is stated as "I caused it".
    - Every reply that closes a task ends with **Known open:** and the
      full list, or **Known open: none**. Leaving an item off because it
      is embarrassing, out of scope, minor, or was found by a non-standard
      tool is hiding it.

## Current grants

- **2026-11-11 — one commit by the agent (amends rule 6; expired on
  completion).** Owner, given in chat: "commit, no byline". Scope: one
  commit of the working tree on the current branch, with no
  `Co-Authored-By` line. No push, tag or release.

- **2026-11-11 — every public validation/debug item out of the public
  API, written by the agent (expired on completion, same day: 33/33 over
  three sessions, recorded in `Validation.md`).** Owner, given
  in chat: "Find ALL pub validation and fix, STOP stopping short", after
  `UartIrq::txi_events` and `StepGenerator::phase_counts` were found
  public. Scope: find every public item in `crates/unor4-hal` that exists
  only for testing, validation or debugging, and take it out of the
  public API; the validation pair and host tests that use them, moved to
  another way of observing the same thing; the host matrix, firmware
  builds and bench runs that prove it; the docs (README, crate README,
  CHANGELOG, `Validation.md`). `Cargo.toml` only for the debug-only
  `usb-trace` feature lines (HAL and test-firmware), which "fix" requires;
  no other manifest changes, no `rust-toolchain.toml`, no git chores.

- **2026-11-11 — Publishing.md's two open items, written by the agent.**
  Owner,
  given in chat: "You can fix issues 1 and 2, right", for Publishing.md's
  "Before 0.2.0" table. Expired on completion, same day: both fixed,
  33/33 over three sessions, recorded in `Validation.md`. Scope:
  - **1:** take `StepGenerator::dtc_latency_probe` and `dtc_delay_probe`
    out of the public API (`crates/unor4-hal/src/step`), with the
    validation pair's manual DTC probe (`r4_validator`'s `X` request, the
    R3 part 2 `x`/`y` commands) that calls them;
  - **2:** audit the transitive dependency licences and word the README's
    and this file's licence claims to match;
  - the host matrix, firmware builds and bench re-run that check them,
    and the docs (README, CHANGELOG, `Validation.md`, `Publishing.md`).

  Excludes `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-11 — `Publishing.md` update (expired on completion, same
  day).** Owner, given in chat: "update Publishing.md", after the licence
  file, the docs.rs table, the release checks and the CHANGELOG landed.
  Scope: `Publishing.md` only, brought up to the repo's state. Amended,
  owner: "If it is no longer an issue GET RID OF IT!": the "Before 0.2.0"
  table lists open problems only.

- **2026-11-11 — release-readiness runs by the agent (expired on
  completion, same day: all passed, 33/33 over three sessions, recorded
  in `Validation.md`).** Owner,
  given in chat: "run the validations to prove it's ready", after the
  crate licence file and the docs.rs table landed for 0.2.0. Scope: the
  publish checks in `Publishing.md` (`cargo package --list`, the
  `usb,uart-irq` doc build, `cargo publish --dry-run --allow-dirty`,
  never a real publish), the host matrix (`just check`, `just clippy`,
  `just test`), the firmware checks and `r4_validator` image, and the
  three `validate all` bench sessions per `Validation.md`. No code or
  config changes, no git chores.

- **2026-11-11 — docs.rs metadata in the crate manifest (expired on
  completion, same day: the table landed in
  `crates/unor4-hal/Cargo.toml`; the doc build that checks it is the
  owner's).** Owner, given in chat: "can you handle issue 3 too", meaning
  Publishing.md's item 3. Scope: a `[package.metadata.docs.rs]` table in
  `crates/unor4-hal/Cargo.toml` (`features = ["usb", "uart-irq"]`,
  `targets = []`, keys checked against <https://docs.rs/about/metadata>
  the same day), and the CHANGELOG entry. No other manifest changes, no
  cargo runs, no git chores.

- **2026-11-11 — crate licence file and crate README (expired on
  completion, same day: `crates/unor4-hal/LICENSE.md` byte-identical to
  the root file, the crate README's License section updated, CHANGELOG
  Fixed entry; the `cargo package --list` check is the owner's).** Owner, given in
  chat: "ok, fix the crate license and readme", after being told the
  `.crate` ships no licence text (`license = "MIT"` names the licence,
  the README only links to it). Scope: a copy of the root `LICENSE.md`
  in `crates/unor4-hal/` so `cargo package` includes it, and the crate
  README's License section. Excludes `Cargo.toml` /
  `rust-toolchain.toml`, cargo runs and git chores.

- **2026-11-04 — the wake timer, as agreed with cart-pole, written by the
  agent (expired on completion, same day: the record landed in
  `Validation.md`, 33/33 over three sessions).** Owner, given in chat: "Imple the feature
  as you and cart-pole agreed on". The agreement (this session's design,
  cart-pole's three replies):
  - a one-shot wake timer on a GPT channel that stops itself through a
    claimed ELC input; `start(Duration)` with the prescaler picked per
    wait, `expired()` from the overflow flag (works with a masked sleep),
    `cancel()`, `is_armed()`; `start` and `cancel` clear the old expiry;
  - ELC inputs claimable by type through `Peripherals`, the step
    generator claiming B, C and D, and `Elc::link` taking a claimed input;
  - the ENA lead fix (never short by a tick-timer count).

  Scope: `crates/unor4-hal` (a wake module, `elc`, `peripherals`, `step`,
  `intc` as needed), host tests and compile-fail cases, the validation
  pair, the host matrix, firmware builds, bench runs, and the docs
  (README, crate README, CHANGELOG, `Validation.md`). The step builders
  keep their `u32` nanoseconds. Excludes `Cargo.toml` /
  `rust-toolchain.toml` and git chores.

- **2026-11-04 — R3 part 3, cancel race, delay waits (expired on
  completion, same day: all 32 checks over three sessions, recorded in
  `Validation.md`).** Owner, given in chat: "Is part 2 is nearly full, add
  part 3. NOT ROCKET SCIENCE!", after "It seems like you continue to stop
  short" on the open items. Scope: the validation pair (the R3 sketch's
  third part, the CLI, the R4 cancel race and delay probe, the delay
  comments), bench runs, and the docs. Excludes `Cargo.toml` /
  `rust-toolchain.toml` and git chores.

- **2026-11-04 — bench checks for the never-run behaviours (expired on
  completion, same day: all six landed in `Validation.md`, 32/32 over both
  sessions, with the two HAL defects they found fixed; no public API
  change, owner: "These should be validating the HAL, not exposing
  unintended features to the public api").** Owner, given in chat: "YES, validate the gaps",
  answering "Do you approve building bench checks for those six?". The
  six: `cancel_stop_at` while the count match fires; a DIR set-up over
  40 µs through `queue`; a sleep over 66 ms at a steady rate, then a
  `queue`; `StepGenerator::free`; STEP on a timer other than GPT320; a
  switch with an external pull-up. Scope: the validation pair
  (`r4_validator`, both `r3-validator` parts, `crates/validation` and its
  tests), HAL fixes in `crates/unor4-hal` if a check finds a defect, the
  host matrix, firmware builds, bench uploads and runs, and the docs
  (`Validation.md`, READMEs, CHANGELOG). Excludes `Cargo.toml` /
  `rust-toolchain.toml` and git chores.

- **2026-11-04 — fix the known gaps (expired on completion, same day:
  S9 retries the native-port open, session 1 17/17 and session 2 13/13;
  the board-run lists updated with the cart-pole rig's report).** Owner, given in chat: "The scope
  was to fix R3 and its fullness, STOP being deceptive about your gaps
  and fix them", after the R3 split's report listed S9 failing and the
  stale "not run on the board" lists. Scope: S9's native-port open in
  `crates/validation` (and its tests), the bench re-run that proves it,
  and the docs that list what has and has not run on a board (README,
  crate README, `Validation.md`, CHANGELOG). Excludes `Cargo.toml` /
  `rust-toolchain.toml` and git chores.

- **2026-11-04 — R3 validator in parts (expired on completion, same day:
  the record landed in `Validation.md`; session 1 16/17 with S9 failing
  on the host's port open, session 2 13/13).** Owner,
  given in chat: "If the R3 is full, break the tests into parts and load
  those parts as diff test sessions", after the R3 sketch reached 32 220
  of 32 256 bytes. Scope: `arduino/validation/r3-validator/` split into
  parts that each fit, `crates/validation` (and its tests) to run a
  session per part, the R4 side only if the split needs it, the bench
  builds, uploads and runs of every part, and `Validation.md`, this
  file's bench section, CHANGELOG. Excludes `Cargo.toml` /
  `rust-toolchain.toml` and git chores.

- **2026-11-04 — docs build in `just check` (expired on completion, same
  day: `just check` passes with `just doc`).** Owner, given in chat: "WHAT DO
  YOU THNINK I WANT!", then "WHY ARE YOU BEING A LAZY ASSHOLE!", after
  being told that `cargo check` misses broken doc links and that closing
  the gap needs a `cargo doc` run at `-D warnings` in the root
  `justfile`'s `check`. Scope: the root `justfile` and this file's
  justfiles section, and the `just check` run that proves it. Excludes
  `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-04 — broken doc link in `run` (expired on completion, same
  day: `cargo doc` at `-D warnings` passes in all six feature sets).** Owner, given in chat: "YES,
  tech debt is bad", answering "Do you grant me fixing that broken link in
  `run`'s doc comment?" (`step_generator.rs` linked the private
  `first_cycle`, failing `cargo doc` at `-D warnings`). Scope: that doc
  comment, and the `cargo doc` runs that check it. No code changes, no
  `Cargo.toml` / `rust-toolchain.toml`, no git chores.

- **2026-11-04 — `poll_tick` timing wording (expired on completion,
  same day: README section "Sleeping through ticks", the step module and
  `poll_tick` docs, 39 doctests pass).** Owner, given in chat: "do
  it", answering "Do you grant me changing the `poll_tick` timing wording
  in the README and in the step module's doc comments, with no code
  changes?", after cart-pole asked whether `poll_tick` must run every
  2 ms at a steady rate. Scope: the README and doc comments in
  `crates/unor4-hal/src/step` only: no code, signature or behaviour
  changes. Includes the host doctest and doc runs that check them.
  Excludes `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-04 — interrupt wake-ups instead of polling, written by the
  agent (expired on completion, same day: the record landed in
  `Validation.md`, `validate all` 28/28).** Owner, given in chat: "YES, I HATE
  polling, fix these", answering "Do you grant me building both features
  now: the switch wake-up and the stop at an exact pulse count, with their
  host tests, bench checks and docs?" (cart-pole's firmware sleeps in
  `WFI` and must not poll). Scope:
  - a switch whose activation edge wakes the CPU, keeping what
    `activated()` reports (`crates/unor4-hal/src/gpio`, `intc`);
  - a step generator stop at an exact hardware pulse count, with an
    interrupt there to wake the CPU (`crates/unor4-hal/src/step`, `elc`,
    `intc`);
  - their host tests and compile-fail cases, the harness outside the repo,
    the validation pair and bench runs, the host matrix and firmware
    builds, the README, CHANGELOG, crate README and `Validation.md`.

  Excludes `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-04 — cart-pole dogfooding fixes, written by the agent
  (expired on completion, same day: the record landed in `Validation.md`,
  `validate all` 26/26).** Owner, given in chat: "ABSOLUETLE fix the issues,
  then respond to cart-pole with your fixes". The issues are the three
  HAL problems listed from cart-pole's dogfooding report:
  - `run` makes the first pulse after the minimum HIGH time and ignores
    the DIR set-up time;
  - starting from stopped or idle (`queue`), a DIR set-up longer than
    about 40 µs is not waited for;
  - `StepGenerator::is_enabled` takes `&mut self`.

  Scope: `crates/unor4-hal/src/step`, its host tests, the validation
  pair, host and firmware builds and bench runs, the README, CHANGELOG
  and `Validation.md`. Not covered: reading owned pin levels and a stop
  at an exact pulse count (features), `Cargo.toml` /
  `rust-toolchain.toml`, and git chores.

- **2026-10-21 — comment-only reference cleanup (expired on completion).**
  Owner grant, given in chat: "fix all docs and code comments" after the
  spec/research docs (`HalSpec.md`, `Limitations.md`, `Plan.md`,
  `Research/slice-06…10-research.md`) were removed. Scope: comment-only
  edits across all crates' `src/**` files, `pins.csv`, and manifest
  comments — no logic, signature, or behavior changes. The pass was
  completed the same day; no grant in force from it.
- **2026-11-03 — bench runs by the agent (in force; amends rule 2).**
  Owner grant, given in chat: "You can, I allow it", answering the proposal
  to let the agent run board listing, uploads and tests. Scope:
  `arduino-cli board list`; building and uploading the validation pair
  (`r4_validator` to the R4, `r3-validator` to the R3) with
  `Validation.md`'s commands; running the validation checks. Excludes erase/bootloader
  tools and full-chip images (Board-safety constraints below), and does not
  cover writing project code (rule 1 still applies). In force until the
  owner amends or revokes it here.
- **2026-11-03 — new-wire sanity check (expired on completion, same day:
  `S11 wire`, 17/17 run recorded in the README).** Owner
  grant, given in chat: "Yes, add a test for the new wire, just a sanity
  check for now". Scope: adding a check of the R3 D8 ↔ R4 D4 wire to the
  validation pair — `crates/test-firmware/src/bin/r4_validator/`,
  `arduino/validation/r3-validator/`, and `crates/validation` — plus its
  README record. No HAL (`crates/unor4-hal`) changes; the real STEP-pin
  check waits for Slice 11.

- **2026-11-03 — Slice 11, written by the agent (expired on completion,
  2026-11-04: Slice 11's README record landed).**
  Owner grant, given in chat: "I approve you write it", answering whether
  the agent writes Slice 11 (the cart-pole step generator) or guides the
  owner. Scope: the Slice 11 code and its proof — new and changed files in
  `crates/unor4-hal` for the step generator, ELC, DTC and 2 ms tick (plus
  the peripherals/pin wiring and host tests they need), the validation pair
  (`r4_validator`, `r3-validator`, `crates/validation`), the plan doc
  `Slice11Plan.md` (owner: "Write a discrete Slice 11 plan doc in the repo
  root"), the README slice record, and the CHANGELOG entry; and the host build, clippy and
  test runs (`just check`, `just clippy`, `just test`, firmware builds)
  needed to write it. Excludes `Cargo.toml`/`rust-toolchain.toml` changes
  and git chores (rule 6). Ends when Slice 11's README record lands, or
  when the owner amends or revokes it here.

- **2026-11-03 — MSRV and version bump (expired on completion).** Owner
  grant, given in chat: "Raise the min rust version to what it MUST be, we
  will make this v 0.2.0", then "make the claim a known good version, or
  reasonable assumption" (no old toolchain installed). Scope: the root
  `Cargo.toml`'s `version` and `rust-version`, and the CHANGELOG entry.

- **2026-11-04 — step generator design document (in force until the
  owner approves or rejects it).** Owner instruction, given in chat:
  "DOCUMENT THIS BEFORE YOU BEGIN!" for a step generator with
  configurable STEP pins (16/32-bit timers, known-supported pins only) and
  DIR on any GPIO. Scope: `StepGeneratorDesign.md` in the repo root, a
  documentation file. No code: implementation needs a separate grant
  after the owner approves the design.

- **2026-11-04 — step generator POC, outside the repo (in force until
  the owner amends or revokes it here).** Owner grant, given in chat:
  "Create a completely detatched workspace and build the POC", then "in a
  temp dir, just a POC not to pollute this repo". Scope: a standalone
  Cargo workspace in a temp directory (not in this repo, no dependency on
  `unor4-hal`) proving the compile-time API of `StepGeneratorDesign.md`
  (experiments V1, V3, V5, V6), plus its `cargo check`/`build`/`test`
  runs. No repo code or config changes, no git chores. Extended
  2026-11-04, owner: "create a new POC adjascent to this one", to a second
  POC (`D:\temp\stepgen-poc2`) with the pin capability table.

- **2026-11-04 — step generator builder in the HAL, written by the agent
  (in force until its record lands in `Validation.md`, or the owner
  amends or revokes it here).** Owner, given in chat, on finding the
  builder missing after
  the pin model: "THE ENTIRE IMPETUS FOR THE POIC WAS TO MAKE A FUCKING
  BUILDER WITH REAL VALIDATOR AND `cargo check` PRE_COMPILE CHECLKS",
  completing the "Do it" grant's step 4 ("the step generator on top, as
  designed"); widened the same day, owner "EXACTLY!" to the full set: a
  validating builder per part (`PulPinBuilder`, `DirPinBuilder`,
  `CounterBuilder`, `TickBuilder`, a cross-checking `StepSetup`) and the
  generator generalised to any timer, either output, any DIR pin, 16-bit
  counters and 16-bit STEP. Widened again the same day, owner: "We should
  have an ENA builder in the step module", then "The builder needs to be
  generic, the use in the implementing firmware is specific to the stepper
  driver": an `EnaPinBuilder` (any pin, the enabling level, the lead time
  from enable to the first DIR change or pulse) as an optional part of
  `StepSetup`. Scope: `StepGeneratorDesign.md` §4.3/§9 in
  `crates/unor4-hal` (`step`, the GPT facts in `pwm`, `elc` events),
  replacing `StepGenerator::d4`, the internals it needs, host tests including compile-fail cases, the validation pair, the
  host matrix, firmware builds and bench runs, the README record,
  CHANGELOG, crate README and the design doc. Excludes `Cargo.toml` /
  `rust-toolchain.toml` changes and git chores.

- **2026-11-04 — builder-shaped setup and `with_peripherals` (in force
  until it lands, or the owner amends or revokes it here).** Owner, given
  in chat: "FIX YOUR BULLSHIT!", answering "Is that the shape you want?".
  The proposal shown:
  - **Setup:** `StepGeneratorBuilder::new().with_pul(PUL).with_dir(DIR)
    .with_ena(ENA).with_counter(COUNTER).with_tick(TICK).build()`, a named
    `const` that cross-checks the parts.
  - **Run time:** `StepGenerator::new(STEPPER).with_peripherals(p)
    .build()?`, taking exactly the hardware the setup names and giving back
    the rest, with double use still a `cargo check` error.
  - **Switches:** the same shape.

  Scope:
  - `crates/unor4-hal`: `step`, `gpio` (switches, pins), `peripherals`
    (generated per-resource used/unused types), and the drivers'
    constructors only where the new `Peripherals` needs it;
  - the validation pair;
  - host tests, doc examples, the README guide, CHANGELOG, the harness
    outside the repo;
  - host and firmware builds and bench runs.

  Excludes `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-04 — README rewrite and `Validation.md` (in force until the
  rewrite lands, or the owner amends or revokes it here).** Owner, given
  in chat: "the current README is SHIT! It should NOT be slice based, it is
  a readme not a history lesson", then "COMPLETELY RE-WRITE IT", answering
  "Do you approve moving the slice records to a new `Validation.md` so I
  can rewrite `README.md` as a user guide?". Scope:
  - `Validation.md` in the repo root holds the slice records, unchanged:
    the hardware witness log, the bench wiring and the `validate`
    commands.
  - `README.md` is rewritten as a user guide, with its examples compiled
    as doctests (a `cfg(doctest)` include in `crates/unor4-hal/src/lib.rs`).
  - This file's references to the README's slice records point to
    `Validation.md`.
  - The crate README links to the guide.
  - Includes the host doctest/clippy runs that check it. Excludes
    `Cargo.toml` / `rust-toolchain.toml` and git chores.

- **2026-11-04 — `SwitchPinBuilder` in the HAL, written by the agent (in
  force until its record lands in `Validation.md`, or the owner amends or
  revokes it here).** Owner, given in chat: "Do it", answering "Do you grant me
  writing `SwitchPinBuilder`, its host tests, harness cases and docs?",
  after agreeing the shape ("SwitchPinBuilder is better", following "TOO
  specific!" on a step-coupled limit-switch design). Scope: a generic
  switch input in `crates/unor4-hal/src/gpio` (and the `intc` hooks its
  activation edge needs): `SwitchPinBuilder` with `with_pin` (header
  pins), a required contact type (`No`/`Nc`), a required pull, an optional
  IRQ digital filter, and the run-time switch (`is_active`, activation
  edge); its host tests and compile-fail cases, the harness outside the
  repo, the host matrix and firmware builds, the CHANGELOG, crate README
  and README record. Limit behaviour stays in firmware. Excludes
  `Cargo.toml` / `rust-toolchain.toml` changes and git chores.

- **2026-11-04 — header pin model in the HAL, written by the agent
  (expired on completion, same day: README record landed, `validate all`
  24/24).** Owner grant, given in chat: "Do it. You make it sound like this
  new design is better accross the board and even smaller", answering the
  proposal to move the HAL onto POC 2's pin model (`D:\temp\stepgen-poc2`):
  `Pin<ID, MODE>` with Arduino header types (`D0`–`D13`, `A0`–`A5`), one
  pin table with hard-coded capabilities (manual Tables 19.5–19.12), and
  drivers validated by trait bounds. Scope, in order: gpio and the pin
  table; ADC and PWM; UART, `UartIrq` and USB; the step module and
  `Peripherals` on the new pins; their host tests; the validation pair
  (`r4_validator`, `r3-validator`, `crates/validation`) as needed; the
  host matrix, firmware builds and bench re-proof (`validate all`); the
  README record, CHANGELOG and crate README. Excludes `Cargo.toml` /
  `rust-toolchain.toml` changes and git chores (rule 6). The step
  generator's own redesign (`StepGeneratorDesign.md`) is the next step on
  top of this, not part of it.

- **2026-11-04 — Slice 11 follow-ups, written by the agent (expired on
  completion, same day: both landed in the README record, 24/24).** Owner grant, given in chat: "Can you do 1 and 2?",
  answering the proposal of (1) a stop-boundary experiment for the
  one-edge miscount plus the `stop()` fix if confirmed, and (2) an opt-in
  tick wake-up (the 2 ms tick waking the CPU from WFI) for cart-pole. Scope:
  `crates/unor4-hal` `step` (and the `intc` hooks the wake-up needs), the
  validation pair (`r4_validator`, `r3-validator`, `crates/validation`),
  their host tests, the README record, CHANGELOG, `Slice11Plan.md`, and
  the host build/clippy/test and firmware builds to write them. Excludes
  `Cargo.toml`/`rust-toolchain.toml` changes and git chores (rule 6). Ends
  when both land in the README record, or when the owner amends or
  revokes it here.

- **2026-11-04 — publishing guide (expired on completion).** Owner grant,
  given in chat: "write a helper doc for me to use to use cargo to publish
  this and include a dryrun". Scope: `Publishing.md` in the repo root, a
  documentation file outside the AGENTS/README/CHANGELOG list.

- **2026-11-04 — repo-rule fixes by the agent (expired on completion, same
  day: host matrix, docs in four feature sets and a 23/23 board run).**
  Owner grant, given in chat: "Fix ANY issue you find, not just your's",
  after reporting multiple types in single files. Scope: fixing violations
  of this file's conventions (one type per file, one aggregated `use`
  tree, no globs, test naming, docs) in any crate's `src/**`, the
  validation sketches and the justfiles, plus the host clippy/test and
  firmware clippy runs that check them. Structure and comment changes
  only — no logic or behaviour changes, no `Cargo.toml` /
  `rust-toolchain.toml` edits, no git chores.

## Board-safety constraints (the test board is the same UNO R4 WiFi)

- The Arduino bootloader occupies flash `0x00000000–0x00003FFF`. Any
  test/example image must keep FLASH ORIGIN at `0x00004000` (see the course
  repo's memory.x for the working example).
- Never suggest Renesas `rfp-cli`, raw `bossac` erase/offset commands, or
  full-chip images to "clear" the board. Safe wipe = upload any valid sketch
  with `arduino-cli upload`.
- The USB port is shared with the ESP32-S3 bridge (uploads, monitor). Any
  API that flips the USB mux to the RA4M1 must be explicit and documented as
  disconnecting the bridge.

## Test bench (hardware available to agents)

Two boards are connected for testing: the UNO R4 WiFi (test board) and a
UNO R3 (stimulus/controller), wired together as follows.

- Run `arduino-cli board list` to get their addresses accurately — never
  assume a COM number. The R4's entry matches FQBN
  `arduino:renesas_uno:unor4wifi`; the R3's matches `arduino:avr:uno`.
- Set `$PORT` to the port of the R4, `$uartPort` to the port of the R3.
- R3 D0 (TX) <--> R4 D1 (RX)
- R3 D1 (RX) <--> R4 D0 (TX)
- R3 D3      <--> R4 D3
- R3 D2 (INT0) <--> R4 D6 (GPT PWM out, slice 7 measurement rig)
- R3 D5 (PWM out) <--> R4 A0 (ADC in, slice 8 measurement rig)
- R3 D8 (ICP1) <--> R4 D4 (GPT320 STEP out, slice 11 step-generator rig;
  added 2026-11-03: Timer1 input capture timestamps every falling edge, so
  the R3 counts pulses exactly and a missed edge shows as a doubled interval)
- R3 D7      <--> R4 D5 (DIR, slice 11 step 5b; added 2026-11-03: the R3
  reads the step generator's DIR level at each STEP edge)
- R3 GND     <--> R4 GND
- R3 D4      <--> R4 RESET (remote-reset wire: the R3 pulses it LOW and
  releases to input — open-drain discipline only, never driven HIGH —
  so the agent can reset the R4, and double-pulse it into bootloader
  mode, without the owner's hands. Commands `!` = single reset, `@` =
  double reset, sent to the R3's `arduino/validation/r3-validator` sketch over its
  serial port. They reach the R3 only while the R4 is not driving SCI2 TX
  (`Validation.md`, Slice 9).)
  - **This wire only works while the R3 is powered on. If the R3 is
    powered off, its D4 clamps the R4's RESET line — the RESET pin
    connection MUST be removed whenever the R3 is unpowered.**
- **The R4 can be forced into bootloader mode over USB.** Briefly
  opening the R4's serial port at 1200 baud (8N1, no data — the baud
  rate itself is the signal) makes the ESP32-S3 bridge double-pulse the
  RA4M1's RESET into bootloader hold. `arduino-cli upload` performs
  this touch automatically before flashing, so the flash path does not
  depend on the D4 wire for bootloader entry — the wire's role is to
  recover the bridge when a crashed/hung R4 firmware holds the USB mux
  (proven: touch → SAM-BA handshake → flash on bridge firmware 0.6.0).
- R4 D2 — free/open (owner, 2026-10-20). The push button that was on
  R4 D2 / R4 GND is no longer part of the bench; the R3 is the button
  (see D3 below).

D0/D1 crossed form the SCI2 UART channel between the boards. D3 straight
through is shared by the validation pair, and is input with pull-up on
both boards except while one side drives it for a test it asked for:
- **Slice 2 input:** the R3 drives it low, then high.
- **Slice 2 button:** the R3 "presses" it the way a switch to GND would:
  driven LOW, then released to input, never driven HIGH.
- **Slice 1 timing:** the R4 drives a square wave.

The retired per-slice programs used D3 as a test-gate line.

While the R4's SCI2 TX driver is active it can corrupt the R3's
bootloader sync, and it blocks PC commands to the R3. `r4_validator`
releases TX whenever it is idle, so the R3 can be uploaded while it runs
(done repeatedly 2026-10-20). With any other R4 image, put the R4 in
bootloader first.

- **The bench is extensible — and the agent proposes extensions.** When a
  future slice would benefit from wiring the bench does not have (an extra
  UART pair, a stimulus line, a trigger, an analog source), the agent
  recommends the addition as part of that slice's plan: pins on both
  boards, purpose, and the automation it enables. The owner does the
  physical wiring. Never assume a connection exists that is not listed
  above.
- **Wires never move.** Re-wiring an existing connection to different
  physical pins is a **no-no** — the bench grows additively and is never
  re-patched between slices. Changing what an already-wired pin is used
  for is entirely possible and allowed; only its physical placement is
  fixed.
- **The bench is planned through Slice 10.** When proposing wiring
  additions, the agent checks them against the pins every remaining slice
  needs, so that the bench as wired for Slice 10 remains compatible with
  every earlier slice's wiring.
- **The proof pattern is a two-board pair.** The pair is:
  - R3 controller: `arduino/validation/r3-validator/r3-validator.ino`,
    built as three parts (`R3_PART=1`: Slices 1–10 and the host-check
    commands; `R3_PART=2`: Slice 11's first checks; `R3_PART=3`: Slice
    11's later checks and the delay probe) because one image no longer
    fits the R3's 32 KB. Each part is uploaded for its own `validate all`
    session; a full validation is all three sessions (`Validation.md`). A new
    R3 check goes in the part it belongs to, and a part that fills up
    splits again rather than losing strings or checks.
  - R4 responder: `crates/test-firmware/src/bin/r4_validator/main.rs`, the
    `r4_validator` binary.
  - Host checks: the `validate` CLI (`crates/validation`), for the slices
    only a PC can observe.

  Together they re-prove every slice on the bench. A new slice extends
  this pair:
  - a request in the R4 responder;
  - a check in the R3 controller;
  - a check in the `validate` CLI, if only a PC can observe it.
- **R3-side code is Arduino sketches, and stays in the repo.** All R3 code
  is a typical Arduino `.ino` sketch (compiled with `arduino-cli`, never
  Rust). It lives in `arduino/validation/` so the hardware evidence stays
  reproducible from the repo alone. The per-slice sketches that used to be
  in `arduino/sketches/` were removed on 2026-10-20:
  - Slices 3, 5, 6, 7 and 8: restore with
    `git restore --source 6649d4c -- arduino/sketches/<name>`.
  - The Slice 9 sketches and the reset/request-response utilities were
    never committed and no longer exist.

## Library conventions (this repo's equivalent of the course's doc rules)

- **One type per file.** Every `pub` struct/enum lives in its own file,
  with the free functions and constants that exist only for it; `mod.rs`
  carries the module docs and re-exports (`gpio/`, `uart/` show the
  pattern). No multi-type grab-bag modules.
- **One aggregated `use` tree per file — for all crates, not just this
  one.** Every path from the same root is one import: internal
  (`crate::…`), workspace (`unor4_hal::…`), and external
  (`embedded_hal::…`, `ra4m1::…` included) — e.g.
  `use crate::{clock::Clocks, gpio::{Input, Pin}, pwm::Pwm};`, never a
  separate `use` statement per item or per level. `super::`/`self::`
  roots are not used: internal imports start at `crate::`. Held by
  convention and review: rustfmt's `imports_granularity` is nightly-only,
  so CI cannot enforce it.
- **No glob imports.** `use foo::*;` is banned in all crates' code, test
  or firmware: a glob hides which names a file actually depends on,
  silently changes meaning when the source crate adds items, and can
  defeat the exactly-once/`deny(missing_docs)` hygiene above. Name what
  you use — explicitly or inside the aggregated tree.
- `#![deny(missing_docs)]` — every public item documented; both board pin
  maps shown in feature-gated pin docs (`d2: WiFi=P104, Minima=P105` style).
- Examples compile in CI; doc examples marked `no_run` where they need
  hardware.
- `CHANGELOG.md` entry per release; semantic versioning; MSRV pinned in
  `rust-toolchain.toml` and proven by CI.
- `LICENSE.md` (MIT) exists **before the first code commit** — the license
  ambiguity of the existing crate is the reason this repo exists
  independently; don't recreate the problem. Every crate compiled into
  the library or the firmware is available under MIT (direct
  dependencies audited 2026-10-18, transitive 2026-11-11), so a single
  MIT file covers the crate. Two exceptions, both outside the compiled
  code (README, Dependency licences): `unicode-ident`'s Unicode-3.0
  (build time only, through `cortex-m`'s procedural macros) and the
  unpublished `validate` host tool's `serialport` (MPL-2.0,
  owner-approved).
- Feature gates: `minima`/`wifi` mutually exclusive via
  `compile_error!`, `wifi` the default, `usb` opt-in.
- The README is the user guide. Its `rust,no_run` examples are compiled
  as doc tests by `just test` (a `cfg(doctest)` include in
  `crates/unor4-hal/src/lib.rs`); a new driver gets its README section and
  example in the same commit. The owner `cargo check`s the getting-started
  firmware before every release.
- Each slice's checks land in the validation pair, in the same commit as
  its record in `Validation.md`: an R4 request in the `r4_validator`
  image, an R3 check, and a `validate`-CLI check if only a PC can see it.
  `Validation.md`'s All-in-one validation subsection is updated in that
  commit: check table, build and run commands, wiring, witnessed run. The
  board proves the slice; `Validation.md` records it.
- Unit tests follow given-when-then: name tests
  `given_<setup>_when_<action>_then_<expectation>` and comment the body
  with `// given` / `// when` / `// then` phases.
- No unlicensed or unaudited dependencies; every new dependency gets a
  license audit (MIT/Apache-2.0, or an owner-approved exception), recorded
  in the README's **Dependency licences** table — not in manifest comments
  (owner, 2026-11-03: those details belong in the README, not source).

## justfiles (developer workflow)

- **Root `justfile`:** `just check`, `just clippy`, `just test` run the
  host matrix — `unor4-hal` in six feature combinations (wifi/minima ×
  base / +uart-irq / +usb,uart-irq) plus `crates/validation`. `check` and
  `clippy` run at `-D warnings`; `test` runs the check matrix first, so
  the feature matrix is defined once. `check` ends with `just doc`:
  `cargo doc` at `-D warnings` over the same matrix, because broken doc
  links are rustdoc lints that `cargo check` never reports.
- **`crates/test-firmware/justfile`:** `just build`, `just build-minima`,
  `just check`, `just clippy` (thumb target, both deployable maps, release,
  `-D warnings`), and `just image` (objcopy to `target/r4_validator.bin`
  for `Validation.md`'s upload flow). It exists because firmware commands must
  run from that directory — its `.cargo/config.toml` pins the thumb
  target; from the repo root they silently build for the host instead.
- **Every justfile starts with `[windows] set shell := ["pwsh.exe",
  "-NoLogo", "-Command"]`** — without it, just falls back to `sh`, which
  does not exist on this machine, and every recipe fails.
- **No `flash`/`upload` recipes.** Flashing is a board-safety operation
  (see Board-safety constraints above): it stays a manual
  `arduino-cli upload` per `Validation.md`, never a one-word recipe.
- `just` picks the nearest justfile walking up from the invocation
  directory, so firmware recipes shadow the root ones inside
  `crates/test-firmware/`. The nested file has no `test` recipe on
  purpose: thumbv7em cannot run test harnesses — host tests live in the
  root `just test`.
