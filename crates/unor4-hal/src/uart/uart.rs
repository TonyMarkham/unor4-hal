//! The SCI channel type: one claim, polling TX/RX.

use crate::{
    clock::Clocks,
    gpio::{Input, Pin},
    uart::{BaudParams, ReadError, ReadResult, SciChannel, calc_baud},
};

/// `PRCR` key + PRC1 write-enable (module-stop registers are PRC1-
/// protected, like the clock registers of Slice 1).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

/// `PSEL` for TXD2/RXD2 on P302/P301 (see the parent module's docs for
/// the verification chain).
pub(crate) const PSEL_SCI2: u8 = 0x04;

/// `PSEL` for TXD9/RXD9 on P109/P110 (see the parent module's docs for
/// the verification chain).
pub(crate) const PSEL_SCI9: u8 = 0x05;

/// One claimed SCI channel, running in polling mode (8N1).
///
/// `S` is the PAC's peripheral marker for the engine — SCI2 (the
/// default, so the bare `Uart` name still means the header UART) or
/// SCI9. The marker is held, not copied, so a second `Uart` for the
/// same channel is impossible through the normal ownership path.
pub struct Uart<S = ra4m1::SCI2> {
    _sci: S,
}

impl Uart<ra4m1::SCI2> {
    /// Claims SCI2 on the header's **D1 = TX** (P302), **D0 = RX**
    /// (P301) — the same physical pins on both board maps.
    ///
    /// Consumes the PAC's `SCI2` (exactly-once claim, like everything
    /// `Peripherals::take` hands out), the two pins in their
    /// reset-default input state, and the resolved clocks (the baud
    /// reference is PCLKA).
    ///
    /// # Panics
    /// If `baud` cannot be reached within 3% of PCLKA.
    pub fn sci2(
        sci: ra4m1::SCI2,
        tx: Pin<'3', 2, Input>,
        rx: Pin<'3', 1, Input>,
        baud: u32,
        clocks: &Clocks,
    ) -> Self {
        release_module_stop_sci2();
        let params: BaudParams =
            calc_baud(clocks.pclka(), baud).expect("requested baud is out of tolerance for PCLKA");
        Self::init(sci, tx, rx, PSEL_SCI2, params)
    }
}

impl Uart<ra4m1::SCI9> {
    /// Claims SCI9 on the board's internal monitor line: **TX = P109**,
    /// **RX = P110**. On the WiFi board these are not header pins — they
    /// run through the level translator to the ESP32-S3, whose bridge
    /// firmware forwards the line to the board's own USB monitor port
    /// (Limitations.md, "phone line 2"). On the Minima the same chip
    /// pins *are* header D11/D12 — pass `pins.d11`/`pins.d12`.
    ///
    /// Consumes the PAC's `SCI9` (exactly-once claim, like everything
    /// `Peripherals::take` hands out), the two pins in their
    /// reset-default input state, and the resolved clocks (the baud
    /// reference is PCLKA).
    ///
    /// # Panics
    /// If `baud` cannot be reached within 3% of PCLKA.
    pub fn sci9(
        sci: ra4m1::SCI9,
        tx: Pin<'1', 9, Input>,
        rx: Pin<'1', 10, Input>,
        baud: u32,
        clocks: &Clocks,
    ) -> Self {
        release_module_stop_sci9();
        let params: BaudParams =
            calc_baud(clocks.pclka(), baud).expect("requested baud is out of tolerance for PCLKA");
        Self::init(sci, tx, rx, PSEL_SCI9, params)
    }
}

impl<S> Uart<S>
where
    S: SciChannel,
{
    /// Shared bring-up. Every SCI engine has the same non-FIFO register
    /// block (the PAC aliases `sci9` to `sci2`), so the sequence Slice 3
    /// proved on hardware applies verbatim — only the module-stop bit,
    /// the pins, and `PSEL` differ per engine.
    fn init<const TX_PORT: char, const TX_PIN: u8, const RX_PORT: char, const RX_PIN: u8>(
        sci: S,
        tx: Pin<TX_PORT, TX_PIN, Input>,
        rx: Pin<RX_PORT, RX_PIN, Input>,
        psel: u8,
        params: BaudParams,
    ) -> Self {
        init_sci(&sci, tx, rx, psel, params);

        Uart { _sci: sci }
    }

    /// Sends one byte, blocking until the data register is free.
    pub fn write_byte(&mut self, byte: u8) {
        // TDRE: 1 = TDR empty (resets to 1, so the first byte never waits).
        while !self._sci.ssr().read().tdre().bit_is_set() {}
        // Writing TDR clears TDRE back to 0.
        self._sci.tdr.write(|w| unsafe { w.bits(byte) });
    }

    /// Sends a slice, then waits for the shift register to drain (TEND)
    /// — the line is guaranteed idle when this returns.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
        while !self._sci.ssr().read().tend().bit_is_set() {}
    }

    /// Reads one received byte, non-blocking.
    pub fn read_byte(&mut self) -> ReadResult<u8> {
        let ssr = self._sci.ssr().read();
        // Error flags first: while ORER is set, RDRF never rises again.
        if ssr.orer().bit_is_set() {
            self.clear_receiver_errors();
            return Err(ReadError::Overrun);
        }
        if ssr.fer().bit_is_set() {
            self.clear_receiver_errors();
            return Err(ReadError::Framing);
        }
        if ssr.per().bit_is_set() {
            self.clear_receiver_errors();
            return Err(ReadError::Parity);
        }
        if !ssr.rdrf().bit_is_set() {
            return Err(ReadError::Empty);
        }
        // Reading RDR clears RDRF (non-FIFO SCI receive flow).
        Ok(self._sci.rdr.read().bits())
    }

    /// Writes 0 to PER/FER/ORER; the other bits are written back as read
    /// (the same read-modify-write FSP's receive-error path does).
    fn clear_receiver_errors(&mut self) {
        let ssr = self._sci.ssr().read().bits();
        self._sci.ssr().write(|w| unsafe { w.bits(ssr & !0x38) });
    }
}

/// Takes SCI2 out of module stop (`MSTPCRB.MSTPB29 = 0`). The MSTP
/// registers are PRC1-protected, so the write window unlocks and relocks
/// `PRCR`. Addresses from the SVD: `MSTP` block at `0x4004_7000`,
/// `MSTPCRB` first at offset `0x00`.
pub(crate) fn release_module_stop_sci2() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrb.modify(|_, w| w.mstpb29().clear_bit());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}

/// Takes SCI9 out of module stop (`MSTPCRB.MSTPB22 = 0`) — same `MSTP`
/// block and `PRCR.PRC1` unlock as SCI2; the SVD names bit 22 "Serial
/// Communication Interface 9 Module Stop".
pub(crate) fn release_module_stop_sci9() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrb.modify(|_, w| w.mstpb22().clear_bit());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}

/// Shared SCI bring-up for both driver modes: pin muxing, format
/// registers, and the baud generator — with the transmitter and
/// receiver on and every interrupt source off. The register sequence is
/// exactly the one Slice 3 proved on hardware (`Uart::init` was the
/// first user; Slice 5's IRQ driver brings an engine up through this
/// too, then turns `RIE`/`TIE` on once its interrupts are linked).
pub(crate) fn init_sci<
    S: SciChannel,
    const TX_PORT: char,
    const TX_PIN: u8,
    const RX_PORT: char,
    const RX_PIN: u8,
>(
    sci: &S,
    tx: Pin<TX_PORT, TX_PIN, Input>,
    rx: Pin<RX_PORT, RX_PIN, Input>,
    psel: u8,
    params: BaudParams,
) {
    // Drive Txd to idle-high BEFORE the pin enters peripheral mode:
    // SPTR is writable while TE = 0, and SPB2IO = 1 / SPB2DT = 1
    // makes the SCI output a static high on TxD, so there is no
    // window where the receiver can latch a false start bit.
    sci.sptr.write(|w| w.spb2io()._1().spb2dt()._1());
    let _tx = tx.into_alternate(psel);
    let _rx = rx.into_alternate(psel);

    // SCI initialization in asynchronous mode (manual, SCI chapter):
    // SCR = 0 first — TE/RE off, all interrupt sources off...
    sci.scr().write(|w| unsafe { w.bits(0) });
    // ...clear any latched flags (FSP's open does the same)...
    sci.ssr().write(|w| unsafe { w.bits(0) });
    // Explicitly leave alternate SCI modes off, rather than relying
    // on hardware-reset values after a bootloader handoff.
    sci.simr1.write(|w| unsafe { w.bits(0) });
    sci.simr2.write(|w| unsafe { w.bits(0) });
    sci.simr3.write(|w| unsafe { w.bits(0) });
    sci.cdr.write(|w| unsafe { w.bits(0) });
    sci.dccr.write(|w| unsafe { w.bits(0x40) });

    // SCMR: force the value FSP writes on every open (CHR1=1 → 8-bit
    // with SMR.CHR=0). The SVD claims CHR1 resets to 1; FSP rewriting
    // it defensively suggests the die's real reset may differ.
    sci.scmr.write(|w| unsafe { w.bits(0xF2) });

    // SMR: async (CM = 0), 8 data bits (CHR = 0 with the SCMR.CHR1
    // reset value 1), no parity (PE = 0), 1 stop bit (STPS = 0),
    // base clock = PCLKA / 4^cks. Only CKS[1:0] is nonzero.
    sci.smr().write(|w| unsafe { w.bits(params.cks) });
    // SEMR: only BGDM/ABCS from the calculation. NFEN = 0 (noise
    // filter off), BRME = 0 (no bit-rate modulation), ABCSE = 0,
    // RXDESEL = 0 (start bit detected as a low level).
    sci.semr.write(|w| unsafe {
        let mut bits: u8 = 0;
        if params.bgdm {
            bits |= 1 << 6;
        }
        if params.abcs {
            bits |= 1 << 4;
        }
        w.bits(bits)
    });
    // BRR: the N of the equation — written while TE/RE are off.
    sci.brr.write(|w| unsafe { w.bits(params.brr) });
    // SCR: internal baud clock (CKE = 00), transmitter and receiver
    // on, all interrupt sources off (polling).
    sci.scr().write(|w| w.cke()._00().te()._1().re()._1());
}
