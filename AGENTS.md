# AGENTS.md — rules for AI agents working in unor4-hal

This is a **learning project with a public audience**: the owner learns
embedded Rust by implementing this HAL themselves, and other Arduino UNO R4
owners will run the result on their boards. Both facts are constraints — the
learning rules the process, the audience raises the quality bar.

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

## Library conventions (this repo's equivalent of the course's doc rules)

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
- Each slice's binary lands with its *Testing on hardware* README
  subsection in the same commit — build command, wiring, observable,
  verified status. The board proves the slice; the README records it.
- Unit tests follow given-when-then: name tests
  `given_<setup>_when_<action>_then_<expectation>` and comment the body
  with `// given` / `// when` / `// then` phases.
- No unlicensed or unaudited dependencies; HalSpec §1's audit rule applies
  to every new dependency.
