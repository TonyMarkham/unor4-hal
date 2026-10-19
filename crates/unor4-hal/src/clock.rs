//! Clock generation circuit: switch the system clock from reset-default
//! MOCO (8 MHz) to HOCO (48 MHz).

/// HOCO frequency in Hz — the UNO R4's system clock source.
pub const HOCO_FREQ_HZ: u32 = 48_000_000;

/// ICLK/1, PCLKA/1, PCLKB/2, PCLKC/1, PCLKD/1, FCLK/2.
///
/// RA4M1 quirk: the reserved BCLK divisor bits [18:16] must mirror PCLKB's
/// divider, so bit 16 carries 0b001 alongside PCKB [10:8] = 0b001.
/// `pub(crate)`: the host unit test in `src/tests/clock.rs` checks this
/// value's invariant.
pub(crate) const SCKDIVCR_48MHZ: u32 = 0x1001_0100;

/// `PRCR` key 0xA5 + PRC0/PRC1 write-enable (clock + low-power registers).
const PRCR_UNLOCK: u16 = 0xA503;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

/// Resolved clock frequencies, in Hz.
#[derive(Clone, Copy, Debug)]
pub struct Clocks {
    sysclk: u32,
    pclka: u32,
    pclkb: u32,
    pclkc: u32,
    pclkd: u32,
    fclk: u32,
}

impl Clocks {
    /// System clock (ICLK) — the CPU and SysTick.
    #[inline]
    pub fn sysclk(&self) -> u32 {
        self.sysclk
    }
    /// Peripheral clock A — the SCI bit-rate generator's base clock
    /// (see `uart/mod.rs` for the vendor cross-check).
    #[inline]
    pub fn pclka(&self) -> u32 {
        self.pclka
    }
    /// Peripheral clock B.
    #[inline]
    pub fn pclkb(&self) -> u32 {
        self.pclkb
    }
    /// Peripheral clock C — the ADC140 reference.
    #[inline]
    pub fn pclkc(&self) -> u32 {
        self.pclkc
    }
    /// Peripheral clock D — the GPT (PWM) reference.
    #[inline]
    pub fn pclkd(&self) -> u32 {
        self.pclkd
    }
    /// Flash clock.
    #[inline]
    pub fn fclk(&self) -> u32 {
        self.fclk
    }
}

/// Runs the startup sequence and returns the resolved frequencies.
/// Consumes `SYSTEM` — clock setup happens exactly once.
pub fn init(system: ra4m1::SYSTEM) -> Clocks {
    // 1. Unlock clock/low-power registers.
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK) });
    // 2. High-speed operating mode; OPCMTSF clears when the transition
    //    completes.
    system.opccr.modify(|_, w| w.opcm()._00());
    while system.opccr.read().opcmtsf().is_1() {}
    // 3. Start HOCO; HOCOSF sets once oscillation is stable.
    system.hococr.modify(|_, w| w.hcstp()._0());
    while system.oscsf.read().hocosf().is_0() {}
    // 4. One flash wait cycle — required above ICLK 32 MHz.
    system.memwait.modify(|_, w| w.memwait()._1());
    // 5. Dividers (see the constant's doc for the BCLK mirror).
    system.sckdivcr.write(|w| unsafe { w.bits(SCKDIVCR_48MHZ) });
    // 6. Switch the system clock source to HOCO.
    system.sckscr.modify(|_, w| w.cksel()._000());
    // 7. Relock.
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });

    Clocks {
        sysclk: HOCO_FREQ_HZ,
        pclka: HOCO_FREQ_HZ,
        pclkb: HOCO_FREQ_HZ / 2,
        pclkc: HOCO_FREQ_HZ,
        pclkd: HOCO_FREQ_HZ,
        fclk: HOCO_FREQ_HZ / 2,
    }
}
