//! The switch requests on D3, each with the R3 pressing it:
//! - the wake-up (`w`): D3 as a `Switch` with `with_edge` and `with_wake`,
//!   the R4 asleep in `WFI` until the R3 presses it;
//! - the external pull-up (`e`): the same switch with
//!   `Pull::External`, its pull-up the R3's own (the ATmega328P's 20–50 kΩ
//!   pin pull-up, datasheet `RPU`), the R4's internal one off.
//!
//! D3 belongs to the Slice 1 and 2 requests between requests, so each
//! request lends the pin to the switch through [`D3Lender`] and takes it
//! back with `free`. WiFi only (D3's IRQ0 is the WiFi pin table's).

use crate::{body::Body, clock::Clock};

use unor4_hal::{
    gpio::{
        Input, InputPullUp, Pin, Pull, Switch, SwitchError, SwitchFilter, SwitchPinBuilder,
        SwitchSetup,
        contact::No,
        pin_map::{self, D3},
    },
    peripherals::{Claim, Release},
};

/// The R3's "button" on D3: pressed is LOW (it drives LOW, then releases),
/// the internal pull-up, the edge latched through the filter, and the
/// edge waking the CPU.
const WAKE_SWITCH: SwitchSetup<D3, No> = SwitchPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_contact(No)
    .with_pull(Pull::Internal)
    .with_edge()
    .with_filter(SwitchFilter::PclkbDiv64)
    .with_wake()
    .build();

/// The R3's switch with no pull-up of the R4's own: D3's pull-up is the
/// R3's pin pull-up (`INPUT_PULLUP`), 20–50 kΩ (ATmega328P datasheet,
/// `RPU`), declared at its largest. Otherwise as [`WAKE_SWITCH`].
const EXTERNAL_SWITCH: SwitchSetup<D3, No> = SwitchPinBuilder::new()
    .with_pin(pin_map::d3)
    .with_contact(No)
    .with_pull(Pull::External(50_000))
    .with_edge()
    .with_filter(SwitchFilter::PclkbDiv64)
    .with_wake()
    .build();

/// WiFi D3 is P105; its pin function register `P105PFS` is at
/// `0x4004_0800 + 0x40 × port + 4 × pin` (the HAL's `pfs_addr`, manual
/// §19.2.5).
const P105PFS: *const u32 = 0x4004_0854 as *const u32;
/// `PmnPFS.PCR`, the pin's internal pull-up (manual §19.2.5, bit 4).
const PFS_PCR: u32 = 1 << 4;
/// The wait for the R3's release, in 1 ms steps on the SysTick clock: it
/// releases 200 ms after the press.
const RELEASE_POLLS: u32 = 1_000;

/// Most `WFI` returns the request waits for the press: every return is an
/// interrupt (the tick wake is off), so this only ends a run in which
/// other interrupts keep coming and the switch never wakes the CPU.
const MAX_WAKES: u32 = 10_000;

/// Lends D3 to a `Switch` the way `Peripherals` lends a pin: `Claim` takes
/// it out, `Release` puts it back.
struct D3Lender(Option<Pin<D3, Input>>);

impl Claim<D3> for D3Lender {
    type Part = Pin<D3, Input>;
    type Rest = D3Lender;

    fn claim(mut self) -> (Pin<D3, Input>, D3Lender) {
        let pin = self.0.take().expect("the lender holds D3 until claimed");
        (pin, self)
    }
}

impl Release<D3> for D3Lender {
    type Part = Pin<D3, Input>;
    type Restored = D3Lender;

    fn release(mut self, part: Pin<D3, Input>) -> D3Lender {
        self.0 = Some(part);
        self
    }
}

/// The `w` request: builds the switch on D3, then sleeps in `WFI` until
/// [`Switch::activated`] reports the R3's press (each `WFI` return is
/// counted), and gives D3 back. Reply `w <activated> <wakes> <active at
/// wake>`: `activated` 1 when the edge was reported after the CPU woke
/// (its interrupt ran and cleared the hardware latch before), `wakes` the
/// `WFI` returns until then (1 when nothing but the switch interrupted),
/// `active at wake` 1 when D3 still read pressed. `w ERR <code>` when the
/// switch could not be built (1 no ICU slot, 2 IRQ0 in use).
pub(crate) fn switch_wake(d3: &mut Option<Pin<D3, InputPullUp>>) -> Body {
    let Some(pin) = d3.take() else {
        return Body::new(b"w NA");
    };
    let lender = D3Lender(Some(pin.into_input()));
    let (mut switch, lender) = match Switch::new(WAKE_SWITCH).with_peripherals(lender).build() {
        Ok(built) => built,
        Err((error, lender)) => {
            *d3 = lender.0.map(Pin::into_input_pullup);
            let code = match error {
                SwitchError::NoIcuSlot => 1,
                SwitchError::IrqInUse => 2,
            };
            let mut body = Body::new(b"w ERR");
            body.num(code);
            return body;
        }
    };
    let mut wakes = 0u32;
    let mut activated = false;
    while !activated && wakes < MAX_WAKES {
        cortex_m::asm::wfi();
        wakes += 1;
        activated = switch.activated();
    }
    let active = switch.is_active();
    let lender = switch.free(lender);
    *d3 = lender.0.map(Pin::into_input_pullup);
    let mut body = Body::new(b"w");
    body.num(u32::from(activated))
        .num(wakes)
        .num(u32::from(active));
    body
}

/// Builds `setup` on D3 through a [`D3Lender`]. On failure D3 goes back to
/// `d3` and the error is the reply: `<letter> ERR <code>` (1 no ICU slot,
/// 2 IRQ0 in use).
fn lend_d3(
    d3: &mut Option<Pin<D3, InputPullUp>>,
    setup: SwitchSetup<D3, No>,
    letter: &[u8],
) -> Result<(Switch<D3, No>, D3Lender), Body> {
    let Some(pin) = d3.take() else {
        let mut body = Body::new(letter);
        body.text(b" NA");
        return Err(body);
    };
    let lender = D3Lender(Some(pin.into_input()));
    Switch::new(setup)
        .with_peripherals(lender)
        .build()
        .map_err(|(error, lender)| {
            *d3 = lender.0.map(Pin::into_input_pullup);
            let code = match error {
                SwitchError::NoIcuSlot => 1,
                SwitchError::IrqInUse => 2,
            };
            let mut body = Body::new(letter);
            body.text(b" ERR").num(code);
            body
        })
}

/// The `e` request: the switch with an external pull-up (the R3's). It
/// builds [`EXTERNAL_SWITCH`] on D3, reads whether D3's internal pull-up
/// is on, reads the idle state (only the R3's pull-up holds D3 HIGH), then
/// sleeps in `WFI` until [`Switch::activated`] reports the R3's press, and
/// waits up to 1 s for the release (the R3 drives LOW for 200 ms, then
/// only its pull-up brings D3 back). Reply `e <internal pull-up> <active
/// at idle> <activated> <wakes> <active at wake> <released>`; `e ERR
/// <code>` when the switch could not be built.
pub(crate) fn switch_external(d3: &mut Option<Pin<D3, InputPullUp>>, clock: &mut Clock) -> Body {
    let (mut switch, lender) = match lend_d3(d3, EXTERNAL_SWITCH, b"e") {
        Ok(built) => built,
        Err(body) => return body,
    };
    // Safety: a read of the pin's function register; nothing is written.
    let pull_up = unsafe { core::ptr::read_volatile(P105PFS) } & PFS_PCR != 0;
    let idle_active = switch.is_active();
    let mut wakes = 0u32;
    let mut activated = false;
    while !activated && wakes < MAX_WAKES {
        cortex_m::asm::wfi();
        wakes += 1;
        activated = switch.activated();
    }
    let active = switch.is_active();
    let mut released = false;
    for _ in 0..RELEASE_POLLS {
        if !switch.is_active() {
            released = true;
            break;
        }
        clock.wait_ms(1);
    }
    let lender = switch.free(lender);
    *d3 = lender.0.map(Pin::into_input_pullup);
    let mut body = Body::new(b"e");
    body.num(u32::from(pull_up))
        .num(u32::from(idle_active))
        .num(u32::from(activated))
        .num(wakes)
        .num(u32::from(active))
        .num(u32::from(released));
    body
}
