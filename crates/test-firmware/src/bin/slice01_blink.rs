//! Slice 1: blink the onboard LED (pin D13, RA4M1 P102) at 1 Hz.
//! Also record the release `text` size here after the first build.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use embedded_hal::delay::DelayNs;
use panic_halt as _; // crash handler: halt the core on panic
use unor4_hal::Peripherals;

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();

    let mut delay = p.delay; // SysTick-based blocking delay
    let mut led = p.pins.d13.into_output(); // D13 = onboard LED

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
