use crate::{IdVec, Scalar};

/// Borrowed views of the internal lists of an [`IdStruct`](super::IdStruct),
/// as returned by [`IdStruct::as_raw_parts`](super::IdStruct::as_raw_parts).
pub struct IdStructRawParts<'a, TBrand: ?Sized, TNum: Scalar> {
    /// The ids currently retained, packed.
    pub live: &'a [TNum::Id<TBrand>],

    /// Per-id: its index in the dense list (`live` followed by `free`),
    /// stored in the pool's integer width `TNum`.
    pub sparse: &'a IdVec<TBrand, TNum>,

    /// Released ids available for reuse; the next id to be recycled is first.
    pub free: &'a [TNum::Id<TBrand>],
}
