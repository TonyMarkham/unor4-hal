# Slice 6 research — UART SCI1 (the WiFi AT channel)

> **2026-10-20:** the per-slice binary named here (`slice06_at.rs`) and its R3 sketch were retired in favour of the validation pair (README, All-in-one validation; S6). Both restore from git `6649d4c`.

> Planning notes assembled 2026-10-19, before any Slice 6 code. Verified
> facts are distinguished from open lookups; every open item names where
> the answer comes from. No code has been written.

## What the slice is

`SCI1` on P501/P502, IRQ mode, talking to the ESP32-S3's AT server. Binary:
`slice06_at.rs` — send `AT\r\n`, print everything received for 30 s via the
slice-4 SCI9 monitor path. Observable: the ESP32-S3 answers. **No bench
wiring changes** — SCI1 is on-board; the bench stays as-is.

## Research session 2 (2026-10-19, same day) — L1 and L3 resolved

### L1 — `PSEL_SCI1` = 0x05 (cross-checked twice; manual citation still owed)

Two independent Renesas sources agree:

1. **FSP** (vendored in the core at
   `variants/UNOWIFIR4/includes/ra/fsp/inc/api/r_ioport_api.h`): the PFS
   `PSEL` groups are `IOPORT_PERIPHERAL_SCI0_2_4_6_8 = 0x04` and
   `IOPORT_PERIPHERAL_SCI1_3_5_7_9 = 0x05` — SCI1 sits in the 0x05 group.
2. **The Arduino core's decode** (`cores/arduino/Serial.cpp`, lines 186–187):
   a pin's `SCI_EVEN_CFG` flag selects the 0x04 group, `SCI_ODD_CFG` selects
   0x05. And `variants/UNOWIFIR4/pinmux.inc` marks **P501 = TXD1,
   `SCI_ODD_CFG`** and **P502 = RXD1, `SCI_ODD_CFG`**.

The chain closes against our own hardware-proven anchors: SCI2 (P302/P301,
`EVEN_CFG`) uses PSEL 0x04 — proven in Slice 3; SCI9 (P109/P110, `ODD_CFG`)
uses 0x05 — proven in Slice 4. Both match the same two sources. So
**PSEL_SCI1 = 0x05**, same value as SCI9.

Still owed per the verification-chain rule: the manual's own citation.
The hardware manual PDF (SparkFun mirror, 1436 pp.) has no ToUnicode maps,
so its pin-function tables could not be machine-extracted here. Owner
lookup: in the manual's pin function chapter (the per-pin function table),
read the `PSEL` column of the **P501 → TXD1** and **P502 → RXD1** rows and
confirm both show the PSEL-5 row. Record chapter/table in the code comment
alongside the value.

*Footnote (curiosity, no impact):* `pinmux.inc` marks P109's SCK1 as
`SCI_EVEN_CFG` while TXD9 is `SCI_ODD_CFG` — the even/odd flag is a PSEL
group selector, not strictly the SCI engine's parity. Both of our pins
(P501/P502) are flagged `ODD_CFG` in every source, so the choice is
unambiguous.

### L3 — Minima: P501/P502 are SWD-connector pins, not header D-pins

`variants/MINIMA/variant.cpp` (lines 52–53) maps P501/P502 as
"(23) TX on SWD connector" / "(24) RX on SWD connector" — named constants,
not D-numbers. They never appear in `gpio::Pins` on either board, so the
`sci1_tx`/`sci1_rx` fields in `Peripherals` need **no `wifi` feature gate**
(unlike `sci9_tx/rx`, which collide with D11/D12 on the Minima). Doc
comments should note the Minima SWD-connector exposure.

## Facts already verified (from the SVD/PAC + behavioral references)

1. **PAC layout — the one real design question, now answered.**
   `ra4m1::SCI1` does *not* deref to `sci2::RegisterBlock` like SCI9 does;
   it derefs to `sci0::RegisterBlock` (0x4007_0020, `pub use self::sci0 as
   sci1`). Every register the driver touches (`SMR`, `BRR`, `SCR`, `TDR`,
   `SSR`, `RDR`, `SCMR`, `SEMR`, `SIMR1-3`, `CDR`, `DCCR`, `SPTR`) is
   **byte-identical between the `sci0` and `sci2` PAC modules, at identical
   offsets** (all touched offsets are ≤ 0x1C; the blocks only diverge in
   FIFO/9-bit registers the driver never touches). So the generic driver
   works over SCI1 — see the design decision below.
2. **Module stop:** `MSTPCRB.MSTPB30` = "Serial Communication Interface 1
   Module Stop" (SVD). Same `PRCR` PRC1 unlock as SCI2/SCI9.
3. **IELSR events:** `Sci1Rxi=0x9E`, `Sci1Txi=0x9F`, `Sci1Tei=0xA0`,
   `Sci1Eri=0xA1` — already transcribed and unit-tested in Slice 5
   (`intc/event.rs`). No intc work needed; the dispatch table and the
   `ATTACHED` bitmask already support channel 2.
4. **ESP32 side** (behavioral reference, `uno-r4-wifi-usb-bridge`
   `UNOR4USBBridge.ino`): the stock ESP32-S3 firmware implements an
   **AT server via SudoMaker/chAT** on its GPIO5/6 at **115200 8N1**,
   running continuously. `AT` → `OK` is a real observable. The USB bridge
   path (SCI9 monitor) is an independent task — the monitor stays alive
   while we drive SCI1. Uploads are unaffected (reset/boot tricks go
   through USB CDC line-coding, not SCI1).
5. **Pins:** P501/P502 are not header pins on the WiFi board
   (Limitations.md, the phone-line table); bare `Pin<'5', 1, Input>`
   handles work via the PFS address math — same pattern as `sci9_tx/rx`.
   The core's `variants/UNOWIFIR4/pinmux.inc` cross-check confirms
   P501 = TXD1, P502 = RXD1 (SCI channel 1).

## Research session 3 (2026-10-19) — L1' and L2 closed from the manual

Source: the owner converted the hardware manual to Markdown
(`RA4M1-hardware-manual.md` in this folder, from
R01UH0887EJ0100 Rev.1.00, Oct 8 2019 — the same revision the existing
comments cite). Both citations below were read from it directly; they also
agree with every cross-check in session 2.

**L1' — `PSEL_SCI1 = 00101b` (0x05).** Manual §19 "I/O Ports", **Table 19.13
"Register settings for input/output pin function (PORT5)"** (manual
page 371):

- P501 column: `TXD1/MOSI1/SDA1` at `PSEL[4:0] = 00101b`
- P502 column: `RXD1/MISO1/SCL1` at `PSEL[4:0] = 00101b`

(Conversion note: Table 19.13 renders the value as "00101h" — an artifact
of the PDF→Markdown conversion; Tables 19.9 and 19.11 render the same row
as `00101b`. Either reading is 5 in the low bits of the PFS `PSEL` field.)

Consistency checks in the same table set: P212/P213 also carry RXD1/TXD1
at `00101b` (Table 19.9, PORT2), P401/P402 likewise (Table 19.11, PORT4)
— every pin that carries TXD1/RXD1 uses PSEL 0x05, and the known anchors
hold: SCI2's TXD2/RXD2 on P302/P301 at `00100b` = 0x04 (Slice 3 proven),
SCI9's TXD9/RXD9 on P109/P110 at `00101b` = 0x05 (Slice 4 proven).

**L2 — `MSTPCRB.MSTPB30`.** Manual §10 "Low Power Modes",
**§10.2.3 "Module Stop Control Register B (MSTPCRB)"** (manual page 173):
bit 30 `MSTPB30` — "Serial Communication Interface 1 Module Stop",
Target module: SCI1; 0 = cancel module-stop, 1 = enter module-stop.

**All library preconditions are now settled. Implementation is unblocked.**

## Lookup tasks remaining before coding

None. L1, L2, L3 are all resolved (sessions 2–3 above).

## Design decision to make (only one)

**How `SciChannel` accepts SCI1.** The trait currently requires
`Deref<Target = sci2::RegisterBlock>`. Two options:

- **(a) Pointer reinterpret:** `impl SciChannel for SCI1 { const PTR =
  ra4m1::SCI1::PTR as *const _; ... }` — minimal churn; the safety
  argument (identical offsets, byte-identical field definitions, verified
  above) goes in a comment with the SVD/manual citation. The
  `sci_channel.rs` doc comment already anticipates "SCI1 joins with
  Slice 6".
- **(b) Register-accessor trait:** generalize the driver over a small
  `SciRegs` trait. Cleaner abstraction, more churn across `uart.rs`/
  `uart_irq.rs`.

Recommendation: **(a)** — the layout identity is verified, it keeps the
Slice 3-proven bring-up sequence verbatim, and the comment can state the
evidence.

## Work items, in order

1. **Library** (one type per file where new types appear; mostly extending
   existing files):
   - `uart/sci_channel.rs`: `impl SciChannel for SCI1` (`CHANNEL_ID = 2`),
     update the module doc that currently says "the PAC aliases sci9 to
     sci2".
   - `uart/uart.rs`: `PSEL_SCI1 = 0x05` (verified: manual §19 Table 19.13,
     research sessions 2–3), `release_module_stop_sci1()` (manual §10.2.3
     `MSTPCRB.MSTPB30`), and a polling `Uart::sci1`
     constructor — the spec promises both modes per engine, and it is
     ~15 lines once `init_sci` is generic.
   - `uart/uart_irq.rs`: `UartIrq::sci1` constructor mirroring `sci9`.
   - `peripherals.rs`: `sci1`, `sci1_tx: Pin<'5',1,Input>`,
     `sci1_rx: Pin<'5',2,Input>` fields — **ungated** (L3: P501/P502 are
     SWD-connector pins on the Minima, never in `gpio::Pins`), with
     doc comments noting the AT-channel role and the Minima SWD
     exposure.
   - `uart/mod.rs`: doc updates + re-exports.
2. **Host tests** (given-when-then naming): nothing new is strictly
   required — baud calc, ring, and event values are covered — but consider
   one test pinning `CHANNEL_ID` uniqueness across the three engines if
   the pattern doesn't already.
3. **Binary `slice06_at.rs`:**
   - `static UART1: UartIrq<SCI1>` (RX 256 is plenty for a banner + OK),
     attach with `p.sci1`, `p.sci1_tx`, `p.sci1_rx`, 115200.
   - SCI9 **polling** monitor (`Uart::sci9`) for output — one IRQ engine
     keeps the surface minimal; the slice-4 monitor path stays usable.
   - Flow: banner over SCI9 → wait a moment for the ESP32 → send
     `AT\r\n` → print raw bytes received for 30 s (hex + ASCII; chAT
     responses contain `\r`) → repeat `AT` every 5 s → final report:
     bytes received, `overflowed()`, `errored()`.
   - Print raw bytes even if nothing human-readable comes back — whatever
     the ESP32 says is the diagnostic.
4. **Build + flash + run** (owner's commands, owner's hands): standard
   `cargo build --release` in `test-firmware`, upload per course
   procedure, then `arduino-cli monitor` on the R4 port and watch for
   `OK`.
5. **Record:** README slice-6 subsection ("Testing on hardware": build
   command, no wiring, observable, verified status/date); close the
   spec's "can the RA4M1 drive the AT channel" question in HalSpec.md (a
   resolved-OQ note, following the OQ-6 format); Plan.md slice-6 "Done
   when" gets its evidence line. CHANGELOG entry only at release.

## Research session 4 (2026-10-19) — hardware result: the board answers `OK`

Implemented per the work items above and run on the bench (R4 `COM3`,
R3 `COM4`):

- **Result:** the ESP32-S3's AT server answered exactly `OK\x0d\x0a`
  (4 bytes) to every `AT\r\n` — **9/9 witnessed rounds** (one complete
  6-round run, one partial 3-round capture; the partial run's own
  `total=24` report covers its unwitnessed rounds),
  `overflowed() = 0`, `errored() = 0`. The slice's observable is met;
  HalSpec OQ-7 records the answer.
- **Two run-infrastructure lessons, both fixed in the artifacts:**
  1. The ESP32 bridge's CDC gates output on **DTR** — a raw host
     `SerialPort` reader must set `DtrEnable = $true`; five silent
     captures were the reader, not the board. `arduino-cli monitor`
     asserts DTR and is unaffected.
  2. A one-shot gate lost a run to an unattached reader. The binary now
     **re-arms** (runs on every falling edge of the bench D3 line, waits
     for it to rise between runs) and the R3 sketch `slice06-gate`
     (removed from the tree 2026-10-20; restorable from git `6649d4c`)
     cycles the line (30 s high / 50 s low), so one flash serves many
     unattended runs. The firmware prints its banner and liveness dots
     *before* the gate, so a silent boot is distinguishable from a
     never-opened gate.
- **State of the files at the end of the session:** library (SCI1
  channel impl, `PSEL_SCI1`, module-stop, polling + IRQ constructors,
  `Peripherals` fields), `slice06_at.rs`, and the gate sketch are
  hardware-verified; README slice-6 section, HalSpec OQ-7, and this
  file record the evidence. `just test` green (4 check + 4 test
  configurations; 33 tests with `uart-irq`). Commit is the owner's.

- **Original risk — chAT's response format to bare `AT`:** resolved by
  observation: exactly `OK\r\n`, no command echo, no banner. The
  contingency plan below is retained for the record.

- If nothing arrives: the counters discriminate — `errored()` > 0
  suggests a baud/format mismatch; zero events at all points at PSEL/
  muxing (L1) or the AT server not running on the ESP32. Differential
  next step: power-cycle the board (fresh ESP32 boot) before theorizing.
