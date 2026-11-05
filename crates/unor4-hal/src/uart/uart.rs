//! The SCI channel type: one claim, polling TX/RX.

use crate::{
    clock::Clocks,
    gpio::{
        Input, Pin, SciPin,
        role::{Rxd, Txd},
    },
    uart::{BaudParams, ReadError, ReadResult, SciChannel, calc_baud},
};

/// One claimed SCI channel, running in polling mode (8N1).
///
/// `S` is the PAC's peripheral marker for the engine — SCI2 (the default,
/// so the bare `Uart` name still means the header UART), SCI9 or SCI1. The
/// marker is held, not copied, so a second `Uart` for the same channel is
/// impossible through the normal ownership path.
pub struct Uart<S = ra4m1::SCI2> {
    _sci: S,
}

impl<S> Uart<S>
where
    S: SciChannel,
{
    /// Claims SCI channel `sci` on `tx` and `rx` at `baud`.
    ///
    /// The pins must carry the channel's `TXD`/`RXD`; both are type checks
    /// on [`SciPin`], so a wrong pair fails `cargo check`:
    /// `` `D1` cannot be `SCI1` `Txd` ``. The common uses:
    /// - header UART, both boards: `Uart::new(p.sci2, p.pins.d1, p.pins.d0, …)`
    ///   (D1 = TX on P302, D0 = RX on P301);
    /// - monitor line to the ESP32-S3 bridge, WiFi:
    ///   `Uart::new(p.sci9, p.sci9_tx, p.sci9_rx, …)` (P109/P110 through the
    ///   level translator); on the Minima the same chip pins are headers
    ///   D11/D12;
    /// - WiFi AT line: `Uart::new(p.sci1, p.sci1_tx, p.sci1_rx, …)`
    ///   (P501/P502, board-internal to the ESP32-S3's AT server, 115200 8N1).
    ///
    /// Consumes the PAC's channel and the two pins in their reset-default
    /// input state (exactly-once claims, like everything `Peripherals::take`
    /// hands out); the baud reference is PCLKA.
    ///
    /// # Panics
    /// If `baud` cannot be reached within 3% of PCLKA.
    pub fn new<Tx, Rx>(
        sci: S,
        tx: Pin<Tx, Input>,
        rx: Pin<Rx, Input>,
        baud: u32,
        clocks: &Clocks,
    ) -> Self
    where
        Tx: SciPin<S, Txd>,
        Rx: SciPin<S, Rxd>,
    {
        S::release_module_stop();
        let params: BaudParams =
            calc_baud(clocks.pclka(), baud).expect("requested baud is out of tolerance for PCLKA");
        init_sci(&sci, tx, rx, params);

        Uart { _sci: sci }
    }

    /// Sends one byte, blocking until the data register is free.
    pub fn write_byte(&mut self, byte: u8) {
        let sci = self._sci.block();
        // TDRE: 1 = TDR empty (resets to 1, so the first byte never waits).
        while !sci.ssr().read().tdre().bit_is_set() {}
        // Writing TDR clears TDRE back to 0.
        sci.tdr.write(|w| unsafe { w.bits(byte) });
    }

    /// Sends a slice, then waits for the shift register to drain (TEND)
    /// — the line is guaranteed idle when this returns.
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
        while !self._sci.block().ssr().read().tend().bit_is_set() {}
    }

    /// Reads one received byte, non-blocking.
    pub fn read_byte(&mut self) -> ReadResult<u8> {
        let sci = self._sci.block();
        let ssr = sci.ssr().read();
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
        Ok(sci.rdr.read().bits())
    }

    /// Writes 0 to PER/FER/ORER; the other bits are written back as read
    /// (the same read-modify-write FSP's receive-error path does).
    fn clear_receiver_errors(&mut self) {
        let sci = self._sci.block();
        let ssr = sci.ssr().read().bits();
        sci.ssr().write(|w| unsafe { w.bits(ssr & !0x38) });
    }
}

/// Shared SCI bring-up for both driver modes: pin muxing, format
/// registers, and the baud generator — with the transmitter and
/// receiver on and every interrupt source off. The register sequence is
/// exactly the one Slice 3 proved on hardware (`Uart::new` was the
/// first user; Slice 5's IRQ driver brings an engine up through this
/// too, then turns `RIE`/`TIE` on once its interrupts are linked). Each
/// pin is muxed with its own `PSEL` from the pin table: TX and RX can
/// differ (see the module docs).
pub(crate) fn init_sci<S, Tx, Rx>(
    sci: &S,
    tx: Pin<Tx, Input>,
    rx: Pin<Rx, Input>,
    params: BaudParams,
) where
    S: SciChannel,
    Tx: SciPin<S, Txd>,
    Rx: SciPin<S, Rxd>,
{
    // One shared register view for every engine (see `SciChannel::block`
    // for the SCI1 layout-identity evidence).
    let sci = sci.block();
    // Drive Txd to idle-high BEFORE the pin enters peripheral mode:
    // SPTR is writable while TE = 0, and SPB2IO = 1 / SPB2DT = 1
    // makes the SCI output a static high on TxD, so there is no
    // window where the receiver can latch a false start bit.
    sci.sptr.write(|w| w.spb2io()._1().spb2dt()._1());
    let _tx = tx.into_alternate(Tx::PSEL);
    let _rx = rx.into_alternate(Rx::PSEL);

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
