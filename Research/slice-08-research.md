# Slice 8 research — ADC (ADC140)

> Planning notes assembled 2026-10-19, before any Slice 8 code. Verified
> facts are cited to the hardware manual, the datasheet, or the PAC;
> cross-check data (Arduino core, Arduino reference) is labeled as such.
> Open items are named explicitly. No code has been written.

Manual citations refer to **R01UH0887EJ0100 Rev.1.00** (the PDF in
`Research/`); datasheet citations to **R01DS0355EJ0110 Rev.1.10**
(`Research/r01ds0355ej0110-ra4m1.md`). Same revision caveats as slice 7:
if a later revision is quoted later, re-map page numbers.

## What the slice is

`adc.rs`: ADC140 driver for A0–A5, single-shot blocking reads
`read(pin) -> Result<u16, Error>` with configurable resolution (12-bit
default, 14-bit available). Binary: `slice08_adc.rs` — prints A0 as raw +
millivolts at 4 Hz through the slice-4 SCI9 monitor path. Done-when
(Plan.md): readings within datasheet tolerance at both rails (GND, 5 V),
values move live as the input changes.

**Spec discrepancies found and fixed in HalSpec.md (2026-10-19, before
any code):**

1. HalSpec §6's trait matrix said "ADC | adc OneShot". **embedded-hal
   1.0.0 has no `adc` module** — docs.rs shows only `delay`, `digital`,
   `i2c`, `pwm`, `spi` (verified 2026-10-19); `embedded-hal-nb` 1.0.0
   only `serial`, `spi`. The ADC traits died with 0.2 and were never
   stabilized for 1.0. Same class of error as slice 7's `Pwm` →
   `SetDutyCycle` finding. HalSpec §5.7 and §6 amended: the driver
   exposes its own `read()` API (the UART module already sets this
   precedent — its trait surface is `embedded-io`, the rest is its own).
2. HalSpec §5.7's "embedded-hal `adc` traits" wording removed with the
   same edit.

## 1. The ADC140 peripheral (manual §35)

- **One unit, 25 analog channels**: AN000–AN014, AN016–AN025
  (§35.1 Table 35.1). Successive approximation.
- **Resolution: 14-bit or 12-bit**, selected per unit by
  `ADCER.ADPRC[1:0]` (§35.2.11): `00b` = 12-bit (**reset value**), `11b`
  = 14-bit; `01b`/`10b` prohibited. Only writable while
  `ADCSR.ADST = 0`. Data format: `ADCER.ADRFMT` flush-right (reset) /
  flush-left; result registers are 16-bit read-only.
- **Clocks:** peripheral module clock PCLKB and conversion clock
  PCLKC (ADCLK), ratio PCLKB:PCLKC ∈ {1:1, 2:1, 4:1, 8:1, 1:2, 1:4} set
  by `SCKDIVCR.PCKB[2:0]`/`PCKC[2:0]` (§35.1 Table 35.1 Note 1; max
  PCLKB 32 MHz, max ADCLK 64 MHz per that note; datasheet Table 2.43
  gives ADCLK 1–48 MHz at AVCC0 2.7–5.5 V, 64 MHz at 4.5–5.5 V).
  **Our config** (`clock.rs` SCKDIVCR_48MHZ): PCLKB = 24 MHz, PCLKC =
  48 MHz → ratio **1:2**. This matters for the sampling-state lower
  limit (§4 below).
- **Reference voltage** (§35.6, `ADHVREFCNT` §35.2.31): high side
  AVCC0 / VREFH0 / internal 1.45 V via `HVSEL[1:0]`; low side AVSS0 /
  VREFL0 via `LVSEL`. **Reset value 0x00 = AVCC0/AVSS0** — the default
  needs no writes. Selecting the internal reference has a dedicated
  discharge procedure (§35.7) — out of slice scope; `HVSEL` stays at
  reset.
- **Operating modes** (§35.3.1): single scan (convert selected channels
  once), continuous scan, group scan (A/B). **Slice 8 uses single scan
  with one channel.**
- **Triggers** (§35.2.12 `ADSTRGR`): software (`ADCSR.ADST`), ELC
  synchronous, ADTRG0 asynchronous. **The software trigger is enabled
  regardless of `TRGE`/`EXTRG`/`TRSA`** (§35.2.12 TRSA description) —
  no trigger setup is needed at all for slice 8.
- **Scan end:** in single scan mode, `ADST` **remains 1 during the
  conversion and is automatically cleared to 0 when all selected
  channels complete**; then the ADC enters a wait state (§35.3.2.1 step
  4). An `ADC140_ADI` interrupt fires without register setting
  (§35.3.2.1 step 3) — polling `ADST` is the slice-8 completion signal;
  the interrupt path is future work (IELSR 029h, Table 11.x).
- **Module stop:** the ADC140 is **initially stopped after reset**;
  after releasing module stop, **wait ≥ 1 µs before starting a
  conversion** (§35.8.5). Bit: `MSTPCRD.MSTPD16` = "14-Bit A/D Converter
  Module" (manual §10, MSTPCRD, p.175; confirmed line "b16 MSTPD16
  14-Bit A/D Converter Module"). `MSTPCRD` sits at MSTP-block offset
  0x0C (A/B/C/D at 0x00/04/08/0C) — the GPT releases in
  `pwm/gpt_channel.rs` already use this address; `PRCR.PRC1` unlock as
  in the SCI releases.
- **ADHSC** (`ADCSR.ADHSC`, §35.2.3): 0 = high-speed conversion mode,
  1 = low-current mode. Rewriting it has a sleep procedure
  (`ADHVREFCNT.ADSLP`, §35.8.8) — the driver sets it once at init while
  stopped, never rewrites it during operation.
- **Sampling states** (`ADSSTRn.SST[7:0]`, §35.2.14): 5–255 ADCLK
  states per channel; **reset value 0Dh (13 states)**. Lower limit
  depends on the PCLKB:PCLKC ratio: **≥ 5 states at 1:1/2:1/4:1/8:1,
  ≥ 6 states at 1:2 or 1:4** → our 1:2 config needs ≥ 6. Per-channel
  registers ADSSTR00–14 map to AN000–AN014; **AN016–AN025 share the
  single ADSSTRL** (Table 35.8) — so A4/A5's sampling time is set in
  ADSSTRL, not ADSSTR21/22.
- **Self-diagnosis** (`ADCER.DIAGM`, `ADRD`): converts an internal 0 V /
  VREF/2 / VREF voltage once per scan; useful health check, optional
  for the slice (§35.2.11). Not required by the done-when.

## 2. Register map (manual §35.2; PAC addresses agree)

ADC140 base **0x4005_C000** (manual §41.1 "Address space" line
"ADC140 | 14-bit A/D Converter | 0x4005 C000"; PAC `ADC140::PTR =
0x4005_c000`). Registers relevant to slice 8:

| Offset | Register | Reset | Role in slice 8 |
|---|---|---|---|
| 0x00 | ADCSR | 0x0000 | ADST (b15) start/auto-clear, ADCS[14:13] = 00 single scan, ADHSC (b10) |
| 0x04 | ADANSA0 | 0 | AN000–AN014 scan select (A0–A3 live here) |
| 0x06 | ADANSA1 | 0 | AN016–AN025 scan select (A4/A5 live here) |
| 0x0E | ADCER | 0x0000 | ADPRC[2:1] resolution, ADRFMT (b15), ACE (b5) |
| 0x10 | ADSTRGR | 0 | TRSA — irrelevant for software trigger |
| 0x7A | ADDISCR | 0 | disconnection assist — keep at reset |
| 0x8A | ADHVREFCNT | 0x00 | HVSEL = 00 → AVCC0/AVSS0 reference (keep) |
| 0x20..0x3E | ADDR0–ADDR14 | 0 | results, AN000–AN014 (ADDR9 = A0) |
| 0x4A, 0x4C | ADDR21, ADDR22 | 0 | results, AN021 (A4), AN022 (A5) |
| 0xDD | ADSSTRL | 0x0D | sampling states for AN016–AN025 (A4/A5) |
| 0xE0..0xEE | ADSSTR00–14 | 0x0D | sampling states AN000–AN014 (00/01/02 = A1–A3, 09 = A0) |

## 3. Pin map — header A0–A5 → AN channels

The analog channels are **identical on both boards** (pins.csv rows
a0–a5: P014, P000, P001, P002, P101, P100 on both maps — no feature
gating needed in the ADC module; the pin docs still show both boards
per the library convention).

| Header | Pin | AN channel | Result register | Select bit | Sampling register |
|---|---|---|---|---|---|
| A0 | P014 | **AN009** | ADDR9 | ADANSA0.ANSA09 | ADSSTR09 |
| A1 | P000 | **AN000** | ADDR0 | ADANSA0.ANSA00 | ADSSTR00 |
| A2 | P001 | **AN001** | ADDR1 | ADANSA0.ANSA01 | ADSSTR01 |
| A3 | P002 | **AN002** | ADDR2 | ADANSA0.ANSA02 | ADSSTR02 |
| A4 | P101 | **AN021** | ADDR21 | ADANSA1.ANSA21 | ADSSTRL |
| A5 | P100 | **AN022** | ADDR22 | ADANSA1.ANSA22 | ADSSTRL |

Sources: manual §19 Table 19.5/19.6 ASEL rows (P000=AN000, P001=AN001,
P002=AN002, P014=AN009/DA0, P100=AN022/CMPIN0, P101=AN021/CMPREF0);
datasheet pin-function tables (P101 "AN021 CMPREF0", P100 "AN022
CMPIN0", P014 "AN009 DA0"). **Cross-checked entry-for-entry against the
Arduino core's `variants/UNOWIFIR4/pinmux.inc`** (behavioral cross-check
only): `P000→CHANNEL_0`, `P001→CHANNEL_1`, `P002→CHANNEL_2`,
`P014→CHANNEL_9`, `P101→CHANNEL_21`, `P100→CHANNEL_22` — every entry
matches. This table becomes the golden constant in `adc/` with a unit
test (same pattern as `src/tests/pwm.rs`).

Notes:
- **A0 = P014 is also DA0** (the DAC output pin). Analog input on it is
  a normal pin function (ASEL); the core reads it like any other
  (`pinmux.inc` gives it CHANNEL_9). DAC output stays out of scope
  (HalSpec §5.7).
- Port setting: `into_analog()` in `gpio/pin.rs` already exists (ASEL=1,
  output stage off) — proven pattern from the pin typestate; the ADC
  constructors will consume `Pin<PORT, PIN, Analog>` like the UART
  driver consumes configured pins.
- §35.8.12: "high precision channels" (AN000–AN004 region) — don't use
  the pin as digital I/O while it is an ADC input; the typestate enforces
  this (the `Analog` state has no digital methods).

## 4. Reference voltage and the 0–5 V range

- Datasheet: analog input range **0 V to VREFH0-selected reference**;
  with `HVSEL = 00b` the high reference is **AVCC0**; VREFH0 must be
  ≤ AVCC0; AVCC0 range 1.6–5.5 V (datasheet §2, A/D characteristics
  conditions lines: "VCC = AVCC0 = 1.6 to 5.5 V, VREFH0 = 2.7 V to
  AVCC0").
- The accuracy tables' conditions assume **VCC = AVCC0**. On the UNO R4
  the digital VCC is 3.3 V (slice 7 measured 3.3 V GPIO output) while
  the analog rail is 5 V — Arduino's own documentation lists the R4 as
  a 5 V board with A0–A5 measuring 0–5 V, 14-bit max (reference table,
  cross-check only). So the board runs the analog domain at 5 V
  regardless; **the rails test in the binary is the proof**: A0 → 5 V
  header reads ~full scale, A0 → 3V3 header reads ~⅔ of it, A0 → GND
  reads ~0. That experiment separates "AVCC0 = 5 V" from any other
  wiring in one round (OQ-A1).
- mV conversion: `mv = raw × 5000 / (2^res − 1)` (32-bit math, no
  float). The 5000 is nominal; OQ-A1's two-rail check bounds the actual
  value.

## 5. Conversion time (manual §35.3.6 Table 35.10) — host-testable calculator

`t_SCAN = t_D + t_DIS + t_DIAG + t_SPL + t_SAM + t_ED`. For slice 8
(software trigger, no disconnection assist, no self-diagnosis, single
channel):

- `t_D = 2 PCLKB + 4 ADCLK`
- `t_SPL = SST × ADCLK + 0.5 ADCLK` (SST = ADSSTRn value)
- `t_SAM` = 31.5 ADCLK (12-bit, high-speed), 40.5 (12-bit, low-current),
  37.5 (14-bit, high-speed), 46.5 (14-bit, low-current)
- `t_ED = 1 PCLKB + 3 ADCLK`

Worked values at PCLKB = 24 MHz, ADCLK = 48 MHz, SST = 13 (reset) —
integer nanoseconds with per-term flooring, exactly as the host tests
assert them (`src/tests/adc.rs`):

| Config | t_D | t_SPL | t_SAM | t_ED | total |
|---|---|---|---|---|---|
| 12-bit, high-speed | 166 | 281 | 656 | 103 | **1206 ns** |
| 14-bit, high-speed | 166 | 281 | 781 | 103 | **1331 ns** |
| 14-bit, low-current | 166 | 281 | 968 | 103 | **1518 ns** |

All far below the 4 Hz print cadence; the poll loop is the practical
cost. The calculator mirrors the PWM `calc_period`/UART `calc_baud`
shape: pure function, host unit tests against this table. It also
enforces the SST lower limit (≥ 6 states at our 1:2 ratio, §35.2.14) if
the API exposes sampling-time configuration.

Datasheet accuracy context (Table 2.42/2.43, high-speed mode,
AVCC0 = 2.7–5.5 V): conversion 0.80 µs (high-precision channel,
SST = 0Dh, PCLKC = 64 MHz); **absolute accuracy ±5 LSB typ / ±20 LSB max
at 14-bit high-precision channels, ±32 LSB max elsewhere**; quantization
±0.5 LSB; DNL/INL ±4/±12 LSB. "High-precision channel" = the AN000–AN004
region (datasheet §2 conditions; §35.8.12 "do not use PORT0 as general
I/O when using the high precision channels") — A1–A3 sit there; A0 (AN009)
and A4/A5 (AN021/22) are normal-precision. The done-when "within
datasheet tolerance at both rails" should be judged against these LSB
bounds translated to mV (±20 LSB ≈ ±6 mV at 14-bit/5 V).

## 6. Register recipe for one single-shot read (facts → sequence)

1. **Init (once, driver constructor):**
   1. `PRCR` unlock PRC1 (0xA502), `MSTPCRD.MSTPD16 = 0`, relock —
      existing MSTP helper pattern (`pwm/gpt_channel.rs`, offset 0x0C).
   2. Wait ≥ 1 µs before the first conversion (§35.8.5) —
      `Delay::delay_us(1)` at construction, or fold into the first
      read's start delay (documented choice).
   3. `ADCER`: ADRFMT = 0 (right-justified, reset), `ADPRC = 11b`
      (14-bit) or `00b` (12-bit, reset) — set explicitly for the
      configured resolution, not inherited; optional `ACE = 1`
      (auto-clear result register after read — the core enables it;
      harmless either way, decide at implementation).
   4. `ADCSR`: ADCS = 00 (single scan), ADHSC per config (0 =
      high-speed), TRGE = 0, EXTRG = 0, DBLE = 0, ADST = 0. ADHSC
      rewrite procedure only matters when changing it later (§35.8.8) —
      one write at init while stopped is outside that procedure's
      concern (the bit is being set from its reset state).
   5. Sampling states: leave ADSSTRn at reset (0Dh = 13 ≥ 6, valid at
      1:2), or expose per-pin SST configuration validated by the
      calculator.
   6. `ADHVREFCNT`: leave at reset (AVCC0/AVSS0). Document that the
      internal-reference path (§35.7) is future work.
2. **Per read:**
   1. Pin already in `Analog` state via `into_analog()`.
   2. Set the channel's select bit: `ADANSA0` (A0–A3) or `ADANSA1`
      (A4/A5). Single-channel scans can OR the bit and clear the others
      or accumulate a mask — the accumulated-mask version doubles as a
      multi-channel scan later; single-bit-per-read is the slice-8
      shape.
   3. `ADCSR.ADST = 1`.
   4. Poll `ADCSR.ADST` until hardware clears it (§35.3.2.1 step 4).
      A bounded poll (e.g. 1 ms timeout → `Error::Timeout`) protects
      against a wedged unit; the computed 1.5 µs conversion makes the
      bound generous.
   5. Read the channel's `ADDRy` — right-justified 12/14-bit value.
3. **Data register format** (§35.2.1): right-justified 12-bit → bits
   11:0; 14-bit → bits 13:0; upper bits read 0. Mask on read for
   defensive correctness regardless of ADPRC state.

## 7. PAC facts (`ra4m1` 0.2.0, local registry source)

- `ADC140` at `0x4005_c000`, `adc140::RegisterBlock` in
  `src/adc140.rs` (flat register list, single block — no channel
  instances).
- Fields verified in the field modules:
  - `adcsr`: `adst`, `adcs` (`_00`/`_01`/`_10` variants), `adhsc`,
    `trge`, `extrg`, `dble`, `gbadie` — all 1-bit variants; also raw
    `bits()` available.
  - `adcer`: `adprc` (bits 1:2, `_00`/`_11` variants), `ace`,
    `diagm/diagld/diagval`, `adrfmt`.
  - `adstrgr`: `trsa` (bits 8:13), `trsb` (bits 0:5).
  - `adansa0`: individual `ansa00`…`ansa14` variant fields; `adansa1`:
    `ansa16`…`ansa25`.
  - `adsstr00…14`, `adsstrl/t/o`: `sst` (bits 0:7).
  - `adhvrefcnt`: `hvsel` (bits 0:1), `lvsel` (bit 4 — note: PAC puts
    LVSEL at bits 2:4 as a 3-bit field; the manual defines it as b4
    with b3:b2 reserved — write 0s in b3:b2 either way), `adslp` (b7).
  - Result registers: `addr: [ADDR; 15]` at 0x20..0x3E (AN000–AN014,
    `.addr().bits()` 16-bit), plus `addr16`…`addr25` individually
    aliased to the same type — `addr21`, `addr22` for A4/A5.
  - `adrd` (self-diagnosis result) available if wanted.
- License MIT OR Apache-2.0 (audited, HalSpec OQ-5).

## 8. Behavioral cross-checks (not transcribed)

- **Arduino core** `cores/arduino/analog.cpp` (fetched 2026-10-19,
  behavioral reference): runs the ADC fixed at the highest resolution
  (14-bit on RA4M1) and scales to the requested read resolution;
  single scan mode, software trigger, right alignment, clear-after-read
  on (`ADC_CLEAR_AFTER_READ_ON`), addition off, `sample_hold_states =
  24`, default vref control `ADC_VREF_CONTROL_AVCC0_AVSS0`;
  `analogReference()` maps AR_EXTERNAL → `VREFH0_AVSS0`, internal
  modes → FSP's vref-control enum (a convenience layer above
  ADHVREFCNT — out of slice scope). Blocking `analogRead` polls the
  FSP status until scan end — the same shape as §6's recipe. All of it
  is consistent with the manual reading in §1/§6.
- **Arduino reference table** (docs, cross-check): UNO R4 — operating
  voltage 5 V, A0–A5 usable, max resolution 14 bits (default
  analogRead 10 bits for compatibility). Supports the AVCC0 = 5 V
  reading; the rails test is the hardware proof (OQ-A1).
- **No existing-crate state to neutralize**: unlike GPT (bootloader
  leaves D13 in PWM), nothing in the bootloader's path uses the ADC;
  the defensive recipe (§6 rewrites every register it relies on while
  stopped) covers any inherited state anyway — slice 7's OQ-P3 answer
  pattern.

## 9. Bench instrumentation (additive; owner wires)

Plan.md's observable needs only **temporary jumpers** (not bench
wires): A0 → GND, A0 → 5 V header; a pot to A0 if available. That
proves the rails; it does not prove linearity.

**Proposed bench extension (one new wire, no re-patching): R3 D5 →
R4 A0.**

- R3 D5 is free (the bench uses R3 D0/D1/D2/D3 only). R4 A0 is unused
  by slices 9–10 (USB CDC and data flash touch no analog pins), so the
  bench stays compatible through Slice 10.
- **R3 sketch (`slice08-source`)**, concept only: `analogWrite` a duty
  staircase (0/25/50/75/100%, ~2 s per step) on D5 and report the
  commanded duty over its own USB serial. D5/D6 are ATmega328p Timer0
  PWM (~976 Hz in the Arduino core — verify in the sketch's own output
  by measuring period on the R4 side, unverified from memory).
- **R4 firmware**: with no RC filter, a 4 Hz sample of a 976 Hz PWM
  wave reads rail values — so instead the firmware **averages N raw
  samples over a window ≥ 10 PWM periods** (~10 ms) and reports mean +
  min + max. At ~1.5 µs/conversion and 976 Hz (~1 ms period) that is
  ~700 samples per PWM period — the mean over whole periods equals
  duty × VREF within the accuracy budget, no components needed. Gate
  line (`slice06-gate` pattern, R3 D3 → R4 D3) re-arms each round, as
  in slices 6–7.
- **What it proves**: monotonic staircase tracking (linearity, not just
  rails), the VREF value itself (mean at a known duty solves for VREF —
  cross-checks OQ-A1's 5000 mV), and the min/max spread gives the
  noise floor at the mid-rail point.
- Voltage compatibility: R3 D5 swings 0–5 V; R4 A0's input range is
  0 V–AVCC0 (5 V rail) — in range with margin. (Reverse direction —
  R4 3.3 V out → R3 in — is the slice-7-proven 3.3 V → AVR case.)

## 10. Existing-crate integration points

- `gpio::Pin::into_analog()` + `gpio::Analog` state — already exist;
  ADC constructors consume them, exactly like `Uart::new` consumes
  configured pins.
- `Clocks::pclkb()` (24 MHz) and `Clocks::pclkc()` (48 MHz) — both
  already resolved and documented (PCLKC documented as "the ADC140
  reference"); the calculator takes them as parameters.
- `Peripherals::take()` — add `adc140: ra4m1::ADC140` with the pin-map
  doc comment (A0–A5 same on both boards → no feature gate on the
  field; pin docs still show both maps).
- Module-stop release precedent: `pwm/gpt_channel.rs` (MSTPCRD at
  offset 0x0C, PRCR 0xA502/0xA500) — the ADC release is the same shape
  with MSTPD16.
- SCI9 monitor path (slice 4) for the binary's prints; `Heartbeat`
  (test-firmware) for the 1 Hz tick; gate-line re-arm pattern from
  slices 6–7.
- Conventions to hold: one type per file (`adc/` with `mod.rs` re-
  exports), one aggregated `use` tree, no globs, `#![deny(missing_docs)]`
  docs on every public item, unit tests named
  `given_<setup>_when_<action>_then_<expectation>`, golden channel
  table test in `src/tests/adc.rs`, `[[bin]]` entry in
  test-firmware's Cargo.toml, README *Testing on hardware* subsection
  with the slice's results in the same commit.

## 11. Open questions (resolve on hardware / during implementation)

> **All resolved on hardware 2026-10-20** — see §14 below and README
> slice 8. The questions are kept for the record.

- **OQ-A1 — reference rail value:** is AVCC0 on the R4 really 5.0 V?
  The 5 V/3V3/GND three-point rails test answers it in one round and
  calibrates the mV conversion (§4, §9).
- **OQ-A2 — A0 as ADC input:** P014 is also DA0; confirm analog input
  works on A0 exactly like the others (the rails test uses A0, so this
  closes automatically).
- **OQ-A3 — accuracy on this board:** the datasheet accuracy tables
  assume VCC = AVCC0, which this board does not satisfy (3.3 V vs
  5 V rail, §4). Measure and record the actual rails/staircase error;
  judge against ±20 LSB (14-bit high-precision) as the tolerance
  budget, noting the off-datasheet condition in the README.
- **OQ-A4 — ADST poll robustness:** confirm the hardware auto-clear
  behaves as §35.3.2.1 promises at 48 MHz (poll loop with a 1 ms
  timeout); if the timeout ever fires, dump ADCSR before theorizing
  (AGENTS.md rule 9).
- **OQ-A5 — SST lower limit at ratio 1:2:** manual says ≥ 6 states
  (§35.2.14); reset 0Dh satisfies it. Only becomes an OQ if SST gets
  exposed as configuration — then the calculator enforces it.
- **OQ-A6 — R3 D5 PWM frequency:** ~976 Hz claimed from memory of the
  ATmega328p Timer0 defaults — **unverified**; the R4-side measurement
  reports the period, so the sketch's own output settles it.

## 12. Effort shape

Same magnitude as slice 7, smaller: one register block, no pin-mux
per-board differences (A0–A5 identical on both maps), no interrupt
plumbing. The new pieces are the resolution/format handling (§6), the
conversion-time calculator with host tests (§5), and the averaging
firmware trick that makes the PWM-staircase observable work without a
filter (§9). Files touched: `crates/unor4-hal/src/adc/` (new module,
~4–6 files), `peripherals.rs` (+1 field), `lib.rs` (+1 mod),
`src/tests/adc.rs` (new), test-firmware `slice08_adc.rs` (+ Cargo.toml
`[[bin]]`), `arduino/sketches/slice08-source` (R3), README subsection,
CHANGELOG at release time.

## 13. Source list for implementation

- RA4M1 hardware manual Rev.1.00: §35 (ADC14 — specs Table 35.1,
  registers §35.2, operation §35.3, triggers §35.3.6, reference §35.6,
  notes §35.8), §19 Tables 19.5/19.6 (ASEL pin functions), §10
  (MSTPCRD p.175, MSTPD16), §8 Table 8.2 (PCLKB/PCLKC).
- RA4M1 datasheet Rev.1.10: §2 A/D conversion characteristics
  (Tables 2.42/2.43 — accuracy, sampling states, impedance), pin
  function tables (P000/P001/P002/P014/P100/P101 rows), supply
  conditions (VREFH0 ≤ AVCC0, AVCC0 1.6–5.5 V).
- `ra4m1` PAC 0.2.0 (`src/adc140.rs`, field modules listed in §7).
- Cross-check only: ArduinoCore-renesas `analog.cpp`/`analog.h`
  (fetched 2026-10-19), `variants/UNOWIFIR4/pinmux.inc`,
  `variants/MINIMA/pinmux.inc`; Arduino reference (analogRead table).
- `pins.csv` golden table (Slice 2) — a0–a5 rows.

## 14. Hardware results (2026-10-20, WiFi board)

One flash per firmware, three aligned gate rounds captured
simultaneously on both boards (README slice 8 has the reproduction
commands and the transcripts; R4 `COM3`, R3 `COM4`):

- **Staircase (the linearity proof).** Every round, five 1.5 s
  averaging windows (631 650 samples each, ~420 kS/s — the §5
  calculator's 1.2 µs conversion plus loop overhead) over the R3 D5
  PWM staircase:

  | Window | Commanded | Mean raw | duty/255 × 4095 | Error |
  |---|---|---|---|---|
  | W0 | 0 | 7 | 0 | +7 LSB (DC low) |
  | W1 | 64 | 1045 | 1028 | +17 LSB |
  | W2 | 127 | 2051 | 2040 | +11 LSB |
  | W3 | 191 | 3073 | 3068 | +4 LSB |
  | W4 | 255 | 4095 | 4095 | 0 (DC high) |

  Worst error +17 LSB ≈ 21 mV — inside the datasheet's ±32 LSB
  (±39 mV) max for A0's (normal-precision) class, and W1/W2/W3 are
  inside even the high-precision ±20 LSB line. **OQ-A3 answered.**
- **Rails:** duty 0 reads 0–7 LSB (≤ 8.5 mV), duty 255 reads 4095 with
  `min=max=4095` (a solid rail, zero ripple) — the two rails land
  exactly on the code range. With the R3's 5 V rail feeding both the
  stimulus and the R4's AVCC0, this confirms the reference-rail
  picture; the nominal-5000 mV claim itself rides on the R3's rail
  being 5.0 V. **OQ-A1 answered within that caveat** (a 3V3-header
  jumper round would pin the absolute value; not run — the driver's
  correctness needs only the two rails, which both landed exactly).
- **A0 as an ADC input works** — the entire rig runs through P014,
  which is also DA0. **OQ-A2 answered.**
- **ADST self-clear poll never timed out** across ~10 M conversions
  (err=0 everywhere); window means repeat within ±2 LSB across rounds.
  **OQ-A4 answered.**
- **All six channels converted** — the diagnostic phase (§ below) ran
  A1–A5 through the same driver for two full captures, ~13 M
  conversions each, zero errors; their *values* float (nothing drives
  them), only A0's accuracy is instrumented.
- **OQ-A5** stands resolved by construction (SST left at reset 0Dh ≥ 6
  states; the calculator documents the 1:2-ratio minimum).
- **OQ-A6:** the R3's PWM frequency was never directly measured, and
  the rig does not need it — each 1.5 s window covers ≥ ~1400 periods
  at the nominal ~976 Hz, and the mean-over-complete-periods identity
  bounds the residual at ≤ 1/1400 of the ripple (≤ ~3 LSB). The
  min/max spread (0/4095 in W1–W3, DC in W0/W4) shows rail-to-rail
  sampling. Frequency stays **unverified but immaterial**.

### The bring-up incident (diagnostic trail, kept for the pattern)

The first rig captures showed every analog channel wandering mid-rail
(~800–3400 raw, correlated across channels — sample-hold crosstalk
between floating pins), and **no channel ever at a rail during duty-0
or duty-255 windows**, while the R3's log confirmed the staircase ran.
That signature is impossible if the wire touches any driven pin: duty
255 is a solid ~5 V DC and can only read full scale. Conclusion: the
D5→A0 path was open. A six-channel live print ruled out a misland on
any A0–A5 (all floated). The owner then found the R3 end parked in
**D4** — not a PWM pin on the ATmega328p (PWM: D3/D5/D6/D9/D10/D11),
so the sketch's staircase never reached the wire. Moving the R3 end to
D5 fixed everything in one round; `slice08_probe` (the 20-pin digital
scan written for the hunt) remains in `test-firmware` as the bench's
open-wire signature tool. Differential-observation lesson honored
throughout: every step eliminated hypotheses by observation, none by
theory.
