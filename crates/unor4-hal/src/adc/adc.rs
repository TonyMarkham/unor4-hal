//! The ADC140 driver: one claimed unit, single-shot blocking reads.

use crate::{
    adc::{Channel, ReadError, ReadResult, Resolution},
    gpio::{Analog, Pin},
};

/// `PRCR` key + PRC1 write-enable (module-stop registers are PRC1-
/// protected — the same unlock the SCI/GPT releases use).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

/// Poll bound for the `ADST` self-clear, in loop iterations. One
/// iteration is a register read plus bookkeeping — tens of cycles at
/// 48 MHz — so this bounds the wait at tens of milliseconds, thousands
/// of times the ~1.5 µs conversion. Reaching it is a fault (see
/// [`ReadError::Timeout`]), not a short wait.
const ADST_POLL_LIMIT: u32 = 100_000;

/// One claimed ADC140 unit, converting one channel at a time.
///
/// The reference is left at its reset selection — `ADHVREFCNT = 0x00`
/// means AVCC0/AVSS0 (manual §35.2.31) — which on the UNO R4 is the
/// board's 5 V analog rail (claimed by the Arduino core's 0–5 V analog
/// range; the slice 8 rails test is the hardware proof). The
/// internal-reference path (manual §35.7) is future work
/// and out of this driver's scope.
///
/// Sampling times (`ADSSTRn`) are left at their reset value (0Dh = 13
/// states), which satisfies the manual's lower limit at this crate's
/// PCLKB:PCLKC ratio of 1:2 (≥ 6 states, §35.2.14) and keeps the
/// conversion near the ~1.3 µs minimum.
pub struct Adc {
    _adc: ra4m1::ADC140,
    resolution: Resolution,
}

impl Adc {
    /// Claims the ADC140 and brings it up in single scan mode.
    ///
    /// Consumes the PAC's `ADC140` (exactly-once claim, like everything
    /// `Peripherals::take` hands out). `resolution` selects the result
    /// width (12-bit is the default; the reset value is also 12-bit).
    ///
    /// Bring-up (manual §35.8.5 + §35.2.3/§35.2.11, with every register
    /// the driver relies on rewritten rather than trusted from a
    /// bootloader handoff):
    ///
    /// 1. module stop released (`MSTPCRD.MSTPD16 = 0`, manual §10,
    ///    MSTPCRD, p.175: bit 16 targets the "14-Bit A/D Converter
    ///    Module");
    /// 2. the §35.8.5 settle — at least 1 µs between module-stop
    ///    release and the first conversion — spent as a cycle-counted
    ///    spin at the known 48 MHz ICLK;
    /// 3. `ADCSR`: single scan mode, high-speed conversion
    ///    (`ADHSC = 0`), no ELC/ADTRG0 triggers (`TRGE`/`EXTRG = 0`),
    ///    no double trigger, stopped (`ADST = 0`);
    /// 4. `ADCER`: right-justified data (`ADRFMT = 0`), the requested
    ///    accuracy (`ADPRC`), auto-clear off, self-diagnosis off;
    /// 5. channel selects cleared — every read writes exactly one.
    pub fn new(adc: ra4m1::ADC140, resolution: Resolution) -> Self {
        Self::release_module_stop();
        // §35.8.5: "After release from the module-stop state, wait for
        // at least 1 µs before starting A/D conversion." One microsecond
        // at the fixed HOCO 48 MHz clock.
        cortex_m::asm::delay(crate::clock::HOCO_FREQ_HZ / 1_000_000);

        adc.adcsr.write(|w| {
            w.adcs()
                ._00() // single scan mode
                .adhsc()
                .clear_bit() // high-speed conversion mode
                .trge()
                .clear_bit()
                .extrg()
                .clear_bit()
                .dble()
                .clear_bit()
                .adst()
                .clear_bit()
        });
        adc.adcer.write(|w| {
            let w = w.adrfmt().clear_bit().ace().clear_bit().diagm().clear_bit();
            match resolution {
                Resolution::Bits12 => w.adprc()._00(),
                Resolution::Bits14 => w.adprc()._11(),
            }
        });
        adc.adansa0.write(|w| unsafe { w.bits(0) });
        adc.adansa1.write(|w| unsafe { w.bits(0) });

        Self {
            _adc: adc,
            resolution,
        }
    }

    /// The resolution this unit was configured for.
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// One single-shot, software-triggered, blocking conversion.
    ///
    /// The pin must be in its analog state
    /// ([`Pin::into_analog`](crate::gpio::Pin::into_analog)); its
    /// position selects the channel — the six header pins A0–A5 are the
    /// only implementors of [`Channel`], so no other pin type-checks.
    ///
    /// Sequence (manual §35.3.2.1):
    ///
    /// 1. the channel's select bit goes into `ADANSA0` (AN000–AN014) or
    ///    `ADANSA1` (AN016–AN025) — exactly one channel per scan;
    /// 2. `ADCSR.ADST = 1` starts the scan. The software trigger is
    ///    enabled regardless of `TRGE`/`EXTRG`/`TRSA` (§35.2.12, TRSA
    ///    description);
    /// 3. `ADST` stays 1 during the conversion and **the hardware
    ///    clears it when the scan completes** (§35.3.2.1 step 4) — the
    ///    poll waits for that self-clear, bounded by
    ///    [`ADST_POLL_LIMIT`];
    /// 4. the channel's `ADDRy` is read, right-justified, masked to the
    ///    configured resolution.
    pub fn read<const PORT: char, const PIN: u8>(
        &mut self,
        _pin: &mut Pin<PORT, PIN, Analog>,
    ) -> ReadResult<u16>
    where
        Pin<PORT, PIN, Analog>: Channel,
    {
        let an = <Pin<PORT, PIN, Analog> as Channel>::AN;
        let adc = &self._adc;

        // 1. Select exactly this channel for group A (single scan).
        if an <= 14 {
            adc.adansa0.write(|w| unsafe { w.bits(1 << an) });
            adc.adansa1.write(|w| unsafe { w.bits(0) });
        } else {
            adc.adansa0.write(|w| unsafe { w.bits(0) });
            adc.adansa1.write(|w| unsafe { w.bits(1 << (an - 16)) });
        }

        // 2. Software start.
        adc.adcsr.modify(|_, w| w.adst().set_bit());

        // 3. Wait for the hardware's self-clear (§35.3.2.1 step 4).
        let mut polls = 0;
        while adc.adcsr.read().adst().is_1() {
            polls += 1;
            if polls > ADST_POLL_LIMIT {
                return Err(ReadError::Timeout);
            }
        }

        // 4. The result. `addr` indexes AN000–AN014; AN016+ sit in
        //    individually-named registers. The `_` arm is unreachable:
        //    `Channel` is sealed to AN ∈ {0, 1, 2, 9, 21, 22}.
        let raw = match an {
            0..=14 => adc.addr[an as usize].read().addr().bits(),
            21 => adc.addr21.read().addr().bits(),
            22 => adc.addr22.read().addr().bits(),
            _ => 0,
        };
        Ok(raw & self.resolution.mask())
    }

    /// Takes the ADC140 out of module stop (`MSTPCRD.MSTPD16 = 0`).
    /// Manual §10 "Low Power Modes", MSTPCRD, p.175: bit 16 targets the
    /// "14-Bit A/D Converter Module". Addresses from the SVD: `MSTP`
    /// block at `0x4004_7000`, `MSTPCRD` at offset `0x0C` (A/B/C/D at
    /// 0x00/04/08/0C) — the same release shape as the GPT drivers.
    fn release_module_stop() {
        let system = unsafe { &*ra4m1::SYSTEM::PTR };
        let mstp = unsafe { &*ra4m1::MSTP::PTR };
        system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
        mstp.mstpcrd.modify(|_, w| w.mstpd16().clear_bit());
        system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
    }
}
