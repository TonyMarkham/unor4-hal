# unor4-hal

An independent hardware abstraction layer (HAL) for the Arduino UNO R4
(Renesas RA4M1, Arm Cortex-M4F), targeting the **UNO R4 Minima** and
**UNO R4 WiFi** boards.

- `no_std`, no allocation, no panics in the library — errors are returned
- typestate GPIO: a pin's mode is tracked in the type system
- cycle-accurate SysTick delay (`embedded-hal::delay::DelayNs`)
- HOCO 48 MHz clock initialization at startup

## Status

Early development — the API is unstable and the crate is not yet published
to crates.io. Verified on real hardware so far: 48 MHz clock init, GPIO
output, and SysTick delay (1,584-byte blinky firmware).

Planned next: UART on all three board-relevant SCI channels (header pins,
USB-bridge monitor path, WiFi AT channel), interrupt support (ICU/IELSR),
PWM, ADC, data flash, and USB CDC-ACM.

## Example

Blink the onboard LED:

```rust
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

Firmware binary dependencies:

```toml
[dependencies]
unor4-hal   = { path = "../unor4-hal" }
embedded-hal = "1.0.0"
cortex-m-rt = { version = "0.7.7", features = ["set-vtor"] }
panic-halt  = "1.0.0"
```

## Building

The target is `thumbv7em-none-eabihf` (Arm Cortex-M4F). Flash through the
Arduino UNO R4's bootloader: convert the ELF to a raw binary and upload it
with `arduino-cli` — the first 16 KiB of flash stay untouched.

## License

MIT — see [LICENSE.md](LICENSE.md).
