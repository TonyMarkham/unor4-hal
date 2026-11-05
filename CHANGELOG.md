# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - Unreleased

### Added

- **Wake timer (`wake::WakeTimer`)** — asked for by the cart-pole firmware,
  whose only CPU wake-up for waits that are not the motor's was the step
  generator's 2 ms tick.
  - **What it is:** a one-shot timed wait on any GPT channel. Its overflow
    stops its own counter in hardware through an ELC input (`GTPSR`), so
    it fires once; the same overflow ends a `WFI` through one ICU slot.
  - **Setup:** `WakeTimerBuilder::new().with_timer::<GPT167>()
    .with_input::<ElcE>().build()` in a named `const`. At run time,
    `WakeTimer::new(WAKE).with_peripherals(p).build()` claims the channel
    and the input by type.
  - **Calls:** `start(Duration)`, `expired()`, `cancel()`, `is_armed()`,
    `longest()`, `free()`. Each wait gets the finest prescaler that reaches
    it and rounds up: 20.8 ns steps up to 1.365 ms, up to 1.398 s on a
    16-bit channel. Past that, `WakeError::TooLong`.
  - **Behaviour:** `expired()` reads the overflow flag, so it works in a
    masked sleep. `start` and `cancel` forget an earlier end, read or not.
  - **Verified on WiFi (`S12 wake`):** nine waits from 5 µs to 1.398 s each
    at least its length and within one step, one `WFI` wake each, the
    counter stopped 2 counts after its overflow; the documented behaviour;
    1 ms and 100 ms timed by the R3 on a wire (1 008 µs, 100 080–100 112
    µs, the R3's clock reading about 0.1 % long).
  - **Found on the bench, fixed before release:** `start` first divided in
    128 bits, a software routine here, adding about 37 µs per prescaler
    tried before the count began. The clock constants are now worked out
    at build, and `start` divides nothing.
- **ELC inputs claimable by type (breaking).** `ELC_GPTA`–`ELC_GPTH` are
  `Peripherals` parts (`elc_a` … `elc_h`, types `ElcA` … `ElcH`).
  - The step generator claims B, C and D in `with_peripherals` and gives
    them back in `free`; its setup does not change.
  - `Elc::link` and `Elc::unlink` take a claimed input (`&p.elc_a`) instead
    of a `GptInput`, so two users of one input fail `cargo check`.

- **Interrupt wake-ups instead of polling** (asked for by the cart-pole
  firmware, which sleeps in `WFI`).
  - **Switch wake-up:** `SwitchPinBuilder::with_wake()` (needs
    `with_edge()`, refused at `cargo check` without it:
    `` with_wake: the wake-up is the edge latch's interrupt; add with_edge() ``).
    The activation edge interrupts the CPU, so it ends a `WFI`.
    `activated()` still reports the edge after the interrupt ran: the
    dispatcher now records each request it acknowledges
    (`Ielsr::take_request`). Verified on WiFi (`S2 wake`): a press on D3
    woke the R4 from `WFI` with no other wake-up, and `activated()`
    reported it.
  - **Stop at an exact pulse count:** `StepGenerator::stop_at(count)`. The
    counter's compare match B stops the STEP timer in hardware through ELC
    input D (`GTPSR.PSELCD`), at the end of pulse `count`, with no pulse
    after it. `set_count_wake(true)` makes the same match wake the CPU;
    `stopped_at_count()` reports it once; `cancel_stop_at()` cancels.
    New errors: `CountNotAhead`, `CountTooFar` (a 16-bit counter reaches
    65 535 ahead), `ReversalWithCountStop`. The generator's build now
    claims two ICU slots (the tick's DTC and the count stop's wake-up).
    `GptChannel::COMPARE_B_EVENT` is new. Verified on WiFi
    (`S11 countstop`): 15 moves of 1 to 65 535 pulses at 1 000, 30 000 and
    55 733/s, armed before and during a move, each ending at exactly its
    count, the R4's count equal to the R3's edges; one move with only the
    count stop to wake the CPU (1 wake-up). The validator counts on a
    16-bit timer; the cart-pole rig ran the count stop on GPT321, a 32-bit
    one (its report, 2026-11-04).
  - The README and module examples sleep in `WFI` instead of polling.

- **ELC (`elc`)** — the Event Link Controller, Slice 11 step 1: `Elc`
  (module-stop release, inherited GPT routes cleared, `ELCON`), `link` /
  `unlink` of an `ElcEvent` to one of the GPT's eight inputs
  (`GptInput::A`–`H`), and software events (`fire`). `Peripherals` gains
  `elc`. Verified on WiFi: GPT321 counted every routed software event
  (1, 100, 10 000) and none after `unlink`.
- **Step generator (`step`)** — Slice 11 step 2, WiFi only: `StepGenerator`
  on header D4 (P106 → GTIOC0B, GPT320) with a fixed LOW pulse and a
  minimum HIGH (`StepConfig`), `run(rate)` for any whole rate the pulse
  shape allows (1 pulse/s to ~133 k/s with the cart-pole 5 µs/2.5 µs
  shape), and a `stop()` that never cuts a pulse (software stop while HIGH,
  hardware stop on the pulse's end while LOW). `calc_step_period` /
  `ns_to_counts` are host-tested. Verified on WiFi at 1, 1 000, 44 600 and
  55 733 pulses/s with 50 clean stops.
- **Exact pulse count** — Slice 11 step 3: `StepGenerator::d4` also claims
  GPT321, which counts every completed pulse in hardware through the ELC
  (no CPU interrupt); `pulses()` reads it, 32-bit wrapping. Verified on
  WiFi: equal to an independent count in 20/20 start/stop runs (72 444
  pulses, up to 55 733/s, stops timed mid-pulse).
- **2 ms hardware tick** — Slice 11 step 4: `StepGenerator::d4` also claims
  GPT164 (the tick: 96 000 PCLKD counts) and GPT165 (tick numbers);
  `Peripherals` gains `gpt164` and `gpt165`. At each tick the pulse count
  and GPT320's phase are captured at one instant; `poll_tick()` returns
  them as a `TickReport` and `predicted_pulses()` the count due at the next
  tick. `pulses_in_tick` and `PULSE_END_LATCH_DELAY` (2 counts, measured on
  the board) are public and host-tested. Verified on WiFi: every
  prediction exact over 27 runs including 2 500-tick stress runs that
  sweep every phase.
- **Rate changes at the tick** — Slice 11 step 5a: `StepGenerator::d4`
  also claims the DTC (`Peripherals` gains `dtc`); `queue(rate)` arms a
  one-shot DTC transfer that writes the period in progress and the new
  steady period at the next tick instant and reads the counter back, so
  `poll_tick` knows the outcome (`TickReport::applied`,
  `old_pulse_first`, `expected_phase`). `Schedule` (public, host-tested)
  describes the periods at a tick; new errors `NotReady` and `TooLate`.
  Verified on WiFi: every change applied at its tick and every prediction
  exact over 2 654 queued ticks, the tick grid confirmed from outside (burst
  starts 4 ms apart on the R3).
- **Rate 0, idle stop and start at the tick** — Slice 11 step 5c:
  `queue(0)` stops pulses at the tick while keeping the time since the last
  one; an idle generator stops itself after 1 s; `queue(rate)` from stopped
  starts it at the tick by hardware; a tick without a queue holds the rate;
  `stop()` cancels anything queued. Verified on WiFi in a 618-tick scenario,
  every prediction exact.
- **Direction and position** — Slice 11 step 5b: rates are signed
  (`run(i32)`, `queue(i32)`); `StepGenerator::d4` also claims header D5
  (P107) as DIR, and `StepConfig` gains `dir_setup_ns` and `forward` (the
  DIR level for positive rates). A reversal at a running tick is written by
  the same one-shot DTC chain: the period write, a delay record, the DIR
  write to `PORT1.PCNTR3`, then the counter read-back. DIR lands after any
  pulse LOW at the tick has ended, 326–342 PCLKD counts (~7 µs) after the
  tick, and the next pulse is placed at least `dir_setup` after the latest
  allowed write (`dir_write_window()`). A reversal while idle or stopped is
  set by the CPU at queue time. `position()` and `TickReport::position`
  give the signed pulse count; `TickReport::dir_changed_at` gives when DIR
  was written. `low_ns` is limited to 400 counts (8.3 µs). Verified on WiFi:
  500 reversals at ±15 000–30 000/s, a ramp through zero and a start from
  stopped; the DIR rule held at every reversal, and the R4 position equalled
  the R3's DIR-signed pulse count in every run.

- **Tick wake-up** — Slice 11 follow-up 2, a cart-pole request:
  `StepGenerator::set_tick_wake(bool)` makes every 2 ms tick raise a CPU
  interrupt (the tick's ICU slot, acknowledged by the HAL's vector table;
  no handler needed), so firmware can sleep in `WFI` between ticks;
  `since_tick_counts()` gives the time since the tick. `intc::Ielsr`
  gains `disable()`. Verified on WiFi: 2 500 ticks of changing rates with
  the CPU asleep between ticks, every tick seen at most 1.9 µs after it.

- **Driver enable (ENA) for the step generator** — a cart-pole request,
  generic by design: `step::EnaPinBuilder` (any header pin, the enabling
  level with no default, the lead from enable to the first DIR change or
  pulse) builds an `EnaSetup`, added with `StepSetup::with_ena`. That
  call refuses ENA on the PUL or the DIR pin at `cargo check`. The
  builder also refuses an enabling level the board itself holds while
  the pin is not driven (reset, the bootloader): WiFi D13 is pulled
  LOW by R52, so ENA there cannot enable on LOW. The
  hand-over gains `.ena(pin)`, which is required exactly when the setup
  has ENA. `StepGenerator::enable` / `disable` / `is_enabled` drive the
  pin; until the lead has passed, a non-zero rate returns the new
  `StepError::Disabled` / `Enabling`. `free` disables the driver and
  returns the pin. Setups without ENA are unchanged (`NoEna` is the
  default type parameter). Not run on the validation rig (it has no ENA
  pin); the cart-pole rig ran it (its report, 2026-11-04).
- **Switch inputs (`gpio::SwitchPinBuilder`)** — a cart-pole request (two
  limit switches), generic by design: any header pin, a required contact
  type (`gpio::contact::No` / `Nc`: which level is active), a required
  pull (`Pull::Internal` / `External(ohms)`, checked at `cargo check`
  against the board's own loads: WiFi D13's 100 kΩ pull-down needs an
  external pull-up of at most 25 kΩ to reach VIH), and an optional IRQ digital filter
  (`SwitchFilter`, only on pins with an `IRQn`). `Switch` reads
  `is_active()` through the contact type. `Switch::with_edge` (IRQ pins
  only) latches the edge into the active level in hardware, so a short
  press between polls is not missed (`activated()`). It refuses a pin
  whose IRQ number another slot already uses (`SwitchError`). Run on the
  validation rig: a normally-open switch on D3, internal pull-up, filter,
  edge and wake-up (`S2 wake`). Run on the cart-pole rig (its report,
  2026-11-04): normally-closed switches on D2 and D3 with the edge
  wake-up. A switch with an external pull-up: run on the validation rig
  (`S2 pullup`), the R3's pin pull-up as the pull-up.
- **`gpio::HeaderPin`** — marks the Arduino header pins. DIR, ENA and
  switch pins require it, so a board-internal pin (`UsbMux`, the ESP32-S3
  bridge's `Sci9Tx`/`Sci9Rx`, `Sci1Tx`/`Sci1Rx`) fails `cargo check`.
  The pin table records the board's own loads on headers (WiFi:
  only D13, from schematic ABX00087; Minima: none recorded,
  unverified).
  `intc::Event::gpio_irq` maps an IRQ number to its ICU event.

### Fixed

- **The ENA lead could end up to one tick-timer count (83 ns) short.**
  `is_enabled` compared the counts elapsed with the lead, but `enable`'s
  timestamp can be taken just before the counter steps, so those counts
  prove one count less of real time. It now needs one count more: the lead
  is never short, and `is_enabled` turns true at most 2 counts (167 ns)
  after it. Host-tested (`tests/ena.rs`). On the bench a 5 µs lead read
  331–342 counts from `enable()` to `is_enabled()` (at least 240), the
  call times included.
- **`StepGenerator::cancel_stop_at` could report a stop at a wrong count**
  (found by the new `S11 cancelrace` bench check). It removed the stop
  source, then read the compare flag. A match landing between the two
  set the flag without stopping the train. Cancel then reported a stop,
  gave up waiting for it, and stopped the train itself, past the count: 15
  of 2 000 swept cancels at 55 733/s. It now reports a stop only if the
  STEP timer stopped; otherwise the cancel wins and the train runs on. 0
  of 2 000 in each of five runs after the fix.
- **`queue` could stay `NotReady` for good after ticks were missed**
  (found by the new `S11 sleep` bench check). The schedule rebuild refused
  whenever the STEP counter had wrapped since the tick. At a rate that
  fits the 2 ms tick a whole number of times, the tick lands at the same
  phase every time. When that phase was within a poll's latency of a
  period's end (118–260 counts measured), every poll refused: 6 of 24
  sleeps at 10 000, 40 000 and 50 000/s still had no schedule after 10
  polls. The rebuild now proves a wrapped steady train from where the
  counter must stand. It also retakes reads that straddle a wrap. 18 of
  18 sleeps then queued at the first poll. Two host tests that expected
  the refusal now expect the rebuild, and four new ones cover what must
  still be refused.
- **Docs: `poll_tick` is not needed every 2 ms at a steady rate**
  (cart-pole dogfooding). The README and the step module said "at least
  once per 2 ms". Once per tick is needed only while the rate changes;
  at a steady rate the pulse train runs without the CPU and the next
  poll catches up (new README section "Sleeping through ticks"). A
  16-bit counter still needs a read every 32 768 pulses. Wording only.
- **The DIR set-up time was not always waited** (cart-pole dogfooding).
  - **`run`:** after a direction change, the first pulse came the PUL
    minimum HIGH time after DIR, not the DIR set-up time. Measured on the
    board with a 5 µs set-up and a 2.5 µs minimum HIGH: 2.00–2.50 µs before;
    4.63–5.06 µs after, with the R3 reading up to 1.56 µs short.
  - **`queue` from stopped or idle:** DIR was written at least 40 µs before
    the tick and the first pulse came the minimum HIGH time after it, so a
    set-up over about 40 µs was not met. The first pulse now waits what is
    left of the set-up; when that does not fit in one period, the first
    period is longer, and the schedule and predictions account for it.
    Host-tested only: the bench's 5 µs set-up never needs it.
  - **`DirPinBuilder`** now refuses a set-up over 1 ms
    (`` with_setup: DIR set-up time over 1 ms ``), which keeps `run`'s
    first pulse inside the 2 ms tick that follows.
  - Bench check `S11 dirlead`: the R3 times every DIR change to the next
    STEP pulse, for starts through `run` (±1 000/s and ±120 000/s) and
    `queue` (from stopped, and through idle). `validate all` 26/26.
- **`StepGenerator::is_enabled` took `&mut self`** for a latch; it takes
  `&self` now (the latch is a `Cell`).

- **Ticks passed over left the step generator stuck** (Slice 11 follow-up
  3, reported by the cart-pole session).
  - **Idle:** the 1 s idle stop needed a schedule, so after a missed tick it
    never fired, and a stray pulse came every 89.48 s.
  - **Moving:** every `queue()` was refused until `stop()`.
  - **Missed reversal:** a reversal applied at a missed tick left `position`
    counting with the wrong sign.
  - The idle stop now reads the registers and the captured phase. The
    schedule is rebuilt from `GTPR`/`GTPBR` once the counter proves no
    overflow since the tick. A missed reversal is split exactly and applied.
  - `TickReport::applied` also covers a change applied at a passed-over
    tick.
  - Verified on WiFi: `S11 skip` 3/3.

- **`StepGenerator::stop()` could cut a pulse** (Slice 11 follow-up 1).
  The cut pulse was never counted, and it was shorter than the driver's
  minimum low time. The old stop raced the counter with a 24-count margin.
  At 55 733/s it cut 0.65 % of stops in a release build and 27 % in an
  unoptimised one. It showed up as a one-edge miscount on the bench.
  `stop()` now writes the longest period first, so no pulse can start,
  lets any pulse in flight end, and then stops. That doesn't depend on how
  fast the code runs. Verified on WiFi: 0 cut pulses in 394 000 random-phase
  stops (release) and 10 000 (unoptimised); the hardware count equalled an
  independent edge count in every burst.
- **`Peripherals` had no `gpt162`**, so `Pwm::d10` (WiFi) and
  `Pwm::d5`/`d10` (Minima) could not be called. `Peripherals::gpt162` is
  added.
- **The UART module claimed `PSEL` is set per SCI channel class** (even
  channels `00100b`, odd `00101b`). The manual's tables have exceptions:
  P102's `TXD2` is under `00101b` (Table 19.6), and `SCK1` is `00100b` on
  P109 but `00101b` on P112 (Table 19.7). `PSEL` is now a per-pin property
  from the pin table, and the docs say so.
- Broken or private intra-doc links (`ADST_POLL_LIMIT`,
  `Pwm::set_duty_cycle`, `UartIrq` without its feature, `StepGenerator` on
  the Minima map); `cargo doc` is warning-free with default features, with
  `usb,uart-irq`, and on the Minima map.

### Changed

- **Validation pair: the R3 validator builds as three parts.** Part 2 had
  reached 91 % of flash. Slice 11's later checks (dirlead, countstop,
  cancelrace, sleep, boot) and a new delay probe moved to part 3. Parts
  2 and 3 share the Slice 11 code, and the linker drops what a part never
  calls: 37 %, 74 % and 42 % of flash. All 32 checks pass over the three
  sessions. The cancel race now draws its moments at random each request:
  0 violations in 18 000 cancels over nine runs.
- **Validation pair: `cortex_m::asm::delay` is a minimum, not a time.**
  The delay probe measured 1.25 CPU cycles per count at its call site;
  the first `S11 sleep` version measured 3 at its own. The validator's
  wait comments now say "at least", and its timed waits run on the
  SysTick clock.
- **Validation pair: the R3 validator builds as two parts** (not part of
  the published crate). The checks had filled the R3's flash (32 220 of
  32 256 bytes). `r3-validator.ino` now builds with `R3_PART=1`
  (Slices 1–10, 11 864 bytes) or `R3_PART=2` (Slice 11, 26 108 bytes), each
  uploaded for its own `validate all` session. The CLI reads the loaded
  part from the R3 and shows the other part's checks as NOT RUN. Session 2
  passed 13/13. Session 1 first passed 16/17: S9 failed because the CLI
  opened the native port the moment it appeared, and Windows refused it
  ("The system cannot find the file specified."). S9 now retries the open
  for up to 10 s and prints the error if it still fails. Session 1 then
  passed 17/17 (the port opened after 227 ms): 28/28 over both sessions.
- **Validation pair: three behaviours that had never run on a board now
  have bench checks** (32/32 over the sessions):
  - `S11 cancelrace`: `cancel_stop_at` at 2 000 swept moments around the
    count match at 55 733/s. 680 stops and 1 320 cancels won, 0 broke the
    contract (a stop is exactly at the count and reported; a cancel leaves
    the train running, not reported); R4 count = R3 edges.
  - `S11 sleep`: 100 ms, 500 ms and 3 s with no generator call at a steady
    rate, then one poll and a `queue` (one of them a reversal). Every
    sleep recovered, `queue` accepted at the first poll (after the rebuild
    fix above), every later
    prediction exact, count and position = the R3's.
  - `S2 pullup`: a switch with `Pull::External`, the R3's pin pull-up the
    only pull-up on D3. The R4's own pull-up read off, D3 idle, the press
    woke the CPU, the release brought D3 back.
  - `S11 boot`: at boot, before any driver takes a field (the order
    rule), the validator builds a second setup (STEP on D3 = GPT321 output
    A, DIR on D5 with a 200 µs set-up). It changes direction four times
    through `queue`, each queued 60 µs before a tick, frees the generator
    while it runs, and builds the main one from what `free` gave back. The
    R3 resets the R4 and watches. DIR leads 212–216 µs, STEP falls R3 36 =
    R4 36 (no cut pulse, none after `free`), the pins back as inputs, the
    timer stopped.
- **One builder shape from setup to running driver (breaking).**
  - **Setup:** `StepSetup::new(PUL, DIR, COUNTER, TICK).with_ena(ENA)` is
    replaced by `StepGeneratorBuilder::new().with_pul(PUL).with_dir(DIR)
    .with_ena(ENA).with_counter(COUNTER).with_tick(TICK).build()`, a named
    `const` that cross-checks the parts. It now also refuses a missing part
    and a 16-bit STEP timer at the setup.
  - **Run time:** the old `StepGenerator::builder(SETUP).pul(…).dir(…)
    .counter(…).tick(…).dtc(…).ena(…).build(&mut elc, &clocks)` is replaced
    by `StepGenerator::new(SETUP).with_peripherals(p).build()`. It claims
    exactly the hardware the setup names and returns
    `Ok((generator, rest_of_p))`, or `Err((StepError::NoIcuSlot, p))` with
    `p` untouched.
  - **Freeing:** `free(SETUP, p)` puts the hardware back into `p`.
    `StepParts` is gone.
  - **Switches:** `Switch::new(pin, SETUP)` and `Switch::with_edge` are
    replaced by `Switch::new(SETUP).with_peripherals(p).build()`. The edge
    latch is part of the setup now: `SwitchPinBuilder::with_edge()` (IRQ
    pins only), and `with_filter` requires it. `free(p)` puts the pin back.
  - **`Peripherals`** is generated with one type per header pin, GPT
    channel and the DTC. A claimed part becomes `Taken` in the returned
    `Peripherals`, so a second use fails to compile:
    `` `D5` is already used by another driver `` through
    `with_peripherals`, or "expected `GPT320`, found `Taken`" directly. New
    module `peripherals`: `Claim`, `Release`, `Taken`, `Board`.
    `Peripherals.elc` is now the started `Elc` driver, not the PAC token.
  - **Pins:** `gpio::Pins` is generic over the 20 header pins' types
    (same field names, `p.pins.d5`).
  - **Build order:** build the `with_peripherals` drivers (and call their
    `free`) before moving fields out directly; a partly moved `p` can't be
    passed whole.
  - Verified on WiFi: `validate all` 25/25.

- **Pins are named by their Arduino header, and drivers check pins by
  trait bounds** (breaking). `gpio::Pin<const PORT: char, const PIN: u8,
  MODE>` is now `Pin<ID, MODE>`, where `ID` is the header's type
  (`pin_map::D6`; `p.pins.d6` is a `Pin<D6, Input>`) or a board-internal
  pin (`UsbMux`, `Sci1Tx`/`Sci1Rx`, WiFi `Sci9Tx`/`Sci9Rx`). One pin table,
  `gpio::pin_map`, gives each port pin's functions once — GPT output
  (`Gtioc`), ADC channel (`AdcPin`), DAC (`DacPin`), IRQ (`IrqPin`) and
  SCI/IIC/SPI/CAN signals (`SciPin`/`IicPin`/`SpiPin`/`CanPin`, with their
  `PSEL`) — from manual Tables 19.5–19.13, plus the header it is on per
  board; the `pins.csv` golden test checks the board columns. A wrong pin
  fails `cargo check` with a message naming it: `` `A0` has no GPT
  output ``, `` `D1` cannot be `SCI1` `Txd` ``.
  - `Pwm::d3`/`d5`/`d6`/`d9`/`d10`/`d11` → `Pwm::new(timer, pin, freq_hz,
    clocks)` on any of the 16 headers with a GPT output; the type is
    `Pwm<D6>`.
  - `Uart::sci2`/`sci9`/`sci1` → `Uart::new(sci, tx, rx, baud, clocks)` on
    any TX/RX pair the table gives the channel; `UartIrq::sci2`/`sci9`/`sci1`
    → `UartIrq::attach`. Each pin is muxed with its own `PSEL`.
  - `adc::Channel` is gone: `Adc::read` takes any pin with an `AdcPin`
    channel, which adds AN019/AN020 on WiFi D10/D13 (Minima D4/D5).
  - `UsbPort::take`, `take_usb_port` and `StepGenerator::d4` take the new
    pin types; `SciChannel` gains `release_module_stop`.

  Verified on WiFi: every path Slices 1–11 proved still works — `validate
  all` 24/24 with the rebuilt R4 image (SCI2/SCI9/SCI1 through the new
  constructors, D6 PWM, A0, USB, the step generator). Pins and functions
  no bench check exercises (PWM on the 10 new headers, ADC on D10/D13, the
  `PSEL` exceptions) are not yet proven on the board.
- **Minimum supported Rust version raised from 1.85 to 1.88** (breaking).
  The 0.1.x code already needed 1.88, so the 1.85 claim was wrong: the
  `usb` feature calls `<[T]>::as_chunks` (stable in 1.88, flagged by
  clippy's `incompatible_msrv` lint), and `usb`/`uart-irq` use `if let …
  && let …` chains. 1.88 is the lowest version consistent with those uses;
  it is not proven by a 1.88 build (the bench builds with current stable,
  1.99). Every dependency needs 1.85 or older.

[0.2.0]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.2.0

## [0.1.2] - 2026-10-21

### Added

- Crate packaging metadata: `keywords` (`arduino`, `embedded`, `hal`,
  `ra4m1`, `no-std`) and `categories` (`embedded`, `hardware-support`,
  `no-std`). crates.io reads these from the manifest at publish time and
  they cannot be added to an already-published version, so they land on
  this release.
- Manifest `rust-version = "1.85"` (inherited from the workspace), so
  users on older toolchains get a clear error instead of an opaque one.

[0.1.2]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.2

## [0.1.1] - 2026-10-21

### Added

- Crate packaging metadata: `repository` link and a crate-facing `README.md`
  (`crates/unor4-hal/README.md`), so crates.io renders them. Published
  versions are immutable, so 0.1.0's page cannot gain these — 0.1.1 exists
  to carry them; no code changed.

[0.1.1]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.1

## [0.1.0] - 2026-10-21

First release. An independent, `no_std` hardware abstraction layer for the
Arduino UNO R4 (Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 WiFi**
(default `wifi` pin map) and **UNO R4 Minima** (`minima` pin map) — exactly
one map per build, enforced with `compile_error!`. No fork, no copied code:
hardware facts are cited to the Renesas RA4M1 hardware manual and SVD; the
Arduino core and FSP are behavioral cross-check data only.

### Added

- **Clock** — HOCO 48 MHz initialization and the `Clocks` token resolving
  PCLKA/PCLKB/PCLKD/ICLK for the other drivers.
- **GPIO** — typestate pins (`Input`, `InputPullUp`, `Output`,
  `OutputOpenDrain`, `Alternate`, `Analog`) with a compile-time `Peripherals`
  exactly-once claim; both board pin maps shown in every pin's docs so the
  documentation never lies about the board you are not building. The compiled
  map is proven equal to a checked-in transcription of the Arduino core's
  board tables by a golden test (`src/tests/pins.rs` + `pins.csv`).
- **Delay** — cycle-accurate SysTick `DelayNs` implementation.
- **UART (polling)** — SCI2 (header D0/D1), SCI9 (the WiFi board's internal
  monitor line) and SCI1 (the WiFi AT line to the ESP32-S3) in 8N1 polling
  mode, with a baud calculator that refuses rates outside 3% of PCLKA
  (`calc_baud`).
- **`uart-irq` feature** — interrupt-driven `UartIrq` with ring buffers,
  carrying `embedded-hal-nb` serial, `embedded-io` and `core::fmt::Write`
  impls; 4 KiB bursts echo byte-exact with no overrun while a heartbeat
  continues on the main loop.
- **Interrupts (`intc`)** — the ICU/IELSR event table, event linking and a
  dispatch registry behind the vector table; the crate's `critical-section`
  provider.
- **PWM (`pwm`)** — GPT timer driver implementing `embedded-hal` 1.0
  `SetDutyCycle`; all six header PWM pins (D3, D5, D6, D9, D10, D11) have
  constructors with engine/chip-pin/output fixed at compile time and locked
  by a golden test; `calc_period` keeps every reachable period inside the
  16-bit `GTPR`.
- **ADC (`adc`)** — single-shot blocking reads on A0–A5 via the ADC140
  (`Channel` trait sealed to the six header pins), 12- or 14-bit resolution,
  and a host-tested `calc_scan_time` (`embedded-hal` 1.0 has no ADC trait;
  `Adc::read` is this crate's own API).
- **USB (`usb` feature)** — the USBFS module as a `usb_device::bus::UsbBus`
  adapter plus a blocking CDC-ACM wrapper over `usbd-serial`. Taking
  `UsbPort` explicitly flips the WiFi board's USB mux (disconnecting the
  ESP32-S3 bridge — documented behavior); recovery paths are `release`, the
  opt-in 1200-baud bootloader knock, or a double RESET.
- **Data flash (`dflash`)** — the 8 KB data flash as byte-programmed,
  block-erased storage: plain reads, byte programming (once per erase,
  blank-checked first — overwrites are refused), 1 KB block erase, and blank
  check by the hardware's blank-check command (an erased byte's read value is
  not guaranteed). Code flash is unreachable through the API; the
  mode-switch helpers run from SRAM via cortex-m-rt's `.data` copy, so no
  `memory.x` change is needed.

### Verified on hardware (UNO R4 WiFi)

Every item above was proven on the bench before release; the README records
each witnessed run, re-provable with the validation pair in
`arduino/validation/` + `crates/validation` (`validate all`, 16/16 on
2026-10-21). Highlights: SCI1 `AT` → `OK` from the stock bridge firmware;
PWM duty/period measured edge-by-edge by a controller board; ADC staircase
error within the datasheet budget; native CDC enumeration, echo and
bridge recovery; data-flash erase+rewrite byte-exact and a boot counter
that advanced once per boot — across resets and across **27 manual power
cycles** — with the log intact.

**UNO R4 Minima builds and passes the host test suite but is
hardware-unverified** — no Minima hardware exists in this project.

[0.1.0]: https://github.com/TonyMarkham/unor4-hal/releases/tag/v0.1.0
