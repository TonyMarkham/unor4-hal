//! The tick DTC's chained transfer-information records.

/// The most records one activation chains.
pub(crate) const MAX_RECORDS: usize = 4;

/// `MRB.CHNE` (bit 7): chain the next record into the same activation,
/// continuously (`CHNS = 0`) — manual §17.2.2, Table 17.3.
pub(crate) const MRB_CHAIN: u8 = 1 << 7;

/// Room for up to [`MAX_RECORDS`] chained transfer-information records
/// (manual §17.3.1, Figure 17.3). Each record is four words: word 0 = MRA
/// (byte 3) | MRB (byte 2), word 1 = SAR, word 2 = DAR, word 3 = CRA (upper
/// half, offset 0Eh) | CRB (lower half, offset 0Ch); a chained record
/// follows directly.
#[repr(C, align(16))]
pub(crate) struct TransferInfo(pub(crate) [[u32; 4]; MAX_RECORDS]);

/// One transfer-information record in the layout of Figure 17.3.
pub(crate) const fn record(mra: u8, mrb: u8, sar: u32, dar: u32, cra: u16, crb: u16) -> [u32; 4] {
    [
        ((mra as u32) << 24) | ((mrb as u32) << 16),
        sar,
        dar,
        ((cra as u32) << 16) | crb as u32,
    ]
}
