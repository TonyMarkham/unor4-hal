//! The run-time builder: a checked setup plus the peripherals it claims.

use crate::{
    builder::{Missing, Provided, Set, slot},
    elc::ElcInput,
    intc::Ielsr,
    peripherals::{Board, Claim},
    pwm::{GptChannel, Width},
    wake::{WakeClock, WakeError, WakeSetup, WakeTimer},
};

/// The run-time builder [`WakeTimer::new`] returns: a checked
/// [`WakeSetup`], then the peripherals to take its channel and ELC input
/// from.
///
/// ```no_run
/// # use unor4_hal::{elc::ElcE, wake::{WakeSetup, WakeTimerBuilder}};
/// # const WAKE: WakeSetup<ra4m1::GPT167, ElcE> =
/// #     WakeTimerBuilder::new().with_timer::<ra4m1::GPT167>().with_input::<ElcE>().build();
/// use unor4_hal::{Peripherals, wake::WakeTimer};
///
/// let p = Peripherals::take().unwrap();
/// let Ok((mut wake, p)) = WakeTimer::new(WAKE).with_peripherals(p).build() else {
///     panic!("no free interrupt slot");
/// };
/// ```
///
/// `build()` claims the channel and the input and returns the rest of the
/// peripherals with them taken: another driver on either fails to compile
/// (`` `GPT167` is already used by another driver ``). Like every
/// `with_peripherals` driver it needs the whole `Peripherals`, so build it
/// before taking fields out directly.
pub struct WakeTimerInit<T, I, Pp> {
    setup: WakeSetup<T, I>,
    peripherals: Pp,
}

impl WakeTimer {
    /// Starts building a wake timer from a checked `setup` (a named
    /// `const`, see [`WakeTimerBuilder`](crate::wake::WakeTimerBuilder)).
    // The entry of the run-time builder (`new(SETUP).with_peripherals(p)
    // .build()`): it returns the builder, and `build` returns the driver.
    #[allow(clippy::new_ret_no_self)]
    pub const fn new<T, I>(setup: WakeSetup<T, I>) -> WakeTimerInit<T, I, Missing> {
        WakeTimerInit {
            setup,
            peripherals: Missing,
        }
    }
}

impl<T, I> WakeTimerInit<T, I, Missing> {
    /// The peripherals to claim the setup's channel and input from.
    pub fn with_peripherals<X>(self, peripherals: X) -> WakeTimerInit<T, I, Set<X>> {
        WakeTimerInit {
            setup: self.setup,
            peripherals: Set(peripherals),
        }
    }
}

impl<T, I, Pp> WakeTimerInit<T, I, Pp> {
    /// Claims the channel and the ELC input and sets them up, stopped:
    /// the channel's overflow linked to its own stop through the input,
    /// and to the CPU through an interrupt slot. Returns the timer with the
    /// rest of the peripherals.
    ///
    /// # Errors
    /// [`WakeError::NoIcuSlot`] when no interrupt slot is free, with the
    /// peripherals given back untouched.
    // The error hands the peripherals back whole; `no_std` has no box.
    #[allow(clippy::result_large_err)]
    pub fn build<X, R1, R2>(self) -> Result<(WakeTimer, R2), (WakeError, X)>
    where
        T: GptChannel,
        I: ElcInput,
        Pp: Provided<slot::with_peripherals, Value = X>,
        X: Claim<T, Part = T, Rest = R1>,
        R1: Claim<I, Part = I, Rest = R2>,
        R2: Board,
    {
        let _ = self.setup;
        let peripherals = self.peripherals.into_value();
        // The slot first: the only thing that can fail, before anything is
        // claimed or written.
        let Some(mut slot) = Ielsr::take_any() else {
            return Err((WakeError::NoIcuSlot, peripherals));
        };
        let (timer, rest) = peripherals.claim();
        let (_input, mut rest): (I, _) = rest.claim();
        T::release_module_stop();
        let regs = timer.block();
        let max_gtpr = <T::Width as Width>::MAX_GTPR;
        // Stopped, up-counting saw wave, no pin, no other event source; the
        // one stop source is the claimed ELC input (`GTPSR.PSELCx`, bits
        // 16–23 for inputs A–H, manual §22.2.6).
        regs.gtcr
            .write(|w| unsafe { w.md()._000().tpcs().bits(0).cst().clear_bit() });
        regs.gtuddtyc.write(|w| w.ud().set_bit());
        regs.gtber.write(|w| unsafe { w.bits(0) });
        regs.gtior.write(|w| unsafe { w.bits(0) });
        regs.gtssr.write(|w| unsafe { w.bits(0) });
        regs.gtcsr.write(|w| unsafe { w.bits(0) });
        regs.gtupsr.write(|w| unsafe { w.bits(0) });
        regs.gtdnsr.write(|w| unsafe { w.bits(0) });
        regs.gticasr.write(|w| unsafe { w.bits(0) });
        regs.gticbsr.write(|w| unsafe { w.bits(0) });
        regs.gtpr.write(|w| unsafe { w.bits(max_gtpr) });
        regs.gtcnt.write(|w| unsafe { w.bits(0) });
        regs.gtst.write(|w| unsafe { w.bits(0) });
        regs.gtpsr
            .write(|w| unsafe { w.bits(1 << (16 + u32::from(I::INPUT.index()))) });
        rest.elc().link_input(I::INPUT, T::ELC_OVERFLOW);
        slot.link(T::OVERFLOW_EVENT);
        slot.enable();
        let clock = WakeClock::new(rest.clocks().pclkd(), max_gtpr);
        Ok((WakeTimer::from_parts(regs, clock, I::INPUT, slot), rest))
    }
}
