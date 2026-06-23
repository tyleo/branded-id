use crate::{Id, IdVec};

/// Borrowed views of the internal lists of an [`IdStruct`](super::IdStruct),
/// as returned by [`IdStruct::as_raw_parts`](super::IdStruct::as_raw_parts).
pub struct IdStructRawParts<'a, TId: Id> {
    /// The ids currently retained, packed.
    pub live: &'a [TId],
    /// Per-id: the index of the id in `live` plus 1. 0 means "not retained".
    pub live_index_plus_one: &'a IdVec<TId::Brand, usize>,
    /// Released ids available for reuse (LIFO).
    pub free: &'a [TId],
}
