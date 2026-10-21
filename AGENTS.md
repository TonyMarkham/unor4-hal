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

Implement [HalSpec.md](HalSpec.md) milestone by milestone: an independent
(**no fork, no copied code**) HAL crate, `unor4-hal`, for the Arduino UNO R4
(Renesas RA4M1) — closing the gaps of the board's existing community
support: SCI1, SCI9, USB CDC, interrupt infrastructure, data flash.
Rationale and background: [Limitations.md](Limitations.md). Build order and
per-slice hardware verification: [Plan.md](Plan.md) — a slice is done when
the board shows it, not when it compiles.

- **Source of truth for requirements is HalSpec.md.** If implementation and
  spec disagree, the spec wins — and gets fixed immediately if the spec is
  the wrong one.
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
   documentation: AGENTS.md, HalSpec.md, Limitations.md, Plan.md, README.md,
   and CHANGELOG.md entries.
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

### Current grants (owner-granted exceptions to rules 1–2)

- 2026-10-20: Owner explicitly authorized rewriting
  `arduino/sketches/slice09-relay/slice09-relay.ino` to perform only
  single/double D4 pulses via serial commands, and subsequently authorized
  adding USB-trigger usage comments. Scope is this rewrite and its comments only;
  no build, test, flash, or install permission granted.

## Board-safety constraints (the test board is the same UNO R4 WiFi)

- The Arduino bootloader occupies flash `0x00000000–0x00003FFF`. Any
  test/example image must keep FLASH ORIGIN at `0x00004000` (see the course
  repo's memory.x for the working example).
- Never suggest Renesas `rfp-cli`, raw `bossac` erase/offset commands, or
  full-chip images to "clear" the board. Safe wipe = upload any valid sketch
  with `arduino-cli upload`.
- The USB port is shared with the ESP32-S3 bridge (uploads, monitor). Any
  API that flips the USB mux to the RA4M1 must be explicit and documented as
  disconnecting the bridge (HalSpec §5.5).

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
- R3 GND     <--> R4 GND
- R3 D4      <--> R4 RESET (remote-reset wire: the R3 pulses it LOW and
  releases to input — open-drain discipline only, never driven HIGH —
  so the agent can reset the R4, and double-pulse it into bootloader
  mode, without the owner's hands. Commands `!` = single reset, `@` =
  double reset, sent to the R3's `validation/r3-validator` sketch over its
  serial port. They reach the R3 only while the R4 is not driving SCI2 TX
  (README, Slice 9).)
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
  every earlier slice's wiring (Plan.md lists the slices; HalSpec lists
  the peripherals).
- **The proof pattern is a two-board pair.** The pair is:
  - R3 controller: `validation/r3-validator/r3-validator.ino`.
  - R4 responder: `crates/test-firmware/src/bin/r4_validator.rs`, the
    `r4_validator` binary.
  - Host script: `validation/validate.ps1`.

  Together they re-prove every slice on the bench. A new slice extends
  this pair:
  - a request in the R4 responder;
  - a check in the R3 controller;
  - a row in the host script, if only a PC can observe it.
- **R3-side code is Arduino sketches, and stays in the repo.** All R3 code
  is a typical Arduino `.ino` sketch (compiled with `arduino-cli`, never
  Rust). It lives in `validation/` so the hardware evidence stays
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
  separate `use` statement per item or per level. Held by convention and
  review: rustfmt's `imports_granularity` is nightly-only, so CI cannot
  enforce it.
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
  independently; don't recreate the problem. All dependencies are
  MIT/Apache-2.0 (license-audited 2026-10-18), so a single MIT file covers
  the crate.
- Feature gates follow HalSpec §3: `minima`/`wifi` mutually exclusive via
  `compile_error!`, `wifi` the default, `usb` opt-in.
- The README's quick example is `cargo check`-verified by the owner before
  every release.
- Each slice's checks land in the validation pair, in the same commit as
  the README record: an R4 request in `r4_validator.rs`, an R3 check, and
  a host-script row if only a PC can see it. The README's All-in-one
  validation subsection is updated in that commit: check table, build and
  run commands, wiring, witnessed run. The board proves the slice; the
  README records it.
- Unit tests follow given-when-then: name tests
  `given_<setup>_when_<action>_then_<expectation>` and comment the body
  with `// given` / `// when` / `// then` phases.
- No unlicensed or unaudited dependencies; HalSpec §1's audit rule applies
  to every new dependency.
