# Slice 10 research — data flash (`dflash.rs`)

> Planning notes assembled 2026-10-21, before any Slice 10 code. Verified
> facts are cited to the hardware manual, the datasheet, or — where the
> manual does not document the register at all — to the Renesas FSP CMSIS
> device header, with the evidence trail in §2. Cross-check data (FSP
> driver, Arduino core) is labeled as such. Open items are named
> explicitly. No code has been written.
>
> **Amended 2026-10-21 (second-opinion review):** an independent review
> of this document's claims (owner-run, against the reviewer's own
> knowledge of RA LP flash) produced five corrections, each verified
> against the sources on disk before acceptance: the once-per-erase
> programming rule (§5, §6 — supersedes the "program-only bits" API
> framing), explicit `DFLEN` init before any read (§5), the `FPMCR`
> read-mode step in the exit sequence (§5), blank-check instead of
> read-for-`FFh` for erased-state questions (§5.2 — the manual is
> internally split, see there), and `FISR` read-modify-write with the
> `SAS` bits preserved (§4). The §7 probe is the review's design.

Citations:

- **Manual Rev 1.10** — R01UH0887EJ0110 Rev.1.10, Sep 29 2023
  (`Research/Renesas RA4M1 User Manual - 1.10.md`). Primary for this
  slice; it supersedes Rev 1.00 for flash facts (Rev 1.10's revision
  history names the flash additions, see §3). Note 2026-10-21: the
  Rev 1.00 and older datasheet copies that earlier slices cite
  (`RA4M1-hardware-manual.md`, `r01ds0355ej0110-ra4m1.md`) have been
  replaced in `Research/` by the Rev 1.10 files — page numbers quoted
  in older slice research refer to those retired copies.
- **Datasheet** — R01DS0355EJ0110 Rev.1.10
  (`Research/Renesas RA4M1 Datasheet - 1.10.md`).
- **FSP device header** — `R7FA4M1AB.h`, installed board package 1.6.0:
  `C:\Users\tony\AppData\Local\Arduino15\packages\arduino\hardware\renesas_uno\1.6.0\variants\UNOWIFIR4\includes\ra\fsp\src\bsp\cmsis\Device\RENESAS\Include\R7FA4M1AB.h`
  (BSD-3-Clause; also upstreamable from ArduinoCore-renesas). Register
  *names/offsets* only — the sequences come from the FSP driver below and
  get proven on hardware.
- **FSP `r_flash_lp.c`** (github.com/renesas/fsp, `ra/fsp/src/r_flash_lp/`,
  BSD-3-Clause) — behavioral reference for the P/E recipes; never
  transcribed. This is the correct FSP module for this chip (§1).

## What the slice is

`dflash.rs`: the RA4M1's 8 KB data flash with `read(offset, buf)`,
`write(offset, buf)` (program-only bits), `erase()` — HalSpec §5.8.
Done-when (Plan.md): a boot counter survives power cycles — 100 power
cycles with zero corruption, plus an erase+rewrite verified byte-exact.

**Bench impact: none.** Research/slice-09-research.md §8 already records
that slice 10 needs no wires.

## 1. The decisive finding: this chip's flash is the LP type, version 3

The RA4M1's flash macro is the **low-power ("LP") type, version 3** —
not the HP/FACI-command architecture the same FSP repo also ships:

- `bsp_feature.h` (same installed package, `ra\fsp\src\bsp\mcu\ra4m1\`):
  `BSP_FEATURE_FLASH_LP_VERSION = 3`, `BSP_FEATURE_FLASH_HP_VERSION = 0`,
  `BSP_FEATURE_FLASH_LP_DF_BLOCK_SIZE = 0x400` (1 KB),
  `BSP_FEATURE_FLASH_LP_DF_WRITE_SIZE = 1` (byte).
- Manual Rev 1.10 §44.2 Table 44.1 confirms the hardware facts: data area
  8 KB, **8-bit programming units, 1-KB erasure units**; §44.2 Table 44.3
  gives read address `4010 0000h–4010 1FFFh` and P/E address
  `FE00 0000h–FE00 1FFFh` (offset between them: `+0xBDF00000`).
- Therefore the FSP module of record is **`r_flash_lp.c`** (FSP names the
  LP-v3 code path; `r_flash_hp.c`'s `R_FACI_HP`/`R_FACI_CMD` registers do
  not exist on this chip — verified against the device header, §2).

Consequences that must not be guessed:

- **`FENTRYR` is at offset 0x3FB2 — not `FENTRYR_MF4` at 0x3FB0.** The
  driver picks `FENTRYR_MF4` only when `BSP_FEATURE_FLASH_LP_VERSION == 4`
  (RA4M2/RA4M3 class); the RA4M1 is v3, so the v3 register applies. Both
  named registers exist in the device header's `R_FACI_LP` block; using
  the wrong one is the easiest way to ship a driver that works on the
  wrong silicon.
- **DF write unit is one byte** — the HalSpec "byte-granular write" is
  hardware-native; no 64-bit padding games like HP code flash.
- **v3 wait times:** mode-transition wait tMS high = 6 µs (v4 uses 16);
  the CF-path discharge (tDIS = 3 µs) is code-flash-only and never runs
  in this driver.
- **tDSTOP after enabling DF access = 5 µs in high-speed mode** — now
  manual-cited (Rev 1.10 §44.4.3, below), which resolves the earlier
  datasheet-vs-FSP discrepancy (FSP uses a conservative 6 µs constant).

## 2. Source-of-record situation (the PWPR precedent, twice over)

The flash registers are absent from every generated source:

- **Manual Rev 1.00**: chapter 44 documents the memory architecture but
  no sequencer registers (chapter ends at usage notes, no register
  section beyond the flash cache).
- **Manual Rev 1.10** (verified by grep over the full text): adds
  §44.4.3 `DFLCTL` plus four identity registers (FMIFRT, UIDRn, PNRn,
  MCUVER). The LP sequencer registers — `FENTRYR`, `FPMCR`, `FPR`,
  `FASR`, `FSARL/H`, `FEARL/H`, `FWBL0`, `FCR`, `FSTATR00/1/2`, `FISR`,
  `FRESETR` — are documented **nowhere in any revision**. (The `FCR`
  hits elsewhere in the manual are the SCI FIFO Control Register; the
  `FENTRYR` hits are standby-chapter side references. Neither is this
  peripheral.)
- **SVD** (`ArduinoCore-renesas/svd/R7FA4M1AB.svd`, downloaded and
  grepped): no FLASH/FACI/FLCN peripheral.
- **`ra4m1` PAC 0.2.0** (local registry source): no flash peripheral —
  a direct consequence of the SVD.

So `dflash.rs` defines the register block itself as raw `volatile`
access — HalSpec §4.5's sanctioned exception (the `PWPR` unlock
precedent), with every offset cited per §4 below. What Rev 1.10 changes
is how much needs the FSP citation:

| Fact | Source of record now |
|---|---|
| DF memory map, 1 KB blocks, byte programming, read/P/E addresses | Manual §44.2 Tables 44.1/44.3 |
| **`FLCN` block base `407E C000h`; `DFLCTL` at `+0x090`; `DFLEN` semantics; tDSTOP 5 µs (high-speed)** | **Manual Rev 1.10 §44.4.3** — the manual now independently confirms the FSP header's `R_FACI_LP_BASE = 0x407EC000` |
| Sequencer registers and the P/E recipes | FSP device header (offsets) + `r_flash_lp.c` (sequences) — **proven on hardware by the first read-back run** (§7) |
| 100,000 P/E cycles | Datasheet Table 2.72 (and Note 1 there: "programming the same address for several times as one erasure is not enabled. Overwriting is prohibited" — independent confirmation of the once-per-erase rule) |
| Erase/program/blank-check timing budgets | **Datasheet §2.15.2 Table 2.73 (high-speed mode, Rev 1.10): 1-KB erase max 286 ms / 237 ms (4 MHz / 32 MHz columns), 1-byte program max 463 µs / 387 µs, 1-KB blank check max 1872 µs / 512 µs.** Our FCLK (24 MHz) sits between the columns — the 4 MHz column is the conservative bound. FSP's `FLASH_LP_MAX_ERASE_DF_BLOCK_TIME_US` (504 ms) is that worst case plus ~1.76× margin; its comment's "299 ms" is the **middle-speed** table (Table 2.74), i.e. the comment cited the wrong operating mode. Driver timeouts: datasheet max, with the FSP margin retained as an explicit, cited constant. |

## 3. Manual facts this slice relies on (all Rev 1.10)

- §44.2 Table 44.1: data flash 8 KB; programming in 8-bit units; erasure
  in 1-KB blocks; erased state reads `FFh`.
- §44.2 Table 44.3: read `4010 0000h–4010 1FFFh`, P/E
  `FE00 0000h–FE00 1FFFh`, blocks 0–7.
- §44.4.3 `DFLCTL` (`FLCN.DFLCTL 407E C090h`): b0 `DFLEN`, data flash
  access enable; reset 0. **After setting `DFLEN`, wait the data flash
  stop recovery time (tDSTOP) before reading or entering P/E mode** —
  5 µs high-speed / 720 ns middle- or low-speed / 10 µs low-voltage. Our
  HAL runs high-speed (`OPCCR.OPCM = 0`), so **5 µs**.
- §44.13 self-programming: BGO — *code flash remains readable while data
  flash is programmed*, so the driver's polling loops may run from code
  flash (no RAM-copied inner loop needed for the sequencer wait itself;
  see §5.1 for the mode-transition caveat).
- §44.14.2: no special settings to read the data flash in normal mode
  (after `DFLEN` + tDSTOP); erased cells read `FFh`.
- §44.15.3: no area can be written twice without an erase first — this
  *is* the program-only write contract.
- §44.15.7/§44.15.9: no P/E in low-speed mode; do not change `DFLEN`
  during P/E; do not access the data flash during code-flash P/E (N/A —
  this driver never touches code flash).
- §44.15.4–5: reset during P/E is abnormal termination → the area must
  be erased again (motivates the recovery path, §4).

## 4. The register block (offsets from the FSP device header; base manual-cited)

`R_FACI_LP` base **`0x407EC000`** (manual Rev 1.10 §44.4.3 via
`FLCN.DFLCTL 407E C090h`; FSP `R_FACI_LP_BASE`). Offsets as used by
`r_flash_lp.c` on this chip:

| Register | Offset | Width | Role in DF operations |
|---|---|---|---|
| `DFLCTL` | 0x090 | 8 | DF access enable (`DFLEN`); manual-cited |
| `FPMCR` | 0x100 | 8 | Flash mode: DF P/E `0x10` (high-speed; `\|0x40` LVPE only outside high-speed), read `0x08` |
| `FASR` | 0x104 | 8 | Area select: `0` = user area (data flash), `1` = extra area (never used here) |
| `FSARL` / `FSARH` | 0x108 / 0x110 | 16 | Start address, **P/E space** for commands |
| `FCR` | 0x114 | 8 | Command: write `0x81`, erase `0x84`, blank check `0x83`, clear `0x00` |
| `FEARL` / `FEARH` | 0x118 / 0x120 | 16 | End address (erase / blank-check range) |
| `FRESETR` | 0x124 | 32 | Flash software reset (`1` then `0`) — error recovery |
| `FSTATR00` | 0x128 | 32 | b3 `BCERR0`: blank-check result; other bits CF-only |
| `FSTATR1` | 0x12C | 32 | b6 `FRDY`: command-ready flag (rises at completion, clears after `FCR` is cleared) |
| `FWBL0` | 0x130 | 32 | Write data (only the low 8 bits are used for DF) |
| `FPR` | 0x180 | 32 | Write-only `0xA5` unlock key, required before *each* `FPMCR` write |
| `FPSR` | 0x184 | 32 | Read-only b0 `PERR` "Protect Error Flag" — set when the `FPR`/`FPMCR` unlock ritual was broken; check after the triple |
| `FISR` | 0x1D8 | 32 | b5:0 `PCKA` flash-clock notification — `(FCLK_MHz − 1) & 0x1F`, writable only in P/E mode. **Read-modify-write:** b7:6 `SAS` (temporary boot swap) shares the register — a whole-register write clobbers startup-area selection |
| `FSTATR2` | 0x1F0 | 32 | Error flags: b4 `ILGLERR`; erase error `b0\|b4`; program error `b1\|b4` (FSP's masks `0x10/0x11/0x12`) |
| `FENTRYR` | 0x3FB2 | 16 | P/E mode entry: write `0xAA80` = DF P/E, `0xAA00` = read mode; poll `0x0080` on entry, `0x0000` on exit |

**Trap:** `FENTRYR_MF4` (0x3FB0) exists in the same block and is the
correct register only for LP **v4** parts — not this chip (§1).

**FCLK math:** `Clocks::fclk()` already returns 24 MHz (48/2,
`SCKDIVCR` FCK/2). FSP requires FCLK ≥ 4 MHz (exactly 1/2/3 MHz also
allowed) — satisfied. `FISR.PCKA = (24 − 1) & 0x1F = 23`, set once while
in DF P/E mode (the driver's `set_fisr` step).

## 5. The operating recipes (FSP `r_flash_lp.c` as behavioral reference)

- **Read:** plain memory-mapped loads from `0x40100000 + offset` — no
  registers, no P/E mode (manual §44.14.2, after §3's `DFLEN` + tDSTOP).
- **Init (once, before any DF access):** if `DFLCTL.DFLEN == 0`, set it
  and wait tDSTOP (5 µs). The manual puts tDSTOP *here* — after `DFLEN`,
  before any read or P/E entry (§44.4.3); `DFLEN` resets to 0, so a
  driver that skips this reads garbage until something else sets the
  bit. (FSP also repeats a tDSTOP delay after the `FENTRYR` write in
  `df_enter_pe_mode`; keep that too — it is the vendor-proven order.)
- **Enter DF P/E:** `FENTRYR = 0xAA80` → wait tDSTOP (5 µs) →
  `FPR = 0xA5; FPMCR = 0x10; FPMCR = ~0x10; FPMCR = 0x10` (the
  value/~value/value triple; FSP's `write_fpmcr`). The triple runs with
  interrupts masked (critical section) — hardening, not documented:
  the sequence must not be interleaved, and `FPSR.PERR` (0x184 bit 0,
  "Protect Error Flag") is the check that it wasn't. Check `PERR`
  after the triple; FSP itself only reads `FPMCR` back.
- **Write one byte:** `FASR = 0`; `FSARH/FSARL = P/E address`;
  `FWBL0 = byte`; `FCR = 0x81` → poll `FSTATR1.FRDY == 1` → `FCR = 0x00`
  → poll `FRDY == 0` → check `FSTATR2` program-error bits. **A byte may
  be programmed once per erase cycle** — §44.15.3 prohibits writing any
  (non-configuration) area twice without an erase in between, even to
  clear more bits. Targeting a non-blank byte is an API error; detecting
  "blank" is the blank-check command's job (§5.2), not a read-for-0xFF.
- **Erase a block:** `FASR = 0`; `FSAR`/`FEAR` = block-aligned P/E range
  (1 KB); `FCR = 0x84` → same FRDY poll pattern → check `FSTATR2`
  erase-error bits. `FEAR` takes the **inclusive last byte** of the
  range — `start + 0x3FF` for a block (FSP's `process_command`:
  `end = start + num_bytes − 1`). Timeout basis: datasheet Table 2.73
  worst case **286 ms/block** (high-speed, 4 MHz column — §2 table);
  FSP's 504 ms constant is that plus ~1.76× margin and is retained as
  the timeout, annotated with this citation.
- **Blank check:** `FSAR`/`FEAR` range; `FCR = 0x83` → FRDY poll →
  result in `FSTATR00.BCERR0`. This — not a read-for-`0xFF` — is the
  way to verify erasure and find free slots (§5.2).
- **Exit P/E:** `FPMCR` back to read mode via the same
  `FPR`/value/~value/value ritual with value `0x08` → wait tMS
  (6 µs, v3) → `FENTRYR = 0xAA00` → poll until `FENTRYR == 0`.
  (`pe_mode_exit` in `r_flash_lp.c` does exactly this; the discharge
  steps in it are code-flash-only and never run here.)
- **Recovery after timeout/error:** `FRESETR = 1; FRESETR = 0` while in
  P/E mode, then the exit sequence (manual §44.15.4/8: the area must be
  erased again after abnormal termination — the API returns an error and
  leaves the policy to the caller).
- **Timeouts:** scale the FSP way — worst-case µs × ICLK_MHz / 6 loop
  iterations. ICLK = 48 MHz via `Clocks::sysclk()`.

### 5.2 Erased-state semantics: the manual disagrees with itself

Two statements in Rev 1.10 pull in opposite directions:

- §44.2 Table 44.1: "Value after erasure: **FFh**" (data flash column),
  and §44.14.2 says erased cells read all 1s.
- §44.7 Table 44.8 (blank-check row): *"Results of reading from data
  flash memory to which nothing is written after erasure are **not
  guaranteed**, so use blank checking to confirm that writing to memory
  has not proceeded after erasure."*

The conservative reading governs the design: **never infer state from a
read of erased cells.** Erasure verification and free-slot detection use
the blank-check command (`FCR = 0x83` → `FSTATR00.BCERR0`); the
erase+rewrite byte-exact check compares only *programmed* bytes against
the expected pattern, and confirms erasure via blank check.

### 5.1 Resolved (owner, 2026-10-21): RAM execution during mode transitions

FSP places `write_fpmcr`, `pe_mode_exit`, `process_command`, and
`r_flash_lp_write` in a RAM section (`FLASH_PE_PLACE_IN_RAM_SECTION`) —
even in DF-only builds with bank programming disabled. Manual chapter 44
does not state the requirement in the text we have; BGO (§44.13.2) only
guarantees code-flash *reads* during DF *programming*, not during the
`FPMCR` mode transitions.

**Owner decision: mirror FSP — the mode-transition helpers run from RAM
(`#[link_section = ".ram_code"]`, section defined in `test-firmware`'s
linker setup).** Rationale: code flash is the runtime's foundation and is
not to be disturbed during mode switches; the vendor-proven configuration
is the safe default and can be relaxed later with evidence, not before.
The open question is closed; the linker section is part of the slice's
test-firmware work.

## 6. API shape for `dflash.rs` (HalSpec §5.8 + decisions for the owner)

- `DataFlash::new()`-style constructor with a once-only guard: **there is
  no PAC peripheral token to consume** (§2), so the driver needs its own
  `take()`-style claim (same critical-section pattern as
  `Peripherals::take`) so two callers can't race the sequencer.
- `read(offset, &mut buf)` — infallible modulo range check (§44.14.2,
  after the init step in §5). A read while any P/E mode is active is a
  type-state error: reads of the data flash during DF P/E are not a
  supported access.
- `write(offset, &buf)` — **each byte may be programmed at most once per
  erase cycle** (§44.15.3 prohibits overwriting without an erase, even
  to clear more bits — this supersedes the earlier "program-only bits"
  framing in HalSpec §5.8's wording, which stays as-is: program-only
  remains true, but *once* per erase). `write()` targets blank cells;
  blankness is established with the blank-check command, not by reading
  for `0xFF` (§5.2). The driver may verify before programming or let the
  sequencer's program-error flags catch it — decide at implementation
  and record the choice.
- `erase_block(block)` / `erase_all()` — explicit, block-scoped.
- Range guard: every operation hard-limited to
  `0x40100000–0x40101FFF` and mapped through the P/E offset
  `+0xBDF00000` internally. Code flash is unreachable through this API
  by construction — a stray write cannot brick the sketch (board-safety:
  data flash is not the boot image).
- Host-unit-testable math (given-when-then per AGENTS.md): offset/
  alignment validation, read→P/E address conversion, `PCKA` derivation,
  timeout scaling.
- No new crate feature — HalSpec lists `dflash.rs` as a core module.

## 7. Verification plan (what the board must show)

1. **Read-only probe (runs first; closes the FSP-sourced-facts OQ
   without writing any flash):**
   1. At boot, dump `DFLCTL` — shows whether `DFLEN` resets to 0 (the
      manual says it does) and whether the bootloader left it set (which
      would hide a missing init step on Arduino-firmware boots only).
   2. Dump `FENTRYR` (0x407EFFB2 — expect `0x0000`) and the word at
      0x407EFFB0 (`FENTRYR_MF4`) for comparison.
   3. Set `DFLEN`, wait tDSTOP, dump all 8 KB of data flash.
   4. Enter DF P/E mode; confirm `FENTRYR` reads `0x0080` and
      `FPSR.PERR` reads 0.
   5. Run one blank check on a block believed erased — non-destructive.
   6. Exit P/E mode (`FPMCR` → read mode, then `FENTRYR = 0xAA00`);
      confirm `FENTRYR` reads `0x0000` and a second 8 KB dump matches
      the first byte for byte.
   This exercises every register address and both mode transitions and
   settles the DFLEN-init and exit-sequence questions before a single
   byte is programmed.
2. **Erase+rewrite byte-exact** (done-when item 2): blank-check a
   scratch block, erase it, blank-check again (§5.2 — not a read-for-
   `FFh`), program a known pattern byte-by-byte, read back and compare;
   erase again and blank-check.
3. **Boot counter** (done-when item 1): append-log counter (§8) — each
   boot programs one fresh slot, zero erases; 100 power cycles with
   zero corruption.
4. **Regression:** every check lands in the validation pair (§8) so
   `validate all` re-proves it.

## 8. Validation-pair integration (Plan.md rules 2–3)

- **R4 requests** — two new letters in `r4_validator/main.rs` (free
  letters include `F`, `K`):
  - `K` — **read-only counter poll**: read the boot counter from the
    data flash (after the §5 init step), reply `K N`. Cheap by design —
    this is what gets polled every second; no P/E mode is entered.
  - `F` — flash self-test, run on demand: the §7.2 erase+rewrite
    round-trip at a scratch offset, replying byte-exact pass/fail.
  SCI2 TX discipline unchanged; the validator never transmits unless
  asked (contract intact — no boot beacon; owner decision 2026-10-21
  after the beacon design was rejected as protocol-breaking).
- **R3 check** — issue the `F` request, verify the reply (covers the
  erase+rewrite done-when, re-runnable by `validate all`).
- **PC check** (`validate S10`) — boot-count persistence across power
  cycles, **R3-polled (owner design, 2026-10-21)**: the R4 runs from the
  barrel jack with USB disconnected; the R3 (still on USB) runs a watch
  mode in which:
  1. **polling happens only while the S10 test is started** — a PC
     command to the R3 enters the watch mode; outside it the R3 behaves
     exactly as today (no polling, no R4 traffic);
  2. the R3 polls `K` at a fixed interval; a poll with **no reply = R4
     unpowered** (jack off) — recorded as a gap, polling continues;
  3. the **first successful reply after the test starts is the
     baseline** (the R4 is already running when the test begins) — not
     a boot event;
  4. each subsequent **no-reply → reply transition = one boot event**;
     its reply carries the boot counter as read from the data flash;
  5. consecutive boot events with strictly increasing counters prove
     retention across real power loss — the ~20-cycle done-when
     (Plan.md, amended 2026-10-21).
  The R3's USB serial is the witness path; the `validate` CLI can start
  the watch mode and record the run, or the owner watches `K n` lines in
  the serial monitor. While the R4 is unpowered, its dead TX hangs on
  the R3's RX input (harmless — input pin, no driving).
- **Wear-minimal counter pattern** (the slice's teaching point): the
  once-per-erase rule (§44.15.3) makes "clear one more bit per boot"
  illegal — instead the counter is an **append log**: each boot programs
  one new byte in a fresh slot of a scratch page; when the page fills,
  erase it and restart (logging the wrap). 100 boots consume 100 of the
  1024 slots with **zero erases** against the 100,000 P/E budget. Free
  slots are found with the blank-check command, not by reading for
  `0xFF` (§5.2). The counter lives in the validator binary, not the
  crate.
- **README record:** one row in the All-in-one check table; build/run
  commands unchanged.
- **Plan.md note:** the Slice 10 entry still names a `slice10_remember`
  binary — written before the 2026-10-20 validation-pair amendment. It
  needs the same owner amendment the other slices got when the slice
  starts.

## 9. Open questions (candidates for OQ-10 in HalSpec)

1. **LP-v3 sequencer registers have no manual documentation on this
   chip** (verified across Rev 1.00 and Rev 1.10). Source of record:
   FSP device header offsets + `r_flash_lp.c` sequences; closed by the
   §7.1 hardware read-back. The `FLCN` base and `DFLCTL` are
   manual-cited (Rev 1.10 §44.4.3).
2. **RAM execution during `FPMCR` mode transitions** — FSP places the
   helpers in RAM even for DF-only builds; the manual doesn't state why.
   — **Resolved 2026-10-21 (owner):** mirror FSP; the helpers run from
   RAM (§5.1).
3. **DF erase-time budget** — **Resolved 2026-10-21:** datasheet
   §2.15.2 Table 2.73 (high-speed): 1-KB erase max 286 ms (conservative
   4 MHz column; 237 ms at 32 MHz). FSP's 504 ms constant = that worst
   case + ~1.76× margin; its "299 ms" comment was the middle-speed
   table (2.74). Driver keeps the margin, annotated with this citation.
   Same tables also give 1-byte program max 463 µs and 1-KB blank check
   max 1872 µs, plus the ±3.5 % FCLK accuracy requirement (§2 table).
4. **Is the flash block module-stoppable?** — **Resolved 2026-10-21
   (lookup):** no MSTPCRB bit exists for flash; it is stoppable only via
   the dedicated `SYSTEM.FLSTOP` register (manual §10.2.11,
   `4001 E09Eh`, bit 0 `FLSTOP`, **reset value 0 = operating** —
   "Code flash and data flash memory operates"). Stopping is an
   explicit SRAM-executed low-power opt-in; the bootloader never does
   it, FSP never does it for flash operations, and neither does this
   driver. No action.

## 10. Evidence trail (everything checked 2026-10-21)

- `Research/RA4M1-hardware-manual.md` (Rev 1.00): chapter 44 read
  end-to-end; TOC verified to end at usage notes (no register section).
- `Research/Renesas RA4M1 User Manual - 1.10.md` (Rev 1.10): §44.4.3
  read; full-text grep for every sequencer register name (only SCI-FIFO
  `FCR` and standby-chapter `FENTRYR` side notes hit); revision history
  read (flash additions = DFLCTL + identity registers only).
- `ArduinoCore-renesas/svd/R7FA4M1AB.svd`: downloaded, grepped — no
  flash peripheral (`0x407EC000` is TSN in that SVD).
- `ra4m1` PAC 0.2.0 (cargo registry source): peripheral module list
  read — no flash module; `system.rs` has no `DFLCTL`.
- Installed board package 1.6.0: `bsp_feature.h` values quoted in §1;
  `R7FA4M1AB.h` `R_FACI_LP_Type` offsets quoted in §4.
- `renesas/fsp` `r_flash_lp.c` (master): DF enter/write/erase/
  blank-check/exit/reset sequences read; `r_flash_hp.c` read far enough
  to confirm it is the wrong module for this chip (its `R_FACI_HP`/
  `R_FACI_HP_CMD` blocks do not exist in the RA4M1 header).
- Arduino core behavioral cross-check: `libraries/EEPROM` +
  `libraries/BlockDevices/virtualEEPROM.cpp` (1.6.0 package) — the
  core's EEPROM is a log-structured page layer over the FSP flash API,
  confirming the spec's split: mechanism in the HAL, wear policy in the
  app layer.
- Second-opinion review (owner-run, 2026-10-21): five corrections,
  each re-verified locally before acceptance — §44.15.3 once-per-erase
  (Rev 1.10 line ~59446), §44.4.3 DFLEN reset 0 + tDSTOP scope,
  Table 44.8's "results of reading … not guaranteed" vs Table 44.1's
  "value after erasure FFh" (both quoted in §5.2), `pe_mode_exit`'s
  `FPMCR` read-mode step (`r_flash_lp.c`), and the `FISR`/`FPSR`
  bitfields (`R7FA4M1AB.h`: `PCKA[5:0]`+`SAS[7:6]`; `FPSR.PERR` at
  0x184). The review's read-only probe became §7.1.
