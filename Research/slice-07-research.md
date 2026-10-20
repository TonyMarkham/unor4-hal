# Slice 7 research — PWM (GPT)

> Planning notes assembled 2026-10-19, before any Slice 7 code. Verified
> facts are cited to the hardware manual or the PAC; cross-check data
> (Arduino core, FSP docs) is labeled as such. Open items are named
> explicitly. No code has been written.

Manual citations refer to **R01UH0887EJ0100 Rev.1.00** (the PDF in
`Research/`). A Rev.1.10 (2023) exists; a spot check shows the GPT chapter
is §22 in both, page numbers differ slightly. If a later citation becomes
owed, re-check the page against whichever revision is quoted.

## What the slice is

`pwm.rs`: GPT timer driver, `embedded-hal` PWM trait, D6 first. Binary:
`slice07_breathe.rs` — sine-table duty profile on D6 (~2 s cycle) while the
Slice-5 SysTick heartbeat keeps running on D13. Done-when (Plan.md): duty
resolution stable at 1 kHz+ with no flicker, heartbeat + breathing
simultaneous. **Owner decision (2026-10-19): no external LED.** The
observable is instrumented — R4 D6 is wired to R3 D2 and the R3 measures
duty, period, and the sine profile (§8). The visual "breathes" test is
replaced by the measured duty sequence.

## 1. The GPT peripheral (manual §22)

- **8 channels:** 2× GPT32 (ch 0–1) + 6× GPT16 (ch 2–7) (§22.1, Table 22.1).
- **Count clock = PCLKD**; bus clock PCLKA (§22.1 Table 22.1, §22.2 Table
  22.2). Cross-check: FSP docs say "The GPT clock is based on the PCLKD
  frequency" (renesas.github.io/fsp, r_gpt module guide). Our `clock.rs`
  already documents PCLKD as "the GPT (PWM) reference" and resolves it to
  48 MHz (SCKDIVCR PCLKD/1). Ratio constraint PCLKA:PCLKD = 1:N is satisfied
  at 1:1.
- **Prescalers (GTCR.TPCS[26:24], §22.2.12 p.412):** PCLKD/1, /4, /16, /64,
  /256, /1024 — per channel, set only while the counter is stopped.
- **Saw-wave PWM mode = GTCR.MD[18:16] = 000b** (§22.2.12). Count direction
  up = `GTUDDTYC.UD = 1` (§22.2.13 p.413).
- **Cycle:** GTPR sets the maximum count; for saw waves the period is
  **(GTPR + 1) counts** (§22.2.20 p.426). GPT16's GTPR is 16-bit effective
  (upper 16 bits read 0, writes ignored).
- **Output pin function (GTIOR, §22.2.14 p.415; Table 22.5 p.418):**
  `GTIOA[4:0]` / `GTIOB[4:0]` pack b4 = initial output level, b3:b2 =
  behavior at cycle end, b1:b0 = behavior at compare match. Pin output
  enable = `OAE`/`OBE` (b8/b24). Stop-level = `OADFLT`/`OBDFLT` (b6/b22).
  The needed encoding for "high from cycle end to compare match, then low"
  (FSP's PWM convention, see cross-check below) is:
  b4=0 (initial low), b3:b2 = 10b (high at cycle end), b1:b0 = 01b (low at
  compare match) → `GTIOA[4:0] = 0b0_10_01`.
- **Duty = the compare match value.** With the encoding above, the pin is
  high from the cycle end through the GTCCRA match, so the high fraction is
  ≈ GTCCRA / (GTPR + 1). Whether the exact fraction is GTCCRA/(GTPR+1) or
  (GTCCRA+1)/(GTPR+1) is an off-by-one to be measured on hardware (OQ-P2).
- **Duty 0% / 100% must go through GTUDDTYC, not GTCCRA** (§22.3.6 p.465,
  Table 22.5 Note 2): when a compare match coincides with the cycle end,
  the b3:b2 (cycle-end) setting wins in saw-wave mode, so GTCCRA = 0 or
  GTCCRA = GTPR cannot produce the opposite extreme. `GTUDDTYC.OADTY[17:16]`
  forces 0%/100% (`10b` = duty 100%, `10`… see bit table: b17,b16 = `1,0`
  → 0%, `1,1` → 100%), optionally latched with `OADTYF` (b18). This maps
  directly onto `set_duty_cycle_fully_off/fully_on`.
- **Glitch-free duty updates:** single buffer — `GTBER.CCRA[17:16] = 01b`
  links GTCCRC as buffer to GTCCRA, transferred at the cycle end (§22.2.17
  p.423; Fig 22.29/§22.3.3.1 p.449–450). Duty writes then go to GTCCRC.
  The 0%/100% duty change via OADTY is likewise reflected at the cycle end
  (§22.3.6). Alternative for slice 7: direct GTCCRA writes (not
  glitch-free) — acceptable for an LED, but the buffer path is the
  documented-correct one and costs nothing.
- **Write protection:** GTWP (§22.2.1 p.392) resets to 0x0000_0000, i.e.
  writes enabled after reset. No unlock sequence needed unless WP was set.
- **Module stop:** `MSTPCRD.MSTPD6 = 0` releases GPT162–167;
  `MSTPCRD.MSTPD5 = 0` releases GPT320–321 (§10 Low Power Modes, MSTPCRD,
  p.175; reset value = all stop). POEG's `MSTPD14` is only needed for
  GTRETRGn trigger input (Table 22.2 Note 1) — not for plain PWM output.
  MSTPCRD writes need `PRCR.PRC1`, same as the SCI module-stop helpers in
  `uart.rs`.

## 2. Register map (manual §22.2 Table 22.4 p.391)

All channels share one address space: `0x4007_8000 + 0x0100 × m`,
m = 0..1 for GPT32 (ch 0,1), m = 2..7 for GPT16 (ch 2..7).

| Offset | Register | Reset | Role in slice 7 |
|---|---|---|---|
| 0x00 | GTWP | 0 | write protect (already unlocked) |
| 0x2C | GTCR | 0 | MD=000, TPCS, CST start/stop |
| 0x30 | GTUDDTYC | 0x0000_0001 | UD=1 up-count; OADTY 0%/100% |
| 0x34 | GTIOR | 0 | GTIOA[4:0], OAE, OADFLT |
| 0x40 | GTBER | 0 | CCRA=01 single buffer |
| 0x48 | GTCNT | 0 | counter |
| 0x4C | GTCCRA | 0xFFFF (GPT16) | duty compare match |
| 0x50 | GTCCRB | 0xFFFF | (unused for one output) |
| 0x54 | GTCCRC | 0xFFFF | GTCCRA's buffer (write duty here) |
| 0x64 | GTPR | 0xFFFF | period − 1 |
| 0x68 | GTPBR | 0xFFFF | GTPR buffer (unused, fixed period) |

So **GPT163 (channel 3) base = 0x4007_8300** — the WiFi D6 channel.

## 3. Pin map — which GTIOC signal drives each header PWM pin

PSEL[4:0] = **00011b** (GPT GTIOC group) for all of them. From manual §19
pin-function tables — Table 19.6 (PORT1(1), p.365), 19.7 (PORT1(2), p.365),
19.10 (PORT3, p.369), 19.12 (PORT4(2), p.371) — and **cross-checked
entry-for-entry against the Arduino core's
`variants/UNOWIFIR4/pinmux.inc` + `variants/MINIMA/pinmux.inc`** (behavioral
cross-check only): every entry matches.

| Header | WiFi pin | GTIOC | GPT ch | Minima pin | GTIOC | GPT ch |
|---|---|---|---|---|---|---|
| D3 | P105 | GTIOC1A | 1 (32-bit) | P104 | GTIOC1B | 1 (32-bit) |
| D5 | P107 | GTIOC0A | 0 (32-bit) | P102 | GTIOC2B | 2 (16-bit) |
| D6 | P111 | GTIOC3A | 3 (16-bit) | P106 | GTIOC0B | 0 (32-bit) |
| D9 | P303 | GTIOC7B | 7 (16-bit) | P303 | GTIOC7B | 7 (16-bit) |
| D10 | P103 | GTIOC2A | 2 (16-bit) | P112 | GTIOC3B | 3 (16-bit) |
| D11 | P411 | GTIOC6A | 6 (16-bit) | P109 | GTIOC1A | 1 (32-bit) |

**Slice 7 target: WiFi D6 = P111 → GTIOC3A → GPT163, PSEL = 00011b.**
All values above also agree with `pins.csv` (the Slice-2 golden table).
Channel collisions to design for later: WiFi D5/D3 sit on the two 32-bit
channels; Minima D11 and D3 share GPT1 (A and B); Minima D10 and D6's
WiFi channel 3 is shared with Minima D10. One pin type per (board, pin,
channel, A/B) pair keeps this explicit.

**Constructor status (post-implementation):** all six pins per map have
constructors, named by header pin (`Pwm::d3`/`d5`/`d6`/`d10`/`d11`
feature-gated per map; `d9` shared — P303 is D9 on both maps), each
locking its table row at compile time. The whole surface is asserted by
the golden constructor test (`src/tests/pwm.rs`), which type-checks
every constructor signature against this table. Hardware-proven: WiFi
D6 (GPT163). The other rows run the same `init` recipe — compile-checked,
not board-run.

## 4. PAC facts (`ra4m1` 0.2.0, local registry source)

- All 8 channels are exposed: `GPT320`, `GPT321` → `gpt320::RegisterBlock`;
  `GPT162`…`GPT167` → the **same** `gpt320::RegisterBlock` layout (GPT321
  is `PTR = 0x4007_8100`, GPT162 = 0x4007_8200, GPT163 = 0x4007_8300, …).
- RegisterBlock fields: `gtwp, gtstr, gtstp, gtclr, gtssr, gtpsr, gtcsr,
  gtupsr, gtdnsr, gticasr, gticbsr, gtcr, gtuddtyc, gtior, gtintad, gtst,
  gtber, gtcnt, gtccra, gtccrb, gtccrc, gtccre, gtccrd, gtccrf, gtpr,
  gtpbr, gtdtcr, gtdvu` — note the E-before-D order of GTCCRE/GTCCRD
  (matches manual Table 22.4: E = 0x58, D = 0x5C).
- Field granularity: `gtcr` exposes `md` (3b), `tpcs` (3b), `cst`;
  `gtior` exposes `gtioa`/`gtiob` as 5-bit variants plus `oae`, `oadflt`
  etc.; `gtuddtyc` exposes `ud`, `oadty` etc.; `gtpr`, `gtccra`, `gtccrc`
  are plain 32-bit RW (GPT16 ignores upper bits per manual).
- License: MIT OR Apache-2.0 (already audited in Slice 5, HalSpec OQ-5).

## 5. Trait shape — and a spec discrepancy to fix

`embedded-hal` 1.0.0's `pwm` module (docs.rs) contains only:
`ErrorKind`, `Error`, `ErrorType`, **`SetDutyCycle`**:

- Required: `max_duty_cycle(&self) -> u16`, `set_duty_cycle(&mut self,
  duty: u16) -> Result<(), Self::Error>`.
- Provided: `set_duty_cycle_fully_off/on`, `..._fraction`,
  `..._percent`.
- Duty type is `u16` → `GTPR + 1` must be ≤ 65 536 for the returned max —
  automatically true for GPT16, and for GPT32 just cap the period.

**The multi-channel `Pwm` trait (Channel/Time/set_period) is embedded-hal
0.2 legacy; it does not exist in 1.0.** HalSpec §6's compliance-matrix row
says "PWM | Pwm" — that is the wrong trait name for 1.0 and should be
amended to `SetDutyCycle` (HalSpec §1: spec wrong → spec fixed). Plan.md's
slice-7 wording "`embedded-hal` `Pwm` traits" should read `SetDutyCycle`.
HalSpec §5.6's "period + duty in ticks, duty 0..=period" API is still
right; `set_period` can be a constructor/frequency parameter since slice 7
uses one fixed frequency.

## 6. Register sequence for one PWM output (facts → recipe)

For WiFi D6 (GPT163, GTIOC3A, P111), synthesizing from §22.3.3.1
(Fig 22.30, p.450), adapted to direct duty writes or single buffering:

1. Module stop off: `MSTPCRD.MSTPD6 = 0` (PRC1 unlocked).
2. Pin: P111 PFS → PSEL = 00011b, ASEL = PMR = 1 (existing
   `into_alternate(0b0_0011)`; the SCI slices proved the PFS sequence).
3. Counter stopped (`GTCR.CST = 0`): set `GTCR.MD = 000b`,
   `GTCR.TPCS = prescaler`; `GTUDDTYC.UD = 1`; `GTPR = period − 1`;
   `GTIOR.GTIOA = 0b0_10_01`, `GTIOR.OAE = 1`; duty via GTCCRA (or
   GTCCRC + `GTBER.CCRA = 01b` for buffered updates).
4. `GTCR.CST = 1` to start.
5. Duty updates: `set_duty_cycle` → GTCCRC (buffered) or GTCCRA (direct);
   0%/100% → `GTUDDTYC.OADTY` (+`OADTYF` semantics per §22.3.6).

Prescaler/period arithmetic (host-unit-testable, same shape as the baud
calculator): pick the smallest divisor d ∈ {1, 4, 16, 64, 256, 1024} with
`(f_pclkd / d) / f_pwm` ≤ 65 536; then `GTPR = round(f_pclkd/(d·f_pwm)) − 1`.
Worked values at f_pclkd = 48 MHz:

| f_pwm | d | GTPR | actual f | duty steps |
|---|---|---|---|---|
| 1 kHz | 1 | 47 999 | exactly 1 000 Hz | 48 000 (~15.5 bit) |
| 490 Hz (Arduino's `STANDARD_PWM_FREQ_HZ`, cross-check) | 4 | 24 489 | 490.0 Hz | 24 490 |
| 10 kHz | 1 | 4 799 | exactly 10 kHz | 4 800 |
| 200 Hz | 4 | 59 999 | 200.0 Hz | 60 000 |

Range notes: GPT16 + d=1 reaches 732 Hz minimum; d=1024 reaches 0.72 Hz
minimum; 48 MHz max (GTPR=0 → 24 MHz output; irrelevant for LEDs).

## 7. Behavioral cross-checks (not transcribed)

- **Arduino core** (`variants/UNOWIFIR4/pinmux.inc`, `FspTimer.cpp/.h`,
  `variant.cpp`): PWM pins D3/D5/D6/D9/D10/D11 ↔ GPT channels
  1/0/3/7/2/6, output A/B as in §3's table; `analogWrite` default
  `STANDARD_PWM_FREQ_HZ = 490.0`, 50% duty; GPT period derived from
  PCLKD with smallest FSP divider that fits 16/32 bits — the same
  algorithm as §6.
- **FSP r_gpt docs**: "The PWM output signal is high at the beginning of
  the cycle and low at the end of the cycle" — the polarity convention
  adopted in §6's GTIOR encoding.
- `variant.cpp initVariant()` notes: **the bootloader leaves D13 (P102 =
  GTIOC2B, GPT162) configured as PWM output**; the core explicitly
  deconfigures it. Our GPIO `into_output` clears PSEL/PMR (Slice 1
  proved D13 works), but a GPT driver claiming any channel should stop
  the counter (`GTCR.CST = 0`) before configuring, and `Peripherals::take`
  or the driver should neutralize GPT162's inherited state (either
  re-enter/exit module stop for a register reset, or explicitly stop it).
  Differential-observation step on hardware: dump GPT162/GTCR and
  GTPR/GTCCRA right after boot before deciding (AGENTS.md rule 9).

## 8. Bench instrumentation (additive; owner wires — decided 2026-10-19)

No LED: the slice's observable is the PWM signal itself, measured by the
R3. One new wire, nothing else:

- **New wire: R4 D6 → R3 D2.** R3 D2 is a hardware interrupt pin (INT0)
  and is free — the bench so far uses R3 D0/D1/D3 only. R4 D6 drives
  3.3 V; the ATmega328P's VIH at 5 V VCC is 0.6 × VCC = 3.0 V, so 3.3 V
  is in spec with margin. (Fallback if marginal in practice: route to
  R3 A0 instead and threshold `analogRead` at mid-scale — weaker, see
  below.)
- **R3 sketch (`slice07-measure`)**, concept only:
  - Edge capture: interrupt on CHANGE, `micros()` timestamps; per period
    compute high time, low time, period; aggregate over a fixed window
    (e.g. 10 000 periods = 10 s at 1 kHz): duty mean/min/max, period
    mean/min/max → settles commanded-vs-measured duty (OQ-P1), the
    1 kHz+ frequency, duty stability ("no flicker" = duty min/max spread
    within a bound), and the 48 000-step resolution claim.
  - Breathing profile: alongside the aggregate, log the rolling duty
    (e.g. one duty sample per 100 ms) — the ~2 s sine cycle should read
    as a smooth 0→100→0% sequence. This is the "breathes" observable,
    measured rather than eyeballed.
  - The existing gate-line mechanism (`slice06-gate` pattern, R3 D3 →
    R4 D3) can re-trigger measurement rounds for a repeatable run.
- **Compatibility with slices 8–10:** R4 A0–A5 (slice 8) untouched; R3 D2
  and R4 D6 are unused by any later slice; wires stay additive. No
  re-patching.

Why not R3 A0 (kept as fallback only): `analogRead` costs ~110 µs/call →
~9 samples per 1 kHz PWM period — duty statistics over thousands of
periods are possible, but period/edge measurements are not. The D2
interrupt capture dominates it for one wire either way.

## 9. Existing-crate integration points

- `Clocks::pclkd()` — already returns 48 MHz (Slice 1).
- `gpio` `into_alternate(psel)` — proven by SCI slices; PSEL = 3 for GPT.
- Module-stop helper precedent: `uart.rs` `release_from_module_stop()`
  (MSTPCRB) → GPT needs the MSTPCRD variant; PRCR unlock/lock identical.
- `Heartbeat` (test-firmware, SysTick poller) — heartbeat needs no GPT
  channel, satisfying "heartbeat + breathing simultaneously" without a
  second timer.
- `Peripherals::take()` singleton — GPT163 (and later channels) become
  claimed tokens like the SCIs; Minima/WiFi feature gates follow the
  established pin-doc pattern (`d6: WiFi=P111/GPT3-A, Minima=P106/GPT0-B`).

## 10. Open questions (resolve on hardware / during implementation)

> **All resolved on hardware 2026-10-19** — see §12 below and README
> slice 7. The questions are kept for the record.

- **OQ-P1 — duty polarity off-by-one:** is the high time exactly GTCCRA
  counts, or GTCCRA + 1? Settled by the R3 duty measurement at a known
  commanded duty (e.g. 25.000% → compare against both formulas).
- **OQ-P2 — 0%/100% extremes:** confirm via the R3 measurement that
  `GTUDDTYC.OADTY` gives a solid 0% / 100% duty (with `OADTYF` vs
  without, per §22.3.6 timing) and that GTCCRA = 0 / GTPR does *not* (Table 22.5 Note 2 predicts it won't).
- **OQ-P3 — bootloader GPT state:** dump GTCR/GTCCRA/GTPR of GPT162 (and
  GPT163) after reset before first use; decide whether the GPT driver
  resets the channel via module-stop toggling or explicit register
  writes. (§7's differential-observation step.)
- **OQ-P4 — buffer vs direct duty writes:** if single-buffered duty
  updates are used, confirm the first-cycle behavior (GTCCRC must be
  pre-loaded; Fig 22.29 shows the buffer transfer at overflow). If any
  first-cycle glitch appears, fall back to direct GTCCRA
  writes and document.
- **OQ-P5 — Rev.1.10 deltas:** none expected for §22's PWM basics (the
  Renesas technical update TN-RA\*-A0125A touches GTCR.MD wording for
  RA4E1/RA4M2/RA6\*, not RA4M1); if the owner later cites Rev.1.10, re-map
  page numbers.

## 12. Hardware results (2026-10-19, WiFi board)

One aligned gate round captured simultaneously on both boards
(README slice 7 has the reproduction commands and the full transcript;
R4 `COM3`, R3 `COM4`):

- **Period:** exactly 1000 µs at every fixed duty (`period=1000us`,
  6013 edges in 10 s = exactly the ~6 s of the three edged staircase
  steps). The 1 kHz / `GTPR = 47999` / PCLKD/1 math from §6 is exact.
- **Duty staircase** (R3 `W3`, aligned with the R4's second staircase):
  samples `0 L L L 252 252 248 248 501 500 500 501 752 752 748 752 749
  H H H` — every commanded duty within ±1 tick of 48 000 (±4
  per-mille). **OQ-P1 answered:** the high fraction is the compare
  value over the period; no off-by-one worth correcting (the ±1 tick
  spread is the R3's `micros()` granularity at 4 µs).
- **0%/100% extremes:** the 0% step reads as an explicit flat **LOW**
  (`L` samples, no edges) and the 100% step as an explicit flat **HIGH**
  (`H` samples, no edges) — `OADTY = 10b` and the never-matching
  compare both work, and the aggregate never shows a 1000-per-mille
  edged duty, confirming Table 22.5 Note 2 (no compare value produces
  0%). **OQ-P2 answered.**
- **Breathing:** windows sweep 59→1000 per-mille with the sine shape
  visible in the 500 ms samples; 2.048 s cycle per the 8 ms/256-step
  update. Breathing-window period means read 1007–1019 µs: the ~2% of
  periods lost cluster at the sine extremes, where the pulse gap drops
  below the R3 ISR duration and edges merge — a rig artifact, not a
  GPT defect (the fixed-duty windows show the exact period).
- **Heartbeat:** the R4 log shows `tick N` every second with no gap
  through gate transitions, staircase, and breathing (ticks 456–516 in
  the captured window) — heartbeat + breathing simultaneous, the
  Plan.md done-when.
- **OQ-P3 answered by construction:** the driver rewrites every
  register it relies on (module stop released, `GTCR`/`GTUDDTYC`/
  `GTIOR`/`GTBER`/`GTPR`/compare registers all written with the counter
  stopped), so no bootloader-inherited GPT state is trusted; output was
  correct from the first period after boot. The post-boot register dump
  became unnecessary — the defensive rewrite is the permanent
  mitigation.
- **OQ-P4 answered:** single-buffered duty updates
  (`GTBER.CCRA/CCRB = 01b`, writes to `GTCCRC/GTCCRE`) with both buffer
  and active registers preloaded at init showed no first-cycle glitch;
  fixed-duty per-period min/max stayed within ±4 per-mille.
- **OQ-P5 stands** (Rev.1.00 citations throughout; no delta encountered).

Also verified incidentally: the `GTPBR` reset trap from the init recipe
is real defensive ground — `GTBER.BD[1] = 1` (period buffer off) ships
in the driver; and the stale-`lastDuty` artifact in the first rig
version (flat phases printing the previous duty) was fixed by the
per-sample flat detection (`H`/`L`).

## 11. Source list for implementation

- RA4M1 hardware manual Rev.1.00: §22 (GPT), §19 Tables 19.6/19.7/19.10/
  19.12 (pin functions), §10 (MSTPCRD), §8 Table 8.2 (PCLKD).
- `ra4m1` PAC 0.2.0 (`gpt320::RegisterBlock`, all 8 channel structs).
- Cross-check only: ArduinoCore-renesas `pinmux.inc` (UNOWIFIR4, MINIMA),
  `FspTimer.cpp/.h`, `variant.cpp`; FSP r_gpt documentation.
- `pins.csv` golden table (Slice 2) for the port-pin maps.
