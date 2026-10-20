//! The SCI channel in interrupt mode: ring buffers served by the
//! engine's RXI/TXI/ERI events, linked through `intc` (Slice 5).
//!
//! The handle is a `&'static` shared one — the user declares the
//! instance as a `static` and passes nothing else — because the
//! interrupt handlers must be able to reach it after `attach` returns.
//! All state is touched under a critical section (`critical-section`
//! provider, see `crate::intc`), which is what makes the manual
//! `unsafe impl Sync` below sound; the rings themselves are single-
//! producer/single-consumer ([`Ring`]).
//!
//! Interrupt flow, per engine (manual, SCI chapter; `SCR` bit numbers
//! from the SVD):
//! - `RIE = 1` requests an RXI event when `SSR.RDRF` sets and an ERI
//!   event when a receive error flag (ORER/FER/PER) sets — one error
//!   path for both, because an uncleared ORER blocks `RDRF` forever.
//! - TXI requests service when TDR can accept the next byte while
//!   `SCR.TIE` is enabled (RA4M1 manual R01UH0887EJ0100 §28.3.8).
//!   The driver primes TDR when idle and leaves subsequent bytes to
//!   TXI. The handler disables TIE when the transmit ring is empty.
//! - `TEIE` (transmit end) stays off; [`UartIrq::flush`] polls `TEND`,
//!   which needs no interrupt because it is only ever called from the
//!   main thread.
//!
//! The three events each get their own IELSR slot (3 per attached
//! engine, allocated lowest-first by [`Ielsr::take_any`]).
//! After attaching, main must call [`crate::intc::enable_interrupts`]
//! outside any critical section to unmask the CPU after the bootloader.

use crate::{
    clock::Clocks,
    gpio::{Input, Pin},
    intc::{Ielsr, register_handler},
    uart::{
        BaudParams, PSEL_SCI2, PSEL_SCI9, ReadError, Ring, SciChannel, calc_baud, init_sci,
        release_module_stop_sci2, release_module_stop_sci9,
    },
};

use core::{
    fmt,
    marker::PhantomData,
    sync::atomic::{AtomicU8, AtomicU32, Ordering},
};
use critical_section as cs;

/// `SSR` error-flag mask: PER (bit 3), FER (4), ORER (5) — the same
/// read-modify-write the polling driver's `clear_receiver_errors` uses.
const SSR_ERROR_MASK: u8 = 0x38;

/// `SCR.TIE` (bit 7) / `SCR.RIE` (bit 6) — interrupt request enables
/// (SVD).
const SCR_TIE_BIT: u8 = 1 << 7;
const SCR_RIE_BIT: u8 = 1 << 6;

/// Main may write TDR only when the ISR has relinquished ownership and
/// the data register can accept a byte. TDRE alone is insufficient.
pub(crate) const fn can_prime_tx(scr: u8, tdre: bool) -> bool {
    scr & SCR_TIE_BIT == 0 && tdre
}

/// Attach-once claims, one bit per [`SciChannel::CHANNEL_ID`].
static ATTACHED: AtomicU8 = AtomicU8::new(0);

/// One claimed SCI engine running in interrupt mode.
///
/// `S` is the PAC's peripheral marker for the engine; `RX`/`TX` are the
/// ring sizes in bytes. Both must be powers of two (checked at
/// construction), and the rings live in the instance's own memory —
/// this type is made for `static` declarations:
///
/// ```ignore
/// static UART9: UartIrq<ra4m1::SCI9> = UartIrq::new();
/// UART9.sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
/// // Main startup, outside any critical section:
/// unsafe { unor4_hal::intc::enable_interrupts() };
/// ```
pub struct UartIrq<S = ra4m1::SCI2, const RX: usize = 256, const TX: usize = 128> {
    rx: Ring<RX>,
    tx: Ring<TX>,
    /// Bytes dropped because the RX ring was full when the RXI handler
    /// ran. The handler always drains `RDR` — leaving it set would
    /// re-assert RXI forever — it just may not have room to keep them.
    overflowed: AtomicU32,
    /// Receive error events (ORER/FER/PER) seen by the ERI handler.
    errored: AtomicU32,
    /// Diagnostic: TXI handler invocations (slice 5 debugging).
    txi_events: AtomicU32,
    _engine: PhantomData<*const S>,
}

// Sound: all interior state is either atomic or touched under a
// critical section, and the register block behind `S::PTR` is claimed
// exactly once per engine at attach time. `Send` stays unimplemented —
// the instance belongs to the engine it attached to.
unsafe impl<S: SciChannel, const RX: usize, const TX: usize> Sync for UartIrq<S, RX, TX> {}

impl<const RX: usize, const TX: usize> UartIrq<ra4m1::SCI2, RX, TX> {
    /// Claims SCI2 on the header's **D1 = TX** (P302), **D0 = RX**
    /// (P301) — the same physical pins on both board maps — and starts
    /// serving it from interrupts. Consumes the PAC's `SCI2` and the
    /// two pins (exactly-once claims, like everything
    /// `Peripherals::take` hands out).
    ///
    /// # Panics
    /// If `baud` cannot be reached within 3% of PCLKA, if SCI2 was
    /// already attached in IRQ mode, or if fewer than three IELSR
    /// slots are free (one each for RXI, TXI, ERI).
    pub fn sci2(
        &'static self,
        sci: ra4m1::SCI2,
        tx: Pin<'3', 2, Input>,
        rx: Pin<'3', 1, Input>,
        baud: u32,
        clocks: &Clocks,
    ) {
        self.attach(
            sci,
            tx,
            rx,
            PSEL_SCI2,
            baud,
            clocks,
            release_module_stop_sci2,
        );
    }
}

impl<const RX: usize, const TX: usize> UartIrq<ra4m1::SCI9, RX, TX> {
    /// Claims SCI9 on the board's internal monitor line: **TX = P109**,
    /// **RX = P110** (see [`Uart::sci9`](super::Uart::sci9) for the
    /// wiring and the Minima header equivalents). Consumes the PAC's
    /// `SCI9` and the two pins.
    ///
    /// # Panics
    /// If `baud` cannot be reached within 3% of PCLKA, if SCI9 was
    /// already attached in IRQ mode, or if fewer than three IELSR
    /// slots are free.
    pub fn sci9(
        &'static self,
        sci: ra4m1::SCI9,
        tx: Pin<'1', 9, Input>,
        rx: Pin<'1', 10, Input>,
        baud: u32,
        clocks: &Clocks,
    ) {
        self.attach(
            sci,
            tx,
            rx,
            PSEL_SCI9,
            baud,
            clocks,
            release_module_stop_sci9,
        );
    }
}

impl<S, const RX: usize, const TX: usize> UartIrq<S, RX, TX>
where
    S: SciChannel,
{
    /// Builds an unattached instance for a `static` declaration.
    // No `Default`: an unattached instance is not a meaningful zero
    // value — `new` exists for `static` declarations only.
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            rx: Ring::new(),
            tx: Ring::new(),
            overflowed: AtomicU32::new(0),
            errored: AtomicU32::new(0),
            txi_events: AtomicU32::new(0),
            _engine: PhantomData,
        }
    }

    /// Shared attach path: bring the engine up exactly as the polling
    /// driver does, then link the three interrupt events through
    /// `intc` and switch `RIE` on. `TIE` stays off until the first
    /// [`UartIrq::write`] queues bytes and primes the idle transmitter
    /// (see [`UartIrq::write`]), so an idle transmitter never raises an
    /// interrupt.
    // Eight parameters: the two pins carry their own const-generic
    // board coordinates, and splitting them off into a config struct
    // would hide the per-channel facts (PSEL, module stop) the
    // constructors above already own.
    #[allow(clippy::too_many_arguments)]
    fn attach<const TX_PORT: char, const TX_PIN: u8, const RX_PORT: char, const RX_PIN: u8>(
        &'static self,
        sci: S,
        tx: Pin<TX_PORT, TX_PIN, Input>,
        rx: Pin<RX_PORT, RX_PIN, Input>,
        psel: u8,
        baud: u32,
        clocks: &Clocks,
        release_module_stop: fn(),
    ) {
        // Attach-once per engine: the rings live here, so a second
        // attach would silently steal the first one's data path.
        let mask = 1 << S::CHANNEL_ID;
        assert!(
            ATTACHED.fetch_or(mask, Ordering::AcqRel) & mask == 0,
            "SCI engine already attached in IRQ mode"
        );

        release_module_stop();
        let params: BaudParams =
            calc_baud(clocks.pclka(), baud).expect("requested baud is out of tolerance for PCLKA");
        init_sci(&sci, tx, rx, psel, params);

        // One IELSR slot per event, handlers before enables (the
        // dispatch registry's contract). TEI is not used: flush polls
        // TEND, so no transmit-end slot or no-op ISR is needed.
        let mut txi = Ielsr::take_any().expect("no free IELSR slot for SCI TXI");
        let mut rxi = Ielsr::take_any().expect("no free IELSR slot for SCI RXI");
        let mut eri = Ielsr::take_any().expect("no free IELSR slot for SCI ERI");

        let ctx = self as *const _ as *const ();
        register_handler(txi.slot(), txi_thunk::<S, RX, TX>, ctx);
        register_handler(rxi.slot(), rxi_thunk::<S, RX, TX>, ctx);
        register_handler(eri.slot(), eri_thunk::<S, RX, TX>, ctx);
        txi.link(S::EVENT_TXI);
        rxi.link(S::EVENT_RXI);
        eri.link(S::EVENT_ERI);
        txi.enable();
        rxi.enable();
        eri.enable();

        // Last step: request-enable for receive (RXI + ERI). Any byte
        // arriving from here on is served by the RXI handler.
        cs::with(|_| {
            let sci = unsafe { &*S::PTR };
            let scr = sci.scr().read().bits();
            sci.scr().write(|w| unsafe { w.bits(scr | SCR_RIE_BIT) });
        });
    }

    /// RXI handler: drains `RDR` into the receive ring. An uncleared
    /// `RDRF` would re-assert the interrupt forever, so the handler
    /// always reads — even if the ring is full and the byte must be
    /// dropped (counted in [`UartIrq::overflowed`]).
    fn isr_rxi(&self) {
        cs::with(|_| {
            let sci = unsafe { &*S::PTR };
            loop {
                let ssr = sci.ssr().read();
                // Error flags are the ERI handler's problem; with ORER
                // set, RDRF never rises again anyway.
                if ssr.orer().bit_is_set() || ssr.fer().bit_is_set() || ssr.per().bit_is_set() {
                    break;
                }
                if !ssr.rdrf().bit_is_set() {
                    break;
                }
                let byte = sci.rdr.read().bits();
                if !self.rx.push(byte) {
                    self.overflowed.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
    }

    /// TXI handler: moves ONE byte from the transmit ring into `TDR`,
    /// and turns `TIE` off when the ring runs dry — an idle transmitter
    /// must not keep interrupting. The non-FIFO SCI needs one TDR
    /// write per request, not a polling
    /// loop that waits for a byte to finish inside the ISR.
    fn isr_txi(&self) {
        self.txi_events.fetch_add(1, Ordering::Relaxed);
        cs::with(|_| {
            let sci = unsafe { &*S::PTR };
            match self.tx.pop() {
                Some(byte) => {
                    sci.tdr.write(|w| unsafe { w.bits(byte) });
                }
                None => {
                    let scr = sci.scr().read().bits();
                    sci.scr().write(|w| unsafe { w.bits(scr & !SCR_TIE_BIT) });
                }
            }
        });
    }

    /// ERI handler: clears the receive error flags (they block `RDRF`
    /// while set) and counts the event. The dropped byte(s) behind the
    /// error are gone regardless — that is what the flags mean.
    fn isr_eri(&self) {
        cs::with(|_| {
            let sci = unsafe { &*S::PTR };
            let ssr = sci.ssr().read().bits();
            if ssr & SSR_ERROR_MASK != 0 {
                sci.ssr()
                    .write(|w| unsafe { w.bits(ssr & !SSR_ERROR_MASK) });
                self.errored.fetch_add(1, Ordering::Relaxed);
            }
        });
    }

    /// Reads one byte from the receive ring, if any. Non-blocking —
    /// bytes arrive in the RXI handler.
    pub fn read_byte(&self) -> Option<u8> {
        cs::with(|_| self.rx.pop())
    }

    /// Queues as many bytes as fit in the transmit ring. Returns how
    /// many were accepted into the TX path (may be 0 — the ring is full
    /// and the TXI handler is still draining); never blocks.
    ///
    /// When TX interrupts are idle and TDR is empty, prime it directly
    /// so a fresh queued write starts. While TXI is active, only the
    /// handler writes TDR; TDRE alone does not establish ownership.
    pub fn write(&self, bytes: &[u8]) -> usize {
        cs::with(|_| {
            let mut n = 0;
            for &byte in bytes {
                if !self.tx.push(byte) {
                    break;
                }
                n += 1;
            }
            if n > 0 {
                let sci = unsafe { &*S::PTR };
                let scr = sci.scr().read().bits();
                // Only take over TDR when TXI has relinquished it.
                // TDRE alone is not an idle test: a TXI may already be
                // pending. Priming then lets that ISR overwrite TDR
                // before the primed byte transfers to the shift register.
                if scr & SCR_TIE_BIT == 0 {
                    sci.scr().write(|w| unsafe { w.bits(scr | SCR_TIE_BIT) });
                }
                if can_prime_tx(scr, sci.ssr().read().tdre().bit_is_set())
                    && let Some(byte) = self.tx.pop()
                {
                    sci.tdr.write(|w| unsafe { w.bits(byte) });
                }
            }
            n
        })
    }

    /// Queues every byte, spinning while the ring is full (the TXI
    /// handler drains it at line rate). Blocks until the last byte is
    /// *queued*, not sent — follow with [`UartIrq::flush`] if the line
    /// must be idle before the next operation.
    pub fn write_all(&self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let queued = self.write(bytes);
            bytes = &bytes[queued..];
        }
    }

    /// Blocks until the transmit ring is empty **and** `SSR.TEND`
    /// shows the shift register drained — the wire is idle when this
    /// returns.
    pub fn flush(&self) {
        loop {
            let empty = cs::with(|_| self.tx.is_empty());
            if empty && unsafe { &*S::PTR }.ssr().read().tend().bit_is_set() {
                return;
            }
        }
    }

    /// Bytes dropped so far because the receive ring was full (see
    /// [`UartIrq::read_byte`]).
    pub fn overflowed(&self) -> u32 {
        self.overflowed.load(Ordering::Relaxed)
    }

    /// Receive error events (ORER/FER/PER) handled so far.
    pub fn errored(&self) -> u32 {
        self.errored.load(Ordering::Relaxed)
    }

    /// Diagnostic: TXI handler invocations so far (slice 5 debugging).
    pub fn txi_events(&self) -> u32 {
        self.txi_events.load(Ordering::Relaxed)
    }

    /// Whether the transmit path is fully idle (ring empty + TEND).
    fn tx_idle(&self) -> bool {
        cs::with(|_| self.tx.is_empty()) && unsafe { &*S::PTR }.ssr().read().tend().bit_is_set()
    }
}

/// RXI dispatch thunk: recovers the instance from the registry context
/// and runs its handler. One monomorphization per engine + ring sizes.
fn rxi_thunk<S: SciChannel, const RX: usize, const TX: usize>(ctx: *const ()) {
    // Safety: the context was registered by `attach` for this exact
    // instance, which lives forever behind a `&'static`.
    let uart = unsafe { &*(ctx as *const UartIrq<S, RX, TX>) };
    uart.isr_rxi();
}

/// TXI dispatch thunk — see [`rxi_thunk`].
fn txi_thunk<S: SciChannel, const RX: usize, const TX: usize>(ctx: *const ()) {
    // Safety: as in `rxi_thunk`.
    let uart = unsafe { &*(ctx as *const UartIrq<S, RX, TX>) };
    uart.isr_txi();
}

/// ERI dispatch thunk — see [`rxi_thunk`].
fn eri_thunk<S: SciChannel, const RX: usize, const TX: usize>(ctx: *const ()) {
    // Safety: as in `rxi_thunk`.
    let uart = unsafe { &*(ctx as *const UartIrq<S, RX, TX>) };
    uart.isr_eri();
}

// ---------------------------------------------------------------------------------------------- //
// Trait impls. All on `&UartIrq`: the user owns a `&'static`, and the
// shared-handle shape (impl for the reference) matches how the driver
// is passed around. `embedded-io`/`embedded-hal-nb`/`nb` compile only
// under the `uart-irq` feature (the whole file is feature-gated).

impl<S: SciChannel, const RX: usize, const TX: usize> fmt::Write for &UartIrq<S, RX, TX> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        (**self).write_all(s.as_bytes());
        Ok(())
    }
}

/// `embedded-hal-nb` error plumbing: the reference handle is what the
/// serial traits are implemented for, so it needs its own `ErrorType`
/// (embedded-hal-nb only blankets `&mut T`).
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_hal_nb::serial::ErrorType
    for &UartIrq<S, RX, TX>
{
    type Error = core::convert::Infallible;
}

/// `embedded-hal-nb` non-blocking receive.
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_hal_nb::serial::Read
    for &UartIrq<S, RX, TX>
{
    fn read(&mut self) -> Result<u8, nb::Error<Self::Error>> {
        (**self).read_byte().ok_or(nb::Error::WouldBlock)
    }
}

/// `embedded-hal-nb` non-blocking transmit: one word per call,
/// `WouldBlock` while the ring is full.
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_hal_nb::serial::Write
    for &UartIrq<S, RX, TX>
{
    fn write(&mut self, word: u8) -> Result<(), nb::Error<Self::Error>> {
        match (**self).write(&[word]) {
            0 => Err(nb::Error::WouldBlock),
            _ => Ok(()),
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if (**self).tx_idle() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// `embedded-io` type plumbing: the error type all blocking IO
/// operations on the reference handle report.
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_io::Io for &UartIrq<S, RX, TX> {
    type Error = ReadError;
}

impl embedded_io::Error for ReadError {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

/// `embedded-io` receive: `ReadError::Empty` is the would-block signal
/// (nothing new since the last read).
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_io::blocking::Read
    for &UartIrq<S, RX, TX>
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        let mut n = 0;
        cs::with(|_| {
            while n < buf.len() {
                match self.rx.pop() {
                    Some(byte) => {
                        buf[n] = byte;
                        n += 1;
                    }
                    None => break,
                }
            }
        });
        if n == 0 { Err(ReadError::Empty) } else { Ok(n) }
    }
}

/// `embedded-io` transmit: blocks until at least one byte is queued
/// (never returns `Ok(0)` for a non-empty buffer) and flushes until the
/// wire is idle.
impl<S: SciChannel, const RX: usize, const TX: usize> embedded_io::blocking::Write
    for &UartIrq<S, RX, TX>
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            return Ok(0);
        }
        loop {
            let queued = (**self).write(buf);
            if queued > 0 {
                return Ok(queued);
            }
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        (**self).flush();
        Ok(())
    }
}
