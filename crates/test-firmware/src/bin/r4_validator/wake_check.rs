//! The wake timer (`t1`–`t3`): GPT167 as a one-shot wake timer stopping
//! itself through ELC input E, measured against GPT321 free-running at
//! PCLKD (20.8 ns counts, 32-bit). WiFi only, like the step generator's
//! checks (GPT321 is free here only because the main step setup leaves it).

use crate::{body::Body, requests::argument};

use core::time::Duration;
use embedded_hal::digital::PinState;
use ra4m1::GPT167;
use unor4_hal::{
    elc::ElcE,
    gpio::{InputPullUp, Pin, pin_map::D3},
    pwm::GptChannel,
    wake::{WakeError, WakeSetup, WakeTimer, WakeTimerBuilder},
};

/// The validator's wake timer: GPT167, stopping itself through `ELC_GPTE`.
pub(crate) const WAKE: WakeSetup<GPT167, ElcE> = WakeTimerBuilder::new()
    .with_timer::<GPT167>()
    .with_input::<ElcE>()
    .build();

/// The waits `t1` measures, ns: cart-pole's ENA lead and trip re-check,
/// each prescaler's last wait and the first of the next, and the longest.
const WAITS: [u64; 9] = [
    5_000,
    100_000,
    1_000_000,
    1_365_333,
    1_365_334,
    21_845_333,
    87_381_333,
    349_525_333,
    1_398_101_333,
];
/// The dividers by prescaler, finest first, and the longest wait each
/// reaches on a 16-bit channel, ns.
const STEPS: [(u32, u64); 6] = [
    (1, 1_365_333),
    (4, 5_461_333),
    (16, 21_845_333),
    (64, 87_381_333),
    (256, 349_525_333),
    (1024, 1_398_101_333),
];
/// How much earlier than the wait the reference may read the end, PCLKD
/// counts: the reference is read just after `start` returns, a few
/// instructions after the count began.
const EARLY_SLACK: u32 = 16;
/// How much later than the wait plus one prescaler step the wake may come,
/// PCLKD counts (10 µs): the stop, the interrupt request and the `WFI`
/// exit.
const LATE_SLACK: u32 = 480;

/// Sets GPT321 as the reference: a free-running 32-bit up-counter at
/// PCLKD.
pub(crate) fn start_reference(reference: &ra4m1::GPT321) {
    ra4m1::GPT321::release_module_stop();
    let regs = reference.block();
    regs.gtcr.write(|w| unsafe { w.bits(0) });
    regs.gtuddtyc.write(|w| w.ud().set_bit());
    regs.gtpr.write(|w| unsafe { w.bits(u32::MAX) });
    regs.gtcnt.write(|w| unsafe { w.bits(0) });
    regs.gtcr.modify(|_, w| w.cst().set_bit());
}

/// The reference's count.
fn now(reference: &ra4m1::GPT321) -> u32 {
    reference.block().gtcnt.read().bits()
}

/// Busy-waits `counts` PCLKD counts on the reference.
fn spin(reference: &ra4m1::GPT321, counts: u32) {
    let start = now(reference);
    while now(reference).wrapping_sub(start) < counts {}
}

/// Sleeps with interrupts masked until the wake timer's wait ends: check,
/// `WFI`, unmask. Returns the reference's count when the last `WFI`
/// returned (before the acknowledgement runs) and the `WFI` returns.
fn sleep_until_expired(wake: &mut WakeTimer, reference: &ra4m1::GPT321) -> (u32, u32) {
    let mut woke = now(reference);
    let mut wakes = 0;
    loop {
        cortex_m::interrupt::disable();
        if wake.expired() {
            unsafe { cortex_m::interrupt::enable() };
            return (woke, wakes);
        }
        cortex_m::asm::wfi();
        woke = now(reference);
        wakes += 1;
        unsafe { cortex_m::interrupt::enable() };
    }
}

/// PCLKD counts in `ns` (48 MHz), rounded up.
fn counts(ns: u64) -> u32 {
    (ns * 48).div_ceil(1_000) as u32
}

/// The prescaler divider `start` picks for a wait of `ns`.
fn divider(ns: u64) -> u32 {
    STEPS
        .iter()
        .find(|(_, longest)| ns <= *longest)
        .map_or(1024, |(divider, _)| *divider)
}

/// The `t<mode>` request.
///
/// - **`t1`, lengths:** each of [`WAITS`] started, slept through masked,
///   timed on the reference. A wait is short if it ended more than
///   [`EARLY_SLACK`] counts before its length, late if more than one
///   prescaler step plus [`LATE_SLACK`] after. Reply `t1 <waits> <short>
///   <late> <most early> <most late> <most wakes> <stop latency>`: counts;
///   `most late` past the step; `most wakes` the `WFI` returns of one wait
///   (1 when only the timer woke the CPU); `stop latency` the most GPT167
///   counted past its overflow before the stop held.
/// - **`t2`, behaviour:** one bit each, 1 = as documented: fired once
///   (after a 100 µs wait, 300 µs more: not armed, no second end, counter
///   still); a start after an unread end reads false until its own end; a
///   cancel just after the end reads false; a cancel before the end stops
///   it and nothing fires; a zero wait ended at once, once; a wait past
///   the longest is refused and the longest is 1.398 101 333 s. Reply `t2
///   <once> <restart> <cancel after> <cancel before> <zero> <too long>`.
/// - **`t3`, on a wire:** D3 driven LOW at `start` and HIGH at the wake,
///   for 1 ms and then 100 ms waits, 50 ms apart; the R3 times both on its
///   INT1. Reply `t3 <ms of the first> <ms of the second>` after both.
pub(crate) fn wake_request(
    req: &[u8],
    wake: &mut WakeTimer,
    reference: &ra4m1::GPT321,
    d3: &mut Option<Pin<D3, InputPullUp>>,
) -> Body {
    match argument(req, 1) {
        1 => lengths(wake, reference),
        2 => behaviour(wake, reference),
        3 => on_wire(wake, reference, d3),
        _ => Body::new(b"t ERR"),
    }
}

/// `t1`: every wait in [`WAITS`].
fn lengths(wake: &mut WakeTimer, reference: &ra4m1::GPT321) -> Body {
    let (mut short, mut late, mut most_early, mut most_late) = (0u32, 0u32, 0u32, 0u32);
    let (mut most_wakes, mut latency) = (0u32, 0u32);
    for ns in WAITS {
        if wake.start(Duration::from_nanos(ns)).is_err() {
            short += 1;
            continue;
        }
        let started = now(reference);
        let (woke, wakes) = sleep_until_expired(wake, reference);
        let elapsed = woke.wrapping_sub(started);
        let wanted = counts(ns);
        let step = divider(ns);
        if elapsed + EARLY_SLACK < wanted {
            short += 1;
        }
        most_early = most_early.max(wanted.saturating_sub(elapsed));
        let past = elapsed.saturating_sub(wanted + step);
        if past > LATE_SLACK {
            late += 1;
        }
        most_late = most_late.max(past);
        most_wakes = most_wakes.max(wakes);
        // Safety: a read of GPT167's counter, which the wake timer owns;
        // nothing is written.
        let stopped_at = unsafe { (*GPT167::PTR).gtcnt.read().bits() };
        latency = latency.max(stopped_at);
    }
    let mut body = Body::new(b"t1");
    body.num(WAITS.len() as u32)
        .num(short)
        .num(late)
        .num(most_early)
        .num(most_late)
        .num(most_wakes)
        .num(latency);
    body
}

/// `t2`: the documented behaviour, one bit each.
fn behaviour(wake: &mut WakeTimer, reference: &ra4m1::GPT321) -> Body {
    // Fired once: a 100 µs wait, then 300 µs more.
    let once = wake.start(Duration::from_micros(100)).is_ok() && {
        sleep_until_expired(wake, reference);
        spin(reference, counts(300_000));
        // Safety: a read of GPT167's counter; nothing is written.
        let first = unsafe { (*GPT167::PTR).gtcnt.read().bits() };
        spin(reference, counts(10_000));
        let second = unsafe { (*GPT167::PTR).gtcnt.read().bits() };
        !wake.is_armed() && !wake.expired() && first == second
    };
    // A start after an unread end: false until its own end.
    let restart = wake.start(Duration::from_micros(50)).is_ok() && {
        spin(reference, counts(150_000));
        let ok_start = wake.start(Duration::from_millis(1)).is_ok();
        let false_at_once = !wake.expired();
        spin(reference, counts(500_000));
        let false_midway = !wake.expired();
        spin(reference, counts(700_000));
        ok_start && false_at_once && false_midway && wake.expired()
    };
    // A cancel just after the end: false.
    let cancel_after = wake.start(Duration::from_micros(50)).is_ok() && {
        spin(reference, counts(100_000));
        wake.cancel();
        !wake.expired()
    };
    // A cancel before the end: stopped, nothing fires.
    let cancel_before = wake.start(Duration::from_millis(1)).is_ok() && {
        spin(reference, counts(100_000));
        wake.cancel();
        let stopped = !wake.is_armed();
        spin(reference, counts(2_000_000));
        stopped && !wake.expired()
    };
    // A zero wait: ended at once, once.
    let zero = wake.start(Duration::ZERO).is_ok() && wake.expired() && !wake.expired();
    // Past the longest: refused; the longest as documented.
    let too_long = wake.start(Duration::from_nanos(1_398_101_334)) == Err(WakeError::TooLong)
        && !wake.is_armed()
        && wake.longest() == Duration::from_nanos(1_398_101_333);
    let mut body = Body::new(b"t2");
    for bit in [once, restart, cancel_after, cancel_before, zero, too_long] {
        body.num(u32::from(bit));
    }
    body
}

/// `t3`: two waits marked on D3 for the R3 to time.
fn on_wire(
    wake: &mut WakeTimer,
    reference: &ra4m1::GPT321,
    d3: &mut Option<Pin<D3, InputPullUp>>,
) -> Body {
    let Some(pin) = d3.take() else {
        return Body::new(b"t3 NA");
    };
    let mut out = pin.into_output_in_state(PinState::High);
    // The R3 arms its capture after the request has gone out.
    spin(reference, counts(20_000_000));
    let mut lengths = [0u32; 2];
    for (index, wait) in [Duration::from_millis(1), Duration::from_millis(100)]
        .into_iter()
        .enumerate()
    {
        out.set_low();
        let started = now(reference);
        if wake.start(wait).is_ok() {
            let (woke, _) = sleep_until_expired(wake, reference);
            out.set_high();
            lengths[index] = woke.wrapping_sub(started) / 48_000;
        } else {
            out.set_high();
        }
        spin(reference, counts(50_000_000));
    }
    *d3 = Some(out.into_input_pullup());
    let mut body = Body::new(b"t3");
    body.num(lengths[0]).num(lengths[1]);
    body
}
