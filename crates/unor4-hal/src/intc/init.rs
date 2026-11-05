//! Startup cleanup of the bootloader's interrupt routes.

/// Discards the bootloader's interrupt routes before any HAL driver links
/// an event. RA4M1 manual R01UH0887EJ0100 §13.4.2 prohibits selecting the
/// same event in multiple IELSR registers, even when an old NVIC line is
/// disabled. The UNO R4 bootloader leaves SCI9 routed in slots 4–7.
pub(crate) fn init() {
    critical_section::with(|_| {
        let nvic = unsafe { &*cortex_m::peripheral::NVIC::PTR };
        let icu = unsafe { &*ra4m1::ICU::PTR };
        unsafe { nvic.icer[0].write(u32::MAX) };
        for slot in &icu.ielsr {
            slot.write(|w| unsafe { w.bits(0) });
        }
        unsafe { nvic.icpr[0].write(u32::MAX) };
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    });
}
