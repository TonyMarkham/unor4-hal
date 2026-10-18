//! Slice 2: D2 (pull-up input) mirrors its level onto D13 (onboard LED).
//!
//! Wiring: a pushbutton between D2 and GND — nothing else. The pin's
//! internal pull-up (the `PCR` bit of `PmnPFS`, RA4M1 hardware manual,
//! I/O ports chapter) idles the line high, so **pressed = low**. No
//! external resistor needed.
//!
//! No debouncing either: a *level* mirror has no edges to double-count —
//! the LED simply follows the contact through the bounce window.

#![no_std] // don't link the standard library — no OS down here
#![no_main] // no standard main(); we provide our own entry point

use panic_halt as _; // crash handler: halt the core on panic
use unor4_hal::Peripherals;

// `#[entry]` becomes the function the vector table points to.
// It can never return (`-> !`) — there is nothing to return *to*.
#[cortex_m_rt::entry]
fn main() -> ! {
    // Claim all peripherals exactly once; also runs the 48 MHz clock init.
    let p = Peripherals::take().unwrap();

    // D2 = P104 on the WiFi map (P105 on Minima — same code, the feature
    // picks the pin). `into_input_pullup` sets PCR and keeps PDR = input.
    let button = p.pins.d2.into_input_pullup();
    let mut led = p.pins.d13.into_output(); // D13 = onboard LED (P102)

    loop {
        // Inherent read (gpio/mod.rs); the `embedded-hal` `InputPin`
        // trait is stamped onto the same type for generic consumers.
        if button.is_low() {
            led.set_high(); // pressed → LED on
        } else {
            led.set_low(); // released → LED off
        }
    }
}
