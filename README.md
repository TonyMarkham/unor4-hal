# unor4-hal

A hardware abstraction layer (HAL) for the Arduino UNO R4 (Renesas RA4M1,
Arm Cortex-M4F), written independently: no fork, no copied code. Pins are
named by their Arduino header, and every driver checks its pins at
`cargo check`. The stepper parts and the switches go further: their builders
also check every number, so a wrong pin, timer or value fails the build
with a message naming it, before anything reaches the board. UART and PWM
don't have builders yet; they check their baud rate and frequency at run
time.

- **Boards:** UNO R4 WiFi (feature `wifi`, the default) and UNO R4 Minima
  (`minima`). Exactly one per build.
- **`no_std`**, no allocation.
- **Errors:** the builders refuse bad setups at `cargo check`. Some run-time
  constructors panic on a bad number instead: `Uart::new` / `attach` on a
  baud rate not reachable within 3 %, `Pwm::new` on an unreachable
  frequency, and `UartIrq::attach` when no interrupt slot is free.
- **Drivers:**
  - typestate GPIO;
  - polled and interrupt-driven UART;
  - PWM, ADC, USB CDC-ACM, 8 KB data flash;
  - the Event Link Controller;
  - a stepper-motor step generator (STEP, DIR, ENA);
  - switch inputs (limit switches, buttons).

Contents:
[Status](#status) ·
[Getting started](#getting-started) ·
[Pins](#pins) ·
[Builders](#builders-write-them-as-named-const) ·
[Stepper motor: PUL, DIR, ENA](#stepper-motor-pul-dir-ena) ·
[Switches](#switches-limit-switches-buttons) ·
[Wake timer](#wake-timer-timed-waits-that-wake-a-sleeping-cpu) ·
[UART](#uart) · [PWM](#pwm) · [ADC](#adc) · [Data flash](#data-flash) ·
[USB](#usb-cdc-acm) · [Features](#cargo-features) ·
[Testing](#testing) · [License](#license)

## Status

Version 0.2.0, unreleased (crates.io has 0.1.2). The API may still change.

- **Proven on a real UNO R4 WiFi:** every driver, through the two-board
  validation rig. It re-proves all of them in three sessions, one per
  part of the R3's checks (33/33, [Validation.md](Validation.md)), including
  STEP on both 32-bit timers and both outputs, `free`, a DIR set-up over
  `queue`'s 40 µs guard, a cancel racing the count stop, long sleeps, and
  the wake timer.
- **Run on the cart-pole rig, not on the validation rig** (its report,
  2026-11-04): the stepper's ENA pin, normally-closed limit switches on D2
  and D3 with the edge wake-up, and the count stop on a 32-bit counter
  (GPT321).
- **UNO R4 Minima:** builds and passes the host tests; there is no Minima
  hardware in this project.

## Getting started

### Tools

```powershell
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
```

Uploads use [arduino-cli](https://docs.arduino.cc/arduino-cli/installation/).

### A firmware project

`Cargo.toml`:

```toml
[dependencies]
unor4-hal    = { git = "https://github.com/TonyMarkham/unor4-hal.git" }  # or "0.2" once published
embedded-hal = "1.0.0"
ra4m1        = "0.2.0"   # the timer types named in setups (GPT164, …)
cortex-m     = "0.7.10"
# `device`: unor4-hal supplies the 32-entry interrupt vector table.
cortex-m-rt  = { version = "0.7.7", features = ["set-vtor", "device"] }
panic-halt   = "1.0.0"
```

`.cargo/config.toml`:

```toml
[build]
target = "thumbv7em-none-eabihf"
```

`memory.x`. The Arduino bootloader owns the first 16 KiB of flash, so your
code starts at `0x4000`; never set the origin to 0, which erases the
bootloader:

```text
MEMORY
{
  FLASH : ORIGIN = 0x00004000, LENGTH = 240K
  RAM   : ORIGIN = 0x20000000, LENGTH = 32K
}
```

`build.rs`, which passes `memory.x` and cortex-m-rt's `link.x` to the
linker:

```rust,ignore
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out.display());
    fs::write(out.join("memory.x"), fs::read_to_string("memory.x").unwrap()).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
}
```

`src/main.rs`, blinking the onboard LED (D13):

```rust,ignore
#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use unor4_hal::Peripherals;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut delay = p.delay;
    let mut led = p.pins.d13.into_output();
    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

Build, convert and upload (find the port with `arduino-cli board list`):

```powershell
cargo build --release
cargo objcopy --release -- -O binary firmware.bin
arduino-cli upload -p COM3 --fqbn arduino:renesas_uno:unor4wifi --input-file firmware.bin
```

The examples below are the body of `main`. Each is compiled by this
repository's `cargo test`, so they match the API.

## Pins

`Peripherals::take()` hands out every pin once, named by its header:
`p.pins.d0` … `p.pins.d13`, `p.pins.a0` … `p.pins.a5`. A pin's type is
`Pin<D6, Mode>`, and its mode changes by consuming it:

```rust,no_run
use unor4_hal::Peripherals;

let p = Peripherals::take().unwrap();
let mut led = p.pins.d13.into_output();     // push-pull output
let button = p.pins.d2.into_input_pullup(); // input with pull-up
led.set_high();
if button.is_low() {
    led.toggle();
}
```

What each header can do (GPT output, ADC channel, UART signal, IRQ) is one
table, `gpio::pin_map`, from the RA4M1 manual. Drivers ask for the
capability they need, so a pin that lacks it is a compile error naming the
pin: `` `A0` has no GPT output ``. Builders write pins as `const` values,
`pin_map::d6`.

## Builders: write them as named `const`

The stepper parts and the switches are set up with builders. A builder
checks two kinds of thing:
- **Types:** the pin has the capability; every required setting is there.
  `cargo check` reports these however you write the builder.
- **Numbers:** timing, pull-up values, board loads, pins used twice. These
  are `assert!`s inside `const fn`s. `cargo check` evaluates them **only
  in a named `const`**.

```rust,ignore
// Checked by `cargo check`: a bad number fails the build, naming the rule.
const PUL: PulSetup<D4, NoDivider> =
    PulPinBuilder::new().with_pin(pin_map::d4).with_timing(9_000, 2_500).build();
// error[E0080]: evaluation panicked: with_timing: LOW time over 8.3 us (400 PCLKD counts, the DIR write window)

// NOT checked at build time: the same mistake in a `let` (or an inline
// `const { }` block) compiles, then panics on the board when it runs.
let pul = PulPinBuilder::new().with_pin(pin_map::d4).with_timing(9_000, 2_500).build();
```

So every example below writes its setups as named `const` items, with the
type spelled out: `const PUL: PulSetup<D4, NoDivider> = …`. Do the same.

### PUL, DIR, ENA and switches: always through their builders

**Never configure a STEP, DIR, ENA or switch pin yourself.** The run-time
builders claim the pins their setup names from `Peripherals`:

```rust,ignore
// Right: each builder claims exactly what its checked setup names.
let Ok((mut stepper, p)) = StepGenerator::new(STEPPER).with_peripherals(p).build() else { … };
let Ok((mut limit, p)) = Switch::new(LIMIT).with_peripherals(p).build() else { … };

// Wrong: compiles, but skips every check and every timing rule.
let mut dir = p.pins.d5.into_output();          // bit-banged DIR: no set-up time,
dir.set_low();                                  // can flip mid-pulse, position wrong
let limit = p.pins.d3.into_input_pullup();      // no contact type, no board-load check
```

Driving these pins through the plain `Pin` API (`into_output`,
`into_input_pullup`, `set_high`, …) bypasses:
- the builders' `cargo check` validation;
- the generator's DIR rule (DIR changes only while STEP is HIGH, a set-up
  time before the next pulse);
- the ENA lead;
- the hardware pulse count and `position`;
- the switch's contact type, pull-up check and edge latch.

Once claimed, a pin belongs to the generator or the switch. Its field in
the returned `Peripherals` becomes `Taken`, so the plain API can no longer
reach it, and handing it to another driver fails to compile:
`` `D5` is already used by another driver ``.

**Order:** build the drivers that use `with_peripherals` (and call their
`free`) first, then the drivers that take fields directly (`Uart::new(p.sci2,
…)`, `Pwm::new(…)`). Moving a field out leaves `p` partly moved, and a
partly moved `p` can't be passed whole to `with_peripherals` or `free`
(`use of partially moved value: p`).

## Stepper motor: PUL, DIR, ENA

`step::StepGenerator` drives a stepper driver (a DM542, a TB6600, an A4988,
…) from a **signed rate in pulses per second**:
- it makes the STEP pulses in hardware;
- it sets DIR from the rate's sign;
- it counts every pulse in hardware;
- it applies a new rate exactly at the next 2 ms tick.

While the rate changes, your loop runs once per tick: read where the motor
is, decide the next rate, queue it. At a steady rate the CPU can sleep
through ticks ([below](#sleeping-through-ticks)).

Each part is set up by its own builder, written as a **named `const`**
([why](#builders-write-them-as-named-const)): then every check below fails
`cargo check`, before anything reaches the board. In a `let`, the number
checks only run, and panic, on the board. A missing setting names itself:
`` setup incomplete: `with_lead` not set ``. `StepGeneratorBuilder`
combines the parts, again as a named `const`. At run time
`StepGenerator::new(STEPPER).with_peripherals(p).build()` claims exactly the
hardware the setup names; never drive those pins yourself
([why](#pul-dir-ena-and-switches-always-through-their-builders)).

### PUL (STEP)

```rust,no_run
use unor4_hal::{
    gpio::pin_map::{self, D4},
    step::{NoDivider, PulPinBuilder, PulSetup},
};

// WiFi D4 is GPT320 output B (32-bit). LOW 5 µs, HIGH at least 2.5 µs.
const PUL: PulSetup<D4, NoDivider> = PulPinBuilder::new()
    .with_pin(pin_map::d4)
    .with_timing(5_000, 2_500)
    .build();

assert_eq!(PUL.low_counts(), 240); // 5 µs at 48 MHz
```

- **The pin:** a header with a 32-bit GPT output: WiFi D2–D5, Minima
  D2/D3/D6/D7/D11/D12. 16-bit STEP timers are refused for now.
- **The timing:** every pulse is LOW for the LOW time; HIGH lasts the rest
  of the period, at least the minimum. The idle level is HIGH.
- **Refused:**
  - a pin without a GPT output (`` `A0` has no GPT output ``);
  - LOW or HIGH under one timer count;
  - LOW over 8.3 µs (a reversal's delayed DIR write has to outlast a
    pulse);
  - HIGH under 1 µs.
- **At run time:**
  - the rate you queue sets the period;
  - `queue` refuses a rate whose period can't fit LOW + minimum HIGH
    (`StepError::RateTooHigh`);
  - `stop()` never cuts a pulse;
  - `pulses()` is the live hardware count;
  - `position()` is the signed count as of the latest tick.

### DIR

```rust,no_run
use embedded_hal::digital::PinState;
use unor4_hal::{
    gpio::pin_map::{self, D5},
    step::{DirPinBuilder, DirSetup},
};

const DIR: DirSetup<D5> = DirPinBuilder::new()
    .with_pin(pin_map::d5)          // any header
    .with_setup(5_000)              // 5 µs from a DIR change to the next pulse
    .with_forward(PinState::High)   // the level for positive rates
    .build();
```

- **You never write DIR.** The sign of the rate is the direction: queue
  `+1000` and DIR goes to the forward level; queue `-1000` and it flips.
- **The flip follows the driver's rule:** DIR changes only while STEP is
  HIGH, and the next pulse comes at least the set-up time later, however
  the flip happens:
  - a reversal while running is written by the DTC at the tick, held back
    until any pulse in flight has ended;
  - from stopped or idle, `queue` sets DIR at least 40 µs before the tick
    and the first pulse comes at least the set-up time after it;
  - `run` from stopped sets DIR, then waits the set-up time before the
    first pulse.
- **Position:** `TickReport::position` splits the pulses around a reversal
  by direction exactly.
- **Refused:** a board-internal pin (`` `…::UsbMux` is a board-internal
  pin, not an Arduino header ``), a set-up time of 0, and one over 1 ms
  (`` with_setup: DIR set-up time over 1 ms ``). Stepper drivers need
  microseconds; the limit keeps `run`'s first pulse inside the 2 ms tick
  that follows.

### ENA (optional)

```rust,no_run
use embedded_hal::digital::PinState;
use unor4_hal::{
    gpio::pin_map::{self, D8},
    step::{EnaPinBuilder, EnaSetup},
};

// Your driver's values. A DM542T enables on LOW; its manual asks for ENA
// 5 µs ahead of DIR and 200 ms for enabling, so the lead is 200 ms.
const ENA: EnaSetup<D8> = EnaPinBuilder::new()
    .with_pin(pin_map::d8)
    .with_enabled(PinState::Low)    // the level that enables the driver
    .with_lead(200_000_000)         // ns from enable to the first DIR change or pulse
    .build();
```

- **The values are your driver's.** The level and the lead have no
  defaults, because drivers differ.
- **The generator starts disabled.** `enable()` drives the enabling level.
  `is_enabled()` (it takes `&self`) turns true once the lead has passed,
  and until then a non-zero rate is refused (`StepError::Enabling`;
  `Disabled` before `enable()`).
- **The lead is real time, never short.** It is measured on the tick timer
  in 83 ns counts, from `enable()`, not in whole ticks. `is_enabled()`
  turns true at most 2 counts (167 ns) after the lead. To sleep through a
  short lead, start a [wake timer](#wake-timer-timed-waits-that-wake-a-sleeping-cpu)
  for it and check `is_enabled()` when it ends.
- **Disabling:** `disable()` needs a stopped generator, because the motor
  loses holding torque.
- **Refused:**
  - a board-internal pin;
  - a missing level or lead;
  - an enabling level the board itself holds while the pin is undriven.
    WiFi D13 is pulled LOW by its LED transistor, so ENA there can't
    enable on LOW: the driver would switch on during reset.
- **Leaving it out:** without ENA, leave out `.with_ena(ENA)` and
  `enable()`. The generator is then always enabled.

### Counter and tick

```rust,no_run
use ra4m1::{GPT164, GPT165, GPT321};
use unor4_hal::step::{CounterBuilder, CounterSetup, TickBuilder, TickSetup};

const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
const TICK: TickSetup<GPT164, GPT165> =
    TickBuilder::new().with_timers::<GPT164, GPT165>().build();
```

- **The counter:** any GPT channel. It counts every pulse in hardware, with
  no CPU involved; a 16-bit counter is extended to 32 bits in software.
- **The tick:** two GPT channels, the 2 ms tick and its tick number.

### All of it: `StepGeneratorBuilder`

```rust,ignore
const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
    StepGeneratorBuilder::new()
        .with_pul(PUL)
        .with_dir(DIR)
        .with_ena(ENA)          // optional
        .with_counter(COUNTER)
        .with_tick(TICK)
        .build();
```

- **Every part by name.** The `const`'s type lists them: seven parameters
  with ENA, six without.
- **Refused:**
  - a missing part (`` setup incomplete: `with_dir` not set ``);
  - a GPT channel used twice;
  - DIR or ENA on the STEP pin, or ENA on DIR;
  - a 16-bit STEP timer (not built yet).
- **The complete example below** compiles the whole chain.

### Running it

| Call | When | What it does |
| --- | --- | --- |
| `StepGenerator::new(STEPPER).with_peripherals(p).build()` | once | claims exactly the pins and timers `STEPPER` names and gives back the rest of `p`. The generator starts stopped: STEP HIGH, DIR forward, ENA disabling |
| `enable()`, then wait for `is_enabled()` | before moving (with ENA) | ENA to the enabling level; waits out the lead |
| `poll_tick()` | once per tick while you change the rate; at a steady rate, whenever the CPU wakes ([below](#sleeping-through-ticks)) | `Some(TickReport)` for the latest tick: `position`, `pulses`, `skipped` (ticks passed over since the previous report) |
| `queue(rate)` | once per tick, after `poll_tick` | the signed rate for the next tick. `0` idles: no pulses, and after 1 s idle the generator stops itself. From stopped, it starts at the next tick |
| `run(rate)` | from stopped | starts at once instead of at the next tick. It sets DIR, and the first pulse comes the PUL minimum HIGH time later, or the DIR set-up time when the direction changed, whichever is longer |
| `stop()` | any time | stops without cutting a pulse |
| `set_tick_wake(true)` | optional | each tick wakes the CPU from `WFI`, so the loop sleeps between ticks instead of polling |
| `stop_at(count)` | before or during a move | stops the train in hardware when `pulses()` reaches `count`, that pulse complete and none after it ([below](#moves-of-an-exact-pulse-count)) |
| `set_count_wake(true)` | optional | the count stop wakes the CPU from `WFI` |
| `stopped_at_count()` | after waking | `true` once after the count stop fired; the generator is then stopped |
| `cancel_stop_at()` | any time | cancels an armed count stop; the train keeps running |
| `disable()` | while stopped (with ENA) | ENA to the disabling level |
| `free(STEPPER, p)` | to take it apart | stops, disables ENA, and puts its pins and timers back into `p` |

What `queue` can refuse:
- **`NotReady`:** the tick wasn't polled yet, a change is already queued,
  it's within one tick of `run`/`stop`, or the generator is rebuilding
  after missed ticks. Retry at the next tick; the running rate holds.
- **`TooLate`:** within 40 µs of the next tick. Retry at the next one.
- **`RateTooHigh`:** clamp the rate.
- **`Disabled` / `Enabling`:** call `enable()` and wait.
- **`ReversalWithCountStop`:** a direction change while pulses run and a
  count stop is armed. A move to a count runs one way.

### Sleeping through ticks

The pulse train needs no CPU. Between rate changes the timer keeps the last
rate, so at a steady rate the CPU can sleep through any number of ticks
(the tick wake off), or a stall such as a data-flash erase can pass them by:
- **The next `poll_tick()` catches up.** It reports the latest tick, with
  `skipped` counting the ticks it passed over. It rebuilds the schedule
  from the timer registers at the first poll early enough in a tick,
  normally that same poll; `queue` returns `NotReady` until then.
- **Nothing is lost.** The position and the pulse count stay right. A
  change queued for a tick that was passed over still takes effect, a
  reversal included, with DIR kept right. An idle generator (rate 0) still
  stops 1 s after its last pulse.
- **What waits for a poll:** rate changes. Each is queued after a polled
  tick and lands at the next one.
- **16-bit counters (GPT162–GPT167):** read the count (`pulses()` or
  `poll_tick()`) at least every 32 768 pulses, 0.25 s at the highest rate,
  or the count it extends to 32 bits in software goes wrong. A 32-bit counter
  (GPT320, GPT321) needs no reads.

On the board:
- stalls of at least 5 ms over rate changes, reversals and idle, each
  recovered with every later prediction exact (`S11 skip`);
- sleeps of 100 ms, 500 ms and 3 s at a steady rate, then one poll and a
  `queue`: the first poll reported 48, 248 and 1 498 ticks passed over,
  `queue` was accepted at the first poll (18 of 18 sleeps), every later prediction was
  exact, and the count and position equalled the R3's, a reversal after
  the sleep included (`S11 sleep`);
- a whole 2 000-pulse move at 30 000/s, about 66 ms, slept through with
  the tick wake off and only the count stop to wake the CPU
  (`S11 countstop`).

### Moves of an exact pulse count

`stop_at(count)` arms a stop in hardware: when the pulse counter reaches
`count`, it stops the STEP timer through the event link controller. That
pulse completes and no pulse comes after it, whatever the CPU is doing.
With `set_count_wake(true)` the same event wakes the CPU, so a move of
exactly `n` pulses needs no polling:

```rust,no_run
use unor4_hal::{intc, step::{StepError, StepGenerator}};

/// Moves exactly `n` pulses at `rate` from rest, asleep until the stop.
fn move_exactly(stepper: &mut StepGenerator, n: u32, rate: i32) -> Result<(), StepError> {
    stepper.stop_at(stepper.pulses().wrapping_add(n))?;
    stepper.set_count_wake(true);
    stepper.set_tick_wake(true); // ticks wake the CPU too, for `queue`
    // From rest, `queue` starts the train at the next tick.
    loop {
        cortex_m::asm::wfi();
        if stepper.poll_tick().is_some() {
            match stepper.queue(rate) {
                Ok(_) => break,
                Err(StepError::NotReady | StepError::TooLate) => {} // next tick
                Err(error) => return Err(error),
            }
        }
    }
    // Asleep until the count stop's interrupt. Ticks still wake the CPU;
    // turn the tick wake off for a CPU that sleeps through the whole move.
    while !stepper.stopped_at_count() {
        cortex_m::asm::wfi();
        let _ = stepper.poll_tick();
    }
    Ok(())
}

unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups
```

- **The count is the hardware's.** `pulses()` equals `count` after the stop,
  and the R4's count equals the edges a second board counts (bench check
  `S11 countstop`).
- **How far ahead:** at least 1 pulse while no pulse can come (stopped, no
  start queued), at least 2 while pulses may run (`CountNotAhead`
  otherwise). With a 16-bit counter (GPT162–GPT167), at most 65 535 ahead
  (`CountTooFar`); a 32-bit counter (GPT320, GPT321) reaches 2³¹ − 1.
- **After the stop** the generator is stopped as after `stop()`: a queued
  change is dropped, and `queue` waits for the next poll.
- **No reversal** while a count stop is armed and pulses run
  (`ReversalWithCountStop`).

### Complete example: stepper with ENA and two limit switches

```rust,no_run
use embedded_hal::digital::PinState;
use ra4m1::{GPT164, GPT165, GPT321};
use unor4_hal::{
    Peripherals,
    gpio::{
        Pull, Switch, SwitchFilter, SwitchPinBuilder, SwitchSetup,
        contact::Nc,
        pin_map::{self, D2, D3, D4, D5, D8},
    },
    intc,
    step::{
        CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup,
        NoDivider, PulPinBuilder, PulSetup, StepError, StepGenerator, StepGeneratorBuilder,
        StepSetup, TickBuilder, TickSetup,
    },
};

// ---- Setup: named consts, checked by `cargo check` ----
const PUL: PulSetup<D4, NoDivider> =
    PulPinBuilder::new().with_pin(pin_map::d4).with_timing(5_000, 2_500).build();
const DIR: DirSetup<D5> = DirPinBuilder::new()
    .with_pin(pin_map::d5)
    .with_setup(5_000)
    .with_forward(PinState::High)
    .build();
const ENA: EnaSetup<D8> = EnaPinBuilder::new()
    .with_pin(pin_map::d8)
    .with_enabled(PinState::Low)
    .with_lead(200_000_000)
    .build();
const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
const TICK: TickSetup<GPT164, GPT165> =
    TickBuilder::new().with_timers::<GPT164, GPT165>().build();
const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
    StepGeneratorBuilder::new()
        .with_pul(PUL)
        .with_dir(DIR)
        .with_ena(ENA)
        .with_counter(COUNTER)
        .with_tick(TICK)
        .build();

// Normally-closed limit switches (a broken wire reads "hit"), on IRQ pins
// with different IRQ numbers, each trip latched in hardware and waking the
// CPU.
const LEFT: SwitchSetup<D3, Nc> = SwitchPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_contact(Nc)
    .with_pull(Pull::Internal)
    .with_edge()
    .with_filter(SwitchFilter::PclkbDiv64)
    .with_wake()
    .build();
const RIGHT: SwitchSetup<D2, Nc> = SwitchPinBuilder::new()
    .with_pin(pin_map::d2)
    .with_contact(Nc)
    .with_pull(Pull::Internal)
    .with_edge()
    .with_filter(SwitchFilter::PclkbDiv64)
    .with_wake()
    .build();

/// Your controller: the next rate in pulses/s; the sign is the direction.
fn next_rate(position: i64) -> i32 {
    (-position as i32).clamp(-20_000, 20_000)
}

// ---- Run time: each builder claims what its setup names ----
let p = Peripherals::take().unwrap();
let Ok((mut stepper, p)) = StepGenerator::new(STEPPER).with_peripherals(p).build() else {
    panic!("no free interrupt slot");
};
let Ok((mut left, p)) = Switch::new(LEFT).with_peripherals(p).build() else { panic!() };
let Ok((mut right, _p)) = Switch::new(RIGHT).with_peripherals(p).build() else { panic!() };
// `_p` is everything not claimed: UART, PWM, ADC… take their fields from it.

// The CPU sleeps in WFI: each 2 ms tick and each limit trip wakes it.
unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups
stepper.set_tick_wake(true);

stepper.enable();
while !stepper.is_enabled() {
    cortex_m::asm::wfi(); // the 200 ms lead, asleep between ticks
}

loop {
    cortex_m::asm::wfi(); // asleep until the next tick or a limit trip
    // A trip latched in hardware since the last pass (it may be what just
    // woke the CPU).
    let left_tripped = left.activated();
    let right_tripped = right.activated();
    let Some(tick) = stepper.poll_tick() else {
        // A limit tripped between ticks: stop now rather than at the next
        // tick (your machine's decision).
        if left_tripped || right_tripped {
            stepper.stop();
        }
        continue;
    };
    // A trip, or a switch still pressed.
    let left_hit = left_tripped || left.is_active();
    let right_hit = right_tripped || right.is_active();
    let mut rate = next_rate(tick.position);
    // Never drive further into a pressed limit; backing off is allowed.
    if (left_hit && rate < 0) || (right_hit && rate > 0) {
        rate = 0;
    }
    match stepper.queue(rate) {
        Ok(_) => {}
        // Rebuilding after missed ticks, or polled too late in this tick:
        // the running rate holds; queue again at the next tick.
        Err(StepError::NotReady | StepError::TooLate) => {}
        Err(_) => stepper.stop(),
    }
}
```

This example assumes D2 is the right-hand limit and D3 the left. Rate 0
stops the pulses at the next tick, and `stop()` stops them at once. Which
one a tripped limit needs is your machine's decision. Nothing here polls:
the CPU sleeps in `WFI` until the tick, a limit trip or another interrupt
(a UART's) wakes it.

### Without `unwrap` or `panic!`: report the error

The examples above use `.unwrap()` and `panic!()` to stay short. Firmware
that denies clippy's `unwrap_used` and `panic` handles the `Err` arm
instead. A refused build hands back the peripherals it was given, with
nothing claimed, so the error can go out on a UART before the firmware
halts:

```rust,no_run
use embedded_hal::digital::PinState;
use ra4m1::{GPT164, GPT165, GPT321};
use unor4_hal::{
    Peripherals,
    gpio::{
        Pull, Switch, SwitchError, SwitchFilter, SwitchPinBuilder, SwitchSetup,
        contact::Nc,
        pin_map::{self, D3, D4, D5, D8},
    },
    step::{
        CounterBuilder, CounterSetup, DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup,
        NoDivider, PulPinBuilder, PulSetup, StepError, StepGenerator, StepGeneratorBuilder,
        StepSetup, TickBuilder, TickSetup,
    },
    intc,
    uart::Uart,
};

const PUL: PulSetup<D4, NoDivider> =
    PulPinBuilder::new().with_pin(pin_map::d4).with_timing(5_000, 2_500).build();
const DIR: DirSetup<D5> = DirPinBuilder::new()
    .with_pin(pin_map::d5)
    .with_setup(5_000)
    .with_forward(PinState::High)
    .build();
const ENA: EnaSetup<D8> = EnaPinBuilder::new()
    .with_pin(pin_map::d8)
    .with_enabled(PinState::Low)
    .with_lead(200_000_000)
    .build();
const COUNTER: CounterSetup<GPT321> = CounterBuilder::new().with_timer::<GPT321>().build();
const TICK: TickSetup<GPT164, GPT165> =
    TickBuilder::new().with_timers::<GPT164, GPT165>().build();
const STEPPER: StepSetup<D4, NoDivider, D5, GPT321, GPT164, GPT165, D8> =
    StepGeneratorBuilder::new()
        .with_pul(PUL)
        .with_dir(DIR)
        .with_ena(ENA)
        .with_counter(COUNTER)
        .with_tick(TICK)
        .build();
const LEFT: SwitchSetup<D3, Nc> = SwitchPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_contact(Nc)
    .with_pull(Pull::Internal)
    .with_edge()
    .with_filter(SwitchFilter::PclkbDiv64)
    .build();

fn step_error_text(error: StepError) -> &'static [u8] {
    match error {
        StepError::NoIcuSlot => b"stepper: no free interrupt slot\r\n",
        _ => b"stepper: build refused\r\n",
    }
}

fn switch_error_text(error: SwitchError) -> &'static [u8] {
    match error {
        SwitchError::NoIcuSlot => b"switch: no free interrupt slot\r\n",
        SwitchError::IrqInUse => b"switch: its IRQ number is already in use\r\n",
    }
}

/// Stops here for good, without `panic!`.
fn halt() -> ! {
    loop {
        cortex_m::asm::wfi();
    }
}

// `None` only if `take` was already called once.
let Some(p) = Peripherals::take() else { halt() };

let (mut stepper, p) = match StepGenerator::new(STEPPER).with_peripherals(p).build() {
    Ok(built) => built,
    Err((error, p)) => {
        // `p` came back whole, so its UART is still free to use.
        let mut serial = Uart::new(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
        serial.write_bytes(step_error_text(error));
        halt()
    }
};

let (mut left, p) = match Switch::new(LEFT).with_peripherals(p).build() {
    Ok(built) => built,
    Err((error, p)) => {
        // `p` is what the stepper build returned: the stepper keeps its parts.
        let mut serial = Uart::new(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
        serial.write_bytes(switch_error_text(error));
        halt()
    }
};

// Every `with_peripherals` build is done: now take fields directly.
let mut serial = Uart::new(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
serial.write_bytes(b"ready\r\n");

unsafe { intc::enable_interrupts() };
stepper.set_tick_wake(true);
stepper.enable();
while !stepper.is_enabled() {
    cortex_m::asm::wfi();
}
loop {
    cortex_m::asm::wfi(); // asleep until the next tick
    if stepper.poll_tick().is_some() {
        let rate = if left.activated() || left.is_active() { 0 } else { -1_000 };
        match stepper.queue(rate) {
            Ok(_) | Err(StepError::NotReady | StepError::TooLate) => {}
            Err(_) => stepper.stop(),
        }
    }
}
```

## Switches: limit switches, buttons

A switch goes between a header pin and GND, with a pull-up (the RA4M1 has
pull-ups only). `gpio::SwitchPinBuilder` checks the setup and
`gpio::Switch` reads it. Write the setup as a **named `const`**
([why](#builders-write-them-as-named-const)): then the pull-up and
board-load checks fail `cargo check`, not the board. At run time
`Switch::new(SETUP).with_peripherals(p).build()` claims the pin; never read
it through `into_input_pullup()` yourself
([why](#pul-dir-ena-and-switches-always-through-their-builders)).

```rust,no_run
use unor4_hal::{
    Peripherals,
    gpio::{
        Pull, Switch, SwitchPinBuilder, SwitchSetup,
        contact::{Nc, No},
        pin_map::{self, A0, D3},
    },
    intc,
};

// A normally-open push button on A0 with a 10 kΩ pull-up on the board.
const BUTTON: SwitchSetup<A0, No> = SwitchPinBuilder::new()
    .with_pin(pin_map::a0)
    .with_contact(No)                 // pressed reads LOW
    .with_pull(Pull::External(10_000))
    .build();
// A normally-closed limit switch on D3 (an IRQ pin): pressed, or a broken
// wire, reads HIGH. `with_edge` latches each press in hardware, and
// `with_wake` makes it wake the CPU from WFI.
const LIMIT: SwitchSetup<D3, Nc> = SwitchPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_contact(Nc)
    .with_pull(Pull::Internal)
    .with_edge()
    .with_wake()
    .build();

let p = Peripherals::take().unwrap();
let Ok((button, p)) = Switch::new(BUTTON).with_peripherals(p).build() else { panic!() };
let (mut limit, _p) = match Switch::new(LIMIT).with_peripherals(p).build() {
    Ok(built) => built,
    Err((_error, _p)) => panic!(), // IrqInUse / NoIcuSlot; the peripherals come back
};

unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-up
loop {
    cortex_m::asm::wfi(); // asleep until the limit trips (or another interrupt)
    if limit.activated() {
        // tripped since the last call, even if released again in between
    }
    if button.is_active() {
        // A0 has no IRQ, so it can't wake the CPU: read whenever awake
    }
}
```

- **`with_contact`:** `No` (normally open: active reads LOW) or `Nc`
  (normally closed: active reads HIGH). No default. Prefer NC for limits:
  a broken wire reads as "hit".
- **`with_pull`:** `Pull::Internal` (10–50 kΩ) or `Pull::External(ohms)`.
  - The build checks the pull-up against the board's own loads. WiFi D13
    has a 100 kΩ pull-down (its LED transistor), so it refuses the internal
    pull-up there and needs an external one of at most 25 kΩ.
  - `External(0)` is refused.
- **`with_edge`, optional** (IRQ pins only): latches the edge into the
  active level in hardware: `activated()` returns it once and clears it.
  - Two switches with edges need different IRQ numbers. A3/A5 share IRQ2,
    A4/D2 IRQ1 and A1/D0 IRQ6, so a clash returns `SwitchError::IrqInUse`
    at `build()`.
  - On a clash the peripherals come back with the error.
- **`with_filter`, optional:** removes glitches shorter than 3 samples (at
  most 8 µs) on the edge latch, so it needs `with_edge`.
- **`with_wake`, optional:** the edge also interrupts the CPU, so it ends a
  `WFI`: the firmware sleeps until the switch trips instead of polling. It
  needs `with_edge` (`` with_wake: the wake-up is the edge latch's
  interrupt; add with_edge() ``). Unmask interrupts
  (`intc::enable_interrupts()`) so the HAL can acknowledge it.
  `activated()` still reports the trip after the interrupt ran. A switch
  already pressed at start-up makes no edge: read `is_active()` once.
  Proven on the bench (`S2 wake`): a press woke the R4 from `WFI`, with
  no other wake-up.
- **At run time:** `is_active()` / `is_inactive()` read the level;
  `free(p)` puts the pin back.
- **Debounce in firmware.** Mechanical bounce lasts milliseconds; the
  filter is not a debouncer. A [wake timer](#wake-timer-timed-waits-that-wake-a-sleeping-cpu)
  times a re-check (say 1 ms after the edge) without polling.
- **Board-internal pins** (`UsbMux`, the ESP32-S3 bridge's lines) are
  refused at `cargo check`.

## Wake timer: timed waits that wake a sleeping CPU

A one-shot timer on one spare GPT channel: each `start` times one wait,
and its end wakes the CPU from `WFI` once. Firmware that sleeps between
events needs it for every wait that isn't the motor's: a switch re-check,
an ENA lead, a reply timeout.

```rust,no_run
use core::time::Duration;
use unor4_hal::{
    Peripherals,
    elc::ElcE,
    intc,
    wake::{WakeSetup, WakeTimer, WakeTimerBuilder},
};

const WAKE: WakeSetup<ra4m1::GPT167, ElcE> = WakeTimerBuilder::new()
    .with_timer::<ra4m1::GPT167>()   // any GPT channel no other driver uses
    .with_input::<ElcE>()            // the ELC input it stops itself through
    .build();

let p = Peripherals::take().unwrap();
let Ok((mut wake, _p)) = WakeTimer::new(WAKE).with_peripherals(p).build() else {
    panic!("no free interrupt slot");
};
unsafe { intc::enable_interrupts() }; // the HAL acknowledges the wake-ups

// Sleep 1 ms with interrupts masked around the check: an end that lands
// between the check and the WFI still ends the WFI.
wake.start(Duration::from_micros(1_000)).unwrap();
loop {
    cortex_m::interrupt::disable();
    if wake.expired() {
        unsafe { cortex_m::interrupt::enable() };
        break;
    }
    cortex_m::asm::wfi();
    unsafe { cortex_m::interrupt::enable() };
}
```

- **One shot, in hardware.** The channel's overflow stops its own counter
  through the ELC input (the count stop's mechanism), so it fires once
  however late the CPU looks.
- **Never short, finest step.** `start` picks the finest prescaler that
  reaches the wait and rounds up. On a 16-bit channel:

  | Wait up to | Step |
  | --- | --- |
  | 1.365 ms | 20.8 ns |
  | 5.46 ms | 83.3 ns |
  | 21.8 ms | 0.333 µs |
  | 87.4 ms | 1.33 µs |
  | 349.5 ms | 5.33 µs |
  | 1.398 s | 21.3 µs |

  A 32-bit channel reaches 89.5 s at 20.8 ns. Past `longest()`, `start`
  returns `WakeError::TooLong`: re-arm in a loop.
- **Calls:** `start(Duration)` replaces a running wait; `expired()` is true
  once after a wait ended; `cancel()` stops it; `is_armed()`; `longest()`;
  `free(WAKE, p)`. `start` and `cancel` forget an earlier end, read or not,
  so a stale end never passes for a new one. A zero wait has ended at once.
- **Masked sleep works:** `expired()` reads the overflow flag, which only
  software clears, not the interrupt the HAL acknowledges.
- **Claimed by type.** The channel and the ELC input become `Taken`; either
  one another driver holds fails `cargo check`, naming it. The step
  generator holds ELC inputs B, C and D. One interrupt slot is taken at
  run time (`WakeError::NoIcuSlot` if none is free).
- **Build it at boot,** with the other `with_peripherals` drivers, before
  taking fields of `Peripherals`.
- **Proven on the bench** (`S12 wake`): nine waits from 5 µs to 1.398 s,
  each at least its length and within one step of it, one `WFI` wake each,
  the counter stopped 2 counts after its overflow; restart, cancel, zero
  and too-long as documented; a 1 ms and a 100 ms wait marked on a wire
  timed by the R3 at 1 008–1 012 µs and 100 080–100 112 µs (the R3's clock
  reads the R4's about 0.1 % long).

## UART

Polled, on any TX/RX pair the pin table gives one SCI channel. Here, SCI2 on
the header's D1 (TX) and D0 (RX):

```rust,no_run
use unor4_hal::{Peripherals, uart::Uart};

let p = Peripherals::take().unwrap();
let mut serial = Uart::new(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
serial.write_bytes(b"hello\r\n");
if let Ok(byte) = serial.read_byte() {
    serial.write_byte(byte); // echo
}
```

Interrupt-driven, with ring buffers (feature `uart-irq`). It implements
`embedded-io`, `embedded-hal-nb` and `core::fmt::Write`:

```rust,no_run
use unor4_hal::{Peripherals, intc, uart::UartIrq};

static SERIAL: UartIrq<ra4m1::SCI2> = UartIrq::new();

let p = Peripherals::take().unwrap();
SERIAL.attach(p.sci2, p.pins.d1, p.pins.d0, 115_200, &p.clocks);
unsafe { intc::enable_interrupts() };
SERIAL.write_all(b"ready\r\n");
loop {
    if let Some(byte) = SERIAL.read_byte() {
        SERIAL.write_all(&[byte]);
    }
}
```

`Uart::new` and `attach` panic if the baud rate can't be reached within 3 %.

## PWM

On any of the 16 headers with a GPT output. The timer is the one the pin
needs (a type check). `embedded-hal`'s `SetDutyCycle`:

```rust,no_run
use embedded_hal::pwm::SetDutyCycle;
use unor4_hal::{Peripherals, pwm::Pwm};

let p = Peripherals::take().unwrap();
// WiFi D6 is GPT163 output A; 1 kHz.
let mut pwm = Pwm::new(p.gpt163, p.pins.d6, 1_000, &p.clocks);
pwm.set_duty_cycle_percent(25).unwrap();
```

## ADC

Single-shot reads at 12 or 14 bits on any pin with an ADC channel (A0–A5,
plus WiFi D10/D13 and Minima D4/D5):

```rust,no_run
use unor4_hal::{
    Peripherals,
    adc::{Adc, Resolution},
};

let p = Peripherals::take().unwrap();
let mut adc = Adc::new(p.adc140, Resolution::Bits12);
let mut a0 = p.pins.a0.into_analog();
if let Ok(raw) = adc.read(&mut a0) {
    let _millivolts = u32::from(raw) * 5_000 / 4_095;
}
```

## Data flash

8 KB in eight 1 KB blocks. A byte can be programmed once per erase; erase
a block (all `0xFF`) before writing it again. Code flash is unreachable
through this API.

```rust,no_run
use unor4_hal::{Peripherals, dflash::DataFlash};

let p = Peripherals::take().unwrap();
let mut flash = DataFlash::take(&p.clocks).unwrap();
flash.erase_block(0).unwrap();       // ~6 ms
flash.write(0, b"config v1").unwrap();
let mut buf = [0u8; 9];
flash.read(0, &mut buf).unwrap();
```

A block erase stalls the CPU for about 6 ms, and 1 KB of writes for about
44 ms. The step generator recovers from the ticks it misses meanwhile.

## USB CDC-ACM

Feature `usb`, plus `usb-device = "0.3.2"` and `usbd-serial = "0.2.2"`.
**On the WiFi board, USB and the ESP32-S3 bridge share the connector:
`UsbPort::take` switches it to the RA4M1, and the bridge (uploads, the
serial monitor) disconnects.** To upload again, reset into the bootloader
(double-tap RESET).

```rust,no_run
use unor4_hal::{
    Peripherals,
    usb::{CdcAcm, UsbPort, Usbfs},
};
use usb_device::{
    LangID,
    bus::UsbBusAllocator,
    device::{StringDescriptors, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

let p = Peripherals::take().unwrap();
let alloc = UsbBusAllocator::new(Usbfs::new(p.usbfs));
let serial = SerialPort::new(&alloc);
let device = UsbDeviceBuilder::new(&alloc, UsbVidPid(0x2341, 0x006D))
    .device_class(USB_CLASS_CDC)
    .strings(&[StringDescriptors::new(LangID::EN_US).product("my device")])
    .unwrap()
    .max_packet_size_0(64)
    .unwrap()
    .build();
let mut cdc = CdcAcm::new(serial, device);
let _port = UsbPort::take(p.usb_mux); // WiFi: the bridge disconnects here
let mut buf = [0u8; 64];
loop {
    cdc.poll();
    if let Ok(n) = cdc.read_available(&mut buf) {
        let _ = cdc.write_all(&buf[..n]);
    }
}
```

## Cargo features

| Feature | Default | What it adds |
| --- | --- | --- |
| `wifi` | yes | the UNO R4 WiFi pin map |
| `minima` | no | the UNO R4 Minima pin map (`default-features = false, features = ["minima"]`) |
| `uart-irq` | no | `UartIrq`: interrupt-driven UART with ring buffers |
| `usb` | no | USBFS as a `usb-device` bus, the CDC-ACM wrapper, `UsbPort` |

`wifi` and `minima` are mutually exclusive; both or neither is a compile
error.

## Testing

Host tests (no hardware), from the repo root:

```powershell
just test
```

They cover the pin table against the Arduino core's board tables, every
builder's checks with their full messages, the clock, UART baud, PWM, ADC,
flash and step-generator maths, and the compiled examples in this README.
`just check` and `just clippy` run the same feature matrix.

On hardware, a UNO R3 and the R4 are wired together, and the `validate` CLI
re-proves every driver in three sessions: the R3's checks no longer fit
its flash in one image, so it is uploaded in three parts. The wiring, the commands and every
witnessed run are in [Validation.md](Validation.md).

## License

MIT — see [LICENSE.md](LICENSE.md).

### Dependency licences

Every direct dependency is licence-audited before it is added; the table
lists them, with versions and licences as `cargo metadata` resolves them
(2026-11-03). Every crate compiled into the library or a firmware image
is available under MIT; the exceptions below are a build-time tool and
the unpublished host tool.

| Crate | Version | Licence | Used by |
| --- | --- | --- | --- |
| `cortex-m` | 0.7.10 | MIT OR Apache-2.0 | library, firmware |
| `cortex-m-rt` | 0.7.7 | MIT OR Apache-2.0 | firmware |
| `critical-section` | 1.2.0 | MIT OR Apache-2.0 | library |
| `embedded-hal` | 1.0.0 | MIT OR Apache-2.0 | library, firmware |
| `embedded-hal-nb` | 1.0.0 | MIT OR Apache-2.0 | library (`uart-irq`) |
| `embedded-io` | 0.7.1 | MIT OR Apache-2.0 | library, firmware |
| `nb` | 1.1.0 | MIT OR Apache-2.0 | library |
| `ra4m1` | 0.2.0 | MIT OR Apache-2.0 | library, firmware |
| `usb-device` | 0.3.2 | MIT | library (`usb`: the bus HAL), firmware (audited 2026-10-20) |
| `usbd-serial` | 0.2.2 | MIT | library (`usb`: the CDC-ACM class), firmware (audited 2026-10-20) |
| `panic-halt` | 1.0.0 | MIT OR Apache-2.0 | firmware |
| `clap` | 4.6.7 | MIT OR Apache-2.0 | `validate` host tool |
| `regex` | 1.13.1 | MIT OR Apache-2.0 | `validate` host tool |
| `serialport` | 4.10.1 | **MPL-2.0** | `validate` host tool only — **owner-approved exception** (2026-10-20): the tool is `publish = false` and never ships in the library or a firmware image |

The transitive dependencies were audited on 2026-11-11 with `cargo tree
-e normal,build --format '{p} {l}'` (library with `usb,uart-irq`, the
firmware for the thumb target, the host tool). Every one offers MIT, alone
or as a choice (`MIT OR Apache-2.0`, `MIT/Apache-2.0`, `Unlicense OR MIT`
for `byteorder`, `memchr`, `aho-corasick`), except:

| Crate | Version | Licence | Pulled in by |
| --- | --- | --- | --- |
| `unicode-ident` | 1.0.27 | (MIT OR Apache-2.0) **AND Unicode-3.0** | `proc-macro2` and `syn`, for `cortex-m`'s procedural macros (`cortex-m-macros`, `cortex-m-rt-macros`). A build-time dependency of the compiler only: procedural macros run on the host and are not linked into the library or a firmware image. Unicode-3.0 is a permissive licence for Unicode's data tables. |
| `serialport` | 4.10.1 | MPL-2.0 | the host tool, above |

### Minimum Rust version

`rust-version = "1.88"`: the `usb` feature calls `<[T]>::as_chunks`
(stable in 1.88, flagged by clippy's `incompatible_msrv`), and `usb` and
`uart-irq` use `if let … && let …` chains. That is the lowest version
consistent with those uses; it is not proven by a 1.88 build (the repo
builds with current stable). Every dependency above needs 1.85 or older.
