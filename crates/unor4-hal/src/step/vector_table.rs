//! The DTC vector table the tick DTC points `DTCVBR` at.

/// The DTC vector table: one transfer-information address per IELSR slot
/// (vector number n at base + 4n, manual §17.3.1). The base must have its
/// low 10 bits zero (§17.2.8), hence the 1 KB alignment.
#[repr(C, align(1024))]
pub(crate) struct VectorTable(pub(crate) [u32; 32]);
