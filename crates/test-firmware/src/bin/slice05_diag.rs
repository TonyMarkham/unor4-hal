//! Slice 5 diagnostic: after the IRQ-mode attach, dumps the live
//! interrupt-pipeline state as hex lines over SCI9 using *polling*
//! TX writes (the wire path slice 4 proved), so no transcription is
//! needed. Not a slice deliverable.
//!
//! Register addresses: ARMv7-M System Control Space (ARM ARM B3.2)
//! and the RA4M1 SVD.

#![no_std]
#![no_main]

use unor4_hal::{Peripherals, uart::UartIrq};

use embedded_hal::delay::DelayNs;
use panic_halt as _;
use ra4m1::SCI9;

static UART9: UartIrq<SCI9> = UartIrq::new();

/// One byte out via polled TDR — bypasses the (broken) IRQ TX path.
fn poll_put(byte: u8) {
    let sci = unsafe { &*SCI9::PTR };
    while !sci.ssr().read().tdre().bit_is_set() {}
    sci.tdr.write(|w| unsafe { w.bits(byte) });
}

fn poll_str(s: &[u8]) {
    for &b in s {
        poll_put(b);
    }
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn poll_hex32(value: u32) {
    poll_str(b"0x");
    for i in (0..32).step_by(4).rev() {
        poll_put(HEX[((value >> i) & 0xF) as usize]);
    }
    poll_str(b"\r\n");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let mut delay = p.delay;
    let mut led = p.pins.d13.into_output();

    // Same gate as slice 5: the monitor attaches first.
    let button = p.pins.d2.into_input_pullup();
    while !button.is_low() {
        delay.delay_ms(10);
    }

    #[cfg(feature = "wifi")]
    UART9.sci9(p.sci9, p.sci9_tx, p.sci9_rx, 115_200, &p.clocks);
    #[cfg(feature = "minima")]
    UART9.sci9(p.sci9, p.pins.d11, p.pins.d12, 115_200, &p.clocks);

    unsafe { unor4_hal::intc::enable_interrupts() };

    // Queue the banner: TIE on + TDRE set = the TXI request is asserted
    // from here on — exactly the request that never gets taken.
    UART9.write_all(b"unor4-hal slice 05 DIAG - SCI9 IRQ, 115200 8N1\r\n");

    // Latch the pipeline state via raw volatile reads.
    let primask: u32;
    let faultmask: u32;
    unsafe {
        core::arch::asm!("mrs {}, primask", out(reg) primask);
        core::arch::asm!("mrs {}, faultmask", out(reg) faultmask);
    }
    let icsr = unsafe { core::ptr::read_volatile(0xE000ED04u32 as *const u32) };
    let vtor = unsafe { core::ptr::read_volatile(0xE000ED08u32 as *const u32) };
    let ielsr1 = unsafe { &*ra4m1::ICU::PTR }.ielsr[1].read().bits();
    let sci = unsafe { &*SCI9::PTR };
    let scr = sci.scr().read().bits();
    let ssr = sci.ssr().read().bits();
    let iser0 = unsafe { core::ptr::read_volatile(0xE000E100u32 as *const u32) };

    // Keep TXI quiet while dumping so the polled lines stay clean.
    let sci = unsafe { &*SCI9::PTR };
    let scr_bits = sci.scr().read().bits();
    sci.scr().write(|w| unsafe { w.bits(scr_bits & !0x80) });

    poll_str(b"PRIMASK   = ");
    poll_hex32(primask);
    poll_str(b"FAULTMASK = ");
    poll_hex32(faultmask);
    poll_str(b"ICSR      = ");
    poll_hex32(icsr);
    poll_str(b"VTOR      = ");
    poll_hex32(vtor);
    poll_str(b"IELSR1    = ");
    poll_hex32(ielsr1);
    poll_str(b"SCR       = ");
    poll_hex32(scr as u32);
    poll_str(b"SSR       = ");
    poll_hex32(ssr as u32);
    poll_str(b"ISER0     = ");
    poll_hex32(iser0);
    poll_str(b"IELSR0    = ");
    poll_hex32(unsafe { &*ra4m1::ICU::PTR }.ielsr[0].read().bits());
    poll_str(b"IELSR2    = ");
    poll_hex32(unsafe { &*ra4m1::ICU::PTR }.ielsr[2].read().bits());

    // Hand TX back to the interrupt path. If interrupts actually fire,
    // the banner in the ring drains and appears in the monitor now.
    sci.scr().write(|w| unsafe { w.bits(scr_bits) });

    // Telemetry: one byte per round through the IRQ path, then the
    // live register state. If the chain works, an X appears each round
    // and TIE stays 1. If the handler cleared TIE (pop hit an empty
    // ring), TIE reads 0. If the request never latches, IELSR1's IR
    // (bit 16) stays 0 while TIE is 1.
    loop {
        UART9.write_all(b"X");
        delay.delay_ms(100);
        // Drain the RX ring and count what arrived this round. If typed
        // bytes reach the board, the RXI handler (slot 0) pushed them
        // here — N must jump by the number of characters typed.
        let mut rx_count: u32 = 0;
        while UART9.read_byte().is_some() {
            rx_count += 1;
        }
        let scr_now = sci.scr().read().bits() as u32;
        let ssr_now = sci.ssr().read().bits() as u32;
        let ielsr1_now = unsafe { &*ra4m1::ICU::PTR }.ielsr[1].read().bits();
        poll_str(b"S ");
        poll_hex32(scr_now);
        poll_str(b"R ");
        poll_hex32(ssr_now);
        poll_str(b"I ");
        poll_hex32(ielsr1_now);
        poll_str(b"T ");
        poll_hex32(UART9.txi_events());
        poll_str(b"N ");
        poll_hex32(rx_count);
        poll_str(b"E ");
        poll_hex32(UART9.errored());
        poll_str(b"O ");
        poll_hex32(UART9.overflowed());
        led.toggle();
        delay.delay_ms(2000);
    }
}
