# Slice 11 plan — step generator on a 2 ms hardware tick

Status: **all steps done** — 1 (ELC driver), 2 (step generator on D4),
3 (exact pulse count), 4 (2 ms tick and latch), 5a (rate applied at the
tick by the DTC), 5c (rate 0, idle stop, start from stopped, held ticks,
trip stop), verified on WiFi 2026-11-03; 5b (signed rates, DIR on D5 at
the tick, position), verified 2026-11-04 with the R4 D5 → R3 D7 wire.
README Slice 11 record. Cart-pole accepted the deviations (5a's late old
pulse, 5b's DIR ~7 µs after the tick, the R13 stop at up to 5.5 µs) on
2026-11-04. Follow-ups the same day (README Slice 11): the one-edge
miscount was `stop()` cutting a pulse, now fixed (follow-up 1), and the
tick can wake the CPU from WFI at cart-pole's request (follow-up 2).
Changes from the plan made on the way:
- The count listens to ELC input B (`GPT0_CCMPB`), which the first
  `stop()` also used for its hardware stop; the fixed `stop()` no longer
  does (follow-up 1).
- GPT165 counts the ticks (one more channel than §3 listed), so a missed
  poll is reported instead of silently merging ticks.
- `predicted_pulses()` returns `Option` (`None` when it cannot be exact).
- A pulse end reaches the tick latch 2 counts after `GTCCRB`
  (`PULSE_END_LATCH_DELAY`, measured on the board; README step 4).
- Step 5a: the DTC write lands 14–17 counts after the tick (measured);
  `GTPBR` buffering is on; a chained read-back of `GTCNT` resolves the one
  race (old pulse inside the write latency) instead of a model; the period
  sequence at each tick is a public `Schedule`; `queue` refuses with
  `NotReady`/`TooLate` rather than guess (guard: 40 µs before the tick).
  After unpolled ticks while running, the schedule is unknown until the
  next `run`.
- Step 5b: DIR is not written at the tick or on the pulse's `GPT0_CCMPB`
  (§3's design). The tick's chain holds the DIR write back with a delay
  record until any pulse LOW at the tick has ended (326–342 counts after
  the tick, measured), and the next pulse follows `dir_setup` after the
  latest allowed write. `low_ns` is therefore limited to 400 counts. A
  reversal while idle or stopped is set by the CPU at queue time.
- Rate 0 (5c): the idle generator stops itself after 1 s, not at the next
  tick.
Step 2 added `StepGenerator::phase_counts()` (the counter's place in the
period), which the validator uses to time stops inside chosen phases. Written by the agent under
the Slice 11 grant (AGENTS.md, Current grants). The README keeps the slice
record; this file is the plan.

Source of the requirements: the cart-pole rig's request and its answers to
four rounds of clarification (2026-11-03). Hardware facts cite the Renesas
RA4M1 User's Manual R01UH0887 Rev 1.10 (`Research/`) unless noted. Anything
not proven on the board is marked **unverified**.

## 1. Goal

Drive a DM542 stepper driver from the UNO R4 WiFi exactly the way the
cart-pole policy was trained: a pulse rate that changes on an exact 2 ms
grid, an exact pulse count latched at each tick, safe direction changes,
and an immediate stop for a tripped limit switch.

Done when every requirement in §2 is shown on the board by the validation
pair and recorded in the README.

## 2. Requirements (all confirmed by cart-pole)

| # | Area | Requirement |
| --- | --- | --- |
| R1 | STEP pin | DM542 PUL on header D4 = P106 → GTIOC0B on GPT320 (Table 19.6; board schematic ABX00087 net `P106_GPT0_B`) |
| R2 | Polarity | rests HIGH, each pulse LOW, stopped = HIGH (the rig drives PUL through an inverting PNP stage) |
| R3 | Boot | P106 floats until the constructor runs; safe on the rig (10 kΩ pull-up holds the transistor off) |
| R4 | Pulse shape | low 5,000 ns, fixed; high ≥ 2,500 ns (DM542 manual §10, t3/t4) |
| R5 | Rate | any whole number 0–55,733 pulses/s (44,600 at the rig's 24 V) |
| R6 | Rate accuracy | delivered as 48 MHz ÷ round(48 MHz ÷ rate): error ≤ rate ÷ 96,000,000 (±0.046 % near 44,600), plus the HOCO's ±1 % — accepted |
| R7 | Tick | hardware-timed, every 2 ms exactly in clock counts, no CPU timing in the path |
| R8 | Rate at the tick | the queued rate takes effect at the tick: the next pulse lands one new period after the last one, or 2,500 ns after the tick if that time has passed |
| R9 | Count | exact (none missed, none doubled), latched at the tick, at the same instant the rate changes |
| R10 | Predicted count | the HAL gives the count predicted for the next tick (the firmware's soft limit, cap √(2·m·d), uses it before the tick) |
| R11 | DIR | flips at the tick when the sign changes; if the tick falls inside a pulse's low time, at that pulse's end (≤ 5 µs late); the next pulse ≥ 5 µs after the flip (DM542 t2); DIR changes only while STEP is HIGH |
| R12 | Missed queue | no new rate before the tick → hold the running rate; report the miss and the rate actually running |
| R13 | Trip stop | `stop()` acts at once — the next pulse boundary, ≤ 5 µs — never at the next tick; it disarms everything the next tick would do |
| R14 | Jump limit m | the firmware's (calibration.toml); nothing in the HAL depends on it |
| R15 | Coexistence | the firmware's `UartIrq` on SCI1 keeps running; counting and tick actions use no CPU interrupt |

Clock: the board has no crystal on the RA4M1 (schematic ABX00087 sheet 1:
XCIN/XCOUT unconnected, EXTAL/XTAL drive the LED matrix), so the HOCO
(48 MHz ±1 %, datasheet Table 2.22, −20 to 85 °C) is the only clock.

## 3. Hardware design

| Piece | How | Source |
| --- | --- | --- |
| STEP output | GPT320 saw-wave PWM at PCLKD/1; `GTIOB[4:0] = 10110b` (start HIGH, LOW at cycle end, HIGH at GTCCRB match); `GTCCRB = 240` (5,000 ns), always inside 0 < GTCCRB < GTPR; `OBDFLT = 1`, `OBHLD = 0` (stopped = HIGH) | §22.2.14, Table 22.5, §22.9.2(5) |
| Pin mux | P106 `PSEL = 00011b` | Table 19.6 |
| Period | 32-bit `GTPR`: 1 pulse/s = 48,000,000 counts fits; min period 360 counts (240 low + 120 high) → 133,333 pulses/s ceiling | §22.2.20 |
| Count | ELC routes `GPT0_CCMPB` (event 0x58, the end of each pulse) to ELC input B, which the hardware stop also uses; GPT321 counts it (`GTUPSR.USELCB`) — completed pulses only | §18.2.3, Table 18.2, §22.2.8, §22.3.1.1(4) |
| ELC delay | 0 cycles between two GPTs (same PCLKD domain) | Table 18.5 |
| Tick | a 16-bit GPT at PCLKD/4, `GTPR = 23,999` → 2.000 ms | §22.2.12 |
| Count latch | tick overflow event → ELC → GPT321 input capture into GTCCRA (`GTICASR.ASELCx`); a pulse event coinciding with the capture is captured as the pre-update value (counted in the next tick) | §22.2.10, §22.9.5(3) |
| Phase at the tick | GPT320's GTCCRA (its A output unused; P107 stays GPIO) captures GPT320's counter on the same tick event → time since the last pulse start, exact | §22.2.10 |
| Rate at the tick | a DTC transfer started by the tick event (IELSR `DTCE`) writes GPT320 `GTPR` and `GTPBR` (adjacent, offsets 0x64/0x68) as one block. The CPU computes the `GTPR` value before the tick so it is always ≥ 120 counts above the counter at the write — writing below it would break the usage rule 0 ≤ GTCNT ≤ GTPR, miss the overflow (overflow only at GTCNT = GTPR) and run ~89 s without a pulse | §17, Table 17.8, Table 22.4, §22.3.1.1, §22.9.3 |
| DIR at the tick | the same DTC block writes D5's `POSR`/`PORR` (set/reset). A reversal tick that falls inside a pulse's low time instead arms a one-shot DTC write on that pulse's `GPT0_CCMPB`. **As built (5b):** one chain — period write, delay record, `PCNTR3` write, read-back — DIR ~7 µs after every reversal tick | §19.2.3 |
| One-shot arming | the DTC is armed for one transfer per tick; on completion the hardware clears `IELSRn.DTCE` and raises the CPU interrupt. An unarmed tick writes nothing → the rate holds (R12); the interrupt tells the firmware the tick happened. **As built:** the NVIC line stays disabled; the firmware learns of the tick from `poll_tick` (GPT165's count) | §17.4 ("On completion of a specified round…") |
| Stop | in the high phase: clear `CST` at once (critical section, counter checked first). In the low phase: arm the hardware stop on `GPT0_CCMPB` (`GTPSR.PSELCx`) — ends at the pulse end. `CST` clears on a hardware stop. **As built (follow-up 1):** that raced the counter and cut pulses (0.65 % of stops at 55 733/s in release, 27 % unoptimised); `stop()` now writes the longest period to `GTPBR`/`GTPR` (no pulse can start), lets a pulse in flight end, then clears `CST` | §22.3.7.2, §22.2.12, §17.2.10 |
| Restart | hardware start on the tick event (`GTSSR.SSELCx`) with GTCNT preloaded while stopped (writes to GTCNT are prohibited while counting) | §22.3.7.1, §22.3.1.1(6) |
| Rate 0 | the tick writes the longest period (no next pulse); at the following tick the HAL stops the counter, before the ~89 s wrap. **As built (5c):** stopped at the first poll after 1 s idle | design |

Channels used: GPT320 (STEP), GPT321 (count), GPT164 (tick), GPT165 (tick
number, added in step 4). ELC GPT
inputs used: two of eight (pulse end; tick). Rate error at a reversal tick
inside a low time (R11): affects at most ~1 reversal tick in 80 (rate before
a reversal ≤ 2,421 pulses/s; low time ≤ 1.2 % of the period).

## 4. API

The HAL owns DIR, because the tick writes it. This section was a sketch in
the plan; the Files table and the signatures now show the API as built
(2026-11-04).

### Repo conventions the code follows (AGENTS.md, Library conventions)

- **One type per file.** Every `pub` struct/enum in its own file, with the
  free functions and constants that exist only for it; `mod.rs` carries the
  module docs (hardware facts with manual citations, as `pwm/mod.rs` does)
  and the re-exports.
- **One aggregated `use` tree per root, `crate::` for internal paths, no
  globs** — in the HAL, the firmware and the `validate` CLI alike.
- **`#![deny(missing_docs)]`:** every public item and field has a `///`
  doc; pin docs show both board maps (`D4 — WiFi = P106, Minima = P103`).
- **Board maps:** `wifi`/`minima` exclusive; the D4 constructor is
  `#[cfg(feature = "wifi")]` (Minima D4 = P103 is not on GPT320).
- **No panics in the library:** constructors and calls return `Result`
  with a module error enum and a `…Result<T>` alias (the `pwm`
  `PeriodError`/`PeriodResult` pattern).
- **Exactly-once claims:** every PAC token (`GPT320`, `GPT321`, `GPT164`,
  `GPT165`, `DTC`) is consumed by the constructor; `Elc` is claimed by
  `Elc::new` and borrowed by `d4`. `Peripherals` gains `elc`, `dtc`,
  `gpt164` and `gpt165`.
- **Host tests** in `src/tests/<module>.rs`, named
  `given_<setup>_when_<action>_then_<expectation>`, bodies commented
  `// given` / `// when` / `// then` — the period/rate maths, the tick
  arithmetic and the schedule are pure functions so they run on the host.
- **Doc examples** `no_run` where they need hardware; examples compile.

### Files

| Path (`crates/unor4-hal/src/`) | Holds |
| --- | --- |
| `elc/mod.rs` | ELC module docs (§18), re-exports |
| `elc/elc.rs` | `Elc`: the claimed ELC — module-stop release, `ELCON`, routing |
| `elc/elc_event.rs` | `ElcEvent`: the event numbers this crate routes (Table 18.3) |
| `elc/gpt_input.rs` | `GptInput`: the eight ELC GPT inputs A–H (`ELSR0–7`) |
| `elc/software_event.rs` | `SoftwareEvent`: `ELC_SWEVT0`/`1` |
| `step/mod.rs` | module docs (hardware facts with citations), re-exports |
| `step/step_generator.rs` | `StepGenerator` and its WiFi `d4` constructor |
| `step/step_config.rs` | `StepConfig` |
| `step/tick_report.rs` | `TickReport` |
| `step/schedule.rs` | `Schedule`: the periods at a tick (host-tested) |
| `step/step_error.rs`, `step/step_result.rs` | `StepError`, `StepResult<T>` |
| `step/calc_step_period.rs` | `calc_step_period`, `ns_to_counts` (host-tested) |
| `step/pulses_in_tick.rs` | `pulses_in_tick`, `PULSE_END_LATCH_DELAY` (host-tested) |
| `step/tick_dtc.rs` | `TickDtc` (`pub(crate)`): the one-shot DTC on the tick's ICU slot |
| `step/transfer_info.rs`, `step/vector_table.rs`, `step/pending.rs` | the DTC record layout, its vector table, a queued change (`pub(crate)`) |
| `tests/step.rs`, `tests/elc.rs` | host tests |

### Signatures (as built)

```rust
pub struct StepConfig {
    pub low_ns: u32,       // ≤ 400 counts (8.3 µs); cart-pole 5 000
    pub min_high_ns: u32,  // ≥ 1 µs; cart-pole 2 500
    pub dir_setup_ns: u32, // ≠ 0; cart-pole 5 000
    pub forward: PinState, // DIR level for positive rates
}

pub struct TickReport {
    pub tick: u32,                   // GPT165, wrapping
    pub pulses: u32,                 // latched count, wrapping
    pub phase_counts: u32,           // GPT320's counter at the tick
    pub skipped: u32,                // ticks not polled since the last report
    pub applied: bool,               // a queued change landed at this tick
    pub old_pulse_first: bool,       // 5a's race: old pulse inside the write latency
    pub expected_phase: Option<u32>, // the schedule's prediction of phase_counts
    pub position: i64,               // signed pulses as of this tick
    pub dir_changed_at: Option<u32>, // reversal: DIR write, counts after the tick
}

impl StepGenerator {
    #[cfg(feature = "wifi")]
    pub fn d4(gpt: GPT320, count_gpt: GPT321, tick_gpt: GPT164,
              tick_count_gpt: GPT165, dtc: DTC,
              step: Pin<'1', 6, Input>, dir: Pin<'1', 7, Input>,
              elc: &mut Elc, config: StepConfig, clocks: &Clocks)
              -> StepResult<Self>;
    pub fn run(&mut self, rate: i32) -> StepResult<u32>;   // start while stopped
    pub fn queue(&mut self, rate: i32) -> StepResult<u32>; // for the next tick
    pub fn stop(&mut self);                                // trip: next pulse boundary
    pub fn poll_tick(&mut self) -> Option<TickReport>;
    pub fn predicted_pulses(&self) -> Option<u32>;
    pub fn pulses(&self) -> u32;
    pub fn position(&self) -> i64;
    pub fn period_counts(&self) -> u32;  // the running period; 0 idle/stopped
    pub fn schedule(&self) -> Option<Schedule>;
    pub fn dir_write_window(&self) -> (u32, u32);
    pub fn is_running(&self) -> bool;
    pub fn low_counts(&self) -> u32;
    pub fn tick_counts(&self) -> u32;
    pub fn set_tick_wake(&mut self, on: bool); // follow-up 2: the tick wakes WFI
    pub fn since_tick_counts(&self) -> u32;    // PCLKD counts since the tick
    // diagnostics: phase_counts, dtc_latency_probe, dtc_delay_probe
}
```

`StepError`: `RateTooHigh`, `ZeroRate` (`calc_step_period` only),
`ConfigInvalid`, `Running`, `NotReady`, `TooLate`. Against the sketch:
the missed-queue report is `TickReport::applied == false` plus
`period_counts()` (the rate actually running), not `missed`/`running_rate`
fields.

Channel choice: GPT164 for the tick, because no PWM constructor uses it.
GPT321 counts, so `Pwm::d3` (WiFi D3 = GTIOC1A on GPT321) cannot coexist
with the step generator in one build; the rig uses D3 as a limit-switch
input. D5 (P107, GPT320's A output) is DIR, so `Pwm::d5` cannot coexist
either. The validator image (Slice 7's D6 PWM is GPT163) is unaffected.

Firmware loop per tick: `poll_tick()` → work out the next rate (soft limit
from `predicted_pulses()`) → `queue(rate)` at least 40 µs before the next
tick.

## 5. Build steps

Each step lands with its bench proof in the validation pair (R4 request,
R3 check, `validate` CLI where needed) and passes `validate all` before the
next starts.

| Step | Code | Bench proof | Done when |
| --- | --- | --- | --- |
| 1 | ELC driver: module stop release (`MSTPCRC.MSTPC14`), `ELCR.ELCON`, `ELSRn` routing, software event (`ELSEGRn`) | R4 routes the ELC software event to GPT321's count-up; fires N software events; replies the count | count = N for N = 1, 100, 10,000 |
| 2 | Step generator on D4: pin mux, polarity, pulse shape, fixed rate, `stop()` | R4 runs fixed rates (1, 1,000, 44,600, 55,733/s); the R3 timestamps every falling edge on D8 (ICP1); at 1/s it also times the low width; repeated `stop()` at random phases | period within the quantized rate ± 1 count; low 5.0 µs (± R3 resolution 0.5 µs); no gap < 2.5 µs; line HIGH after every stop |
| 3 | Exact count through the ELC | R4 reports the ELC count after bursts and stops; the R3 counts the same edges | R4 count = R3 count, every run, at 55,733/s too |
| 4 | 2 ms tick, count and phase latched at each tick, `poll_tick()`, `predicted_pulses()` | R4 at fixed rates reports per-tick latched deltas and predictions | every delta = rate × 2 ms ± 1; prediction = latched value; R3 total = R4 total. The tick period itself is the R4's own claim here (no tick pin reaches the R3); 5a confirms it externally |
| 5a | DTC writes GPT320 `GTPR`/`GTPBR` at the tick (rate only, one direction); one-shot arming | R4 queues a rate staircase; R3 timestamps STEP | every period change lands on the 2 ms grid (R3 clock); no gap < 2.5 µs; counts match |
| 5b | DIR at the tick (DTC `POSR`/`PORR`), including a reversal tick inside a pulse's low time | R4 plays reversals across 0 within a tick, some timed to land in a low time; R3 timestamps STEP and reads DIR (D7) at each STEP edge | DIR correct for every pulse; no DIR change while STEP is LOW; signed position = R3's signed count. Needs the R4 D5 → R3 D7 wire |
| 5c | Missed queue, `stop()` disarming the tick, rate 0, long idle | R4 skips a queue on purpose, trips `stop()` at random phases, idles at rate 0 for > 89 s once | missed tick holds the rate and is reported; stop ≤ 5 µs; no pulse after a trip until the next queue; no pulse during the idle |

## 6. Bench

| Wire | Status | Use |
| --- | --- | --- |
| R4 D4 → R3 D8 (ICP1) | wired 2026-11-03; every S11 check uses it | STEP timestamps (Timer1 input capture; a missed edge shows as a doubled interval) |
| R4 D5 → R3 D7 | wired 2026-11-04 | DIR read in the STEP capture ISR at each STEP fall (`S11 reverse`); a read after the pulse has ended is counted as uncertain |

The R3's capture ISR has one pulse period to run (17.9 µs = 287 cycles at
55,733/s, interrupt response ≥ 4 cycles, ATmega328P datasheet §7.7.1).
**Unverified:** that the R3's other interrupts (Timer0 `millis`, serial)
never block longer; a block would show as a doubled interval, not a silent
miss.

## 7. Open items

| Item | Status |
| --- | --- |
| `GTPR` written while counting | works as designed: every queued change applied and every prediction exact (5a, 5b, 5c) |
| DTC latency | measured: first record 14–17 counts after the tick; a delay record of N words 2N + 22…32 counts |
| `stop()` latency | ≤ the rest of a pulse in flight + the code's own time (release: ≤ low time + ~0.5 µs; unoptimised: ~10 µs more); never cuts a pulse (follow-up 1) |
| ELC counting at 55,733/s | exact in every run (steps 3–5c), apart from the one open miscount below |
| Queue guard window | 40 µs (1 920 counts) before the tick |
| R4 D5 → R3 D7 wire | wired 2026-11-04 |
| One-edge miscount | **fixed** (follow-up 1): `stop()` cut pulses; the race is gone; 0 cut pulses in 394 000 release and 10 000 unoptimised stops |
| Cart-pole acceptance | accepted 2026-11-04: 5a's late old pulse, 5b's DIR ~7 µs after the tick, the R13 stop at up to 5.5 µs |
| Tick wake-up | **done** (follow-up 2): `set_tick_wake(true)`; every tick seen ≤ 1.9 µs after it with the CPU in WFI, 2 500 ticks |
| UNO R4 Minima | no hardware; Minima D4 is P103 (not a GPT320 pin), so the D4 constructor is WiFi-only |

## 8. Handover to cart-pole (2026-11-04)

Paste-ready summary for the cart-pole session. Everything here was proven
on a UNO R4 WiFi by the validation pair (README, Slice 11). unor4-hal
0.2.0 is unreleased: use the git dependency until it is published.

**Please confirm you accept these two deviations from the agreed rules,
and the R13 stop figure at the end of this section:**

1. **R8, late old pulse (step 5a).** If the old period ends within about
   0.35 µs after a tick, its pulse fires before the DTC's write. The
   transition period then follows that pulse, so that one period runs
   longer than R8 asks (by up to the old remainder + 2.5 µs). This happened
   at about 2 % of ticks in random 30 000–55 733/s runs. It is reported
   (`TickReport::old_pulse_first`) and predicted exactly.
2. **R11, DIR timing (step 5b).** A reversal flips DIR about 7 µs after
   the tick (326–342 PCLKD counts), every time. It is not at the tick,
   and not at the end of a pulse in flight. The next pulse comes at least
   `dir_setup` after the latest allowed DIR write, so with your timing
   the first pulse after a reversal is at least ~13 µs after the tick.
   DIR still never changes while STEP is LOW, and it stays ≥ 5 µs ahead of
   the next pulse. A reversal while idle (rate 0) or stopped is set by the
   CPU at queue time, ≥ 40 µs before the tick.

**Set-up**

```rust
let mut elc = Elc::new(p.elc);
let mut stepper = StepGenerator::d4(
    p.gpt320, p.gpt321, p.gpt164, p.gpt165, p.dtc,
    p.pins.d4,                 // STEP, P106; rests HIGH, pulses LOW
    p.pins.d5,                 // DIR, P107
    &mut elc,
    StepConfig {
        low_ns: 5_000,
        min_high_ns: 2_500,
        dir_setup_ns: 5_000,
        forward: PinState::High, // the DIR level your wiring gives +rates
    },
    &p.clocks,
)?;
```

It uses GPT320, GPT321, GPT164 and GPT165, the DTC, ELC GPT inputs B and C,
one ICU slot, and pins D4 and D5. `Pwm::d3` (GPT321) and `Pwm::d5`
(GPT320, and the DIR pin) can't be used alongside it. `UartIrq` on SCI1
keeps working: the validator runs both in one image.

**Per-tick loop**

```rust
stepper.set_tick_wake(true);                // the tick wakes WFI
unsafe { intc::enable_interrupts() };       // so its acknowledgement runs
loop {
    cortex_m::asm::wfi();                    // wakes at each tick (and UART)
    if let Some(t) = stepper.poll_tick() {   // at least once per 2 ms
        // t.pulses, t.position: latched by hardware at the tick instant
        // t.applied == false: no change landed; the rate held
        //   (stepper.period_counts() is what runs; 0 = idle/stopped)
        // t.skipped > 0: you polled too late; queue refuses until restart
        let predicted = stepper.predicted_pulses(); // Option<u32>
        let rate: i32 = /* your policy; sign = direction */;
        stepper.queue(rate)?;                      // ≥ 40 µs before the tick
    }
    // limit switch tripped: stepper.stop() — never cuts a pulse; returns
    // after the rest of a pulse in flight (≤ low time + ~0.5 µs in a
    // release build), cancels anything queued
}
```

- `queue` errors:
  - `TooLate`: within 40 µs of the tick.
  - `NotReady`: the latest tick not polled, the schedule not known yet
    (one full tick after `run`/`stop`), or a change already queued.
  - `RateTooHigh`: the pulse shape doesn't fit.
- `queue(0)` stops pulses at the tick. An idle generator stops itself
  after 1 s. `queue(rate)` while stopped starts it at the tick, with the
  first pulse `min_high` after it.
- **Counts:** a pulse counts when it ends. A pulse that ends within
  2 PCLKD counts after a tick counts at the next tick
  (`PULSE_END_LATCH_DELAY`, measured).
- **Position:** a pulse that started before a DIR flip counts in the old
  direction.

**Timing facts (measured on the board unless noted)**

| Fact | Value |
| --- | --- |
| Tick | 96 000 PCLKD counts (2 ms on the HOCO, ±1 %, datasheet) |
| Rate | 48 MHz ÷ round(48 MHz ÷ rate): error ≤ rate ÷ 96 000 000 |
| Rate ceiling with your pulse shape | period ≥ 361 counts (~133 000/s); proven to 55 733/s |
| Rate change at the tick | DTC write 14–17 counts after the tick |
| Low time limit | `low_ns` ≤ 400 counts (8.3 µs) |
| Count | exact: the one miscount seen was `stop()` cutting a pulse (the cut pulse was never counted), fixed 2026-11-04; 0 cut pulses in 394 000 random-phase stops since, hardware count = independent edge count in every run |

`R6`, `R7`, `R9`, `R10`, `R12` and `R15` hold as agreed; the README's
Slice 11 record has each run.

**R13, one more figure to note:** `stop()` never cuts a pulse, and it
returns once the counter has stopped. That takes at most the low time plus
~0.5 µs in a release build: 5.5 µs with your shape, not the agreed
≤ 5 µs. The worst case is a call just as a pulse starts: that pulse runs to
its end. (Updated 2026-11-04 after the stop fix. The first `stop()` could
cut a pulse; see the README, Slice 11 follow-up 1. In an unoptimised build
the stop takes ~10 µs longer, but still never cuts a pulse.)

## 9. Out of scope

- The per-tick computation — the ramp, the jump limit m and the soft limit,
  and queueing each next rate between ticks: cart-pole firmware. The 2 ms
  tick itself (R7) and what it applies and latches are the HAL's.
- The limit-switch filter (`trip_us`): cart-pole firmware; it calls
  `stop()`.
- Multiple steppers: possible on GPT321 as a second STEP channel only if
  counting moves to 16-bit channels (§3 channels); not requested.
