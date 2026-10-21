//! The USBFS bus adapter: `usb_device::UsbBus` on the RA4M1 USBFS.
//!
//! Register facts are manual §27 (R01UH0887EJ0100) unless noted; the
//! bring-up sequence was hardware-proven by the slice 9 probe
//! (2026-10-20). Bit constants carry their
//! section numbers. The adapter translates hardware CTSQ/DVSQ and packet
//! completion flags into usb-device events; usb-device owns request parsing.

use core::cell::Cell;
use usb_device::{
    UsbDirection, UsbError,
    bus::{PollResult, UsbBus},
    endpoint::{EndpointAddress, EndpointType},
};

// --- FIFO data ports (§27.2.4: CFIFO 0x14, D0FIFO 0x18, D1FIFO 0x1C)
// and their control registers (§27.2.6: CFIFOCTR 0x22, D0FIFOCTR 0x2A).
// Control IN uses the PAC's 16-bit CFIFO accessor and an 8-bit final
// byte for odd lengths; OUT and bulk use 8-bit data port access (§27.2.4).
const CFIFO: *mut u8 = 0x4009_0014 as *mut u8;
const CFIFOCTR: *mut u16 = 0x4009_0022 as *mut u16;
const D0FIFO: *mut u8 = 0x4009_0018 as *mut u8;
const D0FIFOCTR: *mut u16 = 0x4009_002A as *mut u16;

// --- FIFOSEL (§27.2.5) / FIFOCTR (§27.2.6) bits (same layout for the
// CFIFO and D0FIFO ports).
/// ISEL b5: 1 = write direction (only meaningful for the DCP).
const FIFOSEL_ISEL: u16 = 1 << 5;
/// BVAL b15 ends a write; BCLR b14 clears the buffer (both single-shot:
/// write the bit alone — everything else in the register is RO or
/// write-0-is-no-op). FRDY b13, DTLN b8:0.
const CTR_BVAL: u16 = 1 << 15;
const CTR_BCLR: u16 = 1 << 14;
const CTR_FRDY: u16 = 1 << 13;
const CTR_DTLN: u16 = 0x01FF;

// --- DCPCTR.PID values (§27.2.25: 00=NAK, 01=BUF, 1x=STALL).
const PID_NAK: u16 = 0b00;
const PID_BUF: u16 = 0b01;
const PID_STALL: u16 = 0b10;

// --- INTSTS0 (§27.2.13) bits used here.
/// VALID b3: setup packet received.
const INTSTS0_VALID: u16 = 1 << 3;
/// VBSTS b7: VBUS present.
const INTSTS0_VBSTS: u16 = 1 << 7;
/// CTSQ[2:0] b2:0 — the OUT-direction stages of a control transfer
/// need the DCP armed to receive (see [`Usbfs::poll`]).
const CTSQ_READ_STATUS: u16 = 2;
const CTSQ_WRITE_DATA: u16 = 3;
const DCP_CCPL: u16 = 1 << 2; // §27.2.25: hardware completes status stage.
const INTSTS0_CTRT: u16 = 1 << 11;
const INTSTS0_DVST: u16 = 1 << 12;
/// Clearing a flag means writing 0 to that bit and 1 to all others
/// (§27.2.13 note 4). This value clears VALID and nothing else.
const INTSTS0_CLEAR_VALID: u16 = !INTSTS0_VALID;

/// `PRCR` unlock: PRC0 + PRC1 (same window `clock.rs` uses). PRC1
/// protects BOTH `USBCKCR` (§8.2.24) and the module-stop registers
/// (§10.2.3) — one window covers both writes. Hardware-proven: a locked
/// MSTPCRB write is silently dropped (probe §10.2).
const PRCR_UNLOCK: u16 = 0xA503;
const PRCR_LOCK: u16 = 0xA500;

/// `SYSCFG` bits (§27.2.1): SCKE b10, USBE b0, DPRPU b4.
const SYSCFG_SCKE: u16 = 1 << 10;
const SYSCFG_USBE: u16 = 1 << 0;
const SYSCFG_DPRPU: u16 = 1 << 4;

/// `USBMC` (§27.2.34): VDCEN b7; b1 is reserved-read-as-1 on write.
const USBMC_VDCEN: u16 = 1 << 7;
const USBMC_RESV_B1: u16 = 1 << 1;

/// Status registers (BRDYSTS §27.2.15, BEMPSTS §27.2.17) carry 10
/// flags: pipe 0 (DCP) at bit 0 .. pipe 9 at bit 9.
const PIPE_STATUS_MASK: u16 = 0x03FF;

/// EP0 status IN is authorized by CCPL, even if the next IN token has
/// not advanced control-write CTSQ from data (3) to status (4) yet.
pub(crate) const fn control_status_in(ctsq: u16, empty: bool, auto_address: bool) -> bool {
    empty && (ctsq == CTSQ_WRITE_DATA || ctsq == 4 || ctsq == 5 || auto_address)
}

#[cfg(feature = "usb-trace")]
fn trace(tag: u8, data: &[u8]) {
    let regs = unsafe { &*ra4m1::USBFS::PTR };
    let mut row = [0u8; 74];
    row[0] = tag;
    row[1] = data.len().min(64) as u8;
    for (i, value) in [
        regs.intsts0.read().bits(),
        regs.cfifosel.read().bits(),
        regs.cfifoctr.read().bits(),
        regs.dcpctr.read().bits(),
    ]
    .into_iter()
    .enumerate()
    {
        row[2 + i * 2..4 + i * 2].copy_from_slice(&value.to_le_bytes());
    }
    let len = usize::from(row[1]);
    row[10..10 + len].copy_from_slice(&data[..len]);
    critical_section::with(|cs| {
        let _ = cs;
        let base = trace_base();
        let count = unsafe { base.add(4).read_volatile() } as usize;
        let next = unsafe { base.add(5).read_volatile() } as usize;
        if count <= 48 && next < 48 {
            for (offset, byte) in row.into_iter().enumerate() {
                unsafe {
                    base.add(8 + next * 74 + offset).write_volatile(byte);
                }
            }
            unsafe {
                base.add(4).write_volatile((count + 1).min(48) as u8);
                base.add(5).write_volatile(((next + 1) % 48) as u8);
            }
        }
    });
}

#[cfg(feature = "usb-trace")]
#[unsafe(link_section = ".uninit.usb_trace")]
static mut TRACE_RAM: core::mem::MaybeUninit<[u8; 0x4000 + 8 + 48 * 74]> =
    core::mem::MaybeUninit::uninit();

#[cfg(feature = "usb-trace")]
fn trace_base() -> *mut u8 {
    // Reserve the low 16 KiB ahead of the trace: the observed bootloader
    // stack occupies 0x20003590..0x20003993. This debug-only storage is
    // retained by cortex-m-rt across external resets, never assumed valid
    // without the header check below.
    unsafe { core::ptr::addr_of_mut!(TRACE_RAM).cast::<u8>().add(0x4000) }
}

/// Starts a fresh RAM-only diagnostic trace (feature `usb-trace`).
#[cfg(feature = "usb-trace")]
pub fn start_diagnostic_trace() {
    critical_section::with(|_| unsafe {
        let base = trace_base();
        for (offset, byte) in b"USB9\0\0\0\0".iter().enumerate() {
            base.add(offset).write_volatile(*byte);
        }
    });
}

/// Reads an optional RAM-only USB diagnostic record; never uses a UART.
/// Layout: tag, data length, four little-endian registers (INTSTS0,
/// CFIFOSEL, CFIFOCTR, DCPCTR), and up to 64 data bytes.
#[cfg(feature = "usb-trace")]
pub fn diagnostic_trace(index: usize) -> Option<[u8; 74]> {
    critical_section::with(|_| unsafe {
        let base = trace_base();
        for (offset, byte) in b"USB9".iter().enumerate() {
            if base.add(offset).read_volatile() != *byte {
                return None;
            }
        }
        let count = usize::from(base.add(4).read_volatile());
        let next = usize::from(base.add(5).read_volatile());
        if count > 48 || index >= count || next >= 48 {
            return None;
        }
        let slot = if count == 48 {
            (next + index) % 48
        } else {
            index
        };
        let mut row = [0; 74];
        for (offset, byte) in row.iter_mut().enumerate() {
            *byte = base.add(8 + slot * 74 + offset).read_volatile();
        }
        (row[1] <= 64
            && matches!(
                row[0],
                b'S' | b'W' | b'F' | b'E' | b'T' | b'P' | b'R' | b'O'
            ))
        .then_some(row)
    })
}

/// The USBFS as a `usb_device` bus.
///
/// Construct with [`Usbfs::new`] (which runs the hardware bring-up) and
/// hand it to `usb_device::UsbBusAllocator::new`. The type owns the PAC
/// peripheral (exactly-once claim). Mux ownership is separate: the
/// application explicitly calls [`super::UsbPort::take`] on WiFi.
///
/// Endpoint model: EP0 is the hardware DCP (64-byte single buffer,
/// CFIFO port only, §27.3.9). Every other endpoint is one hardware
/// pipe, and the endpoint number `usb-device` sees equals the pipe
/// number: pipes 1–5 are bulk, 6–9 are interrupt (§27.2.27 TYPE table).
/// That keeps the endpoint↔pipe mapping fixed and allocation-order-free
/// while making every (EPNUM, DIR) pair unique as the hardware demands.
pub struct Usbfs {
    _usbfs: ra4m1::USBFS,
    /// ep address → pipe (0 = DCP, −1 = unallocated), indexed
    /// `[direction][index]` (0 = OUT, 1 = IN).
    ep_pipe: Cell<[[i8; 16]; 2]>,
    /// DVSQ at the previous poll — bus reset and suspend/resume are
    /// *transitions* of the device state (§27.2.13 DVSQ table).
    last_dvsq: Cell<u8>,
    /// Whether DPRPU has been driven (we connect once VBUS is sensed).
    connected: Cell<bool>,
    /// IN status ZLP is handled by CCPL, not by the FIFO (§27.3.9.2).
    status_in_pending: Cell<bool>,
    /// OUT status completion must be reported to usb-device as a ZLP.
    status_out_pending: Cell<bool>,
    status_out_ready: Cell<bool>,
    /// SET_ADDRESS is completed automatically by the USBFS.
    auto_address: Cell<bool>,
    /// Successfully queued IN packets awaiting a completion report.
    in_pending: Cell<u16>,
    #[cfg(feature = "usb-trace")]
    last_trace_state: Cell<u16>,
}

impl Usbfs {
    /// Claims the USBFS and runs the hardware bring-up.
    ///
    /// Sequence (manual-cited; hardware-proven in the probe):
    /// 1. PRC1 window: `USBCKCR.USBCLKSEL = 1` (UCLK = HOCO 48 MHz,
    ///    §8.2.24 — legal only in device mode, only while SCKE = 0) and
    ///    `MSTPCRB.MSTPB11 = 0` (module-stop release, §10.2.3 — PRC1-
    ///    protected, and a 32-bit register: narrow writes are dropped,
    ///    probe §10.2).
    /// 2. `SYSCFG.SCKE = 1` with read-back confirm (§27.2.1 note 2).
    /// 3. `USBMC.VDCEN = 1` (USB regulator; the board runs the internal
    ///    LDO — the core sets VDCEN on both variants, its variant header
    ///    defines `VUSB_LDO_ENABLE 1`), preserving reserved b1.
    /// 4. `SYSCFG.USBE = 1` — the module starts (§27.2.1).
    ///
    /// The D+ pull-up (the connect signal, §27.3.1.3) is **not** driven
    /// here — usb-device 0.3.2's builder freezes the allocator and calls
    /// [`UsbBus::enable`], which connects if VBUS is present. Later polls
    /// handle VBUS arriving after enable (`VBSTS`, §27.2.13).
    ///
    /// Taking the connector ([`super::UsbPort::take`]) is the
    /// application's separate decision and may happen before or after
    /// this call; the device only becomes visible to the host once both
    /// have happened.
    ///
    /// # Panics
    /// If the SCKE read-back fails — there is no recovery path once the
    /// module clock refused to start.
    pub fn new(usbfs: ra4m1::USBFS) -> Self {
        let system = unsafe { &*ra4m1::SYSTEM::PTR };
        let mstp = unsafe { &*ra4m1::MSTP::PTR };
        let regs = Self::regs();

        system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK) });
        // UCLK = HOCO.
        system
            .usbckcr
            .modify(|_, w| unsafe { w.bits(system.usbckcr.read().bits() | 1) });
        // Release module stop (32-bit write through the PAC).
        mstp.mstpcrb
            .modify(|_, w| unsafe { w.bits(mstp.mstpcrb.read().bits() & !(1 << 11)) });
        system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });

        // SCKE with read-back confirm (§27.2.1 note 2). While SCKE = 0
        // only SYSCFG may be touched — every other access follows this.
        regs.syscfg
            .modify(|_, w| unsafe { w.bits(regs.syscfg.read().bits() | SYSCFG_SCKE) });
        let mut scke = false;
        for _ in 0..10_000 {
            if regs.syscfg.read().bits() & SYSCFG_SCKE != 0 {
                scke = true;
                break;
            }
        }
        assert!(scke, "USBFS SCKE read-back failed after module start");

        // USB regulator on (keep reserved b1 at its read-as-1 value).
        regs.usbmc.modify(|_, w| unsafe {
            w.bits(regs.usbmc.read().bits() | USBMC_VDCEN | USBMC_RESV_B1)
        });
        // Module operation enable.
        regs.syscfg
            .modify(|_, w| unsafe { w.bits(regs.syscfg.read().bits() | SYSCFG_USBE) });

        Self {
            _usbfs: usbfs,
            ep_pipe: Cell::new([[-1i8; 16]; 2]),
            last_dvsq: Cell::new(0xFF), // no spurious Reset before the host acts
            connected: Cell::new(false),
            status_in_pending: Cell::new(false),
            status_out_pending: Cell::new(false),
            status_out_ready: Cell::new(false),
            auto_address: Cell::new(false),
            in_pending: Cell::new(0),
            #[cfg(feature = "usb-trace")]
            last_trace_state: Cell::new(0xFFFF),
        }
    }

    #[inline]
    fn regs() -> &'static ra4m1::usbfs::RegisterBlock {
        unsafe { &*ra4m1::USBFS::PTR }
    }

    pub(super) fn prepare_wakeup(&self) {
        critical_section::with(|_| {
            let (out, inp) = self.direction_masks();
            Self::regs().brdyenb.write(|w| unsafe { w.bits(out) });
            Self::regs().bempenb.write(|w| unsafe { w.bits(inp) });
        });
    }

    pub(super) fn control_transfer_idle(&self) -> bool {
        critical_section::with(|_| Self::regs().intsts0.read().bits() & (INTSTS0_VALID | 7) == 0)
    }

    pub(super) fn reboot_to_bootloader(&self, port: super::UsbPort) -> ! {
        critical_section::with(|_| {
            let regs = Self::regs();
            regs.intenb0.write(|w| unsafe { w.bits(0) });
            regs.syscfg
                .modify(|r, w| unsafe { w.bits(r.bits() & !SYSCFG_DPRPU) });
            // Arduino bootloader handoff ABI, NOT a USBFS hardware fact:
            // installed ArduinoCore-renesas 1.6.0 cores/arduino/boot.h
            // specifies 0x07738135 at VBTBKR[0..4]. Manual §11.2.6
            // requires 8-bit VBTBKR access. §12.1 explicitly lists it
            // under PRC1 protection (despite research §10.1's old claim).
            // Byte order: the installed bootloader (dfu_wifi.hex, offset
            // 0x21EA) compares one 32-bit load of VBTBKR0..3, and on this
            // silicon that load returns VBTBKR0 as the MOST significant
            // byte (Slice 9 diagnostic rig, README Slice 9: bytes
            // 35 81 73 07 read as 0x35817307).
            // Big-endian byte writes therefore present the magic.
            let system = unsafe { &*ra4m1::SYSTEM::PTR };
            system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK) });
            system.vbtcr1.modify(|r, w| unsafe { w.bits(r.bits() | 1) });
            for _ in 0..100_000 {
                if system.vbtsr.read().bits() & (1 << 4) != 0 {
                    break;
                }
            }
            let valid = system.vbtsr.read().bits() & (1 << 4) != 0;
            if !valid {
                system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
                panic!("VBATT backup domain not ready for bootloader marker");
            }
            let mut stored = true;
            for (index, byte) in 0x0773_8135u32.to_be_bytes().into_iter().enumerate() {
                unsafe {
                    let register = (0x4001_E500 as *mut u8).add(index);
                    register.write_volatile(byte);
                    stored &= register.read_volatile() == byte;
                }
            }
            system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
            assert!(stored, "bootloader marker read-back failed");
            port.release();
            cortex_m::peripheral::SCB::sys_reset();
        })
    }

    /// The PIPEnCTR register for pipe 1–9 (§27.2.30: 0x70 + 2·(n−1);
    /// the PAC spells pipes 1–5 as an array and 6–9 as named fields).
    #[inline]
    fn pipe_ctr(&self, pipe: u8) -> &ra4m1::usbfs::PIPECTR {
        let regs = Self::regs();
        match pipe {
            1..=5 => &regs.pipectr[(pipe - 1) as usize],
            6 => &regs.pipe6ctr,
            7 => &regs.pipe7ctr,
            8 => &regs.pipe8ctr,
            9 => &regs.pipe9ctr,
            _ => unreachable!("pipe number is 1..=9 by construction"),
        }
    }

    /// Resolves an endpoint address to its hardware pipe (0 = DCP).
    fn pipe_of(&self, ep_addr: EndpointAddress) -> Result<u8, UsbError> {
        let map = self.ep_pipe.get();
        let pipe = map[usize::from(ep_addr.is_in())][ep_addr.index()];
        if pipe < 0 {
            Err(UsbError::InvalidEndpoint)
        } else {
            Ok(pipe as u8)
        }
    }

    /// Selects the CFIFO port for DCP access and waits for FRDY
    /// (§27.3.7 (1): confirm CURPIPE read-back, then FRDY = 1).
    fn select_cfifo(&self, isel_write: bool) -> Result<(), UsbError> {
        let regs = Self::regs();
        let sel = if isel_write {
            FIFOSEL_ISEL | (1 << 10)
        } else {
            0
        };
        regs.cfifosel.write(|w| unsafe { w.bits(sel) });
        for _ in 0..1_000 {
            if regs.cfifosel.read().bits() & 0x043F == sel
                && regs.cfifoctr.read().bits() & CTR_FRDY != 0
            {
                return Ok(());
            }
        }
        Err(UsbError::WouldBlock)
    }

    /// Selects a pipe on the D0FIFO port (§27.3.7: pipes 1–9 may use
    /// D0FIFO) and waits for FRDY.
    fn select_d0fifo(&self, pipe: u8) -> Result<(), UsbError> {
        let regs = Self::regs();
        regs.d0fifosel.write(|w| unsafe { w.bits(pipe as u16) });
        for _ in 0..1_000 {
            if regs.d0fifosel.read().bits() & 0x000F == u16::from(pipe)
                && regs.d0fifoctr.read().bits() & CTR_FRDY != 0
            {
                return Ok(());
            }
        }
        Err(UsbError::WouldBlock)
    }

    /// Reads one received packet from the selected FIFO port into `buf`
    /// (caller pre-checks the length against DTLN). A zero-length
    /// packet cannot be read — clear it with BCLR instead (§27.3.7).
    /// Returns the packet length.
    fn fifo_read(&self, port: *mut u8, ctr: *mut u16, buf: &mut [u8]) -> usize {
        // Single-core, polled: no other context touches this FIFO
        // between the select in the caller and these accesses.
        let len = unsafe { ((ctr.read_volatile() & CTR_DTLN) as usize).min(buf.len()) };
        for b in buf[..len].iter_mut() {
            *b = unsafe { port.read_volatile() };
        }
        if len == 0 {
            unsafe { ctr.write_volatile(CTR_BCLR) };
        }
        len
    }

    /// Arms every allocated OUT pipe (PID = BUF) — used at enable and
    /// after bus reset. IN pipes stay NAK: [`UsbBus::write`] arms them
    /// per packet and [`UsbBus::poll`] disarms on completion, so each
    /// IN packet yields exactly one `ep_in_complete` event.
    fn arm_out_pipes(&self) {
        let map = self.ep_pipe.get();
        for &pipe in map[0].iter() {
            if pipe > 0 {
                self.pipe_ctr(pipe as u8)
                    .modify(|_, w| unsafe { w.bits(PID_BUF) });
            }
        }
    }

    /// Clears the BRDY/BEMP status flags selected by the masks
    /// (write-0-to-clear with 1s elsewhere, §27.2.15 note 1 / §27.2.17
    /// note 1; reserved bits stay 0).
    fn clear_pipe_status(&self, clear_brdy: u16, clear_bemp: u16) {
        let regs = Self::regs();
        regs.brdysts.write(|w| unsafe {
            w.bits(!(regs.brdysts.read().bits() & clear_brdy) & PIPE_STATUS_MASK)
        });
        regs.bempsts.write(|w| unsafe {
            w.bits(!(regs.bempsts.read().bits() & clear_bemp) & PIPE_STATUS_MASK)
        });
    }

    /// Bitmasks of allocated OUT resp. IN pipes (bit n = pipe n; bit 0
    /// = DCP, which carries both directions of the control endpoint).
    fn direction_masks(&self) -> (u16, u16) {
        let map = self.ep_pipe.get();
        let mut out = 1u16; // DCP OUT reception matters
        let mut inp = 1u16; // DCP IN completion matters
        for &pipe in map[0].iter().skip(1) {
            if pipe > 0 {
                out |= 1 << pipe;
            }
        }
        for &pipe in map[1].iter().skip(1) {
            if pipe > 0 {
                inp |= 1 << pipe;
            }
        }
        (out, inp)
    }
}

// Safety: every shared-reference UsbBus entry point below serializes
// its complete register/Cell transaction with the Cortex-M single-core
// critical-section provider. Endpoint handles may therefore be used
// from different interrupt contexts without interleaving FIFO selection
// or racing Cells. The adapter never exposes its Cells or register refs.
unsafe impl Sync for Usbfs {}

// Private, unlocked operations. Only the serialized trait entry points
// below call these after construction/allocation.
impl Usbfs {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        match ep_type {
            EndpointType::Control => {
                // DCP MXPS accepts only 8-byte steps up to 64
                // (§27.2.24 MXPS table); DEVSEL = 0 in device mode.
                if !matches!(max_packet_size, 8 | 16 | 32 | 64) {
                    return Err(UsbError::InvalidEndpoint);
                }
                // usb-device pins the control endpoint addresses (EP0
                // OUT/IN) in `UsbDevice::build` — exactly the mapping we
                // want, so a pinned index 0 is accepted (anything else
                // stays a caller error).
                if let Some(a) = ep_addr
                    && (a.index() != 0 || a.direction() != ep_dir)
                {
                    return Err(UsbError::InvalidEndpoint);
                }
                let mut map = self.ep_pipe.get();
                let direction = usize::from(ep_dir == UsbDirection::In);
                if map[direction][0] == 0 {
                    return Err(UsbError::InvalidEndpoint);
                }
                map[direction][0] = 0;
                self.ep_pipe.set(map);
                Self::regs()
                    .dcpmaxp
                    .write(|w| unsafe { w.bits(max_packet_size) });
                Ok(EndpointAddress::from_parts(0, ep_dir))
            }
            EndpointType::Bulk | EndpointType::Interrupt => {
                let is_in = ep_dir == UsbDirection::In;
                // Pipe classes (§27.2.27 TYPE table): 1–5 bulk, 6–9
                // interrupt-only. Allocation: the lowest free pipe of
                // the right class; endpoint number = pipe number.
                let (class_lo, class_hi) = match ep_type {
                    EndpointType::Bulk => (1u8, 5u8),
                    _ => (6u8, 9u8),
                };
                let map = self.ep_pipe.get();
                let free = |p: u8| !map[0].contains(&(p as i8)) && !map[1].contains(&(p as i8));
                let pipe = if let Some(address) = ep_addr {
                    let pipe = address.index() as u8;
                    if address.direction() != ep_dir
                        || !(class_lo..=class_hi).contains(&pipe)
                        || !free(pipe)
                    {
                        return Err(UsbError::InvalidEndpoint);
                    }
                    pipe
                } else {
                    (class_lo..=class_hi)
                        .find(|&p| free(p))
                        .ok_or(UsbError::EndpointOverflow)?
                };
                // MXPS limits (§27.2.29 MXPS[8:0] table).
                // Hardware pipe 1/2 can hold larger packets for other
                // transfer types, but full-speed USB bulk allows only
                // 8/16/32/64 (usb-device::UsbBusAllocator::bulk contract).
                let mps_ok = match ep_type {
                    EndpointType::Bulk => matches!(max_packet_size, 8 | 16 | 32 | 64),
                    _ => (1..=64).contains(&max_packet_size),
                };
                if !mps_ok {
                    return Err(UsbError::InvalidEndpoint);
                }
                let type_bits: u16 = match ep_type {
                    EndpointType::Bulk => 0b01 << 14,
                    _ => 0b10 << 14, // pipes 6–9: interrupt
                };
                let dir_bit: u16 = if is_in { 1 << 4 } else { 0 };
                let mut map2 = map;
                map2[usize::from(is_in)][pipe as usize] = pipe as i8;
                self.ep_pipe.set(map2);

                let regs = Self::regs();
                regs.pipesel.write(|w| unsafe { w.bits(pipe as u16) });
                // TYPE/EPNUM/DIR while PID = NAK (§27.2.27 note 1 — a
                // fresh pipe is NAK by reset).
                regs.pipecfg
                    .write(|w| unsafe { w.bits(type_bits | dir_bit | pipe as u16) });
                regs.pipemaxp.write(|w| unsafe { w.bits(max_packet_size) });
                regs.pipeperi.write(|w| unsafe { w.bits(0) });
                self.pipe_ctr(pipe)
                    .modify(|_, w| unsafe { w.bits(PID_NAK) });
                Ok(EndpointAddress::from_parts(pipe as usize, ep_dir))
            }
            // No class needs isochronous yet (pipes 1–2 would be the
            // hardware class, §27.3.12). Refuse rather than misconfigure.
            EndpointType::Isochronous { .. } => Err(UsbError::Unsupported),
        }
    }

    fn enable(&mut self) {
        // Connect: the D+ pull-up IS the attach signal (§27.3.1.3) —
        // drive it only once VBUS is sensed (bus-powered board).
        let regs = Self::regs();
        if regs.intsts0.read().bits() & INTSTS0_VBSTS != 0 {
            regs.syscfg
                .modify(|_, w| unsafe { w.bits(regs.syscfg.read().bits() | SYSCFG_DPRPU) });
            self.connected.set(true);
        }
        // OUT pipes listen from here; the host resets us soon after.
        self.arm_out_pipes();
    }

    fn reset(&self) {
        let regs = Self::regs();
        // Hardware resets PID/control state (§27.2.25). A VALID setup
        // observed now can already belong to the new enumeration: do
        // not discard it simply because software services reset late.
        regs.d0fifosel.write(|w| unsafe { w.bits(0) });
        regs.d1fifosel.write(|w| unsafe { w.bits(0) });
        for pipe in 1..=9 {
            if self
                .ep_pipe
                .get()
                .iter()
                .any(|map| map.contains(&(pipe as i8)))
            {
                // Initialize allocated buffers and DATA0 while the bus
                // reset holds PID NAK (§27.2.30, Table 27.9).
                let ctr = self.pipe_ctr(pipe);
                ctr.write(|w| unsafe { w.bits(1 << 9) });
                ctr.write(|w| unsafe { w.bits(1 << 8) });
            }
        }
        self.clear_pipe_status(PIPE_STATUS_MASK, PIPE_STATUS_MASK);
        self.last_dvsq.set(1); // we are in the default state now
        self.in_pending.set(0);
        self.status_in_pending.set(false);
        self.status_out_pending.set(false);
        self.status_out_ready.set(false);
        self.auto_address.set(false);
        self.arm_out_pipes();
    }

    fn set_device_address(&self, _addr: u8) {
        // The hardware auto-responds to a correct SET_ADDRESS and
        // applies the address itself (§27.3.9.2 (4)). A stale status
        // ZLP from the auto-response is discarded by the setup-path
        // BCLR in [`Self::read`].
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize, UsbError> {
        if ep_addr.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }
        let pipe = self.pipe_of(ep_addr)?;
        let regs = Self::regs();
        if pipe == 0 {
            #[cfg(feature = "usb-trace")]
            trace(b'W', buf);
            let ctsq = regs.intsts0.read().bits() & 7;
            // After software consumes the final control-OUT data packet,
            // CTSQ may still be 3: the host's next IN token advances it to
            // status stage 4. usb-device already asks us to acknowledge
            // that status here. CCPL, not a CFIFO ZLP, authorizes the
            // hardware response (§27.2.25 / §27.3.3.5 Figure 27.16).
            // Observed by the Slice 9 diagnostic rig (README Slice 9):
            // SET_LINE_CODING's seven bytes were
            // read at CTSQ=3, followed immediately by write(&[]).
            if control_status_in(ctsq, buf.is_empty(), self.auto_address.get()) {
                self.status_in_pending.set(true);
                if !self.auto_address.get() {
                    regs.dcpctr
                        .modify(|r, w| unsafe { w.bits((r.bits() & !3) | PID_BUF | DCP_CCPL) });
                }
                return Ok(0);
            }
            // DCP IN. One packet per write; the previous one is still
            // in flight until poll() reports its BEMP (and NAKs the
            // pipe again).
            if self.in_pending.get() & 1 != 0 {
                return Err(UsbError::WouldBlock);
            }
            let mps = (regs.dcpmaxp.read().bits() & 0x7F) as usize;
            if buf.len() > mps {
                return Err(UsbError::BufferOverflow);
            }
            let selected = self.select_cfifo(true);
            #[cfg(feature = "usb-trace")]
            if selected.is_err() {
                trace(b'E', &[]);
            }
            selected?;
            #[cfg(feature = "usb-trace")]
            trace(b'F', &[]);
            unsafe {
                CFIFOCTR.write_volatile(CTR_BCLR); // drop any stale content
            }
            // BCLR changes buffer ownership. Confirm FRDY again before
            // touching the FIFO (§27.2.4 / §27.2.6) — by polling FRDY
            // alone: rewriting FIFOSEL here would restart the selection
            // sequence and confound the measurement.
            let mut ready = false;
            for _ in 0..1_000 {
                if unsafe { CFIFOCTR.read_volatile() } & CTR_FRDY != 0 {
                    ready = true;
                    break;
                }
            }
            #[cfg(feature = "usb-trace")]
            trace(b'R', &[u8::from(ready)]);
            if !ready {
                return Err(UsbError::WouldBlock);
            }
            unsafe {
                // §27.2.4 Table 27.5: little-endian 16-bit FIFO access.
                let (pairs, remainder) = buf.as_chunks::<2>();
                for pair in pairs {
                    regs.cfifo().write(|w| w.bits(u16::from_le_bytes(*pair)));
                }
                if let Some(&byte) = remainder.first() {
                    regs.cfifol().write(|w| w.bits(byte));
                }
                if buf.len() < mps {
                    // Short packet: end writing explicitly (§27.3.7); a
                    // full packet auto-transmits on its last byte.
                    CFIFOCTR.write_volatile(CTR_BVAL);
                }
                regs.dcpctr
                    .modify(|_, w| w.bits(regs.dcpctr.read().bits() & !0b11 | PID_BUF));
            }
            self.in_pending.set(self.in_pending.get() | 1);
            #[cfg(feature = "usb-trace")]
            trace(b'T', &[]);
            Ok(buf.len())
        } else {
            let ctr = self.pipe_ctr(pipe);
            if self.in_pending.get() & (1 << pipe) != 0 {
                return Err(UsbError::WouldBlock);
            }
            regs.pipesel.write(|w| unsafe { w.bits(pipe as u16) });
            let mps = (regs.pipemaxp.read().bits() & 0x01FF) as usize;
            if buf.len() > mps {
                return Err(UsbError::BufferOverflow);
            }
            self.select_d0fifo(pipe)?;
            unsafe {
                D0FIFOCTR.write_volatile(CTR_BCLR);
                self.select_d0fifo(pipe)?;
                for &b in buf {
                    D0FIFO.write_volatile(b);
                }
                if buf.len() < mps {
                    D0FIFOCTR.write_volatile(CTR_BVAL);
                }
                ctr.modify(|_, w| w.bits(ctr.read().bits() & !0b11 | PID_BUF));
            }
            self.in_pending.set(self.in_pending.get() | (1 << pipe));
            Ok(buf.len())
        }
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize, UsbError> {
        if ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }
        let pipe = self.pipe_of(ep_addr)?;
        let regs = Self::regs();
        if pipe == 0 {
            let i0 = regs.intsts0.read().bits();
            if i0 & INTSTS0_VALID != 0 {
                // Setup packet (§27.3.9.2 (1)): the request registers
                // hold the 8 bytes; VALID must be cleared before the
                // response is built, and the FIFO buffer dropped (it
                // may hold a stale aborted transfer — e.g. the
                // hardware's auto-responded SET_ADDRESS status ZLP).
                if buf.len() < 8 {
                    return Err(UsbError::BufferOverflow);
                }
                let req = regs.usbreq.read().bits();
                let val = regs.usbval.read().bits().to_le_bytes();
                let idx = regs.usbindx.read().bits().to_le_bytes();
                let len = regs.usbleng.read().bits().to_le_bytes();
                // §27.2.19: BMREQUESTTYPE is b7:0; BREQUEST is b15:8.
                buf[0] = req as u8;
                buf[1] = (req >> 8) as u8;
                buf[2] = val[0];
                buf[3] = val[1];
                buf[4] = idx[0];
                buf[5] = idx[1];
                buf[6] = len[0];
                buf[7] = len[1];
                regs.intsts0
                    .write(|w| unsafe { w.bits(INTSTS0_CLEAR_VALID) });
                self.in_pending.set(self.in_pending.get() & !1);
                self.status_in_pending.set(false);
                self.status_out_pending.set(false);
                self.status_out_ready.set(false);
                self.auto_address
                    .set(req == 0x0500 && idx == [0, 0] && len == [0, 0]);
                // SETUP is in request registers, NOT the FIFO. DCP BCLR
                // is legal without FRDY (§27.2.6), unlike a FIFO read.
                regs.cfifosel.write(|w| unsafe { w.bits(0) });
                unsafe {
                    CFIFOCTR.write_volatile(CTR_BCLR);
                }
                self.clear_pipe_status(0x0001, 0x0001);
                #[cfg(feature = "usb-trace")]
                trace(b'S', &buf[..8]);
                return Ok(8);
            }
            if self.status_out_ready.replace(false) {
                return Ok(0);
            }
            // OUT data (control-write data stage or the status OUT).
            if regs.brdysts.read().bits() & 0x0001 == 0 {
                return Err(UsbError::WouldBlock);
            }
            self.select_cfifo(false)?;
            if (regs.cfifoctr.read().bits() & CTR_DTLN) as usize > buf.len() {
                return Err(UsbError::BufferOverflow);
            }
            self.clear_pipe_status(0x0001, 0x0000); // §27.2.15 note 2: before FIFO access
            let len = self.fifo_read(CFIFO, CFIFOCTR, buf);
            #[cfg(feature = "usb-trace")]
            trace(b'O', &buf[..len]);
            Ok(len)
        } else {
            if regs.brdysts.read().bits() & (1 << pipe) == 0 {
                return Err(UsbError::WouldBlock);
            }
            self.select_d0fifo(pipe)?;
            let bits = unsafe { D0FIFOCTR.read_volatile() };
            if (bits & CTR_DTLN) as usize > buf.len() {
                return Err(UsbError::BufferOverflow);
            }
            self.clear_pipe_status(1 << pipe, 0x0000);
            let len = self.fifo_read(D0FIFO, D0FIFOCTR, buf);
            Ok(len)
        }
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        let Ok(pipe) = self.pipe_of(ep_addr) else {
            return;
        };
        let regs = Self::regs();
        // Both control register layouts share PID/PBUSY/SQCLR. Their
        // addresses are §27.2.25 (DCP) and §27.2.30 (pipes 1..9).
        let address = if pipe == 0 {
            0x4009_0060
        } else {
            0x4009_0070 + 2 * usize::from(pipe - 1)
        };
        let ctr = address as *mut u16;
        let read = || unsafe { ctr.read_volatile() };
        let write = |bits| unsafe { ctr.write_volatile(bits) };
        let previous = read() & 3;
        if stalled {
            // §27.2.30: BUF -> STALL11; NAK -> STALL10.
            let pid = PID_STALL | (previous & 1);
            write((read() & !3) | pid);
            return;
        }
        // STALL11 -> STALL10 -> NAK, never directly to BUF (§27.2.30).
        if previous == 3 {
            write((read() & !3) | PID_STALL);
        }
        write(read() & !3);
        if pipe != 0 && previous & 2 != 0 {
            // USB CLEAR_FEATURE(ENDPOINT_HALT) also resets DATA0.
            for _ in 0..10_000 {
                if read() & (1 << 5) == 0 {
                    break;
                }
            }
            if read() & (1 << 5) != 0 {
                return;
            }
            write(read() | (1 << 8));
        }
        if ep_addr.is_out() && (pipe != 0 || regs.intsts0.read().bits() & 7 == CTSQ_WRITE_DATA) {
            write((read() & !3) | PID_BUF);
        }
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        let Ok(pipe) = self.pipe_of(ep_addr) else {
            return false;
        };
        let pid = if pipe == 0 {
            Self::regs().dcpctr.read().bits() & 0b11
        } else {
            self.pipe_ctr(pipe).read().bits() & 0b11
        };
        pid & 0b10 != 0
    }

    fn suspend(&self) {
        // Nothing to do: the hardware tracks suspension in DVSQ on its
        // own (§27.2.13), and the polled adapter keeps answering polls
        // (which return `Suspend` until resume). SOF-based suspend
        // bookkeeping (SOFCFG) stays out of scope — see `usb/mod.rs`.
    }

    fn resume(&self) {
        // Symmetric no-op: the DVSQ transition back to configured is
        // reported as [`PollResult::Resume`] on the next poll.
    }

    fn poll(&self) -> PollResult {
        let regs = Self::regs();
        let i0 = regs.intsts0.read().bits();
        let dvsq = ((i0 >> 4) & 0b111) as u8;
        let ctsq = i0 & 0b111;
        // The captured VBSTS/DVSQ remain valid for this poll. Clear the
        // VBUS/resume event latches so an optional bell can be rearmed.
        regs.intsts0
            .write(|w| unsafe { w.bits(!((1 << 15) | (1 << 14))) });

        // Lazy connect: VBUS may appear after enable() ran without it.
        if !self.connected.get() && i0 & INTSTS0_VBSTS != 0 {
            regs.syscfg
                .modify(|_, w| unsafe { w.bits(regs.syscfg.read().bits() | SYSCFG_DPRPU) });
            self.connected.set(true);
        }

        // A second reset can occur while already Default. Acknowledge
        // DVST and inspect its state, rather than relying only on a
        // DVSQ transition (§27.3.3.4, Figure 27.15).
        let device_event = i0 & INTSTS0_DVST != 0;
        if device_event {
            regs.intsts0.write(|w| unsafe { w.bits(!INTSTS0_DVST) });
        }
        if dvsq == 1 && (device_event || self.last_dvsq.get() != 1) {
            self.last_dvsq.set(1);
            return PollResult::Reset;
        }

        // Suspend = DVSQ enters 1xx; resume = back to configured (011).
        if dvsq >= 4 && self.last_dvsq.get() < 4 {
            self.last_dvsq.set(dvsq);
            return PollResult::Suspend;
        }
        if dvsq == 3 && self.last_dvsq.get() >= 4 {
            self.last_dvsq.set(dvsq);
            return PollResult::Resume;
        }
        // Arm class OUT endpoints after the host has configured us, not
        // just during the reset callback: a USB bus reset forces pipe PIDs
        // back to NAK (§27.2.30). The Slice 9 diagnostic rig observed pipe 1 still NAK
        // after enumeration, and Windows bulk writes timed out.
        if dvsq == 3 && self.last_dvsq.get() != 3 {
            self.arm_out_pipes();
        }
        self.last_dvsq.set(dvsq);

        let raw_brdy = regs.brdysts.read().bits();
        let raw_bemp = regs.bempsts.read().bits();
        #[cfg(feature = "usb-trace")]
        {
            let state = (i0 & 0x7F) | ((raw_brdy & 1) << 8) | ((raw_bemp & 1) << 9);
            if self.last_trace_state.replace(state) != state {
                trace(
                    b'P',
                    &[
                        raw_brdy as u8,
                        (raw_brdy >> 8) as u8,
                        raw_bemp as u8,
                        (raw_bemp >> 8) as u8,
                    ],
                );
            }
        }
        // ep_out: only allocated OUT pipes (IN pipes signal completion
        // through BEMP); ep_in_complete: allocated IN pipes.
        let (out_mask, in_mask) = self.direction_masks();
        let mut ep_out = raw_brdy & out_mask;
        let mut ep_in_complete = raw_bemp & in_mask & self.in_pending.get();
        let ep_setup = u16::from(i0 & INTSTS0_VALID != 0);

        // Status-stage ZLPs bypass the FIFO and BEMP (§27.3.3.3,
        // §27.3.9.2). Translate their idle transition for usb-device.
        if i0 & INTSTS0_CTRT != 0 {
            regs.intsts0.write(|w| unsafe { w.bits(!INTSTS0_CTRT) });
        }
        if ep_setup == 0 && ctsq == 0 {
            if self.status_in_pending.replace(false) {
                ep_in_complete |= 1;
            }
            if self.status_out_pending.replace(false) {
                self.status_out_ready.set(true);
            }
        }
        if self.status_out_ready.get() {
            ep_out |= 1;
        }

        // Finish the IN data packet before enabling its OUT status.
        if ep_in_complete & 1 != 0 {
            regs.dcpctr.modify(|r, w| unsafe { w.bits(r.bits() & !3) });
        }
        if ep_setup == 0 && ctsq == CTSQ_READ_STATUS {
            self.status_out_pending.set(true);
            regs.dcpctr
                .modify(|r, w| unsafe { w.bits((r.bits() & !3) | PID_BUF | DCP_CCPL) });
        }

        if ep_out != 0 || ep_in_complete != 0 || ep_setup != 0 {
            // Report BEMP once per transfer: clear what we report
            // (write-0-to-clear, §27.2.17 note 1) and NAK the pipe
            // until the class writes the next packet.
            if ep_in_complete != 0 {
                self.clear_pipe_status(0x0000, ep_in_complete);
                self.in_pending.set(self.in_pending.get() & !ep_in_complete);
                let map = self.ep_pipe.get();
                for &pipe in map[1].iter().skip(1) {
                    if pipe > 0 && ep_in_complete & (1 << pipe) != 0 {
                        self.pipe_ctr(pipe as u8)
                            .modify(|_, w| unsafe { w.bits(PID_NAK) });
                    }
                }
            }
            return PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            };
        }
        PollResult::None
    }
}

impl UsbBus for Usbfs {
    fn alloc_ep(
        &mut self,
        direction: UsbDirection,
        address: Option<EndpointAddress>,
        kind: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> Result<EndpointAddress, UsbError> {
        critical_section::with(|_| {
            self.alloc_ep(direction, address, kind, max_packet_size, interval)
        })
    }

    fn enable(&mut self) {
        critical_section::with(|_| Usbfs::enable(self));
    }

    fn reset(&self) {
        critical_section::with(|_| Usbfs::reset(self));
    }

    fn set_device_address(&self, address: u8) {
        critical_section::with(|_| Usbfs::set_device_address(self, address));
    }

    fn write(&self, endpoint: EndpointAddress, data: &[u8]) -> Result<usize, UsbError> {
        critical_section::with(|_| Usbfs::write(self, endpoint, data))
    }

    fn read(&self, endpoint: EndpointAddress, data: &mut [u8]) -> Result<usize, UsbError> {
        critical_section::with(|_| Usbfs::read(self, endpoint, data))
    }

    fn set_stalled(&self, endpoint: EndpointAddress, stalled: bool) {
        critical_section::with(|_| Usbfs::set_stalled(self, endpoint, stalled));
    }

    fn is_stalled(&self, endpoint: EndpointAddress) -> bool {
        critical_section::with(|_| Usbfs::is_stalled(self, endpoint))
    }

    fn suspend(&self) {
        critical_section::with(|_| Usbfs::suspend(self));
    }

    fn resume(&self) {
        critical_section::with(|_| Usbfs::resume(self));
    }

    fn poll(&self) -> PollResult {
        critical_section::with(|_| Usbfs::poll(self))
    }
}
