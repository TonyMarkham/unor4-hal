//! The ICU event-code table (OQ-2): every event an `IELSRn` slot can
//! select, transcribed from the hardware manual's ICU chapter.
//!
//! Source and verification: Renesas RA4M1 hardware manual (R01UH0888),
//! ICU chapter event-code table, cross-checked entry-for-entry against
//! two independent transcriptions of the same table (the `ra-hal`
//! crate's `InterruptEvent` enum — itself citing manual §13/§18 — and a
//! community transcription with the same values). Codes exercised on
//! real hardware so far: the SCI2 and SCI9 RXI/TXI/ERI entries (Slices
//! 3–5 runs). The 0xB7–0xDA range is not enumerated here because the
//! cross-checks only populate its last entry (0xDB, SCE guard) — the
//! remaining codes are unverified and no driver needs them yet.
//!
//! `0x00` ("nothing is selected", the `IELSRn` reset value), `0x0E`,
//! `0x16` and `0x40` (reserved/unused per the same tables) are absent
//! on purpose: an `Ielsr` slot with no [`Ielsr::link`] call is the
//! unselected state.

/// One peripheral event selectable in an `IELSRn` slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Event {
    /// ICU IRQ0 — external pin interrupt 0.
    GpioIrq0 = 0x01,
    /// ICU IRQ1 — external pin interrupt 1.
    GpioIrq1 = 0x02,
    /// ICU IRQ2 — external pin interrupt 2.
    GpioIrq2 = 0x03,
    /// ICU IRQ3 — external pin interrupt 3.
    GpioIrq3 = 0x04,
    /// ICU IRQ4 — external pin interrupt 4.
    GpioIrq4 = 0x05,
    /// ICU IRQ5 — external pin interrupt 5.
    GpioIrq5 = 0x06,
    /// ICU IRQ6 — external pin interrupt 6.
    GpioIrq6 = 0x07,
    /// ICU IRQ7 — external pin interrupt 7.
    GpioIrq7 = 0x08,
    /// ICU IRQ8 — external pin interrupt 8.
    GpioIrq8 = 0x09,
    /// ICU IRQ9 — external pin interrupt 9.
    GpioIrq9 = 0x0A,
    /// ICU IRQ10 — external pin interrupt 10.
    GpioIrq10 = 0x0B,
    /// ICU IRQ11 — external pin interrupt 11.
    GpioIrq11 = 0x0C,
    /// ICU IRQ12 — external pin interrupt 12.
    GpioIrq12 = 0x0D,
    /// ICU IRQ14 — external pin interrupt 14 (13 reserved).
    GpioIrq14 = 0x0F,
    /// ICU IRQ15 — external pin interrupt 15.
    GpioIrq15 = 0x10,
    /// DMAC0 transfer end.
    Dmac0Int = 0x11,
    /// DMAC1 transfer end.
    Dmac1Int = 0x12,
    /// DMAC2 transfer end.
    Dmac2Int = 0x13,
    /// DMAC3 transfer end.
    Dmac3Int = 0x14,
    /// DTC transfer complete.
    DtcComplete = 0x15,
    /// Canceling from Snooze mode.
    IcuSnoozeCancel = 0x17,
    /// Flash ready interrupt (FCU FRDYI).
    FcuFrdyi = 0x18,
    /// Voltage monitor 1 interrupt (LVD1).
    LvdLvd1 = 0x19,
    /// Voltage monitor 2 interrupt (LVD2).
    LvdLvd2 = 0x1A,
    /// VBATT low voltage detect.
    LvdVbatt = 0x1B,
    /// Main clock oscillation stop (MOSC stop).
    CgcMoscStop = 0x1C,
    /// Standby Snooze request (SYSTEM SNZREQ).
    SystemSnzreq = 0x1D,
    /// AGT0 interrupt.
    Agt0Int = 0x1E,
    /// AGT0 compare match A.
    Agt0CompareA = 0x1F,
    /// AGT0 compare match B.
    Agt0CompareB = 0x20,
    /// AGT1 interrupt.
    Agt1Int = 0x21,
    /// AGT1 compare match A.
    Agt1CompareA = 0x22,
    /// AGT1 compare match B.
    Agt1CompareB = 0x23,
    /// Independent watchdog underflow (NMI).
    IwdtUnderflow = 0x24,
    /// Watchdog underflow (NMI).
    WdtUnderflow = 0x25,
    /// RTC alarm interrupt.
    RtcAlarm = 0x26,
    /// RTC periodic interrupt.
    RtcPeriod = 0x27,
    /// RTC carry interrupt.
    RtcCarry = 0x28,
    /// ADC140 scan end interrupt.
    Adc0ScanEnd = 0x29,
    /// ADC140 scan end interrupt for group B.
    Adc0ScanEndB = 0x2A,
    /// ADC140 window A compare match.
    Adc0WindowA = 0x2B,
    /// ADC140 window B compare match.
    Adc0WindowB = 0x2C,
    /// ADC140 window compare match (down crossing).
    Adc0WindowCmpm = 0x2D,
    /// ADC140 window compare match (up crossing).
    Adc0WindowCmpum = 0x2E,
    /// Low-power comparator channel 0 interrupt.
    Acmplp0Int = 0x2F,
    /// Low-power comparator channel 1 interrupt.
    Acmplp1Int = 0x30,
    /// USBFS D0FIFO DMA/DTC transfer request.
    UsbfsD0fifo = 0x31,
    /// USBFS D1FIFO DMA/DTC transfer request.
    UsbfsD1fifo = 0x32,
    /// USBFS interrupt.
    UsbfsInt = 0x33,
    /// USBFS resume interrupt.
    UsbfsResume = 0x34,
    /// IIC0 receive data full.
    Iic0Rxi = 0x35,
    /// IIC0 transmit data empty.
    Iic0Txi = 0x36,
    /// IIC0 transmit end.
    Iic0Tei = 0x37,
    /// IIC0 transfer error.
    Iic0Eri = 0x38,
    /// IIC0 wakeup interrupt.
    Iic0Wui = 0x39,
    /// IIC1 receive data full.
    Iic1Rxi = 0x3A,
    /// IIC1 transmit data empty.
    Iic1Txi = 0x3B,
    /// IIC1 transmit end.
    Iic1Tei = 0x3C,
    /// IIC1 transfer error.
    Iic1Eri = 0x3D,
    /// SSIE0 transmit data empty.
    Ssi0Txi = 0x3E,
    /// SSIE0 receive data full.
    Ssi0Rxi = 0x3F,
    /// SSIE0 error interrupt.
    Ssi0Int = 0x41,
    /// CTSU write request.
    CtsuWrite = 0x42,
    /// CTSU read request.
    CtsuRead = 0x43,
    /// CTSU measurement end.
    CtsuEnd = 0x44,
    /// Key matrix interrupt (KINTKR).
    KeyInt = 0x45,
    /// Data operation circuit interrupt (DOPCI).
    DocInt = 0x46,
    /// CAC frequency error interrupt.
    CacFrequencyError = 0x47,
    /// CAC measurement end interrupt.
    CacMeasurementEnd = 0x48,
    /// CAC overflow interrupt.
    CacOverflow = 0x49,
    /// CAN0 error interrupt.
    Can0Error = 0x4A,
    /// CAN0 receive FIFO interrupt.
    Can0FifoRx = 0x4B,
    /// CAN0 transmit FIFO interrupt.
    Can0FifoTx = 0x4C,
    /// CAN0 mailbox reception complete.
    Can0MailboxRx = 0x4D,
    /// CAN0 mailbox transmission complete.
    Can0MailboxTx = 0x4E,
    /// IOPORT event 1 (port 1).
    IoportEvent1 = 0x4F,
    /// IOPORT event 2 (port 2).
    IoportEvent2 = 0x50,
    /// IOPORT event 3 (port 3).
    IoportEvent3 = 0x51,
    /// IOPORT event 4 (port 4).
    IoportEvent4 = 0x52,
    /// ELC software event 0.
    ElcSoftwareEvent0 = 0x53,
    /// ELC software event 1.
    ElcSoftwareEvent1 = 0x54,
    /// POEG group 0 interrupt (port output disable).
    Poeg0Event = 0x55,
    /// POEG group 1 interrupt (port output disable).
    Poeg1Event = 0x56,
    /// GPT0 capture/compare match A.
    Gpt0Ccmpa = 0x57,
    /// GPT0 capture/compare match B.
    Gpt0Ccmpb = 0x58,
    /// GPT0 compare match C.
    Gpt0Cmpc = 0x59,
    /// GPT0 compare match D.
    Gpt0Cmpd = 0x5A,
    /// GPT0 compare match E.
    Gpt0Cmpe = 0x5B,
    /// GPT0 compare match F.
    Gpt0Cmpf = 0x5C,
    /// GPT0 counter overflow.
    Gpt0Overflow = 0x5D,
    /// GPT0 counter underflow.
    Gpt0Underflow = 0x5E,
    /// GPT1 capture/compare match A.
    Gpt1Ccmpa = 0x5F,
    /// GPT1 capture/compare match B.
    Gpt1Ccmpb = 0x60,
    /// GPT1 compare match C.
    Gpt1Cmpc = 0x61,
    /// GPT1 compare match D.
    Gpt1Cmpd = 0x62,
    /// GPT1 compare match E.
    Gpt1Cmpe = 0x63,
    /// GPT1 compare match F.
    Gpt1Cmpf = 0x64,
    /// GPT1 counter overflow.
    Gpt1Overflow = 0x65,
    /// GPT1 counter underflow.
    Gpt1Underflow = 0x66,
    /// GPT2 capture/compare match A.
    Gpt2Ccmpa = 0x67,
    /// GPT2 capture/compare match B.
    Gpt2Ccmpb = 0x68,
    /// GPT2 compare match C.
    Gpt2Cmpc = 0x69,
    /// GPT2 compare match D.
    Gpt2Cmpd = 0x6A,
    /// GPT2 compare match E.
    Gpt2Cmpe = 0x6B,
    /// GPT2 compare match F.
    Gpt2Cmpf = 0x6C,
    /// GPT2 counter overflow.
    Gpt2Overflow = 0x6D,
    /// GPT2 counter underflow.
    Gpt2Underflow = 0x6E,
    /// GPT3 capture/compare match A.
    Gpt3Ccmpa = 0x6F,
    /// GPT3 capture/compare match B.
    Gpt3Ccmpb = 0x70,
    /// GPT3 compare match C.
    Gpt3Cmpc = 0x71,
    /// GPT3 compare match D.
    Gpt3Cmpd = 0x72,
    /// GPT3 compare match E.
    Gpt3Cmpe = 0x73,
    /// GPT3 compare match F.
    Gpt3Cmpf = 0x74,
    /// GPT3 counter overflow.
    Gpt3Overflow = 0x75,
    /// GPT3 counter underflow.
    Gpt3Underflow = 0x76,
    /// GPT4 capture/compare match A.
    Gpt4Ccmpa = 0x77,
    /// GPT4 capture/compare match B.
    Gpt4Ccmpb = 0x78,
    /// GPT4 compare match C.
    Gpt4Cmpc = 0x79,
    /// GPT4 compare match D.
    Gpt4Cmpd = 0x7A,
    /// GPT4 compare match E.
    Gpt4Cmpe = 0x7B,
    /// GPT4 compare match F.
    Gpt4Cmpf = 0x7C,
    /// GPT4 counter overflow.
    Gpt4Overflow = 0x7D,
    /// GPT4 counter underflow.
    Gpt4Underflow = 0x7E,
    /// GPT5 capture/compare match A.
    Gpt5Ccmpa = 0x7F,
    /// GPT5 capture/compare match B.
    Gpt5Ccmpb = 0x80,
    /// GPT5 compare match C.
    Gpt5Cmpc = 0x81,
    /// GPT5 compare match D.
    Gpt5Cmpd = 0x82,
    /// GPT5 compare match E.
    Gpt5Cmpe = 0x83,
    /// GPT5 compare match F.
    Gpt5Cmpf = 0x84,
    /// GPT5 counter overflow.
    Gpt5Overflow = 0x85,
    /// GPT5 counter underflow.
    Gpt5Underflow = 0x86,
    /// GPT6 capture/compare match A.
    Gpt6Ccmpa = 0x87,
    /// GPT6 capture/compare match B.
    Gpt6Ccmpb = 0x88,
    /// GPT6 compare match C.
    Gpt6Cmpc = 0x89,
    /// GPT6 compare match D.
    Gpt6Cmpd = 0x8A,
    /// GPT6 compare match E.
    Gpt6Cmpe = 0x8B,
    /// GPT6 compare match F.
    Gpt6Cmpf = 0x8C,
    /// GPT6 counter overflow.
    Gpt6Overflow = 0x8D,
    /// GPT6 counter underflow.
    Gpt6Underflow = 0x8E,
    /// GPT7 capture/compare match A.
    Gpt7Ccmpa = 0x8F,
    /// GPT7 capture/compare match B.
    Gpt7Ccmpb = 0x90,
    /// GPT7 compare match C.
    Gpt7Cmpc = 0x91,
    /// GPT7 compare match D.
    Gpt7Cmpd = 0x92,
    /// GPT7 compare match E.
    Gpt7Cmpe = 0x93,
    /// GPT7 compare match F.
    Gpt7Cmpf = 0x94,
    /// GPT7 counter overflow.
    Gpt7Overflow = 0x95,
    /// GPT7 counter underflow.
    Gpt7Underflow = 0x96,
    /// OPS UVW edge event.
    OpsUvwEdge = 0x97,
    /// SCI0 receive data full.
    Sci0Rxi = 0x98,
    /// SCI0 transmit data empty.
    Sci0Txi = 0x99,
    /// SCI0 transmit end.
    Sci0Tei = 0x9A,
    /// SCI0 receive error.
    Sci0Eri = 0x9B,
    /// SCI0 address match event.
    Sci0Am = 0x9C,
    /// SCI0 receive data full or receive error (combined source).
    Sci0RxiOrEri = 0x9D,
    /// SCI1 receive data full.
    Sci1Rxi = 0x9E,
    /// SCI1 transmit data empty.
    Sci1Txi = 0x9F,
    /// SCI1 transmit end.
    Sci1Tei = 0xA0,
    /// SCI1 receive error.
    Sci1Eri = 0xA1,
    /// SCI1 address match event.
    Sci1Am = 0xA2,
    /// SCI2 receive data full.
    Sci2Rxi = 0xA3,
    /// SCI2 transmit data empty.
    Sci2Txi = 0xA4,
    /// SCI2 transmit end.
    Sci2Tei = 0xA5,
    /// SCI2 receive error.
    Sci2Eri = 0xA6,
    /// SCI2 address match event.
    Sci2Am = 0xA7,
    /// SCI9 receive data full.
    Sci9Rxi = 0xA8,
    /// SCI9 transmit data empty.
    Sci9Txi = 0xA9,
    /// SCI9 transmit end.
    Sci9Tei = 0xAA,
    /// SCI9 receive error.
    Sci9Eri = 0xAB,
    /// SCI9 address match event.
    Sci9Am = 0xAC,
    /// SPI0 receive buffer full.
    Spi0Rxi = 0xAD,
    /// SPI0 transmit buffer empty.
    Spi0Txi = 0xAE,
    /// SPI0 idle event.
    Spi0Idle = 0xAF,
    /// SPI0 error interrupt.
    Spi0Eri = 0xB0,
    /// SPI0 transmission complete event.
    Spi0Tei = 0xB1,
    /// SPI1 receive buffer full.
    Spi1Rxi = 0xB2,
    /// SPI1 transmit buffer empty.
    Spi1Txi = 0xB3,
    /// SPI1 idle event.
    Spi1Idle = 0xB4,
    /// SPI1 error interrupt.
    Spi1Eri = 0xB5,
    /// SPI1 transmission complete event.
    Spi1Tei = 0xB6,
    /// SCE tamper detection interrupt.
    SceTadi = 0xDB,
}

/// Every `Event`, for the table-integrity tests (`src/tests/intc.rs`).
/// Kept next to the enum so a new variant gets added here in the same
/// edit.
#[cfg(test)]
pub(crate) const ALL_EVENTS: &[Event] = &[
    Event::GpioIrq0,
    Event::GpioIrq1,
    Event::GpioIrq2,
    Event::GpioIrq3,
    Event::GpioIrq4,
    Event::GpioIrq5,
    Event::GpioIrq6,
    Event::GpioIrq7,
    Event::GpioIrq8,
    Event::GpioIrq9,
    Event::GpioIrq10,
    Event::GpioIrq11,
    Event::GpioIrq12,
    Event::GpioIrq14,
    Event::GpioIrq15,
    Event::Dmac0Int,
    Event::Dmac1Int,
    Event::Dmac2Int,
    Event::Dmac3Int,
    Event::DtcComplete,
    Event::IcuSnoozeCancel,
    Event::FcuFrdyi,
    Event::LvdLvd1,
    Event::LvdLvd2,
    Event::LvdVbatt,
    Event::CgcMoscStop,
    Event::SystemSnzreq,
    Event::Agt0Int,
    Event::Agt0CompareA,
    Event::Agt0CompareB,
    Event::Agt1Int,
    Event::Agt1CompareA,
    Event::Agt1CompareB,
    Event::IwdtUnderflow,
    Event::WdtUnderflow,
    Event::RtcAlarm,
    Event::RtcPeriod,
    Event::RtcCarry,
    Event::Adc0ScanEnd,
    Event::Adc0ScanEndB,
    Event::Adc0WindowA,
    Event::Adc0WindowB,
    Event::Adc0WindowCmpm,
    Event::Adc0WindowCmpum,
    Event::Acmplp0Int,
    Event::Acmplp1Int,
    Event::UsbfsD0fifo,
    Event::UsbfsD1fifo,
    Event::UsbfsInt,
    Event::UsbfsResume,
    Event::Iic0Rxi,
    Event::Iic0Txi,
    Event::Iic0Tei,
    Event::Iic0Eri,
    Event::Iic0Wui,
    Event::Iic1Rxi,
    Event::Iic1Txi,
    Event::Iic1Tei,
    Event::Iic1Eri,
    Event::Ssi0Txi,
    Event::Ssi0Rxi,
    Event::Ssi0Int,
    Event::CtsuWrite,
    Event::CtsuRead,
    Event::CtsuEnd,
    Event::KeyInt,
    Event::DocInt,
    Event::CacFrequencyError,
    Event::CacMeasurementEnd,
    Event::CacOverflow,
    Event::Can0Error,
    Event::Can0FifoRx,
    Event::Can0FifoTx,
    Event::Can0MailboxRx,
    Event::Can0MailboxTx,
    Event::IoportEvent1,
    Event::IoportEvent2,
    Event::IoportEvent3,
    Event::IoportEvent4,
    Event::ElcSoftwareEvent0,
    Event::ElcSoftwareEvent1,
    Event::Poeg0Event,
    Event::Poeg1Event,
    Event::Gpt0Ccmpa,
    Event::Gpt0Ccmpb,
    Event::Gpt0Cmpc,
    Event::Gpt0Cmpd,
    Event::Gpt0Cmpe,
    Event::Gpt0Cmpf,
    Event::Gpt0Overflow,
    Event::Gpt0Underflow,
    Event::Gpt1Ccmpa,
    Event::Gpt1Ccmpb,
    Event::Gpt1Cmpc,
    Event::Gpt1Cmpd,
    Event::Gpt1Cmpe,
    Event::Gpt1Cmpf,
    Event::Gpt1Overflow,
    Event::Gpt1Underflow,
    Event::Gpt2Ccmpa,
    Event::Gpt2Ccmpb,
    Event::Gpt2Cmpc,
    Event::Gpt2Cmpd,
    Event::Gpt2Cmpe,
    Event::Gpt2Cmpf,
    Event::Gpt2Overflow,
    Event::Gpt2Underflow,
    Event::Gpt3Ccmpa,
    Event::Gpt3Ccmpb,
    Event::Gpt3Cmpc,
    Event::Gpt3Cmpd,
    Event::Gpt3Cmpe,
    Event::Gpt3Cmpf,
    Event::Gpt3Overflow,
    Event::Gpt3Underflow,
    Event::Gpt4Ccmpa,
    Event::Gpt4Ccmpb,
    Event::Gpt4Cmpc,
    Event::Gpt4Cmpd,
    Event::Gpt4Cmpe,
    Event::Gpt4Cmpf,
    Event::Gpt4Overflow,
    Event::Gpt4Underflow,
    Event::Gpt5Ccmpa,
    Event::Gpt5Ccmpb,
    Event::Gpt5Cmpc,
    Event::Gpt5Cmpd,
    Event::Gpt5Cmpe,
    Event::Gpt5Cmpf,
    Event::Gpt5Overflow,
    Event::Gpt5Underflow,
    Event::Gpt6Ccmpa,
    Event::Gpt6Ccmpb,
    Event::Gpt6Cmpc,
    Event::Gpt6Cmpd,
    Event::Gpt6Cmpe,
    Event::Gpt6Cmpf,
    Event::Gpt6Overflow,
    Event::Gpt6Underflow,
    Event::Gpt7Ccmpa,
    Event::Gpt7Ccmpb,
    Event::Gpt7Cmpc,
    Event::Gpt7Cmpd,
    Event::Gpt7Cmpe,
    Event::Gpt7Cmpf,
    Event::Gpt7Overflow,
    Event::Gpt7Underflow,
    Event::OpsUvwEdge,
    Event::Sci0Rxi,
    Event::Sci0Txi,
    Event::Sci0Tei,
    Event::Sci0Eri,
    Event::Sci0Am,
    Event::Sci0RxiOrEri,
    Event::Sci1Rxi,
    Event::Sci1Txi,
    Event::Sci1Tei,
    Event::Sci1Eri,
    Event::Sci1Am,
    Event::Sci2Rxi,
    Event::Sci2Txi,
    Event::Sci2Tei,
    Event::Sci2Eri,
    Event::Sci2Am,
    Event::Sci9Rxi,
    Event::Sci9Txi,
    Event::Sci9Tei,
    Event::Sci9Eri,
    Event::Sci9Am,
    Event::Spi0Rxi,
    Event::Spi0Txi,
    Event::Spi0Idle,
    Event::Spi0Eri,
    Event::Spi0Tei,
    Event::Spi1Rxi,
    Event::Spi1Txi,
    Event::Spi1Idle,
    Event::Spi1Eri,
    Event::Spi1Tei,
    Event::SceTadi,
];
