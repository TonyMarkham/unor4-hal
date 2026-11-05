//! The boot-time step checks (`b`): a second generator setup run, freed and
//! replaced at boot. `StepGenerator::free` and a new `build` take the
//! whole `Peripherals`, so (the README's order rule) they run before any
//! driver takes a field. The R3 resets this board and watches these
//! pulses (R3 command `B`). WiFi only, like the rest of the step
//! generator's checks.
//!
//! The setup covers what the main one cannot:
//! - STEP on D3: GPT321, output A (the main setup: GPT320, output B);
//! - a DIR set-up of 200 µs, longer than `queue`'s 40 µs guard, waited
//!   through `queue` from stopped and through idle;
//! - `free` while the train runs, then the main generator built from the
//!   peripherals it gave back;
//! - an ENA pin (D2, free on the bench) with a 5 µs lead: `enable()` to
//!   `is_enabled()` timed on GPT166, free-running at PCLKD. The timing
//!   brackets both calls, so it shows the lead is at least 5 µs and how
//!   soon `is_enabled()` follows it; the one-count margin itself is
//!   host-tested (`tests/ena.rs`).

use crate::{
    body::Body,
    step_run::{COUNTER, TICK, next_tick, queue_retrying, watch_ticks},
};

use embedded_hal::digital::PinState;
use ra4m1::{GPT162, GPT164, GPT165};
use unor4_hal::{
    Peripherals,
    gpio::pin_map::{self, D2, D3, D5},
    pwm::GptChannel,
    step::{
        DirPinBuilder, DirSetup, EnaPinBuilder, EnaSetup, NoDivider, PulPinBuilder, PulSetup,
        StepError, StepGenerator, StepGeneratorBuilder, StepSetup,
    },
};

/// STEP on D3 (GPT321, output A), the main setup's pulse shape.
const ALT_PUL: PulSetup<D3, NoDivider> = PulPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_timing(5_000, 2_500)
    .build();
/// DIR on D5 with a 200 µs set-up: past `queue`'s 40 µs head start, so the
/// HAL must lengthen the first period after each DIR change.
const ALT_DIR: DirSetup<D5> = DirPinBuilder::new()
    .with_pin(pin_map::d5)
    .with_setup(ALT_DIR_SETUP_NS)
    .with_forward(PinState::High)
    .build();
/// The set-up time, ns.
const ALT_DIR_SETUP_NS: u32 = 200_000;
/// ENA on D2 (free on the bench: nothing is wired to it), enabling HIGH,
/// with cart-pole's 5 µs lead.
const ALT_ENA: EnaSetup<D2> = EnaPinBuilder::new()
    .with_pin(pin_map::d2)
    .with_enabled(PinState::High)
    .with_lead(ENA_LEAD_NS)
    .build();
/// The ENA lead, ns.
const ENA_LEAD_NS: u32 = 5_000;
/// The lead in PCLKD counts (48 MHz): the shortest `enable()` →
/// `is_enabled()` the timing may show.
const ENA_LEAD_COUNTS: u32 = ENA_LEAD_NS * 48 / 1_000;
/// The boot-time setup: STEP D3, DIR D5, ENA D2, the main setup's counter
/// and tick.
const ALT_SETUP: StepSetup<D3, NoDivider, D5, GPT162, GPT164, GPT165, D2> =
    StepGeneratorBuilder::new()
        .with_pul(ALT_PUL)
        .with_dir(ALT_DIR)
        .with_ena(ALT_ENA)
        .with_counter(COUNTER)
        .with_tick(TICK)
        .build();

/// The rate, pulses/s: slow enough for the R3 to timestamp every pulse
/// with `micros()`.
const RATE: i32 = 1_000;
/// Ticks before the first change: the R3 is already watching (it armed
/// before the reset), this only separates the start from the boot.
const QUIET_TICKS: u32 = 50;
/// Ticks each direction runs, every prediction checked.
const RUN_TICKS: u32 = 5;
/// Ticks idle (rate 0) before a flip through idle.
const IDLE_TICKS: u32 = 3;
/// How long before the next tick each direction change is queued, PCLKD
/// counts (60 µs): just outside `queue`'s 40 µs guard, so DIR changes
/// about 60 µs before the tick and the HAL must hold the first pulse back
/// another 140 µs for the 200 µs set-up. Queued early in the tick instead,
/// DIR would lead by about 2 ms and the set-up would never be tested.
const LATE_BEFORE_TICK: u32 = 2_880;
/// Ticks a late `queue` is retried at before it counts as refused.
const LATE_TRIES: u32 = 5;

/// `PmnPFS` of WiFi D3 (P105) and D5 (P107): `0x4004_0800 + 0x40 × port
/// + 4 × pin` (the HAL's `pfs_addr`, manual §19.2.5).
const P105PFS: *const u32 = 0x4004_0854 as *const u32;
const P107PFS: *const u32 = 0x4004_085C as *const u32;
/// `PDR` (output), `PMR` (peripheral) and `PSEL`: all clear on a GPIO
/// input (manual §19.2.5).
const PFS_NOT_INPUT: u32 = (1 << 2) | (1 << 16) | (0x1F << 24);

/// What the boot-time checks found, reported by `b`.
#[derive(Default)]
pub(crate) struct BootFree {
    /// The alternate setup was built.
    ran: bool,
    /// Direction changes started through `queue` (4 expected).
    starts: u32,
    /// Refused `queue` calls.
    errors: u32,
    /// Phase misses, count misses and checked ticks (`watch_ticks`).
    checks: [u32; 3],
    /// `free` gave the peripherals back.
    freed: bool,
    /// D3 and D5 read as GPIO inputs after `free`.
    pins_input: bool,
    /// GPT321 (the STEP timer) stopped after `free`.
    timer_stopped: bool,
    /// GPT162's raw count after `free`: the pulses that ended.
    count: u32,
    /// PCLKD counts from just before `enable()` to `is_enabled()` reading
    /// true.
    ena_counts: u32,
}

impl BootFree {
    /// Runs the boot-time checks on `p` and gives `p` back whole: build the
    /// alternate setup, four direction changes (from stopped, through idle,
    /// from stopped, through idle), `free` while running, read back the
    /// pins, the STEP timer and the count. Must run before any driver takes
    /// a field of `p`.
    pub(crate) fn run(p: Peripherals) -> (BootFree, Peripherals) {
        let mut report = BootFree::default();
        let (mut stepper, p) = match StepGenerator::new(ALT_SETUP).with_peripherals(p).build() {
            Ok(built) => built,
            Err((_, p)) => return (report, p),
        };
        report.ran = true;
        report.ena_counts = time_enable(&mut stepper, p.gpt166.block());
        for _ in 0..QUIET_TICKS {
            next_tick(&mut stepper);
        }
        for (index, sign) in [-1, 1, -1, 1].into_iter().enumerate() {
            if index % 2 == 0 {
                // From stopped.
                stepper.stop();
            } else {
                // Through idle: rate 0 for a few ticks first.
                report.count_queue(queue_retrying(&mut stepper, 0), false);
                watch_ticks(&mut stepper, IDLE_TICKS, &mut report.checks);
            }
            report.count_queue(queue_late(&mut stepper, sign * RATE), true);
            watch_ticks(&mut stepper, RUN_TICKS, &mut report.checks);
        }
        // Still running at +1 000/s: `free` must stop without cutting a pulse.
        let Ok(p) = stepper.free(ALT_SETUP, p) else {
            panic!("free refused the setup the generator was built from");
        };
        report.freed = true;
        // Safety: reads of the pins' function registers; nothing is written.
        let (d3, d5) = unsafe {
            (
                core::ptr::read_volatile(P105PFS),
                core::ptr::read_volatile(P107PFS),
            )
        };
        report.pins_input = (d3 | d5) & PFS_NOT_INPUT == 0;
        report.timer_stopped = p.gpt321.gtcr.read().cst().bit_is_clear();
        report.count = p.gpt162.gtcnt.read().bits();
        (report, p)
    }

    /// Counts one `queue` outcome: a start when `start`, else only refusals.
    fn count_queue(&mut self, accepted: bool, start: bool) {
        match (accepted, start) {
            (true, true) => self.starts += 1,
            (true, false) => {}
            (false, _) => self.errors += 1,
        }
    }

    /// The `b` reply: `b <ran> <starts> <errors> <phase misses> <count
    /// misses> <checked ticks> <freed> <pins input> <timer stopped>
    /// <count> <ena counts> <lead counts>`: the last two in PCLKD counts,
    /// `enable()` to `is_enabled()` and the lead.
    pub(crate) fn reply(&self) -> Body {
        let mut body = Body::new(b"b");
        body.num(u32::from(self.ran))
            .num(self.starts)
            .num(self.errors)
            .num(self.checks[0])
            .num(self.checks[1])
            .num(self.checks[2])
            .num(u32::from(self.freed))
            .num(u32::from(self.pins_input))
            .num(u32::from(self.timer_stopped))
            .num(self.count)
            .num(self.ena_counts)
            .num(ENA_LEAD_COUNTS);
        body
    }
}

/// Queues `rate` [`LATE_BEFORE_TICK`] counts before the next tick: polls a
/// tick (so `queue` targets the next), waits until that late in it, and
/// queues, retrying at the next tick while refused with `NotReady` /
/// `TooLate`, at most [`LATE_TRIES`] ticks. False on any other refusal.
fn queue_late(stepper: &mut StepGenerator, rate: i32) -> bool {
    for _ in 0..LATE_TRIES {
        next_tick(stepper);
        let late = stepper.tick_counts() - LATE_BEFORE_TICK;
        while stepper.since_tick_counts() < late {}
        match stepper.queue(rate) {
            Ok(_) => return true,
            Err(StepError::NotReady | StepError::TooLate) => {}
            Err(_) => return false,
        }
    }
    false
}

/// Times `enable()` to `is_enabled()` on `reference` (GPT166, set here as a
/// free-running 16-bit up-counter at PCLKD, stopped and cleared after):
/// PCLKD counts from just before the call to the first true read.
fn time_enable(stepper: &mut StepGenerator, reference: &ra4m1::gpt320::RegisterBlock) -> u32 {
    ra4m1::GPT166::release_module_stop();
    reference.gtcr.write(|w| unsafe { w.bits(0) });
    reference.gtuddtyc.write(|w| w.ud().set_bit());
    reference.gtpr.write(|w| unsafe { w.bits(0xFFFF) });
    reference.gtcnt.write(|w| unsafe { w.bits(0) });
    reference.gtcr.modify(|_, w| w.cst().set_bit());
    let start = reference.gtcnt.read().bits();
    stepper.enable();
    while !stepper.is_enabled() {}
    let end = reference.gtcnt.read().bits();
    reference.gtcr.write(|w| unsafe { w.bits(0) });
    reference.gtcnt.write(|w| unsafe { w.bits(0) });
    end.wrapping_sub(start) & 0xFFFF
}
