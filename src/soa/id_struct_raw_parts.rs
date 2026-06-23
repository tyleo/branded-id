use crate::{Id, IdVec};

/// Borrowed views of the internal lists of an [`IdStruct`](super::IdStruct),
/// as returned by [`IdStruct::as_raw_parts`](super::IdStruct::as_raw_parts).
pub struct IdStructRawParts<'a, TId: Id> {
    /// The ids currently retained, packed.
    pub live: &'a [TId],
    /// Per-id: its index in the dense list (`live` followed by `free`).
    pub sparse: &'a IdVec<TId::Brand, usize>,
    /// Released ids available for reuse; the next id to be recycled is first.
    pub free: &'a [TId],
}
