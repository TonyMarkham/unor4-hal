//! The delay probe (`m`): how long `cortex_m::asm::delay` really waits on
//! this board, timed on the SysTick clock. The validator's waits are
//! written as delay counts; the probe gives their real length.

use crate::{body::Body, clock::Clock};

/// The delay counts probed, with how many calls each is timed over, so
/// every row lasts long enough for the clock's 1 ms steps, while no row
/// outlasts the SysTick wrap (349 ms): the clock is read only before and
/// after a row, so a longer row loses a wrap (the first version's 8 × 2.4
/// million counts read 151 ms for about 500). The counts are the ones the
/// validator uses (in `step_run`, `cancel_race`), repeated.
const PROBES: [(u32, u32); 4] = [(2_400_000, 4), (960_000, 10), (240_000, 40), (1_200, 8_000)];

/// The `m` request. For each probe, the milliseconds its calls took in
/// total. Reply `m <ms 2 400 000 x4> <ms 960 000 x10> <ms 240 000 x40>
/// <ms 1 200 x8 000>`. At one CPU cycle per count (48 MHz) each row takes
/// 200 ms; on the bench it measured 1.25 cycles per count (250 ms).
pub(crate) fn delay_probe(clock: &mut Clock) -> Body {
    let mut body = Body::new(b"m");
    for (count, calls) in PROBES {
        let start = clock.now_ms();
        for _ in 0..calls {
            cortex_m::asm::delay(count);
        }
        body.num((clock.now_ms() - start) as u32);
    }
    body
}
