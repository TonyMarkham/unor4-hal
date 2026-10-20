//! Links the PAC's GPT channel markers to the shared register block.

mod private {
    /// Sealed supertrait: nothing outside this module can implement it,
    /// so the set of [`GptChannel`] types is closed — the chip's channel
    /// list is a hardware fact, not an extension point.
    pub trait Sealed {}
}

/// Marks a PAC peripheral that is one of the RA4M1's eight GPT channels.
///
/// The manual gives every channel the same register layout at
/// `0x4007_8000 + 0x0100 × m` (§22.2, Table 22.4), but the PAC splits
/// the channels across two identically-shaped block types: `GPT320`/
/// `GPT321` deref to `gpt320::RegisterBlock`, `GPT162`–`GPT167` to
/// `gpt162::RegisterBlock` (`ra4m1` 0.2.0). Both PAC blocks declare the
/// same registers in the same order with the same 32-bit access widths
/// (checked field-for-field in the generated sources: `gtwp` … `gtdvu`),
/// and every register this driver touches sits at the manual's Table
/// 22.4 offset in both — so, exactly like [`crate::uart::SciChannel`],
/// one generic driver works over a `gpt320`-typed view. GPT16's counters
/// and compare registers are 16-bit effective (manual §22.2.20: upper
/// bits read 0, writes ignored), which the shared u32-typed registers
/// express fine.
pub trait GptChannel: private::Sealed {
    /// Raw pointer to the channel's register block, typed as the
    /// `gpt320::RegisterBlock` layout (see the trait docs for why that
    /// typing is valid for every channel, GPT16 included).
    const PTR: *const ra4m1::gpt320::RegisterBlock;

    /// Takes the channel's half of the GPT out of module stop:
    /// `MSTPCRD.MSTPD6 = 0` for GPT162–GPT167, `MSTPCRD.MSTPD5 = 0` for
    /// GPT320–GPT321 (manual §10, MSTPCRD, p.175). `MSTP*` writes
    /// require `PRCR.PRC1` write-enable, same as the SCI releases.
    fn release_module_stop();

    /// A shared view of the channel's registers.
    ///
    /// Safe to call on a claimed channel: the peripheral token is
    /// consumed exactly once by a constructor, so no other code can
    /// reach the same registers, and the register block lives at its
    /// fixed address forever.
    fn block(&self) -> &'static ra4m1::gpt320::RegisterBlock {
        unsafe { &*Self::PTR }
    }
}

impl private::Sealed for ra4m1::GPT320 {}
impl private::Sealed for ra4m1::GPT321 {}
impl private::Sealed for ra4m1::GPT162 {}
impl private::Sealed for ra4m1::GPT163 {}
impl private::Sealed for ra4m1::GPT164 {}
impl private::Sealed for ra4m1::GPT165 {}
impl private::Sealed for ra4m1::GPT166 {}
impl private::Sealed for ra4m1::GPT167 {}

/// `PRCR` key + PRC1 write-enable — the same unlock the SCI
/// module-stop releases use (`MSTP*` registers are PRC1-protected).
const PRCR_UNLOCK_PRC1: u16 = 0xA502;
/// `PRCR` relock (permission bits cleared).
const PRCR_LOCK: u16 = 0xA500;

/// Takes GPT320–GPT321 out of module stop (`MSTPCRD.MSTPD5 = 0`).
/// Manual §10 "Low Power Modes", MSTPCRD, p.175: bit 5 targets
/// "GPT321 to GPT320". Addresses from the SVD: `MSTP` block at
/// `0x4004_7000`, `MSTPCRD` at offset `0x0C` (A/B/C/D at 0x00/04/08/0C).
pub(crate) fn release_module_stop_gpt32() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrd.modify(|_, w| w.mstpd5()._0());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}

/// Takes GPT162–GPT167 out of module stop (`MSTPCRD.MSTPD6 = 0`) — same
/// block and unlock as the GPT32 release. Manual §10, MSTPCRD, p.175:
/// bit 6 targets "GPT167 to GPT162"; the SVD field `mstpd6` matches.
pub(crate) fn release_module_stop_gpt16() {
    let system = unsafe { &*ra4m1::SYSTEM::PTR };
    let mstp = unsafe { &*ra4m1::MSTP::PTR };
    system.prcr.write(|w| unsafe { w.bits(PRCR_UNLOCK_PRC1) });
    mstp.mstpcrd.modify(|_, w| w.mstpd6()._0());
    system.prcr.write(|w| unsafe { w.bits(PRCR_LOCK) });
}

impl GptChannel for ra4m1::GPT320 {
    const PTR: *const ra4m1::gpt320::RegisterBlock = ra4m1::GPT320::PTR;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt32();
    }
}

impl GptChannel for ra4m1::GPT321 {
    const PTR: *const ra4m1::gpt320::RegisterBlock = ra4m1::GPT321::PTR;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt32();
    }
}

impl GptChannel for ra4m1::GPT162 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT162::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}

impl GptChannel for ra4m1::GPT163 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT163::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}

impl GptChannel for ra4m1::GPT164 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT164::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}

impl GptChannel for ra4m1::GPT165 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT165::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}

impl GptChannel for ra4m1::GPT166 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT166::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}

impl GptChannel for ra4m1::GPT167 {
    const PTR: *const ra4m1::gpt320::RegisterBlock =
        ra4m1::GPT167::PTR as *const ra4m1::gpt320::RegisterBlock;

    fn release_module_stop() {
        crate::pwm::release_module_stop_gpt16();
    }
}
