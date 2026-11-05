//! The global CPU interrupt unmask.

/// Unmasks CPU interrupts after startup and handler registration.
///
/// The UNO R4 bootloader leaves PRIMASK set, and cortex-m-rt does not
/// clear it. Call once from main after attaching interrupt-driven
/// peripherals. Enabling an individual IELSR slot does not unmask the CPU.
///
/// # Safety
/// Must not be called inside a critical section or any other region
/// whose correctness requires interrupts to remain globally disabled.
pub unsafe fn enable_interrupts() {
    unsafe { cortex_m::interrupt::enable() };
}
